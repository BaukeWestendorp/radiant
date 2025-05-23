use crate::{
    show::{self, Show},
    ui::FRAME_CELL_SIZE,
};
use gpui::{
    App, Bounds, DragMoveEvent, Empty, Entity, EntityId, FocusHandle, Focusable, MouseButton,
    MouseUpEvent, Path, Pixels, Point, ReadGlobal, Size, Window, bounds, canvas, deferred, div,
    point, prelude::*, px, size,
};
use pool::{PoolFrame, PoolFrameKind, PresetPoolFrameKind};
use ui::{
    ActiveTheme,
    utils::{snap_point, z_stack},
};
use window::{WindowFrame, WindowFrameKind, graph_editor::GraphEditorFrame};

use super::GRID_SIZE;

mod pool;
mod window;

pub enum FrameKind {
    Window(Entity<WindowFrame>),
    Pool(Entity<PoolFrame>),
}

pub struct Frame {
    pub bounds: Bounds<u32>,
    pub kind: FrameKind,
    focus_handle: FocusHandle,

    pub resized_moved_bounds: Option<Bounds<u32>>,
}

impl Frame {
    pub fn handle_header_drag(
        &mut self,
        event: &DragMoveEvent<HeaderDrag>,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let HeaderDrag { frame_entity_id, start_mouse_position } = *event.drag(cx);
        if frame_entity_id != cx.entity_id() {
            return;
        };

        let grid_diff = mouse_grid_diff(event.event.position, start_mouse_position);

        let size = self.bounds.size.map(|d| d as i32);
        let origin = self.bounds.origin.map(|d| d as i32) + grid_diff;

        let bounds = limit_new_bounds(&bounds(origin, size));
        self.resized_moved_bounds = Some(bounds);
        cx.notify();
    }

    pub fn handle_resize_drag(
        &mut self,
        event: &DragMoveEvent<ResizeHandleDrag>,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ResizeHandleDrag { frame_entity_id, start_mouse_position } = *event.drag(cx);
        if frame_entity_id != cx.entity_id() {
            return;
        };

        let grid_diff = mouse_grid_diff(event.event.position, start_mouse_position);

        let size = size(
            self.bounds.size.width as i32 + grid_diff.x,
            self.bounds.size.height as i32 + grid_diff.y,
        );
        let origin = self.bounds.origin.map(|d| d as i32);

        let bounds = limit_new_bounds(&bounds(origin, size));
        self.resized_moved_bounds = Some(bounds);
        cx.notify();
    }

    pub fn release_resize_move(
        &mut self,
        _event: &MouseUpEvent,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(new_bounds) = self.resized_moved_bounds {
            self.bounds = new_bounds;
            self.resized_moved_bounds = None;
            cx.notify();
        };
    }

    fn render_resize_handle(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let id = "resize-handle";
        div()
            .id(id)
            .absolute()
            .right_0()
            .bottom_0()
            .size_4()
            .child(
                canvas(
                    |_, _, _| {},
                    |b, _, w, cx| {
                        let b = b + point(px(-1.0), px(-1.0));
                        let mut path = Path::new(b.bottom_left());
                        path.line_to(b.top_right());
                        path.line_to(b.bottom_right());
                        path.line_to(b.bottom_left());
                        w.paint_path(path, cx.theme().colors.border);
                    },
                )
                .size_full(),
            )
            .occlude()
            .on_drag(
                ResizeHandleDrag {
                    frame_entity_id: cx.entity_id(),
                    start_mouse_position: w.mouse_position(),
                },
                |_, _, _, cx| cx.new(|_| Empty),
            )
            .on_drag_move(cx.listener(Self::handle_resize_drag))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::release_resize_move))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::release_resize_move))
    }

    fn render_frame_content(
        &mut self,
        _w: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match &self.kind {
            FrameKind::Window(window_frame) => window_frame.clone().into_any_element(),
            FrameKind::Pool(pool_frame) => pool_frame.clone().into_any_element(),
        }
    }

    fn render_resize_move_highlight(
        &mut self,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match self.resized_moved_bounds {
            Some(bounds) => div()
                .w(FRAME_CELL_SIZE * dbg!(bounds).size.width as f32)
                .h(FRAME_CELL_SIZE * bounds.size.height as f32)
                .left(FRAME_CELL_SIZE * bounds.origin.x as f32)
                .top(FRAME_CELL_SIZE * bounds.origin.y as f32)
                .border_2()
                .border_color(cx.theme().colors.border_focused),
            None => div(),
        }
    }
}

fn allowed_bounds() -> Bounds<i32> {
    const GRID_BOUNDS: Bounds<i32> = Bounds {
        origin: Point { x: 0, y: 0 },
        size: Size { width: GRID_SIZE.width as i32, height: GRID_SIZE.height as i32 },
    };

    GRID_BOUNDS
}

fn limit_new_bounds(bounds: &Bounds<i32>) -> Bounds<u32> {
    let allowed_bounds = allowed_bounds();
    let mut bounds = allowed_bounds.intersect(&bounds);
    bounds.size = bounds.size.max(&size(1, 1));
    bounds.origin = bounds
        .origin
        .min(&point(GRID_SIZE.width as i32 - 1, GRID_SIZE.height as i32 - 1))
        .max(&point(-bounds.size.width, -bounds.size.height));

    bounds.map(|d| d as u32)
}

fn mouse_grid_diff(
    mouse_position: Point<Pixels>,
    start_mouse_position: Point<Pixels>,
) -> Point<i32> {
    let start_mouse_grid = snap_point(start_mouse_position, FRAME_CELL_SIZE);
    let mouse_cell_fract = mouse_position.map(|d| d % FRAME_CELL_SIZE);
    let mouse_diff = mouse_position - start_mouse_grid - mouse_cell_fract;
    mouse_diff.map(|d| (d / FRAME_CELL_SIZE) as i32)
}

impl Render for Frame {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let resize_handle = self.render_resize_handle(w, cx).into_any_element();
        let frame_content = self.render_frame_content(w, cx).into_any_element();

        let frame = div()
            .child(z_stack([frame_content.into_any_element(), resize_handle]).size_full())
            .absolute()
            .w(FRAME_CELL_SIZE * self.bounds.size.width as f32)
            .h(FRAME_CELL_SIZE * self.bounds.size.height as f32)
            .left(FRAME_CELL_SIZE * self.bounds.origin.x as f32)
            .top(FRAME_CELL_SIZE * self.bounds.origin.y as f32)
            .into_any_element();

        let resize_move_highlight = deferred(self.render_resize_move_highlight(w, cx));

        z_stack([frame, resize_move_highlight.into_any_element()])
    }
}

impl Frame {
    pub fn into_show(&self, cx: &App) -> show::Frame {
        let kind = match &self.kind {
            FrameKind::Window(window_frame) => {
                let window_frame = window_frame.read(cx);
                show::FrameKind::Window(match &window_frame.kind {
                    WindowFrameKind::EffectGraphEditor(graph_editor_frame) => {
                        let asset = &graph_editor_frame.read(cx).asset;
                        show::WindowFrameKind::EffectGraphEditor(asset.read(cx).id)
                    }
                })
            }
            FrameKind::Pool(pool_frame) => show::FrameKind::Pool(match &pool_frame.read(cx).kind {
                PoolFrameKind::EffectGraphs => show::PoolFrameKind::EffectGraphs,
                PoolFrameKind::FixtureGroups => show::PoolFrameKind::FixtureGroups,
                PoolFrameKind::Cues => show::PoolFrameKind::Cues,
                PoolFrameKind::Sequences => show::PoolFrameKind::Sequences,
                PoolFrameKind::Executors => show::PoolFrameKind::Executors,
                PoolFrameKind::Preset(kind) => match kind {
                    PresetPoolFrameKind::Dimmer => show::PoolFrameKind::DimmerPresets,
                    PresetPoolFrameKind::Position => show::PoolFrameKind::PositionPresets,
                    PresetPoolFrameKind::Gobo => show::PoolFrameKind::GoboPresets,
                    PresetPoolFrameKind::Color => show::PoolFrameKind::ColorPresets,
                    PresetPoolFrameKind::Beam => show::PoolFrameKind::BeamPresets,
                    PresetPoolFrameKind::Focus => show::PoolFrameKind::FocusPresets,
                    PresetPoolFrameKind::Control => show::PoolFrameKind::ControlPresets,
                    PresetPoolFrameKind::Shapers => show::PoolFrameKind::ShapersPresets,
                    PresetPoolFrameKind::Video => show::PoolFrameKind::VideoPresets,
                },
            }),
        };

        show::Frame { bounds: self.bounds, kind }
    }

    pub fn from_show(from: &show::Frame, w: &mut Window, cx: &mut Context<Self>) -> Self {
        let frame = cx.entity();
        let kind = match &from.kind {
            show::FrameKind::Window(kind) => match kind {
                show::WindowFrameKind::EffectGraphEditor(asset_id) => {
                    let editor_frame = cx.new(|cx| {
                        let asset = Show::global(cx).assets.effect_graphs.get(asset_id).unwrap();
                        GraphEditorFrame::new(asset.clone(), w, cx)
                    });

                    let window_frame = cx.new(|_| {
                        WindowFrame::new(WindowFrameKind::EffectGraphEditor(editor_frame), frame)
                    });

                    FrameKind::Window(window_frame)
                }
            },
            show::FrameKind::Pool(kind) => {
                let pool_frame_kind = match kind {
                    show::PoolFrameKind::EffectGraphs => PoolFrameKind::EffectGraphs,
                    show::PoolFrameKind::FixtureGroups => PoolFrameKind::FixtureGroups,
                    show::PoolFrameKind::Cues => PoolFrameKind::Cues,
                    show::PoolFrameKind::Sequences => PoolFrameKind::Sequences,
                    show::PoolFrameKind::Executors => PoolFrameKind::Executors,
                    show::PoolFrameKind::DimmerPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Dimmer)
                    }
                    show::PoolFrameKind::PositionPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Position)
                    }
                    show::PoolFrameKind::GoboPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Gobo)
                    }
                    show::PoolFrameKind::ColorPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Color)
                    }
                    show::PoolFrameKind::BeamPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Beam)
                    }
                    show::PoolFrameKind::FocusPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Focus)
                    }
                    show::PoolFrameKind::ControlPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Control)
                    }
                    show::PoolFrameKind::ShapersPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Shapers)
                    }
                    show::PoolFrameKind::VideoPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Video)
                    }
                };

                let pool_frame = cx.new(|_| PoolFrame::new(pool_frame_kind, frame));
                FrameKind::Pool(pool_frame)
            }
        };

        Self {
            kind,
            bounds: from.bounds,
            focus_handle: cx.focus_handle(),
            resized_moved_bounds: None,
        }
    }
}

impl Focusable for Frame {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

pub struct ResizeHandleDrag {
    frame_entity_id: EntityId,
    start_mouse_position: Point<Pixels>,
}

pub struct HeaderDrag {
    frame_entity_id: EntityId,
    start_mouse_position: Point<Pixels>,
}

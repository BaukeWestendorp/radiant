use gpui::{
    App, Bounds, Deferred, DismissEvent, Div, DragMoveEvent, Empty, Entity, EntityId, MouseButton,
    MouseDownEvent, MouseUpEvent, Pixels, Point, ReadGlobal, Size, Window, anchored, bounds,
    deferred, div, point, prelude::*, size,
};
use ui::ActiveTheme;
use ui::utils::{bounds_updater, snap_point, z_stack};

use super::Frame;
use crate::show::{self, Show};
use crate::ui::{FRAME_CELL_SIZE, FrameSelector};

pub const GRID_SIZE: Size<u32> = Size { width: 18, height: 12 };

#[derive(Debug, Clone)]
pub struct Page {
    label: String,
    frames: Vec<Entity<Frame>>,
    frame_selector: Option<(Entity<FrameSelector>, Point<Pixels>)>,

    bounds: Bounds<Pixels>,

    pub frame_bounds_highlight: Entity<FrameBoundsHighlight>,
    should_open_frame_selector: bool,
    state: FrameInteractionState,
}

impl Page {
    pub fn new(label: String, cx: &mut App) -> Self {
        Self {
            label,
            frames: Vec::new(),
            frame_selector: None,

            bounds: Bounds::default(),

            frame_bounds_highlight: cx.new(|_cx| FrameBoundsHighlight::new()),
            should_open_frame_selector: false,
            state: FrameInteractionState::default(),
        }
    }

    pub fn add_frame(&mut self, frame: Entity<Frame>, cx: &mut Context<Self>) {
        cx.observe(&frame, |_, _, cx| cx.notify()).detach();
        self.frames.push(frame);
        cx.notify();
    }

    pub fn remove_frame(&mut self, frame_id: EntityId, cx: &mut Context<Self>) {
        self.frames.retain(|frame| frame.entity_id() != frame_id);
        cx.notify();
    }

    fn open_frame_selector(
        &mut self,
        position: Point<Pixels>,
        frame_bounds: Bounds<u32>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let page = cx.entity();
        let frame_selector = cx.new(|cx| FrameSelector::new(page, frame_bounds, window, cx));

        cx.subscribe(&frame_selector, |this, _, _: &DismissEvent, cx| {
            this.close_frame_selector(cx);
            cx.notify();
        })
        .detach();

        self.frame_selector = Some((frame_selector, position));
        cx.notify();
    }

    fn close_frame_selector(&mut self, cx: &mut Context<Self>) {
        self.frame_selector = None;
        self.finish_interaction(cx);
    }
}

impl Page {
    fn render_background(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        let grid = ui::dot_grid(FRAME_CELL_SIZE, cx.theme().colors.grid)
            .w(GRID_SIZE.width as f32 * FRAME_CELL_SIZE)
            .h(GRID_SIZE.height as f32 * FRAME_CELL_SIZE);

        z_stack([
            grid.into_any_element(),
            div()
                .id("page_background")
                .size_full()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, event: &MouseDownEvent, _, cx| {
                        this.start_new_frame(event.position, cx);
                    }),
                )
                .on_drag(cx.entity_id(), |_, _, _, cx| cx.new(|_| Empty))
                .on_drag_move(cx.listener(move |this, event: &DragMoveEvent<EntityId>, _, cx| {
                    if event.drag(cx) != &cx.entity_id() {
                        return;
                    }

                    this.drag_new_frame(event.event.position, cx);
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, event: &MouseUpEvent, window, cx| {
                        this.end_new_frame(event.position, window, cx);
                    }),
                )
                .into_any_element(),
        ])
        .size_full()
        .cursor_crosshair()
    }

    fn render_frame_selector(&mut self, cx: &mut Context<Self>) -> Option<Deferred> {
        self.frame_selector.as_ref().map(|(frame_selector, position)| {
            let selector = div()
                .w_128()
                .h_72()
                .on_mouse_down_out(cx.listener(|this, _, _window, cx| {
                    cx.stop_propagation();
                    this.close_frame_selector(cx);
                }))
                .child(frame_selector.clone());

            deferred(anchored().position(*position).child(selector)).with_priority(1)
        })
    }
}

impl Render for Page {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let background = self.render_background(window, cx).into_any_element();
        let frames = z_stack(self.frames.clone()).size_full().into_any_element();
        let frame_selector = self.render_frame_selector(cx).map(|f| f.into_any_element());
        let frame_bounds_highlight = self.frame_bounds_highlight.clone().into_any_element();
        let bounds_updater = bounds_updater(cx.entity(), |this, bounds, _cx| this.bounds = bounds)
            .into_any_element();

        div().size_full().child(
            z_stack(
                [
                    Some(background),
                    Some(frames),
                    frame_selector,
                    Some(frame_bounds_highlight),
                    Some(bounds_updater),
                ]
                .into_iter()
                .flatten(),
            )
            .size_full(),
        )
    }
}

impl Page {
    pub fn start_new_frame(&mut self, mouse_position: Point<Pixels>, cx: &mut Context<Self>) {
        match self.state {
            FrameInteractionState::Finished => {
                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    let origin = mouse_position.map(|d| (d / FRAME_CELL_SIZE) as u32);
                    fbh.show(bounds(origin, size(1, 1)), cx);
                    self.should_open_frame_selector = true;
                });

                self.state =
                    FrameInteractionState::StartNew { start_mouse_position: mouse_position };

                cx.notify();
            }
            _ => {}
        }
    }

    pub fn drag_new_frame(&mut self, mouse_position: Point<Pixels>, cx: &mut Context<Self>) {
        match self.state {
            FrameInteractionState::StartNew { start_mouse_position }
            | FrameInteractionState::DraggingNew { start_mouse_position } => {
                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    fbh.resize(start_mouse_position, mouse_position, cx);
                });

                self.state = FrameInteractionState::DraggingNew { start_mouse_position };

                cx.notify();
            }
            _ => {}
        }
    }

    pub fn end_new_frame(
        &mut self,
        mouse_position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.state {
            FrameInteractionState::StartNew { .. } | FrameInteractionState::DraggingNew { .. } => {
                let limited_bounds =
                    self.frame_bounds_highlight.update(cx, |fbh, _cx| fbh.limited_bounds());

                if let Some(limited_bounds) = limited_bounds {
                    if self.should_open_frame_selector {
                        self.open_frame_selector(mouse_position, limited_bounds, window, cx);
                    }
                }

                self.frame_bounds_highlight.update(cx, |fbh, cx| fbh.clear(cx));

                self.state = FrameInteractionState::EndNew;

                cx.notify();
            }
            _ => {}
        }
    }

    pub fn start_move_frame(
        &mut self,
        frame: &Entity<Frame>,
        mouse_position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        match self.state {
            FrameInteractionState::Finished => {
                let frame_bounds = frame.read(cx).bounds;
                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    fbh.show(frame_bounds, cx);
                });

                self.state = FrameInteractionState::StartMove {
                    start_mouse_position: mouse_position,
                    frame_entity_id: frame.entity_id(),
                };

                cx.notify();
            }
            _ => {}
        }
    }

    pub fn drag_move_frame(&mut self, mouse_position: Point<Pixels>, cx: &mut Context<Self>) {
        match self.state {
            FrameInteractionState::StartMove { start_mouse_position, frame_entity_id }
            | FrameInteractionState::DraggingMove { start_mouse_position, frame_entity_id } => {
                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    fbh.move_to(start_mouse_position, mouse_position, cx);
                });
                self.state =
                    FrameInteractionState::DraggingMove { start_mouse_position, frame_entity_id };
                cx.notify();
            }
            _ => {}
        }
    }

    pub fn end_move_frame(&mut self, frame: &Entity<Frame>, cx: &mut Context<Self>) {
        match self.state {
            FrameInteractionState::StartMove { frame_entity_id, .. }
            | FrameInteractionState::DraggingMove { frame_entity_id, .. } => {
                if frame.entity_id() != frame_entity_id {
                    return;
                }

                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    if fbh.is_allowed(cx) {
                        if let Some(limited_bounds) = fbh.limited_bounds() {
                            frame.update(cx, |frame, cx| {
                                frame.bounds = limited_bounds;
                                cx.notify();
                            });
                        }
                    }
                    fbh.hide(cx);
                });

                self.finish_interaction(cx);
            }
            _ => {}
        }
    }

    pub fn start_resize_frame(
        &mut self,
        frame_bounds: Bounds<u32>,
        frame_entity_id: EntityId,
        mouse_position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        match self.state {
            FrameInteractionState::Finished => {
                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    fbh.show(frame_bounds, cx);
                });

                self.state = FrameInteractionState::StartResize {
                    start_mouse_position: mouse_position,
                    frame_entity_id,
                };

                cx.notify();
            }
            state => {
                log::warn!("State machine in invalid state in `start_resize_frame`: {state:?}")
            }
        }
    }

    pub fn drag_resize_frame(&mut self, mouse_position: Point<Pixels>, cx: &mut Context<Self>) {
        match self.state {
            FrameInteractionState::StartResize { start_mouse_position, frame_entity_id }
            | FrameInteractionState::DraggingResize { start_mouse_position, frame_entity_id } => {
                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    fbh.resize(start_mouse_position, mouse_position, cx);
                });

                cx.notify();

                self.state =
                    FrameInteractionState::DraggingResize { start_mouse_position, frame_entity_id }
            }
            _ => {}
        }
    }

    pub fn end_resize_frame(&mut self, frame: &Entity<Frame>, cx: &mut Context<Self>) {
        match self.state {
            FrameInteractionState::StartResize { frame_entity_id, .. }
            | FrameInteractionState::DraggingResize { frame_entity_id, .. } => {
                if frame.entity_id() != frame_entity_id {
                    return;
                }

                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    if fbh.is_allowed(cx) {
                        if let Some(limited_bounds) = fbh.limited_bounds() {
                            frame.update(cx, |frame, cx| {
                                frame.bounds = limited_bounds;
                                cx.notify();
                            });
                        }
                    }
                    fbh.hide(cx);
                });

                self.finish_interaction(cx);
            }
            _ => {}
        }
    }

    pub fn finish_interaction(&mut self, cx: &mut Context<Self>) {
        match self.state {
            FrameInteractionState::EndNew => {
                self.state = FrameInteractionState::Finished;

                self.frame_bounds_highlight.update(cx, |fbh, cx| {
                    fbh.hide(cx);
                    self.should_open_frame_selector = false;
                });

                cx.notify();
            }
            FrameInteractionState::StartMove { .. }
            | FrameInteractionState::DraggingMove { .. }
            | FrameInteractionState::StartResize { .. }
            | FrameInteractionState::DraggingResize { .. } => {
                self.state = FrameInteractionState::Finished;
                cx.notify();
            }

            state => {
                log::warn!("State machine in invalid state in `finish_interaction`: {state:?}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum FrameInteractionState {
    #[default]
    Finished,

    StartNew {
        start_mouse_position: Point<Pixels>,
    },
    DraggingNew {
        start_mouse_position: Point<Pixels>,
    },
    EndNew,

    StartMove {
        start_mouse_position: Point<Pixels>,
        frame_entity_id: EntityId,
    },
    DraggingMove {
        start_mouse_position: Point<Pixels>,
        frame_entity_id: EntityId,
    },

    StartResize {
        start_mouse_position: Point<Pixels>,
        frame_entity_id: EntityId,
    },
    DraggingResize {
        start_mouse_position: Point<Pixels>,
        frame_entity_id: EntityId,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct FrameBoundsHighlight {
    pub initial_bounds: Option<Bounds<u32>>,
    pub active_bounds: Option<Bounds<i32>>,
    is_visible: bool,
}

impl FrameBoundsHighlight {
    pub fn new() -> Self {
        Self { initial_bounds: None, active_bounds: None, is_visible: false }
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.initial_bounds = None;
        self.active_bounds = None;
        cx.notify();
    }

    pub fn show(&mut self, initial_bounds: Bounds<u32>, cx: &mut Context<Self>) {
        self.initial_bounds = Some(initial_bounds);
        self.active_bounds = Some(initial_bounds.map(|d| d as i32));
        self.is_visible = true;
        cx.notify();
    }

    pub fn hide(&mut self, cx: &mut Context<Self>) {
        self.is_visible = false;
        cx.notify();
    }

    pub fn limited_bounds(&mut self) -> Option<Bounds<u32>> {
        self.limit_active_bounds();
        self.active_bounds.map(|b| b.map(|d| d as u32))
    }

    fn limit_active_bounds(&mut self) {
        const GRID_BOUNDS: Bounds<i32> = Bounds {
            origin: Point { x: 0, y: 0 },
            size: Size { width: GRID_SIZE.width as i32, height: GRID_SIZE.height as i32 },
        };

        let Some(active_bounds) = &mut self.active_bounds else { return };

        *active_bounds = GRID_BOUNDS.intersect(&active_bounds);

        active_bounds.size = active_bounds.size.max(&size(1, 1));
        active_bounds.origin = active_bounds
            .origin
            .min(&point(GRID_BOUNDS.size.width - 1, GRID_BOUNDS.size.height - 1))
            .max(&point(-active_bounds.size.width, -active_bounds.size.height));
    }

    pub fn is_allowed(&self, cx: &App) -> bool {
        let Some(initial_bounds) = self.initial_bounds else { return false };
        let Some(active_bounds) = self.active_bounds else { return false };

        let frames = &Show::global(cx).layout.read(cx).main_window.loaded_page.frames;
        !frames
            .iter()
            .filter(|frame| frame.bounds != initial_bounds)
            .any(|frame| frame.bounds.map(|d| d as i32).intersects(&active_bounds))
    }

    pub fn move_to(
        &mut self,
        start_mouse_position: Point<Pixels>,
        mouse_position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let Some(active_bounds) = &mut self.active_bounds else { return };
        let Some(initial_bounds) = &mut self.initial_bounds else { return };

        let grid_diff = mouse_grid_diff(start_mouse_position, mouse_position);
        let new_origin = initial_bounds.origin.map(|d| d as i32) + grid_diff;

        active_bounds.origin = new_origin;
        active_bounds.size = initial_bounds.size.map(|d| d as i32);
        self.limit_active_bounds();
        cx.notify();
    }

    pub fn resize(
        &mut self,
        start_mouse_position: Point<Pixels>,
        mouse_position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let Some(active_bounds) = &mut self.active_bounds else { return };
        let Some(initial_bounds) = &mut self.initial_bounds else { return };

        let grid_diff = mouse_grid_diff(start_mouse_position, mouse_position);
        let new_size = size(
            initial_bounds.size.width as i32 + grid_diff.x,
            initial_bounds.size.height as i32 + grid_diff.y,
        );

        active_bounds.size = new_size;
        self.limit_active_bounds();
        cx.notify();
    }
}

fn mouse_grid_diff(
    start_mouse_position: Point<Pixels>,
    mouse_position: Point<Pixels>,
) -> Point<i32> {
    let start_mouse_grid = snap_point(start_mouse_position, FRAME_CELL_SIZE);
    let mouse_cell_fract = mouse_position.map(|d| d % FRAME_CELL_SIZE);
    let mouse_diff = mouse_position - start_mouse_grid - mouse_cell_fract;
    mouse_diff.map(|d| (d / FRAME_CELL_SIZE) as i32)
}

impl Render for FrameBoundsHighlight {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_bounds {
            Some(bounds) if self.is_visible() => {
                let border_color = if self.is_allowed(cx) {
                    cx.theme().colors.border_focused
                } else {
                    cx.theme().colors.border
                };

                div()
                    .w(FRAME_CELL_SIZE * bounds.size.width as f32)
                    .h(FRAME_CELL_SIZE * bounds.size.height as f32)
                    .left(FRAME_CELL_SIZE * bounds.origin.x as f32)
                    .top(FRAME_CELL_SIZE * bounds.origin.y as f32)
                    .border_1()
                    .border_color(border_color)
                    .bg(cx.theme().colors.highlight)
            }
            _ => div(),
        }
    }
}

impl Page {
    pub fn into_show(&self, cx: &App) -> show::Page {
        show::Page {
            label: self.label.clone(),
            frames: self
                .frames
                .clone()
                .into_iter()
                .map(|frame| frame.read(cx).into_show(cx))
                .collect(),
        }
    }

    pub fn from_show(
        layout: &Entity<show::Layout>,
        window: &mut Window,
        cx: &mut Context<Page>,
    ) -> Self {
        let loaded_page = layout.read(cx).main_window.loaded_page.clone();
        let mut page = Self::new(loaded_page.label.clone(), cx);

        for frame in &loaded_page.frames.clone() {
            let page_view = cx.entity();
            let frame = cx.new(|cx| Frame::from_show(frame, cx.entity(), page_view, window, cx));
            cx.observe(&frame, |_, _, cx| cx.notify()).detach();
            page.add_frame(frame, cx);
        }

        page
    }
}

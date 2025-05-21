use crate::ui::FRAME_CELL_SIZE;
use crate::{layout::page, show::effect_graph};
use gpui::{
    App, Bounds, DragMoveEvent, Empty, Entity, Focusable, MouseButton, MouseUpEvent, Pixels, Point,
    Window, div, point, prelude::*, size,
};
use graph_editor::GraphEditorFrame;
use ui::{ActiveTheme, ContainerStyle, InteractiveColor, container, h6};

use super::Frame;

pub mod graph_editor;

pub struct WindowFrame {
    frame: Entity<Frame>,
    pub kind: WindowFrameKind,

    pub is_dragging: bool,
}

impl WindowFrame {
    pub fn new(kind: WindowFrameKind, frame: Entity<Frame>) -> Self {
        Self { kind, frame, is_dragging: false }
    }
}

impl WindowFrame {
    fn title(&self) -> &str {
        match &self.kind {
            WindowFrameKind::EffectGraphEditor(_) => "Effect Graphs Editor",
        }
    }

    fn render_header(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title().to_string();

        let border_color = if self.frame.focus_handle(cx).contains_focused(w, cx) {
            cx.theme().colors.border_focused
        } else {
            cx.theme().colors.header_border
        };

        let background_color = if self.is_dragging {
            cx.theme().colors.header_background.lighten(0.25)
        } else {
            cx.theme().colors.header_background
        };

        let bounds = self.frame.read(cx).bounds;
        let mouse_position = w.mouse_position();

        div()
            .id("window_header")
            .w_full()
            .h(FRAME_CELL_SIZE / 2.0)
            .on_drag((bounds, mouse_position), |_, _, _, cx| cx.new(|_| Empty))
            .on_drag_move(cx.listener(Self::handle_on_drag_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_on_mouse_up))
            .child(
                container(ContainerStyle {
                    background: background_color,
                    border: border_color,
                    text_color: cx.theme().colors.text,
                })
                .size_full()
                .p_2()
                .flex()
                .items_center()
                .child(h6(title)),
            )
    }

    fn render_content(&mut self, w: &mut Window, cx: &mut App) -> impl IntoElement {
        let content = match &self.kind {
            WindowFrameKind::EffectGraphEditor(frame) => frame.clone(),
        };

        container(ContainerStyle::normal(w, cx)).size_full().child(content)
    }
}

impl WindowFrame {
    fn handle_on_drag_move(
        &mut self,
        event: &DragMoveEvent<(Bounds<u32>, Point<Pixels>)>,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (start_cell_bounds, start_mouse_position) = event.drag(cx);
        let start_frame_origin =
            point(start_cell_bounds.origin.x as i32, start_cell_bounds.origin.y as i32);

        let mouse_position = event.event.position;

        let mouse_diff = point(
            mouse_position.x - start_mouse_position.x,
            mouse_position.y - start_mouse_position.y,
        );

        let cell_diff =
            point((mouse_diff.x / FRAME_CELL_SIZE) as i32, (mouse_diff.y / FRAME_CELL_SIZE) as i32);

        let new_frame_origin = start_frame_origin + cell_diff;

        let new_size = size(
            (start_cell_bounds.size.width as i32 - cell_diff.x).min(page::GRID_SIZE.width as i32),
            (start_cell_bounds.size.height as i32 - cell_diff.y).min(page::GRID_SIZE.height as i32),
        );

        self.frame.update(cx, move |frame, cx| {
            frame.bounds.origin =
                point(new_frame_origin.x.max(0) as u32, new_frame_origin.y.max(0) as u32);
            frame.bounds.size = size(new_size.width.max(1) as u32, new_size.height.max(1) as u32);
            cx.notify();
        });

        self.is_dragging = true;
        cx.notify();
    }

    fn handle_on_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_dragging = false;
        cx.notify();
    }
}

impl Render for WindowFrame {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.render_header(w, cx))
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .size_full()
                    .p_px()
                    .child(self.render_content(w, cx)),
            )
            .overflow_hidden()
    }
}

pub enum WindowFrameKind {
    EffectGraphEditor(Entity<GraphEditorFrame<effect_graph::Def>>),
}

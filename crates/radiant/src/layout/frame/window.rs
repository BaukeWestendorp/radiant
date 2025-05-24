use crate::show::effect_graph;
use crate::ui::FRAME_CELL_SIZE;
use gpui::{App, Empty, Entity, Focusable, MouseButton, Window, div, prelude::*};
use graph_editor::GraphEditorFrame;
use ui::{ActiveTheme, ContainerStyle, container, h6};

use super::Frame;

pub mod graph_editor;

pub struct WindowFrame {
    frame: Entity<Frame>,
    pub kind: WindowFrameKind,
}

impl WindowFrame {
    pub fn new(kind: WindowFrameKind, frame: Entity<Frame>) -> Self {
        Self { kind, frame }
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

        div()
            .id("window_header")
            .w_full()
            .h(FRAME_CELL_SIZE / 2.0)
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, event, w, cx| {
                    this.frame.update(cx, |frame, cx| {
                        frame.handle_right_mouse_click_header(event, w, cx)
                    });
                }),
            )
            .on_drag(
                super::HeaderDrag {
                    frame_entity_id: self.frame.entity_id(),
                    start_mouse_position: w.mouse_position(),
                },
                |_, _, _, cx| cx.new(|_| Empty),
            )
            .on_drag_move(cx.listener(|this, event, w, cx| {
                this.frame.update(cx, |frame, cx| frame.handle_header_drag(event, w, cx));
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, event, w, cx| {
                    this.frame.update(cx, |frame, cx| frame.release_resize_move(event, w, cx));
                }),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, event, w, cx| {
                    this.frame.update(cx, |frame, cx| frame.release_resize_move(event, w, cx));
                }),
            )
            .child(
                container(ContainerStyle {
                    background: cx.theme().colors.header_background,
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

use gpui::{
    div, px, rgb, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::{layout::LAYOUT_CELL_SIZE, ui::uniform_grid::uniform_grid};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Window {
    pub kind: WindowKind,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WindowKind {
    ColorPresetPool,
}

impl WindowKind {
    pub fn window_title(&self) -> &str {
        match self {
            WindowKind::ColorPresetPool => "Color Preset Pool",
        }
    }

    pub fn show_header(&self) -> bool {
        match self {
            WindowKind::ColorPresetPool => false,
        }
    }
}

pub struct WindowView {
    window: Model<Window>,
}

impl WindowView {
    pub fn build(window: Model<Window>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| {
            let this = Self { window };

            this
        })
    }

    fn render_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let window = self.window.read(cx);

        div()
            .child(window.kind.window_title().to_string())
            .flex()
            .items_center()
            .w_full()
            .px_3()
            .h_10()
            .bg(rgb(0x222280))
            .border_color(rgb(0x0000ff))
            .border_1()
            .rounded_t_md()
    }

    fn render_content(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let window = self.window.read(cx);
        match window.kind {
            WindowKind::ColorPresetPool => uniform_grid(
                cx.view().clone(),
                "pool_items",
                window.cols,
                window.rows,
                |view, range, cx| {
                    let mut items = Vec::new();
                    dbg!(&range);
                    for i in range {
                        items.push(
                            div()
                                // FIXME: We have to set the size here because for now grid can't calculate them from the rows and columns...
                                .w(px(LAYOUT_CELL_SIZE as f32))
                                .h(px(LAYOUT_CELL_SIZE as f32))
                                .child(
                                    div()
                                        .bg(rgb(0x202020))
                                        .border_color(rgb(0x303030))
                                        .border_1()
                                        .rounded_md()
                                        .w_full()
                                        .h_full()
                                        .child(format!("Item {}", i)),
                                ),
                        );
                    }
                    items
                },
            ),
        }
    }
}

impl Render for WindowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = div()
            .bg(rgb(0x202020))
            .rounded_b_md()
            .child(self.render_content(cx));

        let window = self.window.read(cx);

        let header = match window.kind.show_header() {
            true => Some(self.render_header(cx)),
            false => None,
        };

        div()
            .children(header)
            .child(content)
            .flex()
            .flex_col()
            .w(px(
                self.window.read(cx).cols as f32 * LAYOUT_CELL_SIZE as f32
            ))
            .h(px(
                self.window.read(cx).rows as f32 * LAYOUT_CELL_SIZE as f32
            ))
    }
}

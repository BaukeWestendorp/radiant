pub mod effect_graph_editor_frame;
pub mod frame_grid;

pub use effect_graph_editor_frame::*;
pub use frame_grid::*;

use gpui::*;
use ui::theme::ActiveTheme;

pub struct Frame<D: FrameDelegate> {
    delegate: D,
}

impl<D: FrameDelegate + 'static> Frame<D> {
    pub fn build(delegate: D, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { delegate })
    }
}

impl<D: FrameDelegate + 'static> Render for Frame<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .children(self.delegate.render_titlebar(cx))
            .child(self.delegate.render_content(cx))
    }
}

pub trait FrameDelegate {
    fn title(&mut self, cx: &mut ViewContext<Frame<Self>>) -> &str
    where
        Self: Sized;

    fn render_titlebar(&mut self, cx: &mut ViewContext<Frame<Self>>) -> Option<impl IntoElement>
    where
        Self: Sized,
    {
        let title = self.title(cx).to_string();

        Some(
            div()
                .w_full()
                .h(px(GRID_SIZE / 2.0))
                .bg(cx.theme().window_header_color)
                .border_1()
                .border_color(black().opacity(0.4))
                .rounded(cx.theme().radius)
                .px_2()
                .child(
                    div()
                        .size_full()
                        .flex()
                        .items_center()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(title),
                ),
        )
    }

    fn render_content(&mut self, cx: &mut ViewContext<Frame<Self>>) -> impl IntoElement
    where
        Self: Sized;
}

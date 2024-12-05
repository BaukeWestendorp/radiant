pub mod cue_editor;
pub mod effect_graph_editor;
pub mod grid;
pub mod pool;

pub use cue_editor::*;
pub use effect_graph_editor::*;
pub use grid::*;
pub use pool::*;

use gpui::*;
use show::Frame;
use ui::{theme::ActiveTheme, StyledExt};

pub trait FrameDelegate {
    fn init(&mut self, _cx: &mut ViewContext<FrameView<Self>>)
    where
        Self: Sized,
    {
    }

    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String
    where
        Self: Sized;

    fn render_header(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> Option<impl IntoElement>
    where
        Self: Sized,
    {
        let title = self.title(cx).to_string();
        let header_content = self.render_header_content(cx).into_element();

        Some(
            div()
                .w_full()
                .h(px(GRID_SIZE / 2.0))
                .bg(cx.theme().frame_header_background)
                .border_color(cx.theme().frame_header_border)
                .rounded(cx.theme().radius)
                .px_2()
                .child(
                    div()
                        .size_full()
                        .h_flex()
                        .justify_between()
                        .text_sm()
                        .text_color(cx.theme().frame_header_text_color)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(title)
                        .child(header_content),
                ),
        )
    }

    fn render_header_content(&mut self, _cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        div().size_full()
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement
    where
        Self: Sized;
}

pub struct FrameView<D: FrameDelegate> {
    frame: Frame,
    delegate: D,
}

impl<D: FrameDelegate + 'static> FrameView<D> {
    pub fn build(frame: Frame, mut delegate: D, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            delegate.init(cx);
            Self { frame, delegate }
        })
    }
}

impl<D: FrameDelegate + 'static> Render for FrameView<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let bounds = self.frame.bounds;
        div()
            .absolute()
            .size_full()
            .v_flex()
            .bg(cx.theme().background)
            .rounded(cx.theme().radius)
            .w(px(bounds.size.width as f32 * GRID_SIZE))
            .h(px(bounds.size.height as f32 * GRID_SIZE))
            .left(px(bounds.origin.x as f32 * GRID_SIZE))
            .top(px(bounds.origin.y as f32 * GRID_SIZE))
            .shadow_sm()
            .children(self.delegate.render_header(cx))
            .child(self.delegate.render_content(cx))
    }
}

pub mod frame_grid;
pub mod test_frame;

pub use frame_grid::*;
pub use test_frame::*;

use gpui::*;

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
        div().size_full().child(self.delegate.render(cx))
    }
}

pub trait FrameDelegate {
    fn title(&mut self, cx: &mut ViewContext<Frame<Self>>) -> &str
    where
        Self: Sized;

    fn render(&mut self, cx: &mut ViewContext<Frame<Self>>) -> impl IntoElement
    where
        Self: Sized;
}

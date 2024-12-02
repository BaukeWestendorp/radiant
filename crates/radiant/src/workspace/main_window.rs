use gpui::*;
use show::Show;
use ui::theme::ActiveTheme;

use super::frame::FrameGridView;

pub struct MainWindow {
    frame_grid: View<FrameGridView>,
}

impl MainWindow {
    pub fn build(show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            frame_grid: FrameGridView::build(show, cx),
        })
    }
}

impl Render for MainWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().text)
            .text_size(cx.theme().font_size)
            .font_family(cx.theme().font_family.clone())
            .child(self.frame_grid.clone())
    }
}

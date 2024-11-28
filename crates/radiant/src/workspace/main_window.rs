use gpui::*;
use ui::theme::ActiveTheme;

use super::frame::FrameGrid;

pub struct MainWindow {
    frame_grid: View<FrameGrid>,
}

impl MainWindow {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            frame_grid: FrameGrid::build(cx),
        })
    }
}

impl Render for MainWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .text_size(cx.theme().font_size)
            .font_family(cx.theme().font_family.clone())
            .child(self.frame_grid.clone())
    }
}

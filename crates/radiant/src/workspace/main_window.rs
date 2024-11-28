use gpui::*;
use ui::theme::ActiveTheme;

pub struct MainWindow {}

impl MainWindow {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_| Self {})
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
            .child("main_window")
    }
}

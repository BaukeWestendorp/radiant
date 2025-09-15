use gpui::prelude::*;
use gpui::{App, ClickEvent, FontWeight, Window, WindowHandle, div, px};
use ui::interactive::button::button;

pub struct MainWindow {}

impl MainWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options(), |window, cx| cx.new(|cx| Self::new(window, cx)))
            .expect("should open main window")
    }

    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn render_titlebar_content(&self, cx: &Context<Self>) -> impl IntoElement {
        let settings_button =
            button("settings", None, "=").on_click(cx.listener(Self::handle_click_settings_button));

        div()
            .size_full()
            .flex()
            .justify_between()
            .items_center()
            .child(div().font_weight(FontWeight::BOLD).pb(px(-2.0)).child("Radiant"))
            .child(settings_button)
    }

    fn handle_click_settings_button(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::window_root()
            .child(ui::utils::todo(cx))
            .titlebar_child(self.render_titlebar_content(cx))
    }
}

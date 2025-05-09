use gpui::{App, FontWeight, Window, div, prelude::*, px};
use main::FRAME_CELL_SIZE;
use ui::{ActiveTheme, ContainerStyle, interactive_container};

pub mod main;
pub mod settings;

pub const DEFAULT_REM_SIZE: gpui::Pixels = gpui::px(14.0);

pub struct VirtualWindow<D: VirtualWindowDelegate> {
    pub delegate: D,
}

impl<D: VirtualWindowDelegate> VirtualWindow<D> {
    pub fn new(delegate: D) -> Self {
        Self { delegate }
    }
}

impl<D: VirtualWindowDelegate + 'static> Render for VirtualWindow<D> {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let style = ContainerStyle::normal(w, cx);
        let content = div()
            .size_full()
            .bg(style.background)
            .border_color(style.border)
            .rounded_b(cx.theme().radius)
            .border_1()
            .border_t_0()
            .occlude()
            .child(self.delegate.render_content(w, cx));
        let header = self.delegate.render_header(w, cx);

        div().flex().flex_col().child(header).child(content).size_full()
    }
}

pub trait VirtualWindowDelegate {
    fn title(&self, cx: &App) -> &str;

    fn on_close_window(&mut self, _w: &mut Window, _cx: &mut Context<VirtualWindow<Self>>)
    where
        Self: Sized;

    fn render_header(
        &mut self,
        _w: &mut Window,
        cx: &mut Context<VirtualWindow<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized + 'static,
    {
        let close_button = interactive_container("close-button", None)
            .on_click(cx.listener(|vw, _, w, cx| vw.delegate.on_close_window(w, cx)))
            .cursor_pointer()
            .px_2()
            .py(px(2.0))
            .child("Close");

        div()
            .bg(cx.theme().colors.header_background)
            .border_color(cx.theme().colors.header_border)
            .rounded_t(cx.theme().radius)
            .border_1()
            .w_full()
            .h(FRAME_CELL_SIZE / 2.0)
            .flex()
            .justify_between()
            .items_center()
            .px_2()
            .child(div().font_weight(FontWeight::BOLD).child(self.title(cx).to_string()))
            .child(close_button)
    }

    fn render_content(
        &mut self,
        w: &mut Window,
        cx: &mut Context<VirtualWindow<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}

use crate::ui::FRAME_CELL_SIZE;
use gpui::{App, Focusable, SharedString, Window, div, prelude::*, px};
use ui::{ActiveTheme, ContainerStyle, h6, interactive_container};

pub struct VirtualWindow<D: VirtualWindowDelegate> {
    pub delegate: D,
}

impl<D: VirtualWindowDelegate> VirtualWindow<D> {
    pub fn new(delegate: D) -> Self {
        Self { delegate }
    }
}

impl<D: VirtualWindowDelegate + 'static> Render for VirtualWindow<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let style = ContainerStyle::normal(window, cx);

        let focus_handle = &self.delegate.focus_handle(cx);

        let content = div()
            .size_full()
            .bg(style.background)
            .border_color(style.border)
            .rounded_b(cx.theme().radius)
            .border_1()
            .border_t_0()
            .child(self.delegate.render_content(window, cx));

        let header = self.delegate.render_header(window, cx);

        div()
            .occlude()
            .track_focus(focus_handle)
            .flex()
            .flex_col()
            .child(header)
            .child(content)
            .size_full()
    }
}

pub trait VirtualWindowDelegate: Focusable {
    fn title(&self, cx: &App) -> SharedString;

    fn show_close_button(&self) -> bool {
        true
    }

    fn on_close_window(&mut self, _window: &mut Window, _cx: &mut Context<VirtualWindow<Self>>)
    where
        Self: Sized,
    {
    }

    fn render_header(
        &mut self,
        window: &mut Window,
        cx: &mut Context<VirtualWindow<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized + 'static,
    {
        let focused = self.focus_handle(cx).contains_focused(window, cx);

        let close_button = interactive_container("close-button", None)
            .destructive(true)
            .on_click(cx.listener(|vw, _, w, cx| vw.delegate.on_close_window(w, cx)))
            .cursor_pointer()
            .px_2()
            .py(px(2.0))
            .child("Close");

        div()
            .bg(cx.theme().colors.header_background)
            .border_color(cx.theme().colors.header_border)
            .when(focused, |e| e.border_color(cx.theme().colors.border_focused))
            .rounded_t(cx.theme().radius)
            .border_1()
            .w_full()
            .h(FRAME_CELL_SIZE / 2.0)
            .flex()
            .justify_between()
            .items_center()
            .px_2()
            .child(h6(self.title(cx).to_string()))
            .children(if self.show_close_button() { Some(close_button) } else { None })
    }

    fn render_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<VirtualWindow<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}

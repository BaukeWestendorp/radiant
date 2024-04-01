use gpui::{
    div, AppContext, FocusHandle, FocusableView, IntoElement, ParentElement, Render, SharedString,
    Styled, View, ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;
use ui::button::Button;

use crate::window::{Window, WindowDelegate};

pub struct Workspace {
    window: View<Window<TestWindowDelegate>>,
    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            window: {
                let delegate = TestWindowDelegate {};
                Window::build(delegate, cx)
            },
            focus_handle: cx.focus_handle(),
        })
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_color(cx.theme().colors().text)
            .font("Zed Sans")
            .child(self.window.clone())
    }
}

struct TestWindowDelegate {}

impl WindowDelegate for TestWindowDelegate {
    fn title(&self, _cx: &mut ViewContext<Window<Self>>) -> Option<SharedString> {
        Some("Test Window".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<Window<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        div().text_color(gpui::red()).child("helo world")
    }

    fn header_items(&mut self, cx: &mut ViewContext<Window<Self>>) -> Vec<impl IntoElement>
    where
        Self: Sized,
    {
        let close_button = Button::new("close_button")
            .on_click(cx.listener(|this, _event, _cx| {
                this.close();
            }))
            .flex()
            .justify_center()
            .items_center()
            .w_10()
            .child("X");

        vec![close_button]
    }
}

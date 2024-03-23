use gpui::{
    div, AppContext, FocusHandle, FocusableView, InteractiveElement, IntoElement, ParentElement,
    Render, Styled, ViewContext, VisualContext, WindowHandle, WindowOptions,
};

pub struct Workspace {
    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn open_window(cx: &mut AppContext) -> WindowHandle<Self> {
        let window_options = WindowOptions::default();

        cx.open_window(window_options, |cx| {
            cx.new_view(|cx| Self {
                focus_handle: cx.focus_handle(),
            })
        })
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("Workspace")
            .font("Zed Sans")
            .text_color(gpui::white())
            .size_full()
            .child("> Fixture 42")
    }
}

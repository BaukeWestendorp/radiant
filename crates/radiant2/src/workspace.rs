use gpui::{
    div, AppContext, FocusHandle, FocusableView, IntoElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use theme::ActiveTheme;

pub struct Workspace {
    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
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
    }
}

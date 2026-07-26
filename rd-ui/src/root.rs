use gpui::{AnyView, FocusHandle, Window, div};
use gpui::{App, Focusable, prelude::*};

use crate::{ActiveTheme, z_stack};

pub(crate) mod action {
    pub const KEY_CONTEXT: &str = "Root";

    gpui::actions!([Tab, TabPrev, Edit]);
}

pub struct Root {
    view: AnyView,

    focus_handle: FocusHandle,
}

impl Root {
    pub fn new(view: impl Into<AnyView>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { view: view.into(), focus_handle: cx.focus_handle() }
    }

    pub fn view(&self) -> &AnyView {
        &self.view
    }

    fn handle_action_tab(&mut self, _: &action::Tab, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }

    fn handle_action_tab_prev(
        &mut self,
        _: &action::TabPrev,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
    }
}

impl Focusable for Root {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.set_rem_size(cx.theme().font_size);

        let content = self.view.clone();

        div()
            .id("root")
            .track_focus(&self.focus_handle)
            .key_context(action::KEY_CONTEXT)
            .on_action(cx.listener(Self::handle_action_tab))
            .on_action(cx.listener(Self::handle_action_tab_prev))
            .relative()
            .size_full()
            .bg(cx.theme().bg_primary)
            .text_color(cx.theme().fg_primary)
            .child(
                z_stack([
                    content.into_any_element(),
                    crate::popup::render_overlay(window, cx).into_any_element(),
                ])
                .size_full(),
            )
    }
}

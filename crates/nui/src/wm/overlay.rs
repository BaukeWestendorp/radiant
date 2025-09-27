use gpui::prelude::*;
use gpui::{
    AnyView, App, ElementId, FocusHandle, Focusable, FontWeight, KeyBinding, SharedString, Window,
    div,
};

use crate::AppExt;
use crate::button::button;
use crate::theme::{ActiveTheme, InteractiveColor};

mod actions {
    pub const KEY_CONTEXT: &str = "Overlay";

    gpui::actions!(text_input, [Close]);
}

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", actions::Close, Some(actions::KEY_CONTEXT))]);
}

#[derive(Debug, Clone)]
pub struct Overlay {
    id: String,
    title: SharedString,
    content: AnyView,
    is_modal: bool,

    focus_handle: FocusHandle,
}

impl Overlay {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<SharedString>,
        content: impl Into<AnyView>,
        focus_handle: FocusHandle,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
            is_modal: false,
            focus_handle,
        }
    }

    pub fn as_modal(mut self) -> Self {
        self.is_modal = true;
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &AnyView {
        &self.content
    }

    pub fn is_modal(&self) -> bool {
        self.is_modal
    }

    fn handle_close(&mut self, _: &actions::Close, window: &mut Window, cx: &mut Context<Self>) {
        cx.update_wm(|wm, _| wm.close_overlay(self.id(), window));
    }
}

impl Render for Overlay {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = div()
            .when(!self.is_modal(), |e| e.size_full())
            .min_h_8()
            .max_h_8()
            .px_2()
            .flex()
            .justify_between()
            .items_center()
            .border_1()
            .border_color(cx.theme().header_border)
            .rounded_t(cx.theme().radius)
            .text_color(cx.theme().header_foreground)
            .bg(cx.theme().header)
            .child(div().font_weight(FontWeight::BOLD).child(self.title().to_string()))
            .child(button("close", None, "X").size_6().on_click(
                cx.listener(|this, _, window, cx| this.handle_close(&actions::Close, window, cx)),
            ));

        let content = div()
            .when(!self.is_modal(), |e| e.size_full())
            .flex()
            .bg(cx.theme().background)
            .border_1()
            .border_t_0()
            .border_color(cx.theme().border)
            .rounded_b(cx.theme().radius)
            .when(cx.theme().shadow, |e| e.shadow_lg())
            .child(self.content().clone());

        let container = div()
            .track_focus(&self.focus_handle)
            .key_context(actions::KEY_CONTEXT)
            .on_action(cx.listener(Self::handle_close))
            .when(!self.is_modal(), |e| e.size_full())
            .flex()
            .flex_col()
            .occlude()
            .child(header)
            .child(content);

        div()
            .id(ElementId::Name(self.id.clone().into()))
            .on_click(
                cx.listener(|this, _, window, cx| this.handle_close(&actions::Close, window, cx)),
            )
            .size_full()
            .p_4()
            .flex()
            .justify_center()
            .items_center()
            .bg(gpui::black().with_opacity(0.5))
            .occlude()
            .child(container)
    }
}

impl Focusable for Overlay {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

pub(super) struct Modal {
    pub content: AnyView,
}

impl Render for Modal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().w_96().flex().justify_center().items_center().p_2().child(self.content.clone())
    }
}

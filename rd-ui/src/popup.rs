use gpui::{
    AnyView, App, Entity, EventEmitter, FocusHandle, Focusable, Global, ReadGlobal, SharedString,
    UpdateGlobal, Window, div, prelude::*,
};

use crate::{
    ActiveTheme, Emphasis, StyledExt, c_flex,
    comp::{Button, ButtonVariant, IconVariant, Labelled, stateful},
    h_flex, v_flex,
};

pub(crate) fn init(cx: &mut App) {
    cx.set_global(PopupGlobal::default());
}

pub(crate) mod action {
    pub const KEY_CONTEXT: &str = "Popup";

    gpui::actions!(popup, [Dismiss]);
}

pub struct InputPopup<T, Input>
where
    T: 'static,
    Input: Render + EventEmitter<stateful::event::Change<T>>,
{
    input: Entity<Input>,

    _marker: std::marker::PhantomData<T>,
}

impl<T, Input> InputPopup<T, Input>
where
    T: 'static,
    Input: Render + Focusable + EventEmitter<stateful::event::Change<T>>,
{
    pub fn new(input: Entity<Input>, window: &mut Window, cx: &mut App) -> Self {
        input.focus_handle(cx).focus(window, cx);

        Self { input, _marker: std::marker::PhantomData }
    }

    pub fn with_on_change(
        self,
        window: &mut Window,
        cx: &mut App,
        on_change: impl Fn(&T, &mut Window, &mut App) + 'static,
    ) -> Self {
        window
            .subscribe(&self.input, cx, move |_, event: &stateful::event::Change<T>, window, cx| {
                let value = &event.0;
                (on_change)(value, window, cx)
            })
            .detach();
        self
    }
}

impl<T, Input> InputPopup<T, Input>
where
    T: 'static,
    Input: Render
        + EventEmitter<stateful::event::Change<T>>
        + EventEmitter<stateful::event::Submit<T>>,
{
    pub fn with_on_submit(
        self,
        window: &mut Window,
        cx: &mut App,
        on_submit: impl Fn(&T, &mut Window, &mut App) + 'static,
    ) -> Self {
        window
            .subscribe(&self.input, cx, move |_, event: &stateful::event::Submit<T>, window, cx| {
                let value = &event.0;
                (on_submit)(value, window, cx)
            })
            .detach();
        self
    }
}

impl<T, Input> Render for InputPopup<T, Input>
where
    T: 'static,
    Input: Render + EventEmitter<stateful::event::Change<T>>,
{
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().items_center().gap_2().size_full().p_2().child(self.input.clone())
    }
}

pub trait PopupAppExt {
    fn set_popup(
        &mut self,
        title: impl Into<SharedString>,
        popup: impl Into<AnyView>,
        return_focus_handle: Option<FocusHandle>,
    );

    fn dismiss_popup(&mut self, window: &mut Window);
}

impl PopupAppExt for App {
    fn set_popup(
        &mut self,
        title: impl Into<SharedString>,
        popup: impl Into<AnyView>,
        return_focus_handle: Option<FocusHandle>,
    ) {
        PopupGlobal::update_global(self, |popup_global, _| {
            popup_global.title = Some(title.into());
            popup_global.content = Some(popup.into());
            popup_global.return_focus_handle = return_focus_handle;
        })
    }

    fn dismiss_popup(&mut self, window: &mut Window) {
        PopupGlobal::update_global(self, |popup_global, cx| {
            popup_global.title = None;
            popup_global.content = None;
            if let Some(focus_handle) = popup_global.return_focus_handle.take() {
                focus_handle.focus(window, cx);
            }
        })
    }
}

pub(crate) struct PopupOverlay {}

impl PopupOverlay {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe_global::<PopupGlobal>(|_, cx| {
            cx.notify();
        })
        .detach();

        Self {}
    }
}

impl Render for PopupOverlay {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = PopupGlobal::global(cx).title.clone().unwrap_or_default();

        let Some(content) = PopupGlobal::global(cx).content.clone() else {
            return gpui::Empty.into_any_element();
        };

        let header = h_flex()
            .px_2()
            .py_1()
            .w_full()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().border_primary)
            .bg(cx.theme().bg_secondary)
            .font_bold()
            .child(div().text_color(cx.theme().fg_secondary).child(title))
            .child(
                Button::new("close", window, cx)
                    .with_variant(ButtonVariant::Ghost)
                    .with_icon(IconVariant::X)
                    .with_action(action::Dismiss)
                    .text_color(cx.theme().indicate.danger),
            );

        c_flex()
            .id("popup")
            .key_context(action::KEY_CONTEXT)
            .occlude()
            .bg(cx.theme().contrast.opacity(0.25))
            .size_full()
            .on_action::<action::Dismiss>(cx.listener(|_, _, window, cx| cx.dismiss_popup(window)))
            .child(
                v_flex()
                    .emphasis_bordered(Emphasis::Primary, cx)
                    .min_w_72()
                    .child(header)
                    .child(content)
                    .on_mouse_down_out(cx.listener(|_, _, window, cx| cx.dismiss_popup(window))),
            )
            .into_any_element()
    }
}

#[derive(Default)]
struct PopupGlobal {
    pub title: Option<SharedString>,
    pub content: Option<AnyView>,
    pub return_focus_handle: Option<FocusHandle>,
}

impl Global for PopupGlobal {}

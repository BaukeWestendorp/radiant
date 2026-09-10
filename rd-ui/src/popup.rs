use gpui::{
    AnyView, App, Entity, EventEmitter, FocusHandle, Focusable, Global, ReadGlobal, SharedString,
    UpdateGlobal, Window, div, prelude::*,
};

use crate::{
    ActiveTheme, Emphasis, StyledExt, c_flex,
    comp::{
        Button, ButtonVariant, Disableable, IconVariant, Labelled, TITLE_BAR_HEIGHT,
        stateful::{InputEvent, InputValue, Submittable},
    },
    h_flex, v_flex,
};

pub(crate) fn init(cx: &mut App) {
    cx.set_global(PopupGlobal::default());
}

pub(crate) mod action {
    pub const KEY_CONTEXT: &str = "Popup";

    gpui::actions!(popup, [Dismiss]);
}

struct Popup {
    pub title: SharedString,
    pub content: AnyView,
    pub size: PopupSize,
    pub return_focus_handle: Option<FocusHandle>,
}

pub struct InputPopup<T, Input>
where
    T: 'static,
    Input: Render + EventEmitter<InputEvent<T>>,
{
    input: Entity<Input>,
    _marker: std::marker::PhantomData<T>,
}

impl<T, Input> InputPopup<T, Input>
where
    T: 'static,
    Input: Render + Focusable + EventEmitter<InputEvent<T>>,
{
    pub fn new(input: Entity<Input>, window: &mut Window, cx: &mut App) -> Self {
        input.focus_handle(cx).focus(window, cx);

        Self { input, _marker: std::marker::PhantomData }
    }

    pub fn with_on_change<E: 'static>(
        self,
        window: &mut Window,
        cx: &mut Context<E>,
        on_change: impl Fn(&InputValue<T>, &mut Window, &mut Context<E>) + 'static,
    ) -> Self {
        cx.subscribe_in(&self.input, window, move |_, _, event: &InputEvent<T>, window, cx| {
            match event {
                InputEvent::Change(value) => (on_change)(value, window, cx),
                _ => {}
            }
        })
        .detach();
        self
    }
}

impl<T, Input> InputPopup<T, Input>
where
    T: 'static,
    Input: Render + Submittable<T> + EventEmitter<InputEvent<T>>,
{
    pub fn with_on_submit<E: 'static>(
        self,
        window: &mut Window,
        cx: &mut Context<E>,
        on_submit: impl Fn(&T, &mut Window, &mut Context<E>) + 'static,
    ) -> Self {
        cx.subscribe_in(&self.input, window, move |_, _, event: &InputEvent<T>, window, cx| {
            match event {
                InputEvent::Submit(value) => {
                    (on_submit)(value, window, cx);
                    cx.pop_popup(window);
                }
                _ => {}
            }
        })
        .detach();
        self
    }
}

impl<T, Input> Render for InputPopup<T, Input>
where
    T: 'static,
    Input: Render + Submittable<T>,
{
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().items_center().gap_2().size_full().p_2().child(self.input.clone()).child(
            Button::new("submit", window, cx)
                .w_full()
                .with_label("Submit")
                .with_disabled(!self.input.read(cx).value(cx).is_valid(), cx)
                .on_click(cx.listener(|this, _, _window, cx| {
                    this.input.update(cx, |input, cx| input.submit(cx))
                })),
        )
    }
}

pub trait PopupAppExt {
    fn push_popup(
        &mut self,
        title: impl Into<SharedString>,
        popup: impl Into<AnyView>,
        size: PopupSize,
        return_focus_handle: Option<FocusHandle>,
    );

    fn pop_popup(&mut self, window: &mut Window);
}

impl PopupAppExt for App {
    fn push_popup(
        &mut self,
        title: impl Into<SharedString>,
        popup: impl Into<AnyView>,
        size: PopupSize,
        return_focus_handle: Option<FocusHandle>,
    ) {
        PopupGlobal::update_global(self, |popup_global, _| {
            popup_global.popups.push(Popup {
                title: title.into(),
                content: popup.into(),
                size,
                return_focus_handle,
            });
        })
    }

    fn pop_popup(&mut self, window: &mut Window) {
        PopupGlobal::update_global(self, |popup_global, cx| {
            let popped = popup_global.popups.pop();
            if let Some(focus_handle) = popped.and_then(|p| p.return_focus_handle) {
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
        let Some(popup) = PopupGlobal::global(cx).popups.last() else {
            return gpui::Empty.into_any_element();
        };

        let title = popup.title.clone();
        let content = popup.content.clone();
        let size = popup.size;

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
            .size_full()
            .occlude()
            .p_4()
            .bg(cx.theme().contrast.opacity(0.25))
            .on_action::<action::Dismiss>(cx.listener(|_, _, window, cx| cx.pop_popup(window)))
            .child(
                v_flex()
                    .mt(TITLE_BAR_HEIGHT)
                    .emphasis_bordered(Emphasis::Primary, cx)
                    .when(size == PopupSize::Auto, |e| e.min_w_72())
                    .when(size == PopupSize::Max, |e| e.size_full())
                    .child(header)
                    .child(content)
                    .on_mouse_down_out(cx.listener(|_, _, window, cx| cx.pop_popup(window))),
            )
            .into_any_element()
    }
}

#[derive(Default)]
struct PopupGlobal {
    pub popups: Vec<Popup>,
}

impl Global for PopupGlobal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopupSize {
    #[default]
    Auto,
    Max,
}

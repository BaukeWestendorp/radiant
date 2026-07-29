use std::{collections::HashMap, rc::Rc};

use gpui::{
    AnyView, AnyWindowHandle, App, BoxShadow, Context, Entity, Focusable, FontWeight, Global,
    IntoElement, ReadGlobal, SharedString, Styled, Window, div, hsla, point, prelude::*, px,
};

use crate::{ActiveTheme, Button, InputDelegate, InputEvent, h_flex, input::InputState, v_flex};

pub(crate) fn init(cx: &mut App) {
    let popup_stacks = cx.new(|_| HashMap::new());
    cx.set_global(PopupGlobal { popup_stacks });
}

pub trait PopupAppExt {
    fn open_popup<F: FnOnce(&mut Window, &mut App) -> Popup>(
        &mut self,
        window: &mut Window,
        popup_builder: F,
    );

    fn close_popup(&mut self, window: &mut Window);
}

impl PopupAppExt for App {
    fn open_popup<F: FnOnce(&mut Window, &mut App) -> Popup>(
        &mut self,
        window: &mut Window,
        popup_builder: F,
    ) {
        let popup = (popup_builder)(window, self);
        let popup_view = self.new(|_| popup);
        PopupGlobal::global(self).popup_stacks.clone().update(self, |stacks, cx| {
            stacks.entry(window.window_handle()).or_default().push(popup_view);
            cx.notify();
        });
    }

    fn close_popup(&mut self, window: &mut Window) {
        PopupGlobal::global(self).popup_stacks.clone().update(self, |stacks, cx| {
            if let Some(stack) = stacks.get_mut(&window.window_handle()) {
                if let Some(popped) = stack.pop() {
                    popped.update(cx, |popped, cx| {
                        if let Some(on_close) = popped.on_close.take() {
                            on_close(window, cx);
                        }
                    });
                }
            }
            cx.notify();
        });
    }
}

pub(crate) struct PopupGlobal {
    pub popup_stacks: Entity<HashMap<AnyWindowHandle, Vec<Entity<Popup>>>>,
}

impl Global for PopupGlobal {}

pub(crate) fn render_overlay(
    window: &mut Window,
    cx: &mut Context<'_, crate::Root>,
) -> impl IntoElement {
    let last_popup = PopupGlobal::global(cx)
        .popup_stacks
        .read(cx)
        .get(&window.window_handle())
        .and_then(|stack| stack.last().cloned());

    div().size_full().children(last_popup.map(|popup| {
        div()
            .flex()
            .justify_center()
            .items_center()
            .occlude()
            .size_full()
            .bg(gpui::black().opacity(0.25))
            .on_any_mouse_down(|_, window, cx| cx.close_popup(window))
            .child(popup)
    }))
}

pub struct Popup {
    title: SharedString,
    kind: PopupKind,
    on_close: Option<Box<dyn FnOnce(&mut Window, &mut App)>>,
}

impl Popup {
    pub fn message(title: impl Into<SharedString>, message: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            kind: PopupKind::Message { message: message.into() },
            on_close: None,
        }
    }

    pub fn input<S: InputDelegate + 'static>(
        title: impl Into<SharedString>,
        input: Entity<InputState<S>>,
        window: &mut Window,
        cx: &mut App,
        on_submit: impl Fn(&S::Value, &mut App) + 'static,
    ) -> Self
    where
        S::Value: Default,
    {
        let on_submit = Rc::new(on_submit);

        window.defer(cx, {
            let input = input.clone();
            move |window, cx| {
                input.focus_handle(cx).focus(window, cx);
            }
        });

        window
            .subscribe(&input, cx, {
                let on_submit = Rc::clone(&on_submit);
                move |_, event, window, cx| match event {
                    InputEvent::Submit(value) => {
                        on_submit(value, cx);
                        cx.close_popup(window)
                    }
                    _ => {}
                }
            })
            .detach();

        let wrapper = cx.new(|_| InputPopup { input: input.clone() });

        let on_close = move |_: &mut Window, cx: &mut App| {
            let value = input.read(cx).value_or_default(cx);
            (on_submit)(&value, cx);
        };

        Self {
            title: title.into(),
            kind: PopupKind::Input { input: wrapper.into() },
            on_close: Some(Box::new(on_close)),
        }
    }

    pub fn custom(title: impl Into<SharedString>, content: impl Into<AnyView>) -> Self {
        Self {
            title: title.into(),
            kind: PopupKind::Custom { content: content.into() },
            on_close: None,
        }
    }

    pub fn title(&self) -> &SharedString {
        &self.title
    }
}

pub enum PopupKind {
    Message { message: SharedString },
    Input { input: AnyView },
    Custom { content: AnyView },
}

impl Render for Popup {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title().clone();

        let header = h_flex()
            .px_2()
            .min_w_full()
            .min_h(window.line_height() * 1.5)
            .max_h(window.line_height() * 1.5)
            .bg(cx.theme().bg_tile_header)
            .border_1()
            .border_color(cx.theme().border_tile_header)
            .rounded_t(cx.theme().radius)
            .text_color(cx.theme().fg_tile_header)
            .font_weight(FontWeight::BOLD)
            .child(title);

        let content = div()
            .size_full()
            .bg(cx.theme().bg_primary)
            .border_1()
            .border_color(cx.theme().border_primary)
            .rounded_b(cx.theme().radius)
            .child(match &self.kind {
                PopupKind::Message { message } => v_flex()
                    .size_full()
                    .items_center()
                    .gap_2()
                    .p_2()
                    .child(
                        div()
                            .text_color(cx.theme().fg_secondary)
                            .w_1_2()
                            .text_center()
                            .child(message.clone()),
                    )
                    .child(
                        Button::new("close", cx.focus_handle())
                            .on_click(|_, window, cx| cx.close_popup(window)),
                    )
                    .into_any_element(),
                PopupKind::Input { input } => input.clone().into_any_element(),
                PopupKind::Custom { content } => content.clone().into_any_element(),
            });

        v_flex()
            .occlude()
            .min_w(px(320.0))
            .max_w_5_6()
            .max_h_5_6()
            .when(cx.theme().shadow, |e| {
                e.shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.3),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(24.0),
                    spread_radius: px(-1.0),
                    inset: false,
                }])
            })
            .child(header)
            .child(content)
    }
}

struct InputPopup<S: InputDelegate> {
    input: Entity<InputState<S>>,
}

impl<S: InputDelegate + 'static> Render for InputPopup<S> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .size_full()
            .p_2()
            .child(div().w_full().child(S::new_element(self.input.clone(), window, cx)))
    }
}

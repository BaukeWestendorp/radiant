use gpui::{
    AnyView, Entity, FocusHandle, Global, MouseDownEvent, UpdateGlobal, Window, canvas, div,
};
use gpui::{App, Focusable, prelude::*};

use crate::{ActiveTheme, PopupOverlay, SettingsAppExt, z_stack};

pub(crate) mod action {
    pub const KEY_CONTEXT: &str = "Root";

    gpui::actions!(
        root,
        [FocusNext, FocusPrevious, OpenSettings, Add, Edit, Delete, SelectionAll, SelectionClear]
    );
}

pub(crate) fn init(cx: &mut App) {
    cx.set_global(FocusGlobal::default());
}

pub struct Root {
    view: AnyView,
    focus_handle: FocusHandle,

    settings_window_content: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    popup_overlay: Entity<PopupOverlay>,
}

impl Root {
    pub fn new(view: impl Into<AnyView>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            view: view.into(),
            focus_handle: cx.focus_handle(),

            settings_window_content: None,
            popup_overlay: cx.new(|cx| PopupOverlay::new(window, cx)),
        }
    }

    pub fn view(&self) -> &AnyView {
        &self.view
    }

    pub fn set_settings_window_content(
        &mut self,
        settings_window_content: Option<impl Fn(&mut Window, &mut App) -> AnyView + 'static>,
    ) {
        self.settings_window_content = settings_window_content
            .map(|f| Box::new(f) as Box<dyn Fn(&mut Window, &mut App) -> AnyView>);
    }

    pub fn with_settings_window_content(
        mut self,
        settings_window_content: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.settings_window_content = Some(Box::new(settings_window_content));
        self
    }

    fn handle_tab(&mut self, _: &action::FocusNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
        FocusGlobal::update_global(cx, |focus, cx| {
            focus.last_focus_was_keyboard = true;
            cx.notify();
        });
    }

    fn handle_tab_prev(
        &mut self,
        _: &action::FocusPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
        FocusGlobal::update_global(cx, |focus, cx| {
            focus.last_focus_was_keyboard = true;
            cx.notify();
        });
    }

    fn handle_open_settings(
        &mut self,
        _: &action::OpenSettings,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(settings_window_content) = &self.settings_window_content else {
            return;
        };

        let window_options = crate::settings_window_options(cx);

        cx.open_settings(Some(window_options), settings_window_content);
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

        let mouse_event_listener = canvas(
            |_, _, _| {},
            |_, _, window, _| {
                window.on_mouse_event(|_: &MouseDownEvent, _, _, cx| {
                    FocusGlobal::update_global(cx, |focus, _| {
                        focus.last_focus_was_keyboard = false;
                    });
                });
            },
        );

        div()
            .id("root")
            .track_focus(&self.focus_handle)
            .key_context(action::KEY_CONTEXT)
            .on_action(cx.listener(Self::handle_tab))
            .on_action(cx.listener(Self::handle_tab_prev))
            .on_action(cx.listener(Self::handle_open_settings))
            .relative()
            .size_full()
            .bg(cx.theme().bg_primary)
            .text_color(cx.theme().fg_primary)
            .child(
                z_stack([
                    content.into_any_element(),
                    self.popup_overlay.clone().into_any_element(),
                    mouse_event_listener.into_any_element(),
                ])
                .size_full(),
            )
    }
}

#[derive(Default)]
pub(crate) struct FocusGlobal {
    pub last_focus_was_keyboard: bool,
}

impl Global for FocusGlobal {}

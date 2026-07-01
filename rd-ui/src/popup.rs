use gpui::{
    AnyView, App, Entity, Focusable, FontWeight, Global, IntoElement, ReadGlobal, SharedString,
    Styled, Window, div, prelude::*, px,
};

use crate::{ActiveTheme, Button, Field, FieldEvent, FieldState, StyledExt, h_flex, v_flex};

pub(crate) fn init(cx: &mut App) {
    let popup = cx.new(|_| None);
    cx.set_global(PopupGlobal { popup });
}

pub trait PopupAppExt {
    fn show_popup<F: FnOnce(&mut App) -> Popup>(&mut self, popup_builder: F);

    fn close_popup(&mut self);
}

impl PopupAppExt for App {
    fn show_popup<F: FnOnce(&mut App) -> Popup>(&mut self, popup_builder: F) {
        let popup = (popup_builder)(self);
        let popup_view = self.new(|_| popup);
        PopupGlobal::global(self).popup.clone().write(self, Some(popup_view));
    }

    fn close_popup(&mut self) {
        PopupGlobal::global(self).popup.clone().write(self, None);
    }
}

pub(crate) struct PopupGlobal {
    pub popup: Entity<Option<Entity<Popup>>>,
}

impl Global for PopupGlobal {}

pub(crate) fn render_overlay(cx: &mut gpui::Context<'_, crate::Root>) -> impl IntoElement {
    let popup = PopupGlobal::global(cx).popup.read(cx).clone();

    div().size_full().children(popup.map(|popup| {
        div()
            .flex()
            .justify_center()
            .items_center()
            .occlude()
            .size_full()
            .bg(cx.theme().contrast.opacity(0.25))
            .on_any_mouse_down(|_, _, cx| cx.close_popup())
            .child(popup)
    }))
}

pub struct Popup {
    title: SharedString,
    kind: PopupKind,
}

impl Popup {
    pub fn yes_no(title: impl Into<SharedString>) -> Self {
        Self { title: title.into(), kind: PopupKind::YesNo }
    }

    pub fn message(title: impl Into<SharedString>, message: impl Into<SharedString>) -> Self {
        Self { title: title.into(), kind: PopupKind::Message { message: message.into() } }
    }

    pub fn text(
        title: impl Into<SharedString>,
        field: Entity<FieldState<SharedString>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        field.focus_handle(cx).focus(window, cx);

        cx.subscribe(&field, |_, event, cx| match event {
            FieldEvent::Submit(_) => cx.close_popup(),
            _ => {}
        })
        .detach();

        Self { title: title.into(), kind: PopupKind::Text { field } }
    }

    pub fn custom(content: impl Into<AnyView>, title: impl Into<SharedString>) -> Self {
        Self { title: title.into(), kind: PopupKind::Custom { content: content.into() } }
    }

    pub fn title(&self) -> &SharedString {
        &self.title
    }
}

pub enum PopupKind {
    YesNo,
    Message { message: SharedString },
    Text { field: Entity<FieldState<SharedString>> },
    Custom { content: AnyView },
}

impl Render for Popup {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title().clone();

        let header = h_flex()
            .px_2()
            .w_full()
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
                PopupKind::YesNo => todo!(),
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
                        Button::new("close").child("Close").on_click(|_, _, cx| cx.close_popup()),
                    )
                    .into_any_element(),
                PopupKind::Text { field: input } => div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .size_full()
                    .p_2()
                    .child(div().w_full().child(Field::new(input.clone())))
                    .into_any_element(),
                PopupKind::Custom { content } => content.clone().into_any_element(),
            });

        let popup = v_flex().size_full().child(header).child(content);

        div()
            .focus(|e| e.border_1().border_color(gpui::red()))
            .occlude()
            .when(cx.theme().shadow, |e| e.shadow_md())
            .w(px(320.0))
            .max_w_3_4()
            .max_h_3_4()
            .child(popup)
    }
}

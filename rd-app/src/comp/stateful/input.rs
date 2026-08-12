use std::str::FromStr as _;

use rd_ui::{
    comp::stateful::{self, Field},
    gpui::{
        App, ElementId, Entity, EventEmitter, FocusHandle, Focusable, SharedString, Window,
        prelude::*,
    },
};

pub struct FixtureIdField {
    field: Entity<Field>,
}

impl FixtureIdField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let field = cx.new(|cx| {
            Field::new(id, window, cx).with_validator(cx, |s| u32::from_str_radix(s, 10).is_ok())
        });

        cx.subscribe(&field, |this, _, _: &stateful::event::Change<SharedString>, cx| {
            cx.emit(stateful::event::Change(this.value(cx)));
        })
        .detach();

        cx.subscribe(&field, |this, _, _: &stateful::event::Submit<SharedString>, cx| {
            cx.emit(stateful::event::Submit(this.value(cx)));
        })
        .detach();

        Self { field }
    }

    pub fn value(&self, cx: &App) -> Option<u32> {
        let text = self.field.read(cx).text(cx);
        u32::from_str_radix(text, 10).ok()
    }
}

impl Focusable for FixtureIdField {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.field.focus_handle(cx)
    }
}

impl Render for FixtureIdField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.field.clone()
    }
}

impl EventEmitter<stateful::event::Submit<Option<u32>>> for FixtureIdField {}
impl EventEmitter<stateful::event::Change<Option<u32>>> for FixtureIdField {}

pub struct AddressField {
    field: Entity<Field>,
}

impl AddressField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let field = cx.new(|cx| {
            Field::new(id, window, cx)
                .with_submit_validator(cx, |s| rd_dmx::Address::from_str(s).is_ok())
                .with_validator(cx, |s| {
                    if s.is_empty() {
                        return true;
                    }

                    let mut parts = s.split('.');

                    let universe_str = parts.next().unwrap_or("");
                    if rd_dmx::UniverseId::from_str(universe_str).is_err() {
                        return false;
                    }

                    if let Some(channel_str) = parts.next() {
                        if !channel_str.is_empty()
                            && rd_dmx::Channel::from_str(channel_str).is_err()
                        {
                            return false;
                        }
                    }

                    parts.next().is_none()
                })
        });

        cx.subscribe(&field, |this, _, _: &stateful::event::Change<SharedString>, cx| {
            cx.emit(stateful::event::Change(this.value(cx)));
        })
        .detach();

        cx.subscribe(&field, |this, _, _: &stateful::event::Submit<SharedString>, cx| {
            cx.emit(stateful::event::Submit(this.value(cx)));
        })
        .detach();

        Self { field }
    }

    pub fn value(&self, cx: &App) -> Option<rd_dmx::Address> {
        let text = self.field.read(cx).text(cx);
        rd_dmx::Address::from_str(text).ok()
    }
}

impl Focusable for AddressField {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.field.focus_handle(cx)
    }
}

impl Render for AddressField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.field.clone()
    }
}

impl EventEmitter<stateful::event::Submit<Option<rd_dmx::Address>>> for AddressField {}
impl EventEmitter<stateful::event::Change<Option<rd_dmx::Address>>> for AddressField {}

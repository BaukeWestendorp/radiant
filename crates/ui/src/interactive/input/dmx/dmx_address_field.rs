use crate::{DmxChannelField, DmxUniverseIdField, TextInputEvent};
use gpui::*;

pub struct DmxAddressField {
    universe_field: Entity<DmxUniverseIdField>,
    channel_field: Entity<DmxChannelField>,
}

impl DmxAddressField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let id = id.into();
        let universe_field_id = ElementId::Name(format!("{}-UniverseField", id.clone()).into());
        let universe_field =
            cx.new(|cx| DmxUniverseIdField::new(universe_field_id, cx.focus_handle(), window, cx));

        let channel_field_id = ElementId::Name(format!("{}-ChannelField", id).into());
        let channel_field =
            cx.new(|cx| DmxChannelField::new(channel_field_id, cx.focus_handle(), window, cx));

        cx.subscribe(&universe_field, |_, _, event: &TextInputEvent, cx| {
            cx.emit(event.clone());
            cx.notify();
        })
        .detach();

        cx.subscribe(&channel_field, |_, _, event: &TextInputEvent, cx| {
            cx.emit(event.clone());
            cx.notify();
        })
        .detach();

        Self { universe_field, channel_field }
    }

    pub fn set_value(&mut self, value: dmx::Address, cx: &mut Context<Self>) {
        self.universe_field.update(cx, |field, cx| field.set_value(value.universe, cx));
        self.channel_field.update(cx, |field, cx| field.set_value(value.channel, cx));
    }

    pub fn value(&self, cx: &App) -> dmx::Address {
        let universe_id = self.universe_field.read(cx).value(cx);
        let channel = self.channel_field.read(cx).value(cx);
        dmx::Address::new(universe_id, channel)
    }
}

impl Render for DmxAddressField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .gap_2()
            .child(self.universe_field.clone())
            .child(self.channel_field.clone())
    }
}

impl EventEmitter<TextInputEvent> for DmxAddressField {}

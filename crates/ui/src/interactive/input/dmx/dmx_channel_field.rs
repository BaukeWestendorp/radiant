use crate::{NumberField, TextInputEvent};
use gpui::*;

pub struct DmxChannelField {
    field: Entity<NumberField>,
}

impl DmxChannelField {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let field = cx.new(|cx| {
            let mut field = NumberField::new(id.into(), focus_handle, window, cx);
            field.set_min(Some(u16::from(dmx::Channel::MIN) as f64));
            field.set_max(Some(u16::from(dmx::Channel::MAX) as f64));
            field.set_step(Some(1.0));
            field.set_value(u16::from(dmx::Channel::MIN) as f64, cx);
            field
        });

        cx.subscribe(&field, |_, _, event: &TextInputEvent, cx| {
            cx.emit(event.clone());
            cx.notify();
        })
        .detach();

        Self { field }
    }

    pub fn set_value(&mut self, value: dmx::Channel, cx: &mut Context<Self>) {
        let valid_float_value = u16::from(value.clamp(dmx::Channel::MIN, dmx::Channel::MAX)) as f64;
        self.field.update(cx, |field, cx| field.set_value(valid_float_value, cx));
    }

    pub fn value(&self, cx: &App) -> dmx::Channel {
        let value = self.field.read(cx).value(cx);
        let valid_u16_value = value
            .clamp(u16::from(dmx::Channel::MIN) as f64, u16::from(dmx::Channel::MAX) as f64)
            as u16;
        dmx::Channel::new(valid_u16_value).expect("should convert field value to valid universe id")
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.field.read(cx).disabled(cx)
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.field.update(cx, |field, cx| field.set_disabled(disabled, cx));
    }
}

impl Render for DmxChannelField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.field.clone()
    }
}

impl EventEmitter<TextInputEvent> for DmxChannelField {}

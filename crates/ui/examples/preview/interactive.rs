use gpui::{Entity, ScrollHandle, Window, div, point, prelude::*, px};
use ui::{
    Checkbox, CheckboxEvent, ContainerStyle, Disableable, DmxAddressField, DmxChannelField,
    DmxUniverseIdField, Draggable, NumberField, Pannable, TextField, container,
};

pub struct InteractiveTab {
    scroll_handle: ScrollHandle,

    disable_fields_checkbox: Entity<Checkbox>,
    text_field: Entity<TextField>,
    masked_text_field: Entity<TextField>,
    number_field: Entity<NumberField>,
    dmx_address_field: Entity<DmxAddressField>,
    dmx_channel_field: Entity<DmxChannelField>,
    dmx_universe_id_field: Entity<DmxUniverseIdField>,

    checkbox: Entity<Checkbox>,
    checkbox_disabled: Entity<Checkbox>,

    draggable: Entity<Draggable>,
    pannable: Entity<Pannable>,
}

impl InteractiveTab {
    pub fn new(w: &mut Window, cx: &mut Context<Self>) -> Self {
        let disable_fields_checkbox = cx.new(|_| Checkbox::new("disable-fields-checkbox"));

        cx.subscribe(&disable_fields_checkbox, |this: &mut Self, _, event: &CheckboxEvent, cx| {
            match event {
                CheckboxEvent::Selected(selected) => {
                    this.text_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                    this.masked_text_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                    this.number_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                    this.dmx_address_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                    this.dmx_channel_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                    this.dmx_universe_id_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                }
            }
        })
        .detach();

        Self {
            scroll_handle: ScrollHandle::new(),

            disable_fields_checkbox,
            text_field: cx.new(|cx| {
                let field = TextField::new("text", cx.focus_handle(), w, cx);
                field.set_placeholder("Text Field Placeholder".into(), cx);
                field
            }),
            masked_text_field: cx.new(|cx| {
                let field = TextField::new("masked-text", cx.focus_handle(), w, cx);
                field.set_placeholder("Masked Text Field Placeholder".into(), cx);
                field.set_masked(true, cx);
                field
            }),
            number_field: cx.new(|cx| NumberField::new("number", cx.focus_handle(), w, cx)),
            dmx_address_field: cx.new(|cx| DmxAddressField::new("address", w, cx)),
            dmx_channel_field: cx
                .new(|cx| DmxChannelField::new("channel", cx.focus_handle(), w, cx)),
            dmx_universe_id_field: cx
                .new(|cx| DmxUniverseIdField::new("universe_id", cx.focus_handle(), w, cx)),

            checkbox: cx.new(|_| Checkbox::new("checkbox")),
            checkbox_disabled: cx.new(|_| Checkbox::new("checkbox-disabled").disabled(true)),

            draggable: cx.new(|cx| {
                Draggable::new("draggable", point(px(40.0), px(40.0)), None, cx.new(|_| ExampleBox))
            }),
            pannable: cx.new(|cx| {
                Pannable::new("pannable", point(px(40.0), px(40.0)), cx.new(|_| ExampleBox))
            }),
        }
    }
}

impl Render for InteractiveTab {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row = |label, input| {
            div()
                .w_full()
                .flex()
                .gap_2()
                .child(div().child(label).w_40())
                .child(div().child(input).w_full())
        };

        let inputs = div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .child(row("Disable Fields", self.disable_fields_checkbox.clone().into_any_element()))
            .child(row("Text", self.text_field.clone().into_any_element()))
            .child(row("Masked Text", self.masked_text_field.clone().into_any_element()))
            .child(row("Number", self.number_field.clone().into_any_element()))
            .child(row("DMX Address", self.dmx_address_field.clone().into_any_element()))
            .child(row("DMX Channel", self.dmx_channel_field.clone().into_any_element()))
            .child(row("DMX Universe ID", self.dmx_universe_id_field.clone().into_any_element()));

        let checkboxes = div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .child(row("Checkbox", self.checkbox.clone().into_any_element()))
            .child(row("Disabled", self.checkbox_disabled.clone().into_any_element()));

        let draggable =
            container(ContainerStyle::normal(w, cx)).w_full().h_64().child(self.draggable.clone());

        let pannable =
            container(ContainerStyle::normal(w, cx)).w_full().h_64().child(self.pannable.clone());

        div()
            .id("typography-tab")
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .size_full()
            .p_2()
            .flex()
            .flex_col()
            .gap_2()
            .child(ui::section("Inputs").mb_4().child(inputs))
            .child(ui::section("Checkboxes").mb_4().child(checkboxes))
            .child(ui::section("Draggable").mb_4().child(draggable))
            .child(ui::section("Pannable").mb_4().child(pannable))
    }
}

struct ExampleBox;

impl Render for ExampleBox {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        container(ContainerStyle::normal(w, cx)).size_20().child("Draggable Box")
    }
}

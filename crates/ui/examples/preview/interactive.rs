use gpui::{Entity, ScrollHandle, Window, div, point, prelude::*, px};
use ui::{
    ContainerStyle, DmxAddressField, DmxChannelField, DmxUniverseIdField, Draggable, NumberField,
    Pannable, TextField, container,
};

pub struct InteractiveTab {
    scroll_handle: ScrollHandle,
    text_field: Entity<TextField>,
    number_field: Entity<NumberField>,
    dmx_address_field: Entity<DmxAddressField>,
    dmx_channel_field: Entity<DmxChannelField>,
    dmx_universe_id_field: Entity<DmxUniverseIdField>,
    draggable: Entity<Draggable>,
    pannable: Entity<Pannable>,
}

impl InteractiveTab {
    pub fn new(w: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),

            text_field: cx.new(|cx| TextField::new("text", cx.focus_handle(), w, cx)),
            number_field: cx.new(|cx| NumberField::new("number", cx.focus_handle(), w, cx)),
            dmx_address_field: cx.new(|cx| DmxAddressField::new("address", w, cx)),
            dmx_channel_field: cx
                .new(|cx| DmxChannelField::new("channel", cx.focus_handle(), w, cx)),
            dmx_universe_id_field: cx
                .new(|cx| DmxUniverseIdField::new("universe_id", cx.focus_handle(), w, cx)),
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
        let input_row = |label, input| {
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
            .child(input_row("Text", self.text_field.clone().into_any_element()))
            .child(input_row("Number", self.number_field.clone().into_any_element()))
            .child(input_row("DMX Address", self.dmx_address_field.clone().into_any_element()))
            .child(input_row("DMX Channel", self.dmx_channel_field.clone().into_any_element()))
            .child(input_row(
                "DMX Universe ID",
                self.dmx_universe_id_field.clone().into_any_element(),
            ));

        let draggable = container(ContainerStyle::normal(w, cx))
            .w_full()
            .h_64()
            .overflow_hidden()
            .child(self.draggable.clone());

        let pannable = container(ContainerStyle::normal(w, cx))
            .w_full()
            .h_64()
            .overflow_hidden()
            .child(self.pannable.clone());

        div()
            .id("typography-tab")
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .size_full()
            .p_2()
            .flex()
            .flex_col()
            .gap_2()
            .child(ui::section("Inputs").child(inputs).mb_4())
            .child(ui::section("Draggable").child(draggable).mb_4())
            .child(ui::section("Pannable").child(pannable).mb_4())
    }
}

struct ExampleBox;

impl Render for ExampleBox {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        container(ContainerStyle::normal(w, cx)).size_20().child("Draggable Box")
    }
}

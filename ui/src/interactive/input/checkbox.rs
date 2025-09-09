use gpui::prelude::*;
use gpui::{ClickEvent, ElementId, EventEmitter, Window, div, px};

use crate::theme::ActiveTheme;
use crate::{Disableable, Selectable};

pub struct Checkbox {
    id: ElementId,
    selected: bool,
    disabled: bool,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), selected: false, disabled: false }
    }

    fn handle_on_click(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected = !self.selected;
        cx.emit(CheckboxEvent::Change(self.selected));
    }
}

impl Disableable for Checkbox {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for Checkbox {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Render for Checkbox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mark = if self.selected {
            Some(div().size_full().rounded_xs().bg(cx.theme().input_secondary))
        } else {
            None
        };

        div()
            .id(self.id.clone())
            .size(window.line_height())
            .bg(cx.theme().input)
            .border_1()
            .border_color(cx.theme().input_border)
            .rounded(cx.theme().radius)
            .flex()
            .items_center()
            .justify_center()
            .p(px(6.0))
            .cursor_not_allowed()
            .when(!self.disabled, |e| {
                e.on_click(cx.listener(Self::handle_on_click)).cursor_default()
            })
            .when(!self.selected, |e| {
                e.bg(cx.theme().selected).border_color(cx.theme().selected_border)
            })
            .children(mark)
    }
}

pub enum CheckboxEvent {
    Change(bool),
}

impl EventEmitter<CheckboxEvent> for Checkbox {}

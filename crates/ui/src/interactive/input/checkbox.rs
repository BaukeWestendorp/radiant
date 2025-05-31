use crate::{ActiveTheme, Disableable, Selectable, interactive_container};
use gpui::{ClickEvent, ElementId, EventEmitter, Window, div, prelude::*, px};

pub struct Checkbox {
    id: ElementId,
    selected: bool,
    disabled: bool,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), selected: false, disabled: false }
    }
}

impl Checkbox {
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
            Some(div().size_full().rounded_xs().bg(cx.theme().colors.bg_selected_bright))
        } else {
            None
        };

        interactive_container(self.id.clone(), None)
            .size(window.line_height())
            .flex()
            .items_center()
            .justify_center()
            .p(px(6.0))
            .cursor_pointer()
            .disabled(self.disabled)
            .selected(self.selected)
            .on_click(cx.listener(Self::handle_on_click))
            .children(mark)
    }
}

pub enum CheckboxEvent {
    Change(bool),
}

impl EventEmitter<CheckboxEvent> for Checkbox {}

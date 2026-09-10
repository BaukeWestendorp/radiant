use gpui::{
    App, ClickEvent, ElementId, EventEmitter, FocusHandle, Focusable, Window, div, prelude::*,
};

use crate::{
    Emphasis, StyledExt, StyledParentExt, StyledStatefulInteractiveElementExt, c_flex,
    comp::{
        Disableable, FocusableComponent, Identifiable,
        stateful::{FormWidget, InputEvent, InputValue, Submittable},
    },
};

pub struct Checkbox {
    id: ElementId,
    focus_handle: FocusHandle,

    checked: bool,
    disabled: bool,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            id: id.into(),
            focus_handle: cx.focus_handle().tab_stop(true),
            checked: false,
            disabled: false,
        }
    }

    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        self.checked = checked;
        cx.emit(InputEvent::Submit(self.checked));
        cx.emit(InputEvent::Change(InputValue::Valid(self.checked)));
        cx.notify();
    }

    pub fn checked(&self) -> bool {
        self.checked
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.set_checked(!self.checked, cx);
    }

    fn handle_click(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.toggle(cx);
    }
}

impl Disableable for Checkbox {
    fn disabled(&self, _cx: &App) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool, _cx: &mut App) {
        self.disabled = disabled;
    }
}

impl Identifiable for Checkbox {
    fn id(&self, _cx: &App) -> &ElementId {
        &self.id
    }
}

impl Focusable for Checkbox {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl FocusableComponent for Checkbox {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = focus_handle;
    }
}

impl Render for Checkbox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = if self.checked() {
            div()
                .size_full()
                .when(self.disabled, |e| e.disabled_emphasis_bordered(Emphasis::Selected, cx))
                .when(!self.disabled, |e| e.emphasis_bordered(Emphasis::Selected, cx))
        } else {
            div()
        };

        c_flex()
            .id(self.id.clone())
            .when(!self.disabled, |e| {
                e.track_focus(&self.focus_handle).focus_ring(&self.focus_handle, window, cx)
            })
            .size(crate::comp::INPUT_SIZE)
            .p_1()
            .when(self.disabled, |e| {
                e.disabled_emphasis_bordered(Emphasis::Secondary, cx).cursor_not_allowed()
            })
            .when(!self.disabled, |e| {
                e.interactive_emphasis_bordered(Emphasis::Secondary, cx)
                    .on_click(cx.listener(Self::handle_click))
            })
            .child(content)
    }
}

impl EventEmitter<InputEvent<bool>> for Checkbox {}

impl Submittable<bool> for Checkbox {
    fn value(&self, _cx: &App) -> InputValue<bool> {
        InputValue::Valid(self.checked)
    }
}

impl FormWidget<bool> for Checkbox {
    fn value(&self, _cx: &App) -> InputValue<bool> {
        InputValue::Valid(self.checked)
    }

    fn set_value(&mut self, value: bool, cx: &mut Context<Self>) {
        self.set_checked(value, cx);
    }
}

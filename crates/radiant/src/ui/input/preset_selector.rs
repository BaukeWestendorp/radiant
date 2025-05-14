use crate::show::asset::AnyPresetId;
use gpui::{ElementId, EventEmitter, FocusHandle, Window, prelude::*};
use ui::interactive_container;

pub struct PresetSelector {
    value: Option<AnyPresetId>,

    id: ElementId,
    focus_handle: FocusHandle,
}

impl PresetSelector {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        _w: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self { value: None, id: id.into(), focus_handle }
    }

    pub fn value(&self) -> Option<&AnyPresetId> {
        self.value.as_ref()
    }

    pub fn set_value(&mut self, value: Option<AnyPresetId>, cx: &mut Context<Self>) {
        self.value = value;
        cx.notify();
        cx.emit(PresetSelectorEvent::Change(self.value));
    }
}

impl Render for PresetSelector {
    fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        interactive_container(self.id.clone(), Some(self.focus_handle.clone())).child("CLICK")
    }
}

pub enum PresetSelectorEvent {
    Change(Option<AnyPresetId>),
}

impl EventEmitter<PresetSelectorEvent> for PresetSelector {}

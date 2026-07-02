use gpui::{App, ElementId, Entity, FocusHandle, RenderOnce, Window, div, prelude::*};
use rd_engine::{gdtf::FixtureTypeId, patch::FixtureKind};
use rd_ui::{Popup, PopupAppExt, interactive_container};

use crate::engine::EngineAppExt;

pub struct FixtureKindPickerState {
    id: ElementId,
    focus_handle: FocusHandle,

    disabled: bool,

    fixture_type_id: Option<FixtureTypeId>,
    dmx_mode: Option<String>,
}

impl FixtureKindPickerState {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            id: id.into(),
            focus_handle: focus_handle.tab_stop(true),

            disabled: false,

            fixture_type_id: None,
            dmx_mode: None,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }

    pub fn fixture_type_id(&self) -> Option<FixtureTypeId> {
        self.fixture_type_id
    }

    pub fn dmx_mode(&self) -> Option<&str> {
        self.dmx_mode.as_deref()
    }

    pub fn fixture_kind(&self) -> Option<FixtureKind> {
        Some(FixtureKind::new(self.fixture_type_id?, self.dmx_mode.as_ref()?.to_string()))
    }
}

#[derive(IntoElement)]
pub struct FixtureKindPicker {
    state: Entity<FixtureKindPickerState>,
}

impl FixtureKindPicker {
    pub fn new(state: Entity<FixtureKindPickerState>) -> Self {
        Self { state }
    }
}

impl RenderOnce for FixtureKindPicker {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let id = state.id.clone();
        let focus_handle = state.focus_handle.clone();
        let disabled = state.disabled();
        let fixture_kind = state.fixture_kind();

        let preview = fixture_kind
            .and_then(|fk| fk.display(&cx.engine_snapshot().patch()))
            .unwrap_or("".to_string());

        interactive_container(id, Some(focus_handle))
            .relative()
            .w_full()
            .disabled(disabled)
            .child(div().size_full().px_1().py_0p5().child(preview))
            .on_click(|_, window, cx| {
                cx.open_popup(window, |_, _| {
                    Popup::message("Select Fixture Kind", "FIXME: Implement Fixture Kind Selector")
                })
            })
    }
}

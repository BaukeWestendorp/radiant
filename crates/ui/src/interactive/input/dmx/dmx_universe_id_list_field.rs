use crate::{TextField, TextInputEvent};
use dmx::UniverseId;
use gpui::*;
use std::str::FromStr;

pub struct DmxUniverseIdListField {
    field: Entity<TextField>,
}

impl DmxUniverseIdListField {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let field = cx.new(|cx| {
            let field = TextField::new(id.into(), focus_handle, window, cx);
            field.set_placeholder("e.g. 0 1 2".into(), cx);
            field
        });

        cx.subscribe(&field, |this, _, event: &TextInputEvent, cx| {
            cx.emit(event.clone());
            cx.notify();
            match event {
                TextInputEvent::Blur => {
                    this.commit_value(cx);
                }
                _ => {}
            }
        })
        .detach();

        Self { field }
    }

    pub fn set_value(&mut self, value: &[dmx::UniverseId], cx: &mut App) {
        let string = value.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
        self.field.update(cx, |field, cx| field.set_value(string.into(), cx));
    }

    pub fn value(&self, cx: &App) -> Vec<dmx::UniverseId> {
        let value = self.field.read(cx).value(cx);
        value
            .split(" ")
            .map(|s| UniverseId::from_str(s))
            .collect::<Result<Vec<dmx::UniverseId>, _>>()
            .unwrap_or_default()
    }

    fn commit_value(&mut self, cx: &mut App) {
        self.set_value(&self.value(cx), cx);
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.field.read(cx).disabled(cx)
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.field.update(cx, |field, cx| field.set_disabled(disabled, cx));
    }
}

impl Render for DmxUniverseIdListField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.field.clone()
    }
}

impl EventEmitter<TextInputEvent> for DmxUniverseIdListField {}

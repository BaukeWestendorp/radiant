use crate::{
    app::AppState,
    show::{Show, asset::AnyPresetId},
    ui::{
        asset_table::AssetTable,
        vw::{VirtualWindow, VirtualWindowDelegate},
    },
};
use gpui::{
    App, ClickEvent, ElementId, Entity, EventEmitter, FocusHandle, Focusable, ReadGlobal,
    UpdateGlobal, Window, div, prelude::*,
};
use ui::{ActiveTheme, InteractiveColor, Orientation, Tab, TabView, Table, interactive_container};

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

impl PresetSelector {
    fn handle_on_click(&mut self, _event: &ClickEvent, w: &mut Window, cx: &mut Context<Self>) {
        AppState::update_global(cx, |state, cx| {
            state.open_preset_selector_window(w, cx);
        });
    }
}

impl Render for PresetSelector {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let label = match self.value() {
            Some(value) => div().child(format!("{:?}", value)),
            None => div().child("Select preset").text_color(cx.theme().colors.text.muted()),
        };

        interactive_container(self.id.clone(), Some(self.focus_handle.clone()))
            .px_1()
            .w_full()
            .h(w.line_height())
            .child(label)
            .on_click(cx.listener(Self::handle_on_click))
    }
}

pub enum PresetSelectorEvent {
    Change(Option<AnyPresetId>),
}

impl EventEmitter<PresetSelectorEvent> for PresetSelector {}

pub struct PresetSelectorWindow {
    focus_handle: FocusHandle,

    tab_view: Entity<TabView>,
}

impl PresetSelectorWindow {
    pub fn new(w: &mut Window, cx: &mut Context<VirtualWindow<Self>>) -> Self {
        let tab_view = cx.new(|cx| {
            let mut tab_view = TabView::new(
                vec![Tab::new(
                    "dimmer",
                    "Dimmer",
                    cx.new(|cx| {
                        Table::new(
                            AssetTable::new(Show::global(cx).assets.dimmer_presets.clone()),
                            "dimmer-table",
                            w,
                            cx,
                        )
                    })
                    .into(),
                )],
                w,
                cx,
            );
            tab_view.set_orientation(Orientation::Vertical);
            tab_view.select_tab_ix(0);
            tab_view
        });

        Self { focus_handle: cx.focus_handle(), tab_view }
    }
}

impl Focusable for PresetSelectorWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl VirtualWindowDelegate for PresetSelectorWindow {
    fn title(&self, _cx: &App) -> gpui::SharedString {
        "Select a Preset".into()
    }

    fn on_close_window(&mut self, _w: &mut Window, cx: &mut Context<VirtualWindow<Self>>) {
        AppState::update_global(cx, |state, _cx| {
            state.close_preset_selector_window();
        })
    }

    fn render_content(
        &mut self,
        _w: &mut Window,
        _cx: &mut Context<VirtualWindow<Self>>,
    ) -> impl IntoElement {
        div().size_full().child(self.tab_view.clone())
    }
}

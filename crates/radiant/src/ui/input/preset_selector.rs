use crate::app::AppState;
use crate::show::{AnyPresetAssetId, Show};
use crate::ui::{AssetTable, AssetTableEvent, VirtualWindow, VirtualWindowDelegate};
use gpui::{
    App, ClickEvent, ElementId, Entity, EventEmitter, FocusHandle, Focusable, ReadGlobal,
    UpdateGlobal, Window, div, prelude::*,
};
use ui::{ActiveTheme, InteractiveColor, Orientation, Tab, TabView, Table, interactive_container};

pub struct PresetSelector {
    value: Option<AnyPresetAssetId>,

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

    pub fn value(&self) -> Option<&AnyPresetAssetId> {
        self.value.as_ref()
    }

    pub fn set_value(&mut self, value: Option<AnyPresetAssetId>, cx: &mut Context<Self>) {
        self.value = value;
        cx.notify();
        cx.emit(PresetSelectorEvent::Change(self.value));
    }
}

impl PresetSelector {
    fn handle_on_click(&mut self, _event: &ClickEvent, w: &mut Window, cx: &mut Context<Self>) {
        let selector = cx.entity();
        AppState::update_global(cx, |state, cx| {
            state.open_preset_selector_window(selector, w, cx);
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
    Change(Option<AnyPresetAssetId>),
}

impl EventEmitter<PresetSelectorEvent> for PresetSelector {}

pub struct PresetSelectorWindow {
    selector: Entity<PresetSelector>,
    focus_handle: FocusHandle,
    tab_view: Entity<TabView>,
}

impl PresetSelectorWindow {
    pub fn new(
        selector: Entity<PresetSelector>,
        w: &mut Window,
        cx: &mut Context<VirtualWindow<Self>>,
    ) -> Self {
        let dimmer_table = AssetTable::new(Show::global(cx).assets.dimmer_presets.clone());
        let dimmer_table_view = cx.new(|cx| Table::new(dimmer_table, "dimmer-table", w, cx));

        let position_table = AssetTable::new(Show::global(cx).assets.position_presets.clone());
        let position_table_view = cx.new(|cx| Table::new(position_table, "position-table", w, cx));

        let gobo_table = AssetTable::new(Show::global(cx).assets.gobo_presets.clone());
        let gobo_table_view = cx.new(|cx| Table::new(gobo_table, "gobo-table", w, cx));

        let color_table = AssetTable::new(Show::global(cx).assets.color_presets.clone());
        let color_table_view = cx.new(|cx| Table::new(color_table, "color-table", w, cx));

        let beam_table = AssetTable::new(Show::global(cx).assets.beam_presets.clone());
        let beam_table_view = cx.new(|cx| Table::new(beam_table, "beam-table", w, cx));

        let focus_table = AssetTable::new(Show::global(cx).assets.focus_presets.clone());
        let focus_table_view = cx.new(|cx| Table::new(focus_table, "focus-table", w, cx));

        let control_table = AssetTable::new(Show::global(cx).assets.control_presets.clone());
        let control_table_view = cx.new(|cx| Table::new(control_table, "control-table", w, cx));

        let shapers_table = AssetTable::new(Show::global(cx).assets.shapers_presets.clone());
        let shapers_table_view = cx.new(|cx| Table::new(shapers_table, "shapers-table", w, cx));

        let video_table = AssetTable::new(Show::global(cx).assets.video_presets.clone());
        let video_table_view = cx.new(|cx| Table::new(video_table, "video-table", w, cx));

        cx.subscribe(&dimmer_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Dimmer(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&position_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Position(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&gobo_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Gobo(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&color_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Color(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&beam_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Beam(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&focus_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Focus(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&control_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Control(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&shapers_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Shapers(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        cx.subscribe(&video_table_view, |this, _table, event, cx| {
            let AssetTableEvent::Selected(asset) = event;
            this.delegate.selector.update(cx, |selector, cx| {
                selector.set_value(Some(AnyPresetAssetId::Video(asset.read(cx).id)), cx);
                AppState::update_global(cx, |state, _cx| {
                    state.close_preset_selector_window();
                });
                cx.notify();
            });
        })
        .detach();

        let tab_view = cx.new(|cx| {
            let mut tab_view = TabView::new(
                vec![
                    Tab::new("dimmer", "Dimmer", dimmer_table_view.into()),
                    Tab::new("position", "Position", position_table_view.into()),
                    Tab::new("gobo", "Gobo", gobo_table_view.into()),
                    Tab::new("color", "Color", color_table_view.into()),
                    Tab::new("beam", "Beam", beam_table_view.into()),
                    Tab::new("focus", "Focus", focus_table_view.into()),
                    Tab::new("control", "Control", control_table_view.into()),
                    Tab::new("shapers", "Shapers", shapers_table_view.into()),
                    Tab::new("video", "Video", video_table_view.into()),
                ],
                w,
                cx,
            );
            tab_view.set_orientation(Orientation::Vertical);
            tab_view.select_tab_ix(0);
            tab_view
        });

        Self { selector, focus_handle: cx.focus_handle(), tab_view }
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

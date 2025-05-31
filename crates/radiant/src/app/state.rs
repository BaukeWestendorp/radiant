use crate::layout::settings::SettingsWindow;
use crate::ui::{PresetSelector, PresetSelectorWindow, VirtualWindow};
use gpui::{App, Entity, Global, Window, prelude::*};

/// Volitile app state.
#[derive(Default)]
pub struct AppState {
    pub settings_window: Option<Entity<VirtualWindow<SettingsWindow>>>,
    pub preset_selector_window: Option<Entity<VirtualWindow<PresetSelectorWindow>>>,
}

impl Global for AppState {}

impl AppState {
    pub fn init(cx: &mut App) {
        cx.set_global(Self::default());
    }

    pub fn open_settings_window(&mut self, window: &mut Window, cx: &mut App) {
        if self.settings_window.is_none() {
            let vw = cx.new(|cx| VirtualWindow::new(SettingsWindow::new(window, cx)));
            self.settings_window = Some(vw);
        }
    }

    pub fn close_settings_window(&mut self) {
        self.settings_window.take();
    }

    pub fn settings_window(&self) -> Option<&Entity<VirtualWindow<SettingsWindow>>> {
        self.settings_window.as_ref()
    }

    pub fn open_preset_selector_window(
        &mut self,
        selector: Entity<PresetSelector>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.preset_selector_window.is_none() {
            let vw =
                cx.new(|cx| VirtualWindow::new(PresetSelectorWindow::new(selector, window, cx)));
            self.preset_selector_window = Some(vw);
        }
    }

    pub fn close_preset_selector_window(&mut self) {
        self.preset_selector_window.take();
    }

    pub fn preset_selector_window(&self) -> Option<&Entity<VirtualWindow<PresetSelectorWindow>>> {
        self.preset_selector_window.as_ref()
    }
}

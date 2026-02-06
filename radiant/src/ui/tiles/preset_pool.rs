use std::collections::HashMap;

use gpui::{AnyElement, App, Bounds, Entity, Pixels, ReadGlobal, Window, div, prelude::*};
use rui::{ActiveTheme, TileDelegate, h_flex};

use crate::app::{
    object::{Preset, PresetId},
    state::AppState,
};

pub struct PresetsPoolTile {
    bounds: Bounds<u32>,
    cell_size: Pixels,
}

impl PresetsPoolTile {
    pub fn new(bounds: Bounds<u32>, cell_size: Pixels) -> Self {
        Self { bounds, cell_size }
    }

    pub fn presets<'a>(&self, cx: &'a App) -> &'a Entity<HashMap<PresetId, Preset>> {
        AppState::global(cx).presets()
    }
}

impl TileDelegate for PresetsPoolTile {
    fn title(&self) -> &str {
        "Presets"
    }

    fn render_content(&self, _window: &mut Window, cx: &App) -> AnyElement {
        let area = self.bounds.size.width * self.bounds.size.height;

        let cells = (0..area).map(|ix| {
            let id = ix + 1;
            let presets = self.presets(cx).read(cx).get(&id);

            match presets {
                Some(presets) => h_flex()
                    .justify_center()
                    .size(self.cell_size)
                    .bg(cx.theme().bg_secondary)
                    .border_1()
                    .border_color(cx.theme().border_secondary)
                    .rounded(cx.theme().radius)
                    .child(presets.name.to_owned()),
                None => div()
                    .size(self.cell_size)
                    .bg(cx.theme().bg_primary)
                    .border_1()
                    .border_color(cx.theme().border_primary)
                    .rounded(cx.theme().radius),
            }
        });

        div().flex().flex_wrap().size_full().children(cells).into_any_element()
    }

    fn show_header(&self, _cx: &App) -> bool {
        false
    }
}

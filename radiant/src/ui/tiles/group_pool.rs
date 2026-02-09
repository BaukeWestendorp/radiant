use std::collections::HashMap;

use gpui::{
    AnyElement, App, Bounds, ElementId, Entity, Pixels, ReadGlobal, Window, div, prelude::*,
};
use rui::{ActiveTheme, TileDelegate, h_flex};

use crate::app::{
    object::{Group, GroupId},
    state::AppState,
};

pub struct GroupsPoolTile {
    bounds: Bounds<u32>,
    cell_size: Pixels,
}

impl GroupsPoolTile {
    pub fn new(bounds: Bounds<u32>, cell_size: Pixels) -> Self {
        Self { bounds, cell_size }
    }

    pub fn groups<'a>(&self, _cx: &'a App) -> &'a Entity<HashMap<GroupId, Group>> {
        todo!();
    }
}

impl TileDelegate for GroupsPoolTile {
    fn title(&self) -> &str {
        "Groups"
    }

    fn render_content(&self, _window: &mut Window, cx: &App) -> AnyElement {
        let area = self.bounds.size.width * self.bounds.size.height;

        let cells = (0..area).map(|ix| {
            let id = ix + 1;
            let group = self.groups(cx).read(cx).get(&id);

            match group {
                Some(group) => h_flex()
                    .id(ElementId::named_usize("group", id as usize))
                    .justify_center()
                    .size(self.cell_size)
                    .bg(cx.theme().bg_secondary)
                    .border_1()
                    .border_color(cx.theme().border_secondary)
                    .rounded(cx.theme().radius)
                    .child(group.name.to_owned())
                    // FIXME: generalize pool tiles and their interactions.
                    .on_click({
                        let fixture_ids = group.fixture_ids.clone();
                        move |_, _, cx| {
                            let fixture_ids = fixture_ids.clone();
                            // FIMXE: Add helper to manage selection.
                            let selection = AppState::global(cx).selection().clone();
                            selection.update(cx, move |selection, cx| {
                                *selection = fixture_ids;
                                cx.notify();
                            });
                        }
                    }),
                None => div()
                    .id(ElementId::named_usize("group", id as usize))
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

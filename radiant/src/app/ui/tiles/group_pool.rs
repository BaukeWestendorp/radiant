use std::collections::HashMap;

use gpui::{
    AnyElement, App, Bounds, ElementId, Entity, Pixels, ReadGlobal as _, UpdateGlobal as _, Window,
    div, prelude::*, relative,
};
use rui::{ActiveTheme, HslaExt as _, TileDelegate, h_flex};

use crate::{
    app::state::AppState,
    object::{Group, GroupId},
};

pub struct GroupsPoolTile {
    bounds: Bounds<u32>,
    cell_size: Pixels,
}

impl GroupsPoolTile {
    pub fn new(bounds: Bounds<u32>, cell_size: Pixels) -> Self {
        Self { bounds, cell_size }
    }

    pub fn groups<'a>(&self, cx: &'a App) -> &'a Entity<HashMap<GroupId, Group>> {
        AppState::global(cx).show().groups()
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

            let id_overlay = div()
                .text_sm()
                .p_1()
                .line_height(relative(0.8))
                .absolute()
                .size_full()
                .text_color(cx.theme().fg_tertiary)
                .child(id.to_string());

            match group {
                Some(group) => div()
                    .id(ElementId::named_usize("group", id as usize))
                    .relative()
                    .size(self.cell_size)
                    .bg(cx.theme().bg_secondary)
                    .border_1()
                    .border_color(cx.theme().border_secondary)
                    .rounded(cx.theme().radius)
                    .hover(|e| {
                        e.bg(cx.theme().bg_secondary.hover())
                            .border_color(cx.theme().border_secondary.hover())
                    })
                    .active(|e| {
                        e.bg(cx.theme().bg_secondary.active())
                            .border_color(cx.theme().border_secondary.active())
                    })
                    .child(id_overlay)
                    .child(h_flex().justify_center().size_full().child(group.name.to_owned()))
                    .on_click({
                        let fixture_ids = group.fixture_ids.clone();
                        move |_, _, cx| {
                            AppState::update_global(cx, |state, cx| {
                                state.show().set_selection(fixture_ids.clone(), cx)
                            });
                        }
                    }),
                None => div()
                    .id(ElementId::named_usize("group", id as usize))
                    .relative()
                    .size(self.cell_size)
                    .bg(cx.theme().bg_primary)
                    .border_1()
                    .border_color(cx.theme().border_primary)
                    .rounded(cx.theme().radius)
                    .child(id_overlay)
                    .child(div().size(self.cell_size)),
            }
        });

        div().flex().flex_wrap().size_full().children(cells).into_any_element()
    }

    fn show_header(&self, _cx: &App) -> bool {
        false
    }
}

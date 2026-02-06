use std::collections::HashMap;

use gpui::{AnyElement, App, Bounds, Pixels, Window, div, prelude::*};
use rui::{ActiveTheme, TileDelegate};
use zeevonk::project::stage::FixtureId;

pub struct GroupsPoolTile {
    bounds: Bounds<u32>,
    cell_size: Pixels,

    groups: HashMap<u32, (String, Vec<FixtureId>)>,
}

impl GroupsPoolTile {
    pub fn new(bounds: Bounds<u32>, cell_size: Pixels) -> Self {
        Self {
            bounds,
            cell_size,
            groups: {
                let mut map = HashMap::new();
                map.insert(2, ("LEDs".to_string(), vec!["101.1".parse().unwrap()]));
                map
            },
        }
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
            let group = self.groups.get(&id);

            match group {
                Some(group) => div()
                    .size(self.cell_size)
                    .bg(cx.theme().bg_secondary)
                    .border_1()
                    .border_color(cx.theme().border_secondary)
                    .rounded(cx.theme().radius)
                    .child(group.0.to_owned()),
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

use gpui::prelude::*;
use gpui::{AnyElement, App, Bounds, Entity, SharedString, Window, bounds, div, point, px, size};
use rd_ui::{PoolTile, PoolTileDelegate, TileDelegate, TileGrid, TileGridState};

use crate::alpha_content;

pub struct TilesPreview {
    tiles: Entity<TileGridState>,
}

impl TilesPreview {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            tiles: cx.new(|cx| {
                let mut grid = TileGridState::new();
                grid.add_tile(AlphaTile {}, bounds(point(0, 0), size(4, 4)));
                grid.add_tile(
                    PoolTile::new(cx.new(|_| BetaTile {}), px(80.0)),
                    bounds(point(5, 0), size(3, 5)),
                );
                grid
            }),
        }
    }
}

impl Render for TilesPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().p_2().size_full().child(TileGrid::new(self.tiles.clone()))
    }
}

struct AlphaTile {}

impl TileDelegate for AlphaTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Alpha Tile".into()
    }

    fn render_content(&self, _bounds: Bounds<u32>, _window: &mut Window, _cx: &App) -> AnyElement {
        alpha_content().into_any_element()
    }
}

struct BetaTile {}

impl PoolTileDelegate for BetaTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Beta Tile".into()
    }

    fn is_occupied(&self, slot_id: u32, _cx: &App) -> bool {
        slot_id == 4
    }

    fn occupied_label(&self, slot_id: u32, _cx: &App) -> String {
        slot_id.to_string()
    }

    fn on_activate_slot(&mut self, slot_id: u32, _window: &mut Window, _cx: &mut App) {
        log::info!("slot '{slot_id}' activated");
    }
}

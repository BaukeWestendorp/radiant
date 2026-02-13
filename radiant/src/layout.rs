use gpui::{App, Bounds, Pixels, Window, prelude::*, px};
use rui::{PoolTile, TileGridState};

use crate::app::ui::tiles::{EffectsPoolTile, FixturesTile, GroupsPoolTile};

const CELL_SIZE: Pixels = px(80.0);

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Layout {
    pub tiles: Vec<LayoutTile>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LayoutTile {
    pub bounds: Bounds<u32>,
    pub kind: LayoutTileKind,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LayoutTileKind {
    Fixtures,

    GroupsPool,
    EffectsPool,
}

impl Layout {
    pub fn to_tile_grid_state(&mut self, window: &mut Window, cx: &mut App) -> TileGridState {
        let mut tile_grid_state = TileGridState::new();

        for tile in &self.tiles {
            match tile.kind {
                LayoutTileKind::Fixtures => {
                    tile_grid_state.add_tile(FixturesTile::new(window, cx), tile.bounds);
                }
                LayoutTileKind::GroupsPool => {
                    let delegate = cx.new(|_cx| GroupsPoolTile::new());
                    tile_grid_state.add_tile(PoolTile::new(delegate, CELL_SIZE), tile.bounds);
                }
                LayoutTileKind::EffectsPool => {
                    let delegate = cx.new(|_cx| EffectsPoolTile::new());
                    tile_grid_state.add_tile(PoolTile::new(delegate, CELL_SIZE), tile.bounds);
                }
            }
        }

        tile_grid_state
    }
}

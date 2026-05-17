use std::{fs, path::Path};

use anyhow::Context;
use gpui::{App, AppContext as _, Bounds, Pixels, Window, px};
use rd_ui::{PoolTile, TileGridState};

use crate::ui::tiles::{EffectsPoolTile, FixturesTile, GroupsPoolTile};

const LAYOUT_FILE_NAME: &str = "layout.json";

const CELL_SIZE: Pixels = px(80.0);

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Layout {
    pub tiles: Vec<LayoutTile>,
}

impl Layout {
    pub fn load_from_showfile_root(showfile_root: impl AsRef<Path>) -> anyhow::Result<Self> {
        let layout_path = showfile_root.as_ref().join(LAYOUT_FILE_NAME);
        let layout_file = fs::File::open(layout_path).context("failed to open layout file")?;
        let layout =
            serde_json::from_reader(layout_file).context("failed to deserialize layout file")?;
        Ok(layout)
    }

    pub fn save_to_showfile_root(&self, showfile_root: impl AsRef<Path>) -> anyhow::Result<()> {
        let layout_path = showfile_root.as_ref().join(LAYOUT_FILE_NAME);

        let layout_json = serde_json::to_string_pretty(self)?;
        fs::write(layout_path, layout_json)?;

        Ok(())
    }
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

use std::{fs, path::Path};

use anyhow::Context as _;
use gpui::{App, AppContext as _, Bounds, Pixels, SharedString, Size, Window};
use rd_ui::{PoolTile, TileGridState};

use crate::ui::tiles::{CueListsPoolTile, EffectPoolTile, FixturesTile, GroupPoolTile};

const LAYOUT_FILE_NAME: &str = "layout.json";

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Layout {
    current_page_ix: usize,
    pages: Vec<LayoutPage>,
}

impl Layout {
    pub fn load_from_showfile_dir(showfile_root: impl AsRef<Path>) -> anyhow::Result<Self> {
        let layout_path = showfile_root.as_ref().join(LAYOUT_FILE_NAME);
        let layout_file = fs::File::open(layout_path).context("failed to open layout file")?;

        serde_json::from_reader(layout_file).context("failed to deserialize layout file")
    }

    pub fn save_to_showfile_dir(&self, showfile_root: impl AsRef<Path>) -> anyhow::Result<()> {
        let layout_path = showfile_root.as_ref().join(LAYOUT_FILE_NAME);
        let layout_file = fs::File::create(layout_path).context("failed to create layout file")?;

        serde_json::to_writer_pretty(layout_file, self).context("failed to serialize layout")?;

        Ok(())
    }

    pub fn set_current_page(&mut self, ix: usize) -> anyhow::Result<()> {
        if ix < self.pages.len() {
            self.current_page_ix = ix;
            Ok(())
        } else {
            anyhow::bail!(
                "attempted to set current page to {}, but only {} pages exist",
                ix,
                self.pages.len()
            );
        }
    }

    pub fn current_page(&self) -> Option<&LayoutPage> {
        self.pages.get(self.current_page_ix).or_else(|| self.pages.first())
    }

    pub fn current_page_ix(&self) -> usize {
        self.current_page_ix
    }

    pub fn pages(&self) -> &[LayoutPage] {
        &self.pages
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct LayoutPage {
    pub label: SharedString,
    pub tiles: Vec<LayoutTile>,
}

impl LayoutPage {
    pub fn to_tile_grid_state(
        &self,
        cell_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> TileGridState {
        let mut tile_grid_state = TileGridState::new();
        for tile in &self.tiles {
            match tile.kind {
                LayoutTileKind::Fixtures => {
                    tile_grid_state.add_tile(FixturesTile::new(window, cx), tile.bounds);
                }
                LayoutTileKind::GroupPool => {
                    let delegate = cx.new(|_cx| GroupPoolTile::new());
                    tile_grid_state.add_tile(PoolTile::new(delegate, cell_size), tile.bounds);
                }
                LayoutTileKind::EffectPool => {
                    let delegate = cx.new(|_cx| EffectPoolTile::new());
                    tile_grid_state.add_tile(PoolTile::new(delegate, cell_size), tile.bounds);
                }
                LayoutTileKind::CueListPool => {
                    let delegate = cx.new(|_cx| CueListsPoolTile::new());
                    tile_grid_state.add_tile(PoolTile::new(delegate, cell_size), tile.bounds);
                }
            }
        }

        tile_grid_state
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LayoutTile {
    pub bounds: Bounds<u32>,
    pub kind: LayoutTileKind,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LayoutTileKind {
    Fixtures,

    GroupPool,
    EffectPool,
    CueListPool,
}

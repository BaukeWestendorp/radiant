use std::num::NonZeroU32;

pub use gpui::prelude::*;
use gpui::{App, Entity, FontWeight, Pixels, Size, Window, bounds, div, point, px, size};
use rd_engine::{LayoutPage, LayoutTileKind, Object, Slot};
use rd_ui::{PoolTile, PoolTileDelegate, TileGrid, TileGridState, h_flex};

use crate::{
    app::ui::tiles::{
        CueListsPoolTile, DimmerPresetPoolTile, ExecutorsTile, FixturesTile, GroupPoolTile,
    },
    engine::EngineManager,
};

const TILE_GRID_SIZE: Size<u32> = size(18, 12);

pub struct LayoutViewer {
    tile_grid: Entity<TileGridState>,
    page_selector: Entity<LayoutPageSelector>,
}

impl LayoutViewer {
    const CELL_SIZE: Size<Pixels> = size(px(80.0), px(80.0));

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let tile_grid = cx.new(|_| TileGridState::new());

        let selected_page = cx.new(|cx| {
            EngineManager::snapshot(cx).objects().layout_pages().all().first().map(|lp| lp.slot())
        });

        let page_selector = cx.new(|cx| LayoutPageSelector::new(selected_page.clone(), window, cx));

        cx.observe_in(&selected_page, window, |this, selected_page, window, cx| {
            let next_state = match selected_page.read(cx) {
                Some(selected_page) => {
                    let selected_page = EngineManager::snapshot(cx)
                        .objects()
                        .layout_pages()
                        .get_by_slot(selected_page)
                        .expect("selected page should exist")
                        .clone();
                    page_to_tile_grid_state(&selected_page, Self::CELL_SIZE, window, cx)
                }
                None => TileGridState::new(),
            };

            this.tile_grid.update(cx, |state, cx| {
                *state = next_state;
                cx.notify();
            })
        })
        .detach();

        selected_page.update(cx, |_, cx| cx.notify());

        Self { tile_grid, page_selector }
    }
}

impl Render for LayoutViewer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(self.page_selector.clone())
            .child(TileGrid::new(self.tile_grid.clone()))
    }
}

struct LayoutPageSelector {
    tile_grid: Entity<TileGridState>,
}

impl LayoutPageSelector {
    const CELL_SIZE: Size<Pixels> = size(px(120.0), px(80.0));

    pub fn new(
        selected_page: Entity<Option<Slot>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            tile_grid: cx.new(|cx| {
                let mut tile_grid = TileGridState::new();
                tile_grid.add_tile(
                    PoolTile::new(
                        cx.new(|_| LayoutPageSelectorTile::new(selected_page.clone())),
                        Self::CELL_SIZE,
                    )
                    .with_show_header_cell(false),
                    bounds(point(0, 0), size(1, TILE_GRID_SIZE.height)),
                );
                tile_grid
            }),
        }
    }
}

impl Render for LayoutPageSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(Self::CELL_SIZE.width)
            .h(TILE_GRID_SIZE.height as f32 * Self::CELL_SIZE.height)
            .child(
                TileGrid::new(self.tile_grid.clone())
                    .with_grid_size(size(1, TILE_GRID_SIZE.height))
                    .with_cell_size(Self::CELL_SIZE),
            )
    }
}

struct LayoutPageSelectorTile {
    selected_page: Entity<Option<Slot>>,
}

impl LayoutPageSelectorTile {
    fn new(selected_page: Entity<Option<Slot>>) -> Self {
        Self { selected_page }
    }

    pub fn layout_page<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a LayoutPage> {
        EngineManager::snapshot(cx)
            .objects()
            .layout_pages()
            .get_by_slot(&Slot::new(NonZeroU32::new(slot).unwrap()))
    }
}

impl PoolTileDelegate for LayoutPageSelectorTile {
    fn title(&self, _cx: &gpui::App) -> gpui::SharedString {
        "Layout Page Selector".into()
    }

    fn is_occupied(&self, slot: u32, cx: &gpui::App) -> bool {
        self.layout_page(slot, cx).is_ok()
    }

    fn occupied_content(&self, slot: u32, cx: &gpui::App) -> impl IntoElement {
        let name = match self.layout_page(slot, cx) {
            Ok(group) => group.name().to_string(),
            Err(_) => "<unknown>".to_string(),
        };

        let is_selected =
            self.selected_page.read(cx).is_some_and(|selected_slot| selected_slot.as_u32() == slot);

        h_flex()
            .justify_center()
            .size_full()
            .child(div().when(is_selected, |e| e.font_weight(FontWeight::BOLD)).child(name))
    }

    fn on_activate_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut gpui::App) {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());
        self.selected_page.write(cx, Some(slot));
    }
}

fn page_to_tile_grid_state(
    page: &LayoutPage,
    cell_size: Size<Pixels>,
    window: &mut Window,
    cx: &mut App,
) -> TileGridState {
    let mut tile_grid_state = TileGridState::new();
    for tile in page.tiles() {
        let bounds = bounds(point(tile.x(), tile.y()), size(tile.width(), tile.height()));
        match tile.kind() {
            LayoutTileKind::Fixtures => {
                tile_grid_state.add_tile(FixturesTile::new(window, cx), bounds);
            }
            LayoutTileKind::Executors => {
                tile_grid_state.add_tile(ExecutorsTile::new(cell_size, window, cx), bounds);
            }
            LayoutTileKind::GroupPool => {
                let delegate = cx.new(|_cx| GroupPoolTile::new());
                tile_grid_state.add_tile(PoolTile::new(delegate, cell_size), bounds);
            }
            LayoutTileKind::CueListPool => {
                let delegate = cx.new(|_cx| CueListsPoolTile::new());
                tile_grid_state.add_tile(PoolTile::new(delegate, cell_size), bounds);
            }
            LayoutTileKind::DimmerPresetPool => {
                let delegate = cx.new(|_cx| DimmerPresetPoolTile::new());
                tile_grid_state.add_tile(PoolTile::new(delegate, cell_size), bounds);
            }
        }
    }

    tile_grid_state
}

pub use gpui::prelude::*;
use gpui::{
    App, Entity, FontWeight, Pixels, ReadGlobal, Size, Window, bounds, div, point, px, size,
};
use rd_ui::{PoolTile, PoolTileDelegate, TileGrid, TileGridState, h_flex};

use crate::{
    app::ui::tiles::{CueListsPoolTile, EffectPoolTile, FixturesTile, GroupPoolTile},
    engine::{Engine, LayoutPage, LayoutTileKind},
};

const TILE_GRID_SIZE: Size<u32> = size(18, 12);

pub struct LayoutViewer {
    tile_grid: Entity<TileGridState>,
    page_selector: Entity<LayoutPageSelector>,
}

impl LayoutViewer {
    const CELL_SIZE: Size<Pixels> = size(px(80.0), px(80.0));

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let layout = Engine::global(cx).layout().clone();

        let tile_grid = cx.new(|cx| match layout.read(cx).clone().current_page() {
            Some(current_page) => {
                page_to_tile_grid_state(current_page, Self::CELL_SIZE, window, cx)
            }
            None => TileGridState::new(),
        });

        cx.observe_in(&layout, window, |this, layout, window, cx| {
            let next_state = match layout.read(cx).clone().current_page() {
                Some(current_page) => {
                    page_to_tile_grid_state(current_page, Self::CELL_SIZE, window, cx)
                }
                None => TileGridState::new(),
            };

            this.tile_grid.update(cx, |state, cx| {
                *state = next_state;
                cx.notify();
            })
        })
        .detach();

        let page_selector = cx.new(|cx| LayoutPageSelector::new(window, cx));

        Self { tile_grid, page_selector }
    }
}

impl Render for LayoutViewer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(TileGrid::new(self.tile_grid.clone()))
            .child(self.page_selector.clone())
    }
}

struct LayoutPageSelector {
    tile_grid: Entity<TileGridState>,
}

impl LayoutPageSelector {
    const CELL_SIZE: Size<Pixels> = size(px(120.0), px(80.0));

    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            tile_grid: cx.new(|cx| {
                let mut tile_grid = TileGridState::new();
                tile_grid.add_tile(
                    PoolTile::new(cx.new(|_| LayoutPageSelectorTile), Self::CELL_SIZE)
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

struct LayoutPageSelectorTile;
impl PoolTileDelegate for LayoutPageSelectorTile {
    fn title(&self, _cx: &gpui::App) -> gpui::SharedString {
        "Layout Page Selector".into()
    }

    fn is_occupied(&self, slot_id: u32, cx: &gpui::App) -> bool {
        let layout = Engine::global(cx).layout().read(cx);
        layout.pages().get(slot_id.saturating_sub(1) as usize).is_some()
    }

    fn occupied_content(&self, slot_id: u32, cx: &gpui::App) -> impl IntoElement {
        let ix = slot_id.saturating_sub(1) as usize;
        let layout = Engine::global(cx).layout().read(cx);

        let label = match layout.pages().get(ix) {
            Some(page) => page.label.to_string(),
            None => "".to_string(),
        };
        let is_selected = layout.current_page_ix() == ix;

        h_flex()
            .justify_center()
            .size_full()
            .child(div().when(is_selected, |e| e.font_weight(FontWeight::BOLD)).child(label))
    }

    fn on_activate_slot(&mut self, slot_id: u32, _window: &mut Window, cx: &mut gpui::App) {
        let layout = Engine::global(cx).layout().clone();
        layout.update(cx, |layout, cx| {
            if let Err(err) = layout.set_current_page(slot_id.saturating_sub(1) as usize) {
                log::error!("Failed to set current layout page: {err}");
            }
            cx.notify();
        });
    }
}

fn page_to_tile_grid_state(
    page: &LayoutPage,
    cell_size: Size<Pixels>,
    window: &mut Window,
    cx: &mut App,
) -> TileGridState {
    let mut tile_grid_state = TileGridState::new();
    for tile in &page.tiles {
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

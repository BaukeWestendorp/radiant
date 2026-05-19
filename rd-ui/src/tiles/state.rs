use gpui::Bounds;

use crate::TileDelegate;

pub struct TileGridState {
    tiles: Vec<Tile>,
}

impl TileGridState {
    pub fn new() -> Self {
        Self { tiles: Vec::new() }
    }

    pub fn add_tile<D: TileDelegate + 'static>(&mut self, delegate: D, bounds: Bounds<u32>) {
        self.tiles.push(Tile::new(delegate, bounds));
    }

    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }
}

pub struct Tile {
    delegate: Box<dyn TileDelegate>,
    bounds: Bounds<u32>,
}

impl Tile {
    pub fn new<D: TileDelegate + 'static>(delegate: D, bounds: Bounds<u32>) -> Self {
        Self { delegate: Box::new(delegate), bounds }
    }

    pub fn delegate(&self) -> &dyn TileDelegate {
        self.delegate.as_ref()
    }

    pub fn delegate_mut(&mut self) -> &mut dyn TileDelegate {
        self.delegate.as_mut()
    }

    pub fn bounds(&self) -> Bounds<u32> {
        self.bounds
    }

    pub fn bounds_mut(&mut self) -> &mut Bounds<u32> {
        &mut self.bounds
    }
}

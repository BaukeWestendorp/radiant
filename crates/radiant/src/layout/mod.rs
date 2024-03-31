pub mod screen;
pub mod window;
pub mod window_grid;

pub use screen::*;
pub use window::*;
pub use window_grid::*;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Layout {
    window_grids: Vec<WindowGrid>,
}

impl Layout {
    pub fn window_grid(&self, id: usize) -> Option<&WindowGrid> {
        self.window_grids.iter().find(|wg| wg.id() == id)
    }
}

use backstage::show::Show;
use gpui::{Empty, IntoElement, Model, WindowContext};

use crate::workspace::layout::window_grid::GridBounds;

use super::PoolWindowDelegate;

pub struct ColorPoolWindowDelegate {
    scroll_offset: i32,
    bounds: GridBounds,
    show: Model<Show>,
    window_id: usize,
}

impl ColorPoolWindowDelegate {
    pub fn new(
        window_id: usize,
        scroll_offset: i32,
        bounds: GridBounds,
        show: Model<Show>,
    ) -> Self {
        Self {
            scroll_offset,
            bounds,
            show,
            window_id,
        }
    }
}

impl PoolWindowDelegate for ColorPoolWindowDelegate {
    fn label(&self) -> String {
        "Colors".to_string()
    }

    fn bounds(&self) -> &GridBounds {
        &self.bounds
    }

    fn scroll_offset(&self) -> i32 {
        self.scroll_offset
    }

    fn render_item_for_id(&self, _id: usize, _cx: &mut WindowContext) -> Option<impl IntoElement> {
        // FIXME: Render color pool item.
        Option::<Empty>::None
    }

    fn window_id(&self) -> usize {
        self.window_id
    }
}

use crate::show::{
    Show,
    asset::{AssetId, Executor},
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct ExecutorPool {}

impl ExecutorPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for ExecutorPool {
    type Item = Executor;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Executors"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(executor) = Show::global(cx).assets.executors.get(&asset_id).map(|eg| eg.read(cx))
        else {
            return None;
        };

        Some(
            div()
                .h_full()
                .flex()
                .flex_col()
                .justify_center()
                .text_center()
                .child(executor.label.clone()),
        )
    }
}

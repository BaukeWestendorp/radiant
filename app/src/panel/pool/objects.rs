use std::num::NonZeroU32;

use gpui::prelude::*;
use gpui::{Window, div};
use radiant::show::{Group, Object, PoolId, Sequence};

use crate::panel::pool::{PoolPanel, PoolPanelDelegate};
use crate::state::with_show;

pub struct ObjectPool<T: Object> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Object> ObjectPool<T> {
    pub fn new() -> Self {
        Self { marker: Default::default() }
    }
}

impl<T: Object + 'static> PoolPanelDelegate for ObjectPool<T> {
    fn cell_has_content(&self, pool_id: NonZeroU32, cx: &mut Context<PoolPanel<Self>>) -> bool {
        let id = with_show(cx, |show| {
            show.objects().get_by_pool_id::<T>(PoolId::new(pool_id)).map(|obj| obj.id())
        });
        id.is_some()
    }

    fn handle_cell_click(
        &self,
        _pool_id: NonZeroU32,
        _event: &gpui::ClickEvent,
        _window: &mut Window,
        _cx: &mut Context<PoolPanel<Self>>,
    ) {
        todo!();
    }

    fn render_cell_content(
        &self,
        pool_id: NonZeroU32,
        _window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement {
        with_show(cx, |show| {
            if let Some(group) = show.objects().get_by_pool_id::<Group>(PoolId::new(pool_id)) {
                div().child(group.name().to_string())
            } else if let Some(sequence) =
                show.objects().get_by_pool_id::<Sequence>(PoolId::new(pool_id))
            {
                div().child(sequence.name().to_string())
            } else {
                div()
            }
        })
    }
}

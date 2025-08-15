use std::num::NonZeroU32;

use gpui::prelude::*;
use gpui::{Window, div};
use radiant::engine::Command;
use radiant::show::{Group, Object, PoolId};

use crate::panel::pool::{PoolPanel, PoolPanelDelegate};
use crate::state::{exec_cmd_and_log_err, with_show};

pub struct ObjectPool<T: Object> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Object> ObjectPool<T> {
    pub fn new() -> Self {
        Self { marker: Default::default() }
    }
}

impl<T: Object + 'static> PoolPanelDelegate for ObjectPool<T> {
    fn cell_has_content(&self, pool_id: NonZeroU32, cx: &mut Context<PoolPanel<Self>>) -> bool
    where
        Self: Sized,
    {
        let id = with_show(cx, |show| {
            show.objects().get_by_pool_id(PoolId::<T>::new(pool_id)).map(|obj| obj.id())
        });
        id.is_some()
    }

    fn handle_cell_click(
        &self,
        pool_id: NonZeroU32,
        _event: &gpui::ClickEvent,
        _window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) where
        Self: Sized,
    {
        if let Some(id) = with_show(cx, |show| {
            show.objects().get_by_pool_id::<Group>(PoolId::new(pool_id)).map(|group| group.id())
        }) {
            exec_cmd_and_log_err(Command::SelectReferencedFixtures { id: id.into() }, cx);
        }
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
            } else {
                div()
            }
        })
    }
}

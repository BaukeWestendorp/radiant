use gpui::prelude::*;
use gpui::{Window, div};
use radiant::engine::Command;
use radiant::show::{AnyObjectId, Object, ObjectId};

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

impl<T: Object + 'static> PoolPanelDelegate for ObjectPool<T>
where
    AnyObjectId: From<ObjectId<T>>,
{
    fn cell_has_content(&mut self, ix: u32, cx: &mut Context<PoolPanel<Self>>) -> bool
    where
        Self: Sized,
    {
        with_show(cx, |show| show.any_object(ObjectId::<T>::new(ix)).is_some())
    }

    fn handle_cell_click(
        &mut self,
        ix: u32,
        _event: &gpui::ClickEvent,
        _window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) where
        Self: Sized,
    {
        let id = ObjectId::<T>::new(ix);
        exec_cmd_and_log_err(Command::SelectReferencedFixtures { id: id.into() }, cx);
    }

    fn render_cell_content(
        &mut self,
        ix: u32,
        _window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement {
        let id = ObjectId::<T>::new(ix);
        let object = with_show(cx, |show| show.any_object(id));
        div().child(object.map(|object| object.name().to_string()).unwrap_or_default())
    }
}

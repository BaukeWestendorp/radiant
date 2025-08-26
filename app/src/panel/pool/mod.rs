use std::num::NonZeroU32;

use gpui::prelude::*;
use gpui::{Bounds, ClickEvent, ElementId, Entity, Window, div};
use radiant::show::{Group, Sequence};
use ui::Disableable;
use ui::org::interactive_container;
use ui::theme::ActiveTheme;
use ui::utils::z_stack;

mod objects;

pub use objects::ObjectPool;

use crate::main_window::CELL_SIZE;

#[derive(Clone)]
pub enum PoolPanelKind {
    Group(Entity<PoolPanel<ObjectPool<Group>>>),
    Sequence(Entity<PoolPanel<ObjectPool<Sequence>>>),
}

pub struct PoolPanel<D: PoolPanelDelegate> {
    bounds: Bounds<u32>,
    delegate: D,
}

impl<D: PoolPanelDelegate> PoolPanel<D> {
    pub fn new(bounds: Bounds<u32>, delegate: D) -> Self {
        Self { bounds, delegate }
    }

    pub fn bounds(&self) -> Bounds<u32> {
        self.bounds
    }
}

impl<D: PoolPanelDelegate + 'static> Render for PoolPanel<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.delegate.render(self.bounds(), window, cx)
    }
}

pub trait PoolPanelDelegate {
    fn cell_has_content(&self, pool_id: NonZeroU32, cx: &mut Context<PoolPanel<Self>>) -> bool
    where
        Self: Sized;

    fn handle_cell_click(
        &self,
        pool_id: NonZeroU32,
        event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) where
        Self: Sized;

    fn render_cell_content(
        &self,
        pool_id: NonZeroU32,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;

    fn render_cell(
        &self,
        pool_id: NonZeroU32,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized + 'static,
    {
        interactive_container(
            ElementId::NamedInteger("pool_cell".into(), u32::from(pool_id).into()),
            None,
        )
        .flex()
        .flex_col()
        .size_full()
        .on_click(cx.listener(move |this, event, window, cx| {
            this.delegate.handle_cell_click(pool_id, event, window, cx);
        }))
        .disabled(!self.cell_has_content(pool_id, cx))
        .child(pool_id.to_string())
        .child(self.render_cell_content(pool_id, window, cx))
    }

    fn render_content(
        &mut self,
        bounds: Bounds<u32>,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized + 'static,
    {
        let area = bounds.size.width * bounds.size.height;
        let mut pool_cells = vec![];
        for ix in 1..area + 1 {
            let pool_id = NonZeroU32::new(ix as u32).unwrap();
            let cell_element = self.render_cell(pool_id, window, cx).into_any_element();
            pool_cells.push(cell_element);
        }

        div()
            .flex()
            .flex_wrap()
            .size_full()
            .children(pool_cells.into_iter().map(|cell| div().size(CELL_SIZE).p_px().child(cell)))
    }

    fn render(
        &mut self,
        bounds: Bounds<u32>,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized + 'static,
    {
        z_stack([
            div()
                .size_full()
                .border_1()
                .border_color(cx.theme().colors.border)
                .rounded(cx.theme().radius)
                .into_any_element(),
            self.render_content(bounds, window, cx).into_any_element(),
        ])
        .size_full()
    }
}

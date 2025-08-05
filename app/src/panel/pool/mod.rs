use std::num::NonZeroU32;

use gpui::prelude::*;
use gpui::{Bounds, ClickEvent, ElementId, Entity, Window, div};
use radiant::show::{Group, Sequence};
use ui::utils::z_stack;
use ui::{ActiveTheme, ContainerStyle, container, interactive_container};

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
    fn cell_has_content(&self, id: NonZeroU32, cx: &mut Context<PoolPanel<Self>>) -> bool
    where
        Self: Sized;

    fn handle_cell_click(
        &self,
        id: NonZeroU32,
        event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) where
        Self: Sized;

    fn render_cell_content(
        &self,
        id: NonZeroU32,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;

    fn render_cell(
        &self,
        id: NonZeroU32,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized + 'static,
    {
        if self.cell_has_content(id, cx) {
            interactive_container(
                ElementId::NamedInteger("pool_cell".into(), u32::from(id).into()),
                None,
            )
            .size_full()
            .child(id.to_string())
            .child(self.render_cell_content(id, window, cx))
            .on_click(cx.listener(move |this, event, window, cx| {
                this.delegate.handle_cell_click(id, event, window, cx);
            }))
            .into_any_element()
        } else {
            container(ContainerStyle::normal(window, cx).disabled())
                .size_full()
                .child(id.to_string())
                .into_any_element()
        }
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
            let id = NonZeroU32::new(ix).unwrap();
            let cell_element = self.render_cell(id, window, cx).into_any_element();
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

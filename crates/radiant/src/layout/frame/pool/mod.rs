use std::ops::{Deref, DerefMut};

use gpui::{Size, Window, div, prelude::*, px};
use show::assets::AssetId;
use ui::{ActiveTheme, ContainerStyle, container, utils::z_stack};

use crate::layout::main::FRAME_CELL_SIZE;

pub mod effect_graph;

pub struct Pool<D: PoolDelegate> {
    delegate: D,
    size: Size<u32>,
}

impl<D: PoolDelegate> Pool<D> {
    pub fn new(delegate: D, size: Size<u32>) -> Self {
        Self { delegate, size }
    }

    fn render_header_cell(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title(cx).to_string();

        container(ContainerStyle {
            background: cx.theme().colors.header_background,
            border: cx.theme().colors.header_border,
            text_color: cx.theme().colors.text,
        })
        .size(FRAME_CELL_SIZE)
        .child(div().h_full().flex().flex_col().justify_center().text_center().child(title))
    }
}

impl<D: PoolDelegate> Deref for Pool<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.delegate
    }
}

impl<D: PoolDelegate> DerefMut for Pool<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.delegate
    }
}

impl<D: PoolDelegate + 'static> Render for Pool<D> {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let area = self.size.width * self.size.height;
        let items = (0..area)
            .map(|id| self.render_cell(AssetId::new(id), w, cx).into_any_element())
            .collect::<Vec<_>>();

        let header_cell = self.render_header_cell(w, cx);

        z_stack([div().size_full().flex().flex_wrap().child(header_cell).children(items)])
            .w(self.size.width as f32 * FRAME_CELL_SIZE)
            .h(self.size.height as f32 * FRAME_CELL_SIZE)
            .overflow_hidden()
    }
}

pub trait PoolDelegate {
    type Item;

    fn title(&self, cx: &mut Context<Pool<Self>>) -> &str
    where
        Self: Sized;

    fn render_cell(
        &mut self,
        asset_id: AssetId<Self::Item>,
        w: &mut Window,
        cx: &mut Context<Pool<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized,
    {
        container(ContainerStyle::normal(w, cx))
            .m_px()
            .size(FRAME_CELL_SIZE - px(2.0))
            .children(self.render_cell_content(asset_id, w, cx))
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        w: &mut Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement>
    where
        Self: Sized;
}

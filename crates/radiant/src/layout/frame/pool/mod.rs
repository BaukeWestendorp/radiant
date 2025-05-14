use std::ops::{Deref, DerefMut};

use crate::show::asset::AssetId;
use gpui::{FocusHandle, Size, Window, div, prelude::*, px};
use ui::{
    ActiveTheme, ContainerStyle, Disableable, InteractiveColor, container, interactive_container,
    utils::z_stack,
};

use crate::layout::main::FRAME_CELL_SIZE;

pub mod effect_graph;
pub mod fixture_group;

pub mod cue;
pub mod executor;
pub mod sequence;

pub mod dimmer_preset;

pub use {cue::*, dimmer_preset::*, effect_graph::*, executor::*, fixture_group::*, sequence::*};

pub struct Pool<D: PoolDelegate> {
    delegate: D,
    size: Size<u32>,

    focus_handle: FocusHandle,
}

impl<D: PoolDelegate> Pool<D> {
    pub fn new(delegate: D, size: Size<u32>, cx: &mut Context<Pool<D>>) -> Self {
        Self { delegate, size, focus_handle: cx.focus_handle() }
    }

    fn render_header_cell(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title(cx).to_string();

        let border_color = if self.focus_handle.contains_focused(w, cx) {
            cx.theme().colors.border_focused
        } else {
            cx.theme().colors.header_border
        };

        container(ContainerStyle {
            background: cx.theme().colors.header_background,
            border: border_color,
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

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_wrap()
            .child(self.render_header_cell(w, cx))
            .children(items)
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
        Self: Sized + 'static,
    {
        let cell_content = self.render_cell_content(asset_id, w, cx).map(|e| e.into_any_element());
        let has_content = cell_content.is_some();
        let cell_content = cell_content.unwrap_or_else(|| div().into_any_element());

        let overlay = div()
            .size_full()
            .pt_neg_0p5()
            .pl_0p5()
            .text_color(cx.theme().colors.text.muted())
            .child(asset_id.as_u32().to_string());

        interactive_container(asset_id.as_u32() as usize, None)
            .size(FRAME_CELL_SIZE - px(2.0))
            .m_px()
            .cursor_pointer()
            .disabled(!has_content)
            .on_click(cx.listener(move |this, _event, _w, cx| {
                this.on_select(asset_id, cx);
                cx.notify();
            }))
            .child(
                z_stack([cell_content.into_any_element(), overlay.into_any_element()]).size_full(),
            )
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        w: &mut Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement>
    where
        Self: Sized;

    fn on_select(&mut self, _asset_id: AssetId<Self::Item>, _cx: &mut Context<Pool<Self>>)
    where
        Self: Sized,
    {
    }
}

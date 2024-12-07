pub mod cue;
pub mod effect_graph;
pub mod group;

use gpui::*;
use prelude::FluentBuilder;
use show::AnyAssetId;
use ui::{interactive_container, theme::ActiveTheme, z_stack, StyledExt};

use super::{FrameDelegate, FrameView, GRID_SIZE};

pub use cue::*;
pub use effect_graph::*;
pub use group::*;

pub trait PoolDelegate {
    fn title(&self, cx: &mut WindowContext) -> String;

    fn render_pool_item(
        &mut self,
        id: AnyAssetId,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement>;

    fn on_select(&mut self, _id: AnyAssetId, _cx: &mut WindowContext) {}

    fn on_new(&mut self, _id: AnyAssetId, _cx: &mut WindowContext) {}
}

pub struct PoolFrameDelegate<D: PoolDelegate> {
    size: Size<u32>,
    pub pool_delegate: D,
}

impl<D: PoolDelegate + 'static> PoolFrameDelegate<D> {
    pub fn new(size: Size<u32>, pool_delegate: D) -> Self {
        Self {
            size,
            pool_delegate,
        }
    }

    fn render_header_cell(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> Div {
        let title = self.title(cx).to_string();
        let split_title = title
            .split_whitespace()
            .map(String::from)
            .map(|s| div().child(s).h_flex().justify_center().h(cx.line_height()))
            .collect::<Vec<_>>();

        div()
            .size(px(GRID_SIZE))
            .bg(cx.theme().frame_header_background)
            .border_1()
            .border_color(cx.theme().frame_header_border)
            .rounded(cx.theme().radius)
            .v_flex()
            .justify_center()
            .font_weight(FontWeight::SEMIBOLD)
            .text_sm()
            .text_color(cx.theme().frame_header_text_color)
            .children(split_title)
    }
}

impl<D: PoolDelegate + 'static> FrameDelegate for PoolFrameDelegate<D> {
    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String {
        self.pool_delegate.title(cx)
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        let header_cell = self.render_header_cell(cx);

        let items = (0..self.size.width * self.size.height).map(|id| {
            let pool_item = self
                .pool_delegate
                .render_pool_item(AnyAssetId(id), cx)
                .map(|e| e.into_element());
            let has_content = pool_item.is_some();

            let content = interactive_container(
                ElementId::NamedInteger("pool-item".into(), id as usize),
                !has_content,
                false,
                cx,
            )
            .size_full()
            .children(pool_item);

            let overlay = div()
                .size_full()
                .pt_1()
                .pl_2()
                .text_color(cx.theme().text_muted)
                .child(id.to_string());

            z_stack([content.into_any_element(), overlay.into_any_element()])
                .size(px(GRID_SIZE - 2.0))
                .m_px()
                .when(has_content, |e| {
                    e.cursor_pointer().on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event, cx| {
                            this.delegate.pool_delegate.on_select(AnyAssetId(id), cx);
                            cx.notify();
                        }),
                    )
                })
                .when(!has_content, |e| {
                    e.on_mouse_down(
                        MouseButton::Right,
                        cx.listener(move |this, _event, cx| {
                            this.delegate.pool_delegate.on_new(AnyAssetId(id), cx);
                            cx.notify();
                        }),
                    )
                })
        });

        z_stack([div()
            .size_full()
            .flex()
            .flex_wrap()
            .child(header_cell)
            .children(items)])
        .w(px(self.size.width as f32 * GRID_SIZE))
        .h(px(self.size.height as f32 * GRID_SIZE))
        .overflow_hidden()
    }

    fn render_header(
        &mut self,
        _cx: &mut ViewContext<FrameView<Self>>,
    ) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }
}

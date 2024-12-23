pub mod effect_graph;
pub mod group;
pub mod sequence;

use gpui::*;
use show::AnyAssetId;
use ui::{z_stack, ActiveTheme, Container, ContainerKind, InteractiveContainer, StyledExt};

use super::{FrameDelegate, FrameView, GRID_SIZE};

pub use effect_graph::*;
pub use group::*;
pub use sequence::*;

pub trait PoolDelegate {
    fn title(&self, cx: &mut WindowContext) -> String;

    fn render_cell_content(
        &mut self,
        id: AnyAssetId,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement>;

    fn on_select(&mut self, _id: AnyAssetId, _cx: &mut WindowContext) {}
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

    fn render_header_cell(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        let title = self.title(cx).to_string();
        let split_title = title
            .split_whitespace()
            .map(String::from)
            .map(|s| div().child(s).h_flex().justify_center().h(cx.line_height()))
            .collect::<Vec<_>>();

        Container::new(ContainerKind::Custom {
            bg: cx.theme().frame_header_background,
            border_color: cx.theme().frame_header_border,
        })
        .size(px(GRID_SIZE))
        .child(
            div()
                .size_full()
                .v_flex()
                .justify_center()
                .font_weight(FontWeight::SEMIBOLD)
                .text_sm()
                .text_color(cx.theme().frame_header_text_color)
                .children(split_title),
        )
    }

    fn render_cell(&mut self, id: u32, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        let cell_content = self
            .pool_delegate
            .render_cell_content(AnyAssetId(id), cx)
            .map(|e| e.into_any_element());
        let has_content = cell_content.is_some();
        let cell_content = cell_content.unwrap_or_else(|| div().into_any_element());

        let overlay = div()
            .size_full()
            .pt_1()
            .pl_2()
            .text_color(cx.theme().text_muted)
            .child(id.to_string());

        InteractiveContainer::new(
            ContainerKind::Element,
            ElementId::NamedInteger("pool-item".into(), id as usize),
            !has_content,
            false,
        )
        .inset(px(1.0))
        .size(px(GRID_SIZE))
        .cursor_pointer()
        .on_click(cx.listener(move |this, _event, cx| {
            this.delegate.pool_delegate.on_select(AnyAssetId(id), cx);
            cx.notify();
        }))
        .child(z_stack([cell_content.into_any_element(), overlay.into_any_element()]).size_full())
    }
}

impl<D: PoolDelegate + 'static> FrameDelegate for PoolFrameDelegate<D> {
    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String {
        self.pool_delegate.title(cx)
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        let header_cell = self.render_header_cell(cx);

        let area = self.size.width * self.size.height;
        let items = (0..area).map(|id| self.render_cell(id, cx));

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

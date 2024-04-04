use gpui::{
    div, px, AnyElement, AppContext, InteractiveElement, Interactivity, IntoElement, Model,
    ParentElement, RenderOnce, ScrollWheelEvent, SharedString, Styled, ViewContext, WindowContext,
};
use smallvec::SmallVec;
use theme::ActiveTheme;
use ui::container::{Container, ContainerStyle};

use super::{WindowDelegate, WindowView};
use crate::geometry::Bounds;
use crate::layout::GRID_SIZE;
use crate::showfile::{PoolWindow, Window, WindowKind};

const ROW_SCROLL_OFFSET_MAX: i32 = 10000;
const SCROLL_SENSITIVITY: f32 = 0.5;

pub trait PoolWindowDelegate
where
    Self: 'static,
{
    fn label(&self) -> String;

    fn window(&self) -> &Model<Window>;

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement>;

    fn render_header_cell(&self, cx: &mut WindowContext) -> AnyElement {
        Container::new(cx)
            .size(GRID_SIZE)
            .container_style(ContainerStyle {
                background: cx.theme().colors().window_header,
                border: cx.theme().colors().window_header_border,
            })
            .flex()
            .justify_center()
            .items_center()
            .child(self.label())
            .into_any_element()
    }

    fn handle_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        window: Model<Window>,
        cx: &mut WindowContext,
    ) where
        Self: Sized + 'static,
    {
        let bounds = bounds(self.window(), cx);
        let scroll_offset = scroll_offset(self.window(), cx);

        let item_count = bounds.area();
        let width = bounds.size.width;

        let scroll_delta_y = event.delta.pixel_delta(px(SCROLL_SENSITIVITY)).y;

        let mut row_offset = scroll_offset;
        row_offset += scroll_delta_y.0 as i32 * width as i32;
        row_offset = row_offset.clamp(0, ROW_SCROLL_OFFSET_MAX - item_count as i32);

        window.update(cx, |window, cx| {
            if let Window {
                kind: WindowKind::Pool(PoolWindow { scroll_offset, .. }),
                ..
            } = window
            {
                *scroll_offset = row_offset;
                cx.notify();
            } else {
                // In production we don't actually want to crash, so let's just do nothing here
                // instead.

                // FIXME: Is there a cleaner way of doing this?
                debug_assert!(
                    false,
                    "Tried updating pool window without having a pool window"
                );
            }
        });
    }

    fn handle_click_item(&mut self, _id: usize, _cx: &mut ViewContext<WindowView<Self>>)
    where
        Self: Sized,
    {
    }
}

impl<T: PoolWindowDelegate + 'static> WindowDelegate for T {
    fn title(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some(self.label().into())
    }

    fn render_header(
        &mut self,
        _cx: &mut ViewContext<WindowView<Self>>,
    ) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }

    fn render_content(&mut self, cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        let bounds = bounds(self.window(), cx);
        let scroll_offset = scroll_offset(self.window(), cx);

        let mut grid = vec![];
        for i in 0..bounds.area() {
            if i == 0 {
                let header_cell = self.render_header_cell(cx);
                grid.push(header_cell);
                continue;
            }
            let id = i + scroll_offset as usize;
            let content = self.render_item_for_id(id, cx);
            let item = div()
                .child(PoolCell::new(id).children(content))
                .on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(move |this, _event, cx| {
                        this.delegate.handle_click_item(id, cx);
                    }),
                )
                .into_any_element();
            grid.push(item);
        }

        div()
            .size_full()
            .flex()
            .flex_wrap()
            .children(grid)
            .on_scroll_wheel(cx.listener(|this, event, cx| {
                this.delegate.handle_scroll(event, this.window.clone(), cx)
            }))
    }
}

fn bounds(window: &Model<Window>, cx: &AppContext) -> Bounds {
    window.read(cx).bounds
}

fn scroll_offset(window: &Model<Window>, cx: &AppContext) -> i32 {
    match window.read(cx).kind {
        WindowKind::Pool(pool_window) => pool_window.scroll_offset,
        _ => panic!("PoolWindowDelegate expects to have a WindowKind::Pool"),
    }
}

#[derive(IntoElement)]
pub struct PoolCell {
    id: usize,
    children: SmallVec<[AnyElement; 1]>,
    interactivity: Interactivity,
}

impl PoolCell {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            children: SmallVec::new(),
            interactivity: Interactivity::default(),
        }
    }
}

impl ParentElement for PoolCell {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl InteractiveElement for PoolCell {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        &mut self.interactivity
    }
}

impl RenderOnce for PoolCell {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let has_content = !self.children.is_empty();

        Container::new(cx)
            .size(GRID_SIZE)
            .relative()
            .child(
                div()
                    .size_full()
                    .absolute()
                    .inset_0()
                    .children(self.children),
            )
            .child(
                div()
                    .absolute()
                    .size_full()
                    .text_sm()
                    .text_color(match has_content {
                        true => cx.theme().colors().text,
                        false => cx.theme().colors().text_muted,
                    })
                    .pl(px(4.0))
                    .child(format!("{}", self.id)),
            )
    }
}

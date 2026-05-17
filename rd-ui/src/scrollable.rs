use gpui::prelude::*;
use gpui::{
    AnyElement, App, ElementId, Entity, Pixels, ScrollHandle, StyleRefinement, Window, div, px,
};
use smallvec::SmallVec;

use crate::util::z_stack;
use crate::{ActiveTheme, StyledExt};

pub const SCROLLBAR_TRACK_WIDTH: Pixels = px(12.0);
pub const SCROLLBAR_PADDING: Pixels = px(2.0);

// FIXME: Implement mouse interaction for scrollbars.
// FIXME: Implement keyboard interaction for Scrollable.
// FIXME: H anv V scrollbars overlap.

fn scrollbar_thumb_size(viewport: Pixels, max_offset: Pixels) -> Pixels {
    let track = viewport - SCROLLBAR_PADDING * 2.0;
    if max_offset > px(0.0) {
        let content = viewport + max_offset;
        (track / content) * track
    } else {
        track
    }
}

fn scrollbar_thumb_pos(track: Pixels, thumb: Pixels, offset: Pixels, max_offset: Pixels) -> Pixels {
    let available = track - thumb - SCROLLBAR_PADDING * 2.0;
    if max_offset > px(0.0) {
        let scroll_ratio = (-offset).clamp(px(0.0), max_offset) / max_offset;
        SCROLLBAR_PADDING + scroll_ratio * available
    } else {
        SCROLLBAR_PADDING
    }
}

fn vertical_scrollbar(cx: &mut App, thumb_height: Pixels, thumb_top: Pixels) -> impl IntoElement {
    div()
        .absolute()
        .top_0()
        .right_0()
        .w(SCROLLBAR_TRACK_WIDTH)
        .h_full()
        .bg(cx.theme().contrast.opacity(0.1))
        .child(
            div()
                .absolute()
                .left_0()
                .w(SCROLLBAR_TRACK_WIDTH)
                .h(thumb_height)
                .top(thumb_top)
                .px(SCROLLBAR_PADDING)
                .child(
                    div()
                        .size_full()
                        .bg(cx.theme().accent)
                        .rounded_full()
                        .when(cx.theme().shadow, |e| e.shadow_md()),
                ),
        )
}

fn horizontal_scrollbar(cx: &mut App, thumb_width: Pixels, thumb_left: Pixels) -> impl IntoElement {
    div()
        .absolute()
        .left_0()
        .bottom_0()
        .h(SCROLLBAR_TRACK_WIDTH)
        .w_full()
        .bg(cx.theme().contrast.opacity(0.1))
        .child(
            div()
                .absolute()
                .left_0()
                .h(SCROLLBAR_TRACK_WIDTH)
                .w(thumb_width)
                .left(thumb_left)
                .py(SCROLLBAR_PADDING)
                .child(
                    div()
                        .size_full()
                        .bg(cx.theme().accent)
                        .rounded_full()
                        .when(cx.theme().shadow, |e| e.shadow_md()),
                ),
        )
}

pub struct ScrollableState {
    scroll_handle: ScrollHandle,
}

impl ScrollableState {
    pub fn new() -> Self {
        Self { scroll_handle: ScrollHandle::new() }
    }

    pub fn scroll_handle(&self) -> &ScrollHandle {
        &self.scroll_handle
    }
}

#[derive(IntoElement)]
pub struct Scrollable {
    id: ElementId,
    state: Entity<ScrollableState>,

    show_scrollbar: bool,

    style: StyleRefinement,
    children: SmallVec<[AnyElement; 2]>,
}

impl Scrollable {
    pub fn new(id: impl Into<ElementId>, state: Entity<ScrollableState>) -> Self {
        Self {
            id: id.into(),
            state,

            show_scrollbar: true,
            style: StyleRefinement::default(),
            children: SmallVec::new(),
        }
    }

    pub fn show_scrollbar(mut self, scrollbar: bool) -> Self {
        self.show_scrollbar = scrollbar;
        self
    }
}

impl RenderOnce for Scrollable {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let scroll_handle = self.state.read(cx).scroll_handle.clone();

        let offset = scroll_handle.offset();
        let max_offset = scroll_handle.max_offset();
        let bounds = scroll_handle.bounds();

        let show_vertical = self.show_scrollbar && max_offset.y > px(0.0);
        let show_horizontal = self.show_scrollbar && max_offset.x > px(0.0);

        let vertical_thumb_height = scrollbar_thumb_size(bounds.size.height, max_offset.y);
        let vertical_thumb_top =
            scrollbar_thumb_pos(bounds.size.height, vertical_thumb_height, offset.y, max_offset.y);

        let horizontal_thumb_width = scrollbar_thumb_size(bounds.size.width, max_offset.x);
        let horizontal_thumb_left =
            scrollbar_thumb_pos(bounds.size.width, horizontal_thumb_width, offset.x, max_offset.x);

        let content = div()
            .id(self.id.clone())
            .track_scroll(&scroll_handle)
            .size_full()
            .overflow_scroll()
            .children(self.children);

        let scroll_handles = div()
            .size_full()
            .when(show_vertical, |e| {
                e.child(vertical_scrollbar(cx, vertical_thumb_height, vertical_thumb_top))
            })
            .when(show_horizontal, |e| {
                e.child(horizontal_scrollbar(cx, horizontal_thumb_width, horizontal_thumb_left))
            });

        z_stack([content.into_any_element(), scroll_handles.into_any_element()])
            .refine_style(&self.style)
    }
}

impl ParentElement for Scrollable {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Scrollable {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

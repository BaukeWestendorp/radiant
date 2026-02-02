use gpui::{
    AnyElement, App, Decorations, MouseButton, Pixels, StyleRefinement, Window, WindowControlArea,
    div, prelude::*, px,
};
use smallvec::SmallVec;

use crate::{ActiveTheme, StyledExt};

pub const TITLE_BAR_HEIGHT: Pixels = px(34.);
#[cfg(target_os = "macos")]
const TITLE_BAR_LEFT_PADDING: Pixels = px(80.);
#[cfg(not(target_os = "macos"))]
const TITLE_BAR_LEFT_PADDING: Pixels = px(12.);
#[cfg(target_os = "macos")]
const TITLE_BAR_RIGHT_PADDING: Pixels = px(9.0);
#[cfg(not(target_os = "macos"))]
const TITLE_BAR_RIGHT_PADDING: Pixels = px(0.0);

#[derive(IntoElement)]
pub struct TitleBar {
    style: StyleRefinement,
    children: SmallVec<[AnyElement; 1]>,
}

impl TitleBar {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: SmallVec::new(),
        }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_client_decorated = matches!(window.window_decorations(), Decorations::Client { .. });
        let is_linux = cfg!(target_os = "linux");
        let is_macos = cfg!(target_os = "macos");

        div()
            .id("title-bar")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(TITLE_BAR_HEIGHT)
            .pl(TITLE_BAR_LEFT_PADDING)
            .pr(TITLE_BAR_RIGHT_PADDING)
            .border_b_1()
            .border_color(cx.theme().title_bar_border)
            .bg(cx.theme().title_bar)
            .refine_style(&self.style)
            .when(is_linux, |e| {
                // FIXME: Add `on_double_click` helper.
                e.on_click(|event, window, _| {
                    if event.click_count() == 2 {
                        window.zoom_window();
                    }
                })
            })
            .when(is_macos, |e| {
                e.on_click(|event, window, _| {
                    // FIXME: Add `on_double_click` helper.
                    if event.click_count() == 2 {
                        window.titlebar_double_click();
                    }
                })
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .id("bar")
                    .window_control_area(WindowControlArea::Drag)
                    .when(window.is_fullscreen(), |e| e.pl_3())
                    .h_full()
                    .justify_between()
                    .flex_shrink_0()
                    .flex_1()
                    .when(is_linux && is_client_decorated, |e| {
                        e.child(
                            div()
                                .top_0()
                                .left_0()
                                .absolute()
                                .size_full()
                                .h_full()
                                .on_mouse_down(MouseButton::Right, move |event, window, _| {
                                    window.show_window_menu(event.position)
                                }),
                        )
                    })
                    .children(self.children),
            )
    }
}

impl ParentElement for TitleBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for TitleBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

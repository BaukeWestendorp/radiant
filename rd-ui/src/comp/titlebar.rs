use gpui::prelude::*;
use gpui::{
    AnyElement, App, Decorations, MouseButton, Pixels, StyleRefinement, Window, WindowControlArea,
    div, px,
};
use smallvec::SmallVec;

use crate::{ActiveTheme, StatefulInteractiveElementExt, h_flex};

pub const TITLE_BAR_HEIGHT: Pixels = px(34.);

#[derive(IntoElement)]
pub struct TitleBar {
    style: StyleRefinement,
    children: SmallVec<[AnyElement; 2]>,
}

impl TitleBar {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: SmallVec::new() }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_client_decorated = matches!(window.window_decorations(), Decorations::Client { .. });
        let is_fullscreen = window.is_fullscreen();
        let is_linux = cfg!(target_os = "linux");
        let is_macos = cfg!(target_os = "macos");

        const DEFAULT_PADDING: Pixels = px(12.0);
        const MACOS_TRAFFIC_LIGHT_PADDING: Pixels = px(80.0);
        let padding_left = if is_macos {
            if is_fullscreen { DEFAULT_PADDING } else { MACOS_TRAFFIC_LIGHT_PADDING }
        } else if is_linux {
            DEFAULT_PADDING
        } else {
            DEFAULT_PADDING
        };

        let padding_right = if is_macos {
            DEFAULT_PADDING
        } else if is_linux {
            DEFAULT_PADDING
        } else {
            DEFAULT_PADDING
        };

        h_flex()
            .id("title_bar")
            .justify_between()
            .min_h(TITLE_BAR_HEIGHT)
            .max_h(TITLE_BAR_HEIGHT)
            .pl(padding_left)
            .pr(padding_right)
            .border_b_1()
            .border_color(cx.theme().title_bar_border)
            .bg(cx.theme().title_bar)
            .when(is_linux, |e| {
                e.on_double_click(|_, window, _| {
                    window.zoom_window();
                })
            })
            .when(is_macos, |e| {
                e.on_double_click(|_, window, _| {
                    window.titlebar_double_click();
                })
            })
            .child(
                h_flex()
                    .id("bar")
                    .window_control_area(WindowControlArea::Drag)
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

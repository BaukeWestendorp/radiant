use std::ops::{Deref, DerefMut};

use gpui::prelude::*;
use gpui::{
    App, ClickEvent, FontWeight, MouseDownEvent, Pixels, TitlebarOptions, Window,
    WindowControlArea, WindowHandle, WindowOptions, div, point, px,
};

use crate::AppExt;
use crate::button::button;
use crate::theme::{ActiveTheme, InteractiveColor};
use crate::utils::z_stack;
use crate::wm::Overlay;

pub const TRAFFIC_LIGHT_WIDTH: Pixels = px(14.0);
pub const TRAFFIC_LIGHT_SPACING: Pixels = px(9.0);

pub trait WindowDelegate: 'static {
    fn create(window: &mut Window, cx: &mut Context<WindowWrapper<Self>>) -> Self
    where
        Self: Sized;

    fn handle_window_save(&self, _window: &mut Window, _cx: &mut Context<WindowWrapper<Self>>)
    where
        Self: Sized,
    {
    }

    fn handle_window_discard(&self, _window: &mut Window, _cx: &mut Context<WindowWrapper<Self>>)
    where
        Self: Sized,
    {
    }

    fn render_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}

pub struct WindowWrapper<D: WindowDelegate> {
    delegate: D,
    window_handle: WindowHandle<Self>,
}

impl<D: WindowDelegate> WindowWrapper<D> {
    pub fn open<F: FnOnce(&mut Window, &mut Context<WindowWrapper<D>>) -> D>(
        cx: &mut App,
        f: F,
    ) -> WindowHandle<Self> {
        cx.open_window(window_options(), |window, cx| {
            cx.new(|cx| {
                let delegate = f(window, cx);
                Self { delegate, window_handle: window.window_handle().downcast().unwrap() }
            })
        })
        .expect("should open window")
    }

    pub fn window_handle(&self) -> WindowHandle<Self> {
        self.window_handle.clone()
    }
}

impl<D: WindowDelegate> Render for WindowWrapper<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let render_overlay = |overlay: &Overlay| {
            let header = div()
                .when(!overlay.is_modal(), |e| e.size_full())
                .min_h_8()
                .max_h_8()
                .px_2()
                .flex()
                .justify_between()
                .items_center()
                .border_1()
                .border_color(cx.theme().header_border)
                .rounded_t(cx.theme().radius)
                .text_color(cx.theme().header_foreground)
                .bg(cx.theme().header)
                .child(div().font_weight(FontWeight::BOLD).child(overlay.title().to_string()))
                .child(button("close", None, "X").size_6().on_click(cx.listener({
                    let overlay_id = overlay.id().to_string();
                    move |this, _: &ClickEvent, _, cx| {
                        let handle = this.window_handle();
                        cx.update_wm(|wm, _| wm.close_overlay(&overlay_id, &handle))
                    }
                })));

            let content = div()
                .when(!overlay.is_modal(), |e| e.size_full())
                .flex()
                .bg(cx.theme().background)
                .border_1()
                .border_t_0()
                .border_color(cx.theme().border)
                .rounded_b(cx.theme().radius)
                .when(cx.theme().shadow, |e| e.shadow_lg())
                .child(overlay.content().clone());

            let container = div()
                .when(!overlay.is_modal(), |e| e.size_full())
                .flex()
                .flex_col()
                .occlude()
                .child(header)
                .child(content);

            div()
                .size_full()
                .p_4()
                .flex()
                .justify_center()
                .items_center()
                .bg(gpui::black().with_opacity(0.5))
                .on_any_mouse_down(cx.listener({
                    let overlay_id = overlay.id().to_string();
                    move |this, _: &MouseDownEvent, _, cx| {
                        let handle = this.window_handle();
                        cx.update_wm(|wm, _| wm.close_overlay(&overlay_id, &handle))
                    }
                }))
                .occlude()
                .child(container)
        };

        let overlay = div()
            .size_full()
            .children(cx.wm().window_overlays(&window.window_handle()).last().map(render_overlay));

        div()
            .size_full()
            .flex()
            .flex_col()
            .font_family("Inter 18pt")
            .text_color(cx.theme().foreground)
            .text_sm()
            .bg(cx.theme().background)
            .child(render_titlebar(window, cx))
            .child(
                z_stack([
                    self.render_content(window, cx).into_any_element(),
                    overlay.into_any_element(),
                ])
                .size_full(),
            )
    }
}

impl<D: WindowDelegate> Deref for WindowWrapper<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.delegate
    }
}

impl<D: WindowDelegate> DerefMut for WindowWrapper<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.delegate
    }
}

pub fn window_options() -> WindowOptions {
    WindowOptions {
        window_bounds: None,
        titlebar: Some(TitlebarOptions {
            appears_transparent: true,
            traffic_light_position: Some(point(TRAFFIC_LIGHT_SPACING, TRAFFIC_LIGHT_SPACING)),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn render_titlebar(window: &Window, cx: &App) -> impl IntoElement {
    let titlebar_height = px(32.0);

    div()
        .id("titlebar")
        .window_control_area(WindowControlArea::Drag)
        .w_full()
        .min_h(titlebar_height)
        .max_h(titlebar_height)
        .pl(TRAFFIC_LIGHT_WIDTH * 3 + TRAFFIC_LIGHT_SPACING * 4)
        .pr(TRAFFIC_LIGHT_SPACING)
        .border_b_1()
        .border_color(cx.theme().title_bar_border)
        .bg(cx.theme().title_bar)
        .flex()
        .items_center()
        .child(
            div()
                .font_weight(FontWeight::BOLD)
                .text_color(cx.theme().foreground.muted())
                .child(window.window_title()),
        )
        .on_click(|event, window, _| {
            if event.click_count() == 2 {
                window.titlebar_double_click();
            }
        })
}

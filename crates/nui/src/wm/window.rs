use std::ops::{Deref, DerefMut};

use gpui::prelude::*;
use gpui::{App, Window, WindowHandle, WindowOptions, div, px};

use crate::theme::ActiveTheme;

pub trait WindowDelegate: 'static {
    fn create(window: &mut Window, cx: &mut App) -> Self
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
    pub fn open<F: FnOnce(&mut Window, &mut App) -> D>(cx: &mut App, f: F) -> WindowHandle<Self> {
        cx.open_window(window_options(), |window, cx| {
            let delegate = f(window, cx);
            cx.new(|_| Self { delegate, window_handle: window.window_handle().downcast().unwrap() })
        })
        .expect("should open window")
    }

    pub fn window_handle(&self) -> WindowHandle<Self> {
        self.window_handle.clone()
    }
}

impl<D: WindowDelegate> Render for WindowWrapper<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .text_color(cx.theme().foreground)
            .font_family("Tamzen")
            .line_height(px(14.0))
            .bg(cx.theme().background)
            .child(self.render_content(window, cx))
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
    WindowOptions { window_bounds: None, ..Default::default() }
}

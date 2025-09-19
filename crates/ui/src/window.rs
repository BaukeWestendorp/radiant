use std::any::TypeId;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use gpui::prelude::*;
use gpui::{
    AnyWindowHandle, App, Entity, Global, TitlebarOptions, UpdateGlobal, Window, WindowHandle,
    WindowOptions, div,
};

use crate::misc::TRAFFIC_LIGHT_POSITION;
use crate::org::root;
use crate::overlay::OverlayContainer;
use crate::utils::z_stack;

pub trait WindowDelegate: 'static {
    fn render_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;

    fn render_titlebar_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized,
    {
        div()
    }
}

pub struct WindowWrapper<D: WindowDelegate> {
    delegate: D,

    overlays: Entity<OverlayContainer>,
}

impl<D: WindowDelegate> WindowWrapper<D> {
    pub fn open<F: FnOnce(&mut Window, &mut App) -> D>(cx: &mut App, f: F) -> WindowHandle<Self> {
        cx.open_window(window_options(), |window, cx| {
            let delegate = f(window, cx);
            cx.new(|cx| Self { delegate, overlays: cx.new(|_| OverlayContainer::new()) })
        })
        .expect("should open window")
    }

    pub fn overlays(&self) -> Entity<OverlayContainer> {
        self.overlays.clone()
    }
}

impl<D: WindowDelegate> Render for WindowWrapper<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        root().flex().flex_col().titlebar_child(self.render_titlebar_content(window, cx)).child(
            z_stack([
                self.render_content(window, cx).into_any_element(),
                self.overlays.clone().into_any_element(),
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
            traffic_light_position: Some(TRAFFIC_LIGHT_POSITION),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[derive(Default)]
pub struct WindowManager {
    opened_windows: HashMap<TypeId, AnyWindowHandle>,
}

impl WindowManager {
    pub fn init(cx: &mut App) {
        cx.set_global(Self::default())
    }

    pub fn close_all_windows(cx: &mut App) {
        Self::update_global(cx, |this, cx| {
            for (_, handle) in this.opened_windows.drain() {
                Self::close_any_window(handle, cx);
            }
        });
    }

    pub fn open_window<D: WindowDelegate, F: FnOnce(&mut Window, &mut App) -> D>(
        cx: &mut App,
        f: F,
    ) {
        Self::close_window::<D>(cx);

        let handle = WindowWrapper::open(cx, f);

        cx.defer(move |cx| {
            Self::update_global(cx, |this, _| {
                this.opened_windows.insert(TypeId::of::<D>(), handle.into());
            });
        });
    }

    pub fn close_window<D: WindowDelegate>(cx: &mut App) {
        cx.defer(move |cx| {
            Self::update_global(cx, |this, cx| {
                if let Some(handle) = this.opened_windows.remove(&TypeId::of::<D>()) {
                    Self::close_any_window(handle.into(), cx);
                }
            });
        });
    }

    fn close_any_window(handle: AnyWindowHandle, cx: &mut App) {
        cx.update_window(handle, |_, window, _| window.remove_window()).ok();
    }
}

impl Global for WindowManager {}

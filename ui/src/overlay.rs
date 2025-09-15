use gpui::prelude::*;
use gpui::{AnyView, App, Global, ReadGlobal, UpdateGlobal, div};

pub(crate) fn init(cx: &mut App) {
    cx.set_global(OverlayManager::new());
}

pub trait OverlayExt {
    fn open_overlay(&mut self, overlay: impl Into<AnyView>);

    fn close_overlay(&mut self);
}

impl OverlayExt for App {
    fn open_overlay(&mut self, overlay: impl Into<AnyView>) {
        OverlayManager::update_global(self, |mm, _| {
            mm.overlay = Some(overlay.into());
        });
    }

    fn close_overlay(&mut self) {
        self.defer(|cx| {
            OverlayManager::update_global(cx, |mm, _| {
                mm.overlay = None;
            });
        });
    }
}

pub(crate) struct OverlayManager {
    overlay: Option<AnyView>,
}

impl OverlayManager {
    pub fn new() -> Self {
        Self { overlay: None }
    }
}

impl Global for OverlayManager {}

pub(crate) fn overlay_container(cx: &App) -> impl IntoElement {
    let overlay = OverlayManager::global(cx).overlay.clone();
    div()
        .size_full()
        .flex()
        .children(overlay.map(|overlay| div().size_full().occlude().child(overlay)))
}

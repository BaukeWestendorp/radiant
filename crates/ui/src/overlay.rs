use std::collections::HashMap;

use gpui::prelude::*;
use gpui::{AnyView, Render, SharedString, Window, div};

use crate::utils::z_stack;

pub struct OverlayContainer {
    overlays: HashMap<SharedString, Overlay>,
}

impl OverlayContainer {
    pub fn new() -> Self {
        Self { overlays: HashMap::new() }
    }

    pub fn open(&mut self, id: impl Into<OverlayId>, overlay: Overlay, cx: &mut Context<Self>) {
        self.overlays.insert(id.into(), overlay);
        cx.notify();
    }

    pub fn close(&mut self, id: impl Into<OverlayId>, cx: &mut Context<Self>) {
        self.overlays.remove(&id.into());
        cx.notify();
    }
}

impl Render for OverlayContainer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let render_overlay = |overlay: Overlay| div().size_full().child(overlay.view);

        let overlays = self.overlays.values().cloned().map(render_overlay);

        z_stack(overlays).size_full()
    }
}

#[derive(Clone)]
pub struct Overlay {
    pub title: SharedString,
    pub view: AnyView,
}

impl Overlay {
    pub fn new(title: impl Into<SharedString>, view: impl Into<AnyView>) -> Self {
        Self { title: title.into(), view: view.into() }
    }
}

pub type OverlayId = SharedString;

use gpui::prelude::*;
use gpui::{AnyView, App, FontWeight, Global, ReadGlobal, SharedString, UpdateGlobal, div};

use crate::interactive::button::button;
use crate::theme::ActiveTheme;

pub(crate) fn init(cx: &mut App) {
    cx.set_global(OverlayManager::new());
}

pub trait OverlayExt {
    fn open_overlay(&mut self, title: impl Into<SharedString>, overlay: impl Into<AnyView>);

    fn close_overlay(&mut self);
}

impl OverlayExt for App {
    fn open_overlay(&mut self, title: impl Into<SharedString>, overlay: impl Into<AnyView>) {
        OverlayManager::update_global(self, |om, _| {
            om.overlay = Some((title.into(), overlay.into()));
        });
    }

    fn close_overlay(&mut self) {
        self.defer(|cx| {
            OverlayManager::update_global(cx, |om, _| {
                om.overlay = None;
            });
        });
    }
}

pub(crate) struct OverlayManager {
    overlay: Option<(SharedString, AnyView)>,
}

impl OverlayManager {
    pub fn new() -> Self {
        Self { overlay: None }
    }
}

impl Global for OverlayManager {}

pub(crate) fn overlay_container(cx: &App) -> impl IntoElement {
    let overlay = OverlayManager::global(cx).overlay.clone();

    div().size_full().children(overlay.map(|(title, content)| {
        let header = div()
            .w_full()
            .h_10()
            .flex()
            .justify_between()
            .items_center()
            .p_2()
            .bg(cx.theme().header)
            .border_1()
            .border_color(cx.theme().header_border)
            .text_color(cx.theme().header_foreground)
            .rounded_t(cx.theme().radius)
            .child(div().font_weight(FontWeight::BOLD).child(title))
            .child(button("close", None, "X").on_click(|_, _, cx| cx.close_overlay()));

        div().flex().flex_col().size_full().p_2().occlude().child(header).child(content)
    }))
}

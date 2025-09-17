use gpui::prelude::*;
use gpui::{
    AnyView, App, AppContext, FocusHandle, Focusable, FontWeight, Global, ReadGlobal, SharedString,
    UpdateGlobal, Window, div,
};

use crate::org::container;
use crate::theme::ActiveTheme;

pub mod actions {
    use gpui::{App, KeyBinding};

    gpui::actions!(modal, [Close]);

    pub const KEY_CONTEXT: &str = "Modal";

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", Close, Some(KEY_CONTEXT))]);
    }
}

pub(crate) fn init(cx: &mut App) {
    cx.set_global(ModalManager::new());
}

pub struct Modal<D: ModalDelegate> {
    title: SharedString,

    pub delegate: D,
    focus_handle: FocusHandle,
}

impl<D: ModalDelegate + 'static> Modal<D> {
    fn new(title: SharedString, delegate: D, focus_handle: FocusHandle) -> Self {
        Self { title, delegate, focus_handle }
    }

    fn handle_close(
        &mut self,
        _event: &actions::Close,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.close_modal();
        cx.notify();
    }
}

impl<D: ModalDelegate + 'static> Focusable for Modal<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub trait ModalDelegate {
    fn render_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Modal<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}

impl<D: ModalDelegate + 'static> Render for Modal<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let container = container(window, cx)
            .size_full()
            .when(cx.theme().shadow, |e| e.shadow_md())
            .child(
                div()
                    .p_2()
                    .font_weight(FontWeight::BOLD)
                    .child(self.title.clone())
                    .border_b_1()
                    .border_color(cx.theme().modal_border),
            )
            .child(div().p_2().child(self.delegate.render_content(window, cx)));

        div()
            .occlude()
            .id("modal")
            .track_focus(&self.focus_handle)
            .key_context(actions::KEY_CONTEXT)
            .on_action(cx.listener(Self::handle_close))
            .on_mouse_down_out(
                cx.listener(|this, _, window, cx| this.handle_close(&actions::Close, window, cx)),
            )
            .child(container)
    }
}

pub trait ModalExt {
    fn open_modal<
        D: ModalDelegate + 'static,
        F: FnOnce(FocusHandle, &mut Window, &mut Context<Modal<D>>) -> D,
    >(
        &mut self,
        title: impl Into<SharedString>,
        window: &mut Window,
        modal_builder: F,
    );

    fn close_modal(&mut self);
}

impl ModalExt for App {
    fn open_modal<
        D: ModalDelegate + 'static,
        F: FnOnce(FocusHandle, &mut Window, &mut Context<Modal<D>>) -> D,
    >(
        &mut self,
        title: impl Into<SharedString>,
        window: &mut Window,
        modal_builder: F,
    ) {
        ModalManager::update_global(self, |mm, cx| {
            let focus_handle = cx.focus_handle();
            let modal = cx.new(|cx| {
                Modal::new(
                    title.into(),
                    modal_builder(focus_handle.clone(), window, cx),
                    focus_handle.clone(),
                )
            });
            window.defer(cx, move |window, _| window.focus(&focus_handle));
            mm.modal = Some(modal.into());
        });
    }

    fn close_modal(&mut self) {
        self.defer(|cx| {
            ModalManager::update_global(cx, |mm, _| {
                mm.modal = None;
            });
        });
    }
}

pub(crate) struct ModalManager {
    modal: Option<AnyView>,
}

impl ModalManager {
    pub fn new() -> Self {
        Self { modal: None }
    }
}

impl Global for ModalManager {}

pub(crate) fn modal_container(cx: &App) -> impl IntoElement {
    let modal = ModalManager::global(cx).modal.clone();
    div().size_full().flex().justify_center().items_center().children(modal)
}

use gpui::prelude::*;
use gpui::{
    AnyView, App, AppContext, FocusHandle, Focusable, Global, KeyBinding, ReadGlobal, UpdateGlobal,
    Window, div,
};

use crate::org::interactive_container;

pub mod actions {
    gpui::actions!(modal, [Close]);
}

pub const MODAL_KEY_CONTEXT: &str = "Modal";

pub(super) fn init(cx: &mut App) {
    cx.set_global(ModalManager::new());

    cx.bind_keys([KeyBinding::new("escape", actions::Close, Some(MODAL_KEY_CONTEXT))]);
}

pub struct Modal<D: ModalDelegate> {
    pub delegate: D,
    focus_handle: FocusHandle,
}

impl<D: ModalDelegate + 'static> Modal<D> {
    fn new(delegate: D, cx: &mut Context<Self>) -> Self {
        Self { delegate, focus_handle: cx.focus_handle() }
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
        interactive_container("modal", Some(self.focus_handle(cx)))
            .key_context(MODAL_KEY_CONTEXT)
            .on_action(cx.listener(Self::handle_close))
            .w_40()
            .h_32()
            .p_2()
            .child(self.delegate.render_content(window, cx))
    }
}

pub trait ModalExt {
    fn open_modal<D: ModalDelegate + 'static, F: FnOnce(&mut Context<Modal<D>>) -> D>(
        &mut self,
        modal_builder: F,
    );

    fn close_modal(&mut self);
}

impl ModalExt for App {
    fn open_modal<D: ModalDelegate + 'static, F: FnOnce(&mut Context<Modal<D>>) -> D>(
        &mut self,
        modal_builder: F,
    ) {
        ModalManager::update_global(self, |mm, cx| {
            mm.modal = Some(cx.new(|cx| Modal::new(modal_builder(cx), cx)).into());
        });
    }

    fn close_modal(&mut self) {
        ModalManager::update_global(self, |mm, _| {
            mm.modal = None;
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

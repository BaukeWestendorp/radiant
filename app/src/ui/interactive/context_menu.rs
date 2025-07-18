use gpui::prelude::*;
use gpui::{Action, DismissEvent, EventEmitter, FocusHandle, SharedString, Window, div};

use crate::ui::{ContainerStyle, container, divider, interactive_container};

pub enum ContextMenuItem {
    Action { label: SharedString, action: Box<dyn Action>, destructive: bool },
    Divider,
}

pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    focus_handle: FocusHandle,
}

impl ContextMenu {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle);

        let _on_blur_subscription =
            cx.on_blur(&focus_handle, window, |this: &mut ContextMenu, window, cx| {
                this.handle_cancel(&actions::Cancel, window, cx)
            });

        window.refresh();

        Self { items: Vec::new(), focus_handle }
    }

    pub fn divider(mut self) -> Self {
        self.items.push(ContextMenuItem::Divider);
        self
    }

    pub fn action<A: Action>(mut self, label: impl Into<SharedString>, action: Box<A>) -> Self {
        self.items.push(ContextMenuItem::Action {
            label: label.into(),
            action,
            destructive: false,
        });
        self
    }

    pub fn destructive_action<A: Action>(
        mut self,
        label: impl Into<SharedString>,
        action: Box<A>,
    ) -> Self {
        self.items.push(ContextMenuItem::Action { label: label.into(), action, destructive: true });
        self
    }
}

impl ContextMenu {
    fn handle_select_next(
        &mut self,
        _: &actions::SelectNext,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn handle_select_previous(
        &mut self,
        _: &actions::SelectPrevious,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn handle_confirm(
        &mut self,
        _: &actions::Confirm,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn handle_cancel(&mut self, _: &actions::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
        cx.emit(DismissEvent);
    }
}

impl ContextMenu {
    fn render_item(&self, item: &ContextMenuItem, cx: &Context<Self>) -> impl IntoElement {
        match item {
            ContextMenuItem::Action { label, action, destructive } => {
                let action = action.boxed_clone();
                interactive_container(SharedString::new(format!("item-{}", label)), None)
                    .destructive(*destructive)
                    .cursor_pointer()
                    .mx_1()
                    .on_click(move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx))
                    .child(div().px_1().child(label.clone()))
                    .into_any_element()
            }
            ContextMenuItem::Divider => divider(cx).w_full().into_any_element(),
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        container(ContainerStyle::normal(window, cx)).occlude().shadow_md().child(
            div()
                .track_focus(&self.focus_handle)
                .key_context(actions::KEY_CONTEXT)
                .id("context_menu")
                .size_full()
                .my_1()
                .flex()
                .flex_col()
                .gap_1()
                .overflow_y_scroll()
                .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                    this.handle_cancel(&actions::Cancel, window, cx)
                }))
                .on_action(cx.listener(Self::handle_select_next))
                .on_action(cx.listener(Self::handle_select_previous))
                .on_action(cx.listener(Self::handle_confirm))
                .on_action(cx.listener(Self::handle_cancel))
                .children(self.items.iter().map(|item| self.render_item(item, cx))),
        )
    }
}

impl EventEmitter<DismissEvent> for ContextMenu {}

pub mod actions {
    use gpui::{App, KeyBinding, actions};

    pub const KEY_CONTEXT: &str = "ContextMenu";

    actions!(graph_editor, [SelectNext, SelectPrevious, Confirm, Cancel]);

    pub fn init(cx: &mut App) {
        bind_keys(cx);
    }

    fn bind_keys(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("down", SelectNext, Some(KEY_CONTEXT)),
            KeyBinding::new("up", SelectPrevious, Some(KEY_CONTEXT)),
            KeyBinding::new("enter", Confirm, Some(KEY_CONTEXT)),
            KeyBinding::new("escape", Cancel, Some(KEY_CONTEXT)),
        ]);
    }
}

use gpui::{
    Div, ElementId, EventEmitter, FocusHandle, SharedString, Window, div, prelude::*, uniform_list,
};
use smallvec::SmallVec;

use crate::{ContainerStyle, Selectable, interactive_container};

use super::SubmitEvent;

pub struct ListItem {
    label: SharedString,
}

impl ListItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into() }
    }
}

pub struct List {
    items: SmallVec<[ListItem; 2]>,
    selected_index: Option<usize>,
    id: ElementId,
    focus_handle: FocusHandle,
}

impl List {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        items: impl IntoIterator<Item = ListItem>,
    ) -> Self {
        Self {
            items: SmallVec::from_iter(items),
            selected_index: None,
            id: id.into(),
            focus_handle,
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn select_index(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected_index = Some(index);
        cx.notify();
    }

    pub fn unselect(&mut self, cx: &mut Context<Self>) {
        self.selected_index = None;
        cx.notify();
    }

    fn render_item(
        &self,
        item: &ListItem,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        div().px_1().child(
            interactive_container(
                ElementId::NamedInteger(self.id.to_string().into(), index as u64),
                None,
            )
            .normal_container_style(ContainerStyle {
                background: gpui::transparent_black(),
                border: gpui::transparent_black(),
                text_color: window.text_style().color,
            })
            .selected(self.selected_index == Some(index))
            .w_full()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _event, window, cx| {
                this.select_index(index, cx);
                this.handle_submit(&actions::Submit, window, cx);
            }))
            .child(div().mx_1().child(item.label.clone())),
        )
    }
}

impl List {
    fn handle_select_next_item(
        &mut self,
        _event: &actions::SelectNextItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.selected_index {
            Some(ix) => {
                self.selected_index = Some((ix + 1) % self.items.len());
            }
            None => {
                self.selected_index = Some(0);
            }
        }
        cx.notify();
    }

    fn handle_select_previous_item(
        &mut self,
        _event: &actions::SelectPreviousItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.selected_index {
            Some(ix) => {
                self.selected_index = Some((ix + self.items.len() - 1) % self.items.len());
            }
            None => {
                self.selected_index = Some(self.items.len() - 1);
            }
        }
        cx.notify();
    }

    fn handle_submit(
        &mut self,
        _event: &actions::Submit,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.emit(SubmitEvent);
    }
}

impl Render for List {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        uniform_list(
            cx.entity(),
            self.id.clone(),
            self.items.len(),
            |this, visible_range, window, cx| {
                let start_ix = visible_range.start;
                this.items[visible_range]
                    .iter()
                    .enumerate()
                    .map(|(ix, item)| this.render_item(item, start_ix + ix, window, cx))
                    .collect()
            },
        )
        .track_focus(&self.focus_handle)
        .key_context(actions::KEY_CONTEXT)
        .py_1()
        .size_full()
        .on_action(cx.listener(Self::handle_select_next_item))
        .on_action(cx.listener(Self::handle_select_previous_item))
        .on_action(cx.listener(Self::handle_submit))
    }
}

impl EventEmitter<SubmitEvent> for List {}

pub mod actions {
    use gpui::{App, KeyBinding, actions};

    pub const KEY_CONTEXT: &str = "List";

    actions!(new_node_menu, [SelectNextItem, SelectPreviousItem, Submit]);

    pub fn init(cx: &mut App) {
        bind_keys(cx);
    }

    fn bind_keys(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("up", SelectPreviousItem, Some(KEY_CONTEXT)),
            KeyBinding::new("down", SelectNextItem, Some(KEY_CONTEXT)),
            KeyBinding::new("enter", Submit, Some(KEY_CONTEXT)),
        ]);
    }
}

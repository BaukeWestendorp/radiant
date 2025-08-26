use std::ops::Range;

use gpui::prelude::*;
use gpui::{
    App, Div, ElementId, EventEmitter, FocusHandle, KeyBinding, SharedString, Window, div,
    uniform_list,
};
use smallvec::SmallVec;

use crate::Selectable;
use crate::interactive::event::SubmitEvent;
use crate::org::{ContainerStyle, interactive_container};

pub const LIST_KEY_CONTEXT: &str = "List";

pub mod actions {
    gpui::actions!(new_node_menu, [SelectNextItem, SelectPreviousItem, Submit]);
}

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", actions::SelectPreviousItem, Some(LIST_KEY_CONTEXT)),
        KeyBinding::new("down", actions::SelectNextItem, Some(LIST_KEY_CONTEXT)),
        KeyBinding::new("enter", actions::Submit, Some(LIST_KEY_CONTEXT)),
    ]);
}

pub struct List<T> {
    items: SmallVec<[T; 2]>,
    selected_index: Option<usize>,
    id: ElementId,
    focus_handle: FocusHandle,
    get_item_label: Box<dyn Fn(&T) -> SharedString>,
}

impl<T: 'static> List<T> {
    pub fn new<F: Fn(&T) -> SharedString + 'static>(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        items: impl IntoIterator<Item = T>,
        get_item_label: F,
    ) -> Self {
        Self {
            items: SmallVec::from_iter(items),
            selected_index: None,
            id: id.into(),
            focus_handle,
            get_item_label: Box::new(get_item_label),
        }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.items.get(self.selected_index?)
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
        item: &T,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let label = (self.get_item_label)(item);

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
            .child(div().mx_1().child(label)),
        )
    }
}

impl<T: 'static> List<T> {
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

impl<T: 'static> Render for List<T> {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        uniform_list(
            self.id.clone(),
            self.items.len(),
            cx.processor(|this, visible_range: Range<usize>, window, cx| {
                let start_ix = visible_range.start;
                this.items[visible_range]
                    .iter()
                    .enumerate()
                    .map(|(ix, item)| this.render_item(item, start_ix + ix, window, cx))
                    .collect()
            }),
        )
        .track_focus(&self.focus_handle)
        .key_context(LIST_KEY_CONTEXT)
        .py_1()
        .size_full()
        .on_action(cx.listener(Self::handle_select_next_item))
        .on_action(cx.listener(Self::handle_select_previous_item))
        .on_action(cx.listener(Self::handle_submit))
    }
}

impl<T: 'static> EventEmitter<SubmitEvent> for List<T> {}

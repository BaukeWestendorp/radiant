use gpui::prelude::*;
use gpui::{AnyElement, App, ElementId, Entity, Window, div};

use crate::{ActiveTheme, Button, h_flex, v_flex};

#[derive(Debug, Clone)]
pub struct TabsState {
    selected: Option<ElementId>,
}

impl TabsState {
    pub fn new() -> Self {
        Self { selected: None }
    }

    pub fn with_selected(mut self, selected: impl Into<ElementId>) -> Self {
        self.selected = Some(selected.into());
        self
    }

    pub fn selected(&self) -> Option<&ElementId> {
        self.selected.as_ref()
    }

    pub fn set_selected(&mut self, id: impl Into<ElementId>) {
        self.selected = Some(id.into());
    }

    pub fn clear_selected(&mut self) {
        self.selected = None;
    }

    pub fn is_selected(&self, id: &ElementId) -> bool {
        self.selected.as_ref().map_or(false, |sel| sel == id)
    }
}

#[derive(IntoElement)]
pub struct Tabs {
    id: ElementId,
    state: Entity<TabsState>,
    variant: TabsVariant,
    tabs: Vec<Tab>,
}

impl Tabs {
    pub fn new(id: impl Into<ElementId>, state: Entity<TabsState>, variant: TabsVariant) -> Self {
        Self { id: id.into(), state, variant, tabs: Vec::new() }
    }

    pub fn tabs(mut self, tabs: impl IntoIterator<Item = Tab>) -> Self {
        self.tabs = tabs.into_iter().collect();
        self
    }
}

impl RenderOnce for Tabs {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let selected_id = self.state.read(cx).selected().cloned();
        let (tab_buttons, content) = {
            let mut tab_buttons = Vec::new();
            let mut content: AnyElement = div().into_any_element();

            for tab in self.tabs.into_iter() {
                let is_selected = selected_id.as_ref().map_or(false, |sel| sel == &tab.id);
                let state = self.state.clone();

                let Tab { id: tab_id, label, disabled, content: tab_content } = tab;

                if is_selected {
                    content = tab_content;
                }

                tab_buttons.push(
                    Button::new(tab_id.clone())
                        .disabled(disabled)
                        .selected(is_selected)
                        .on_click(move |_, _, cx| {
                            state.update(cx, |state, cx| {
                                state.set_selected(tab_id.clone());
                                cx.notify();
                            });
                        })
                        .child(label),
                );
            }

            (tab_buttons, content)
        };

        match self.variant {
            TabsVariant::Sidebar => div()
                .id(self.id.clone())
                .flex()
                .size_full()
                .bg(cx.theme().bg_primary)
                .child(
                    v_flex()
                        .min_w_48()
                        .max_w_48()
                        .h_full()
                        .gap_1()
                        .p_1()
                        .bg(cx.theme().bg_secondary)
                        .border_r_1()
                        .border_color(cx.theme().border_secondary)
                        .children(tab_buttons),
                )
                .child(content),
            TabsVariant::Top => div()
                .id(self.id.clone())
                .flex()
                .flex_col()
                .bg(cx.theme().bg_primary)
                .size_full()
                .child(
                    h_flex()
                        .w_full()
                        .gap_1()
                        .p_1()
                        .bg(cx.theme().bg_secondary)
                        .border_b_1()
                        .border_color(cx.theme().border_secondary)
                        .children(tab_buttons),
                )
                .child(content),
        }
    }
}

pub struct Tab {
    pub id: ElementId,
    pub label: String,
    pub content: AnyElement,
    pub disabled: bool,
}

impl Tab {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<String>,
        content: impl Into<AnyElement>,
    ) -> Self {
        Self { id: id.into(), label: label.into(), content: content.into(), disabled: false }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsVariant {
    Top,
    Sidebar,
}

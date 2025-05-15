use gpui::{AnyElement, AnyView, Div, SharedString, Window, div, prelude::*};

use crate::{ActiveTheme, Disableable, Selectable, interactive_container};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    id: SharedString,
    label: SharedString,
    disabled: bool,
    content: AnyView,
}

impl Tab {
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        content: AnyView,
    ) -> Self {
        Self { id: id.into(), label: label.into(), disabled: false, content }
    }

    pub fn id(&self) -> &SharedString {
        &self.id
    }

    pub fn set_label(&mut self, label: impl Into<SharedString>) {
        self.label = label.into();
    }

    pub fn label(&self) -> &SharedString {
        &self.label
    }
}

impl Disableable for Tab {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct TabView {
    tabs: Vec<Tab>,
    selected_tab: Option<SharedString>,
    orientation: Orientation,
}

impl TabView {
    pub fn new(tabs: Vec<Tab>, _w: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { tabs, selected_tab: None, orientation: Orientation::default() }
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn select_tab_ix(&mut self, ix: usize) {
        self.selected_tab = self.tabs.get(ix).map(|tab| tab.id.clone());
    }

    pub fn select_tab(&mut self, id: Option<SharedString>) {
        self.selected_tab = id;
    }

    pub fn selected_tab(&self) -> Option<&Tab> {
        self.selected_tab.as_ref().and_then(|id| self.tabs.iter().find(|tab| tab.id == *id))
    }

    pub fn selected_tab_id(&self) -> Option<&SharedString> {
        self.selected_tab.as_ref()
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
}

impl TabView {
    pub fn render_tabs(&mut self, cx: &mut Context<Self>) -> Div {
        let tabs = self.tabs.clone().into_iter().map(|tab| {
            interactive_container(tab.id.clone(), None)
                .cursor_pointer()
                .w_full()
                .disabled(tab.disabled)
                .selected(self.selected_tab_id() == Some(&tab.id))
                .on_click(cx.listener(move |this, _, _, _| {
                    this.selected_tab = Some(tab.id.clone());
                }))
                .child(div().px_2().py_1().child(tab.label.clone()))
        });

        div()
            .flex()
            .w_full()
            .border_color(cx.theme().colors.border)
            .when(self.orientation == Orientation::Vertical, |e| {
                e.max_w_32().flex_col().border_r_1()
            })
            .when(self.orientation == Orientation::Horizontal, |e| e.border_b_1())
            .gap_2()
            .p_2()
            .children(tabs)
    }

    pub fn render_content(&mut self, _cx: &mut Context<Self>) -> AnyElement {
        let Some(tab) = self.selected_tab() else {
            return div().into_any_element();
        };

        tab.content.clone().into_any_element()
    }
}

impl Render for TabView {
    fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tabs = self.render_tabs(cx);
        let content = div().size_full().child(self.render_content(cx));

        div()
            .flex()
            .when(self.orientation == Orientation::Horizontal, |e| e.flex_col())
            .size_full()
            .child(tabs)
            .child(content)
    }
}

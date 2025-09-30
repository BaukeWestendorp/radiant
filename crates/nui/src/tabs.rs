use gpui::prelude::*;
use gpui::{AnyElement, AnyView, Div, SharedString, Window, div};

use crate::button::button;
use crate::theme::ActiveTheme;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    id: SharedString,
    label: SharedString,
    disabled: bool,
    view: AnyView,
}

impl Tab {
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        view: impl Into<AnyView>,
    ) -> Self {
        Self { id: id.into(), label: label.into(), disabled: false, view: view.into() }
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

    pub fn view(&self) -> &AnyView {
        &self.view
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
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

pub struct Tabs {
    tabs: Vec<Tab>,
    selected_tab: Option<SharedString>,
    orientation: Orientation,
    show_tabs: bool,
}

impl Tabs {
    pub fn new(tabs: Vec<Tab>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let selected_tab = tabs.first().map(|tab| tab.id.clone());

        Self { tabs, selected_tab, orientation: Orientation::default(), show_tabs: true }
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

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn set_show_tabs(&mut self, show_tabs: bool) {
        self.show_tabs = show_tabs;
    }

    pub fn with_show_tabs(mut self, show_tabs: bool) -> Self {
        self.show_tabs = show_tabs;
        self
    }

    pub fn show_tabs(&self) -> bool {
        self.show_tabs
    }
}

impl Tabs {
    pub fn render_tabs(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        let tabs = self
            .tabs
            .clone()
            .into_iter()
            .map(|tab| {
                let selected = self.selected_tab_id() == Some(&tab.id);
                div().w_full().child(
                    button(tab.id.clone(), None, div().w_full().child(tab.label.clone()))
                        .w_full()
                        .disabled(tab.disabled)
                        .selected(selected)
                        .on_click(cx.listener(move |this, _, _, _| {
                            this.selected_tab = Some(tab.id.clone());
                        })),
                )
            })
            .collect::<Vec<_>>();

        div()
            .bg(cx.theme().background)
            .flex()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .when(self.orientation == Orientation::Vertical, |e| {
                e.flex_col().content_stretch().min_w_40().max_w_40().border_b_0().border_r_1()
            })
            .gap_2()
            .children(tabs)
    }

    pub fn render_content(&mut self, _cx: &mut Context<Self>) -> AnyElement {
        let Some(tab) = self.selected_tab() else {
            return div().into_any_element();
        };

        tab.view.clone().into_any_element()
    }
}

impl Render for Tabs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tabs = if self.show_tabs { Some(self.render_tabs(window, cx)) } else { None };
        let view = self.render_content(cx);

        div()
            .flex()
            .when(self.orientation == Orientation::Horizontal, |e| e.flex_col())
            .size_full()
            .children(tabs)
            .child(view)
    }
}

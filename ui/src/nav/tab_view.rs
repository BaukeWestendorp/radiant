use gpui::prelude::*;
use gpui::{AnyElement, AnyView, Div, SharedString, Window, div};

use crate::{ActiveTheme, Disableable, Selectable, button};

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
    show_tabs: bool,
    render_header: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement>>,
}

impl TabView {
    pub fn new(tabs: Vec<Tab>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let selected_tab = match tabs.first() {
            Some(tab) => Some(tab.id.clone()),
            None => None,
        };

        Self {
            tabs,
            selected_tab,
            orientation: Orientation::default(),
            show_tabs: true,
            render_header: None,
        }
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

    pub fn set_show_tabs(&mut self, show_tabs: bool) {
        self.show_tabs = show_tabs;
    }

    pub fn show_tabs(&self) -> bool {
        self.show_tabs
    }

    pub fn set_header<F>(&mut self, f: F)
    where
        F: Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static,
    {
        self.render_header = Some(Box::new(f));
    }
}

impl TabView {
    pub fn render_tabs(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let tabs = self
            .tabs
            .clone()
            .into_iter()
            .enumerate()
            .map(|(ix, tab)| {
                let selected = self.selected_tab_id() == Some(&tab.id);
                div()
                    .w_full()
                    .p_2()
                    .border_color(cx.theme().colors.border)
                    .when(ix == 0, |e| e.border_l_1())
                    .when(ix == self.tabs.len() - 1, |e| e.border_r_1())
                    .when(self.tabs.len() > 1, |e| {
                        e.when(!selected && self.orientation == Orientation::Horizontal, |e| {
                            e.border_b_1()
                        })
                        .when(selected && self.orientation == Orientation::Horizontal, |e| {
                            e.border_x_1()
                        })
                        .when(!selected && self.orientation == Orientation::Vertical, |e| {
                            e.border_r_1()
                        })
                        .when(selected && self.orientation == Orientation::Vertical, |e| {
                            e.border_y_1()
                        })
                    })
                    .child(
                        button(tab.id.clone(), None, tab.label.clone())
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
            .flex()
            .when(self.orientation == Orientation::Vertical, |e| e.flex_col())
            .children(self.render_header.as_ref().map(|f| {
                div()
                    .min_w_24()
                    .max_w_24()
                    .flex()
                    .justify_center()
                    .items_center()
                    .border_b_1()
                    .border_color(cx.theme().colors.border)
                    .child(f(window, cx))
            }))
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tabs = if self.show_tabs { Some(self.render_tabs(window, cx)) } else { None };
        let content = div().size_full().child(self.render_content(cx));

        div()
            .flex()
            .when(self.orientation == Orientation::Horizontal, |e| e.flex_col())
            .size_full()
            .children(tabs)
            .child(content)
    }
}

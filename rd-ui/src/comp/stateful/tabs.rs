use gpui::{
    AnyView, App, ElementId, FocusHandle, Focusable, SharedString, Window, div, prelude::*,
};

use crate::{
    ActiveTheme, HslaExt, StyledExt, StyledParentExt, c_flex,
    comp::{FocusableComponent, Icon, IconSize, IconVariant, Identifiable, Labelled},
    h_flex,
};

pub(crate) mod action {
    pub const KEY_CONTEXT: &str = "Tabs";

    gpui::actions!(tabs, [Next, Previous]);
}

pub struct Tabs {
    id: ElementId,
    focus_handle: FocusHandle,

    tabs: Vec<Tab>,
    selected: Option<usize>,
    direction: TabsDirection,
}

impl Tabs {
    pub fn new(id: impl Into<ElementId>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            id: id.into(),
            focus_handle: cx.focus_handle().tab_stop(true),

            tabs: Vec::new(),
            selected: None,
            direction: TabsDirection::default(),
        }
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
    }

    pub fn with_tab(mut self, tab: Tab) -> Self {
        self.add_tab(tab);
        self
    }

    pub fn add_tabs(&mut self, tabs: impl Iterator<Item = Tab>, cx: &mut Context<Self>) {
        self.tabs.extend(tabs);
        self.selected = Some(self.tabs.len() - 1);
        cx.notify();
    }

    pub fn with_tabs(mut self, tabs: Vec<Tab>) -> Self {
        self.tabs = tabs;
        self
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn selected_tab(&self) -> Option<&Tab> {
        self.selected.and_then(|ix| self.tabs.get(ix))
    }

    pub fn select(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = Some(ix);
        if let Some(tab) = self.selected_tab() {
            if let Some(content) = &tab.content {
                content.focus_handle(cx).focus(window, cx);
            }
        }
        cx.notify();
    }

    pub fn with_selected(mut self, ix: usize) -> Self {
        self.selected = Some(ix);
        self
    }

    pub fn direction(&self) -> TabsDirection {
        self.direction
    }

    pub fn set_direction(&mut self, direction: TabsDirection, cx: &mut Context<Self>) {
        self.direction = direction;
        cx.notify();
    }

    pub fn with_direction(mut self, direction: TabsDirection) -> Self {
        self.direction = direction;
        self
    }

    fn handle_next(&mut self, _action: &action::Next, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selected) = self.selected {
            let next = (selected + 1) % self.tabs.len();
            self.select(next, window, cx);
        }
    }

    fn handle_previous(
        &mut self,
        _action: &action::Previous,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected) = self.selected {
            let previous = (selected + self.tabs.len() - 1) % self.tabs.len();
            self.select(previous, window, cx);
        }
    }
}

impl Identifiable for Tabs {
    fn id(&self, _cx: &App) -> &ElementId {
        &self.id
    }
}

impl Focusable for Tabs {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl FocusableComponent for Tabs {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = focus_handle;
    }
}

impl Render for Tabs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vertical = self.direction == TabsDirection::Vertical;

        let tabs = div()
            .flex()
            .when(vertical, |e| e.flex_col().w_40().min_w_40().min_w_40().h_full())
            .when(!vertical, |e| {
                e.w_full()
                    .h(crate::comp::INPUT_SIZE)
                    .min_h(crate::comp::INPUT_SIZE)
                    .max_h(crate::comp::INPUT_SIZE)
            })
            .border_color(cx.theme().border_secondary)
            .children(self.tabs.iter().enumerate().map(|(ix, tab)| {
                let selected = self.selected == Some(ix);
                let label = h_flex()
                    .w_full()
                    .gap_1()
                    .items_center()
                    .when_some(tab.icon(), |e, icon| e.child(Icon::new(icon, IconSize::ExtraSmall)))
                    .when_some(tab.label(), |e, label| e.child(label.clone()));

                c_flex()
                    .id(ix)
                    .track_focus(&tab.focus_handle)
                    .focus_ring(&tab.focus_handle, window, cx)
                    .w_full()
                    .px_2()
                    .bg(cx.theme().bg_secondary)
                    .text_color(cx.theme().fg_secondary)
                    .hover(|e| e.bg(cx.theme().bg_secondary.hover()))
                    .active(|e| {
                        e.bg(cx.theme().bg_secondary.active()).top(cx.theme().button_depression)
                    })
                    .when(!vertical, |e| e.w_full().border_b_1().when(ix != 0, |e| e.border_l_1()))
                    .when(vertical, |e| {
                        e.h(crate::comp::INPUT_SIZE)
                            .w_full()
                            .justify_start()
                            .border_r_1()
                            .border_t_1()
                            .when(ix == 0, |e| e.border_t_0())
                            .when(selected, |e| e.border_r_0())
                    })
                    .when(selected, |e| {
                        e.bg(cx.theme().bg_primary)
                            .font_semibold()
                            .text_color(cx.theme().fg_selected)
                            .when(self.tabs.len() > 1, |e| e.border_b_0())
                    })
                    .border_color(cx.theme().border_primary)
                    .on_click(cx.listener(move |this, _, window, cx| this.select(ix, window, cx)))
                    .child(label)
            }))
            .when(vertical, |e| {
                e.child(
                    div()
                        .flex()
                        .flex_grow_1()
                        .bg(cx.theme().bg_secondary)
                        .border_t_1()
                        .border_r_1()
                        .border_color(cx.theme().border_primary),
                )
            });

        let content = match self.selected_tab().and_then(|tab| tab.content()) {
            Some(content) => div()
                .track_focus(&content.focus_handle)
                .focus_ring(&content.focus_handle, window, cx)
                .size_full()
                .child(content.content().clone()),
            None => div().size_full(),
        };

        div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .focus_ring(&self.focus_handle, window, cx)
            .key_context(action::KEY_CONTEXT)
            .on_action::<action::Next>(cx.listener(Self::handle_next))
            .on_action::<action::Previous>(cx.listener(Self::handle_previous))
            .flex()
            .bg(cx.theme().bg_primary)
            .when(!vertical, |e| e.flex_col())
            .size_full()
            .child(tabs)
            .child(content)
    }
}

pub struct Tab {
    label: Option<SharedString>,
    icon: Option<IconVariant>,
    content: Option<TabContent>,
    focus_handle: FocusHandle,
}

impl Tab {
    pub fn new(label: impl Into<SharedString>, cx: &App) -> Self {
        Self {
            label: Some(label.into()),
            icon: None,
            content: None,
            focus_handle: cx.focus_handle().tab_stop(true),
        }
    }

    pub fn content(&self) -> Option<&TabContent> {
        self.content.as_ref()
    }

    pub fn set_content(&mut self, content: impl Into<AnyView>, cx: &App) {
        self.content = Some(TabContent::new(content, cx));
    }

    pub fn with_content(mut self, content: impl Into<AnyView>, cx: &App) -> Self {
        self.set_content(content, cx);
        self
    }
}

impl Focusable for Tab {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl FocusableComponent for Tab {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = focus_handle;
    }
}

impl Labelled for Tab {
    fn label(&self) -> Option<&SharedString> {
        self.label.as_ref()
    }

    fn set_label(&mut self, label: impl Into<Option<SharedString>>) {
        self.label = label.into();
    }

    fn icon(&self) -> Option<IconVariant> {
        self.icon
    }

    fn set_icon(&mut self, icon: impl Into<Option<IconVariant>>) {
        self.icon = icon.into();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TabsDirection {
    #[default]
    Auto,
    Horizontal,
    Vertical,
}

pub struct TabContent {
    content: AnyView,
    focus_handle: FocusHandle,
}

impl TabContent {
    pub fn new(content: impl Into<AnyView>, cx: &App) -> Self {
        Self { content: content.into(), focus_handle: cx.focus_handle().tab_stop(true) }
    }

    pub fn content(&self) -> &AnyView {
        &self.content
    }
}

impl Focusable for TabContent {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

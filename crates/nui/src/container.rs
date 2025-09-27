use gpui::prelude::*;
use gpui::{
    AnyElement, App, Div, ElementId, FocusHandle, Hsla, Interactivity, Stateful, StyleRefinement,
    Window, div,
};
use smallvec::SmallVec;

use crate::theme::{ActiveTheme, InteractiveColor};

pub fn container(window: &Window, cx: &App) -> Container {
    Container::new(window, cx)
}

#[derive(IntoElement)]
pub struct Container {
    base: Div,
    children: SmallVec<[AnyElement; 2]>,
    style: ContainerStyle,
}

impl Container {
    pub fn new(window: &Window, cx: &App) -> Self {
        Self { base: div(), children: SmallVec::new(), style: ContainerStyle::normal(window, cx) }
    }

    pub fn style_normal(mut self, window: &Window, cx: &App) -> Self {
        self.style = ContainerStyle::normal(window, cx);
        self
    }

    pub fn style_focused(mut self, window: &Window, cx: &App) -> Self {
        self.style = ContainerStyle::focused(window, cx);
        self
    }

    pub fn style_selected(mut self, window: &Window, cx: &App) -> Self {
        self.style = ContainerStyle::selected(window, cx);
        self
    }

    pub fn style_disabled(mut self) -> Self {
        self.style = self.style.disabled();
        self
    }

    pub fn style_hovered(mut self) -> Self {
        self.style = self.style.hovered();
        self
    }

    pub fn style_active(mut self) -> Self {
        self.style = self.style.active();
        self
    }
}

impl ParentElement for Container {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Container {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Container {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Container {}

impl RenderOnce for Container {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .bg(self.style.background)
            .border_1()
            .border_color(self.style.border)
            .rounded(cx.theme().radius)
            .text_color(self.style.text_color)
            .overflow_hidden()
            .children(self.children)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContainerStyle {
    pub background: Hsla,
    pub border: Hsla,
    pub text_color: Hsla,
}

impl ContainerStyle {
    pub fn normal(window: &Window, cx: &App) -> Self {
        Self {
            background: cx.theme().secondary,
            border: cx.theme().border,
            text_color: window.text_style().color,
        }
    }

    pub fn focused(window: &Window, cx: &App) -> Self {
        Self {
            background: cx.theme().focus,
            border: cx.theme().focus_border,
            text_color: window.text_style().color,
        }
    }

    pub fn selected(window: &Window, cx: &App) -> Self {
        Self {
            background: cx.theme().selected,
            border: cx.theme().selected_border,
            text_color: window.text_style().color,
        }
    }

    pub fn disabled(&self) -> Self {
        Self {
            background: self.background.disabled(),
            border: self.border.disabled(),
            text_color: self.text_color.disabled(),
        }
    }

    pub fn hovered(&self) -> Self {
        Self {
            background: self.background.hovered(),
            border: self.border.hovered(),
            text_color: self.text_color.hovered(),
        }
    }

    pub fn active(&self) -> Self {
        Self {
            background: self.background.active(),
            border: self.border.active(),
            text_color: self.text_color.active(),
        }
    }
}

pub fn interactive_container(
    id: impl Into<ElementId>,
    focus_handle: Option<FocusHandle>,
) -> InteractiveContainer {
    InteractiveContainer::new(id, focus_handle)
}

#[derive(IntoElement)]
pub struct InteractiveContainer {
    disabled: bool,
    selected: bool,

    base: Stateful<Div>,
    children: SmallVec<[AnyElement; 2]>,
    focus_handle: Option<FocusHandle>,

    disabled_interactivity: Interactivity,
}

impl InteractiveContainer {
    fn new(id: impl Into<ElementId>, focus_handle: Option<FocusHandle>) -> Self {
        Self {
            disabled: false,
            selected: false,

            base: div().id(id.into()),
            children: SmallVec::new(),
            focus_handle,

            disabled_interactivity: Interactivity::default(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for InteractiveContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for InteractiveContainer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity().base_style
    }
}

impl InteractiveElement for InteractiveContainer {
    fn interactivity(&mut self) -> &mut Interactivity {
        if self.disabled { &mut self.disabled_interactivity } else { self.base.interactivity() }
    }
}

impl StatefulInteractiveElement for InteractiveContainer {}

impl From<InteractiveContainer> for AnyElement {
    fn from(container: InteractiveContainer) -> Self {
        container.into_any_element()
    }
}

impl RenderOnce for InteractiveContainer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focused = self.focus_handle.as_ref().is_some_and(|fh| fh.is_focused(window));

        let mut style = if focused {
            ContainerStyle::focused(window, cx)
        } else if self.selected {
            ContainerStyle::selected(window, cx)
        } else {
            ContainerStyle::normal(window, cx)
        };

        if self.disabled {
            style = style.disabled();
        }

        if self.disabled || self.selected {
            self.base
                .focusable()
                // We have to use this instead of .block_mouse_down()
                // because that implementation only blocks MouseButton::Left.
                .on_any_mouse_down(|_, _, cx| cx.stop_propagation())
        } else if let Some(focus_handle) = &self.focus_handle {
            self.base.track_focus(focus_handle)
        } else {
            self.base.focusable()
        }
        .bg(style.background)
        .border_1()
        .border_color(style.border)
        .rounded(cx.theme().radius)
        .text_color(style.text_color)
        .overflow_hidden()
        .occlude()
        .when(self.disabled, |e| e.cursor_not_allowed())
        .when(!self.disabled, |e| {
            let hover_active_style =
                if !focused && !self.selected { ContainerStyle::normal(window, cx) } else { style };

            e.hover(|e| {
                e.bg(hover_active_style.hovered().background)
                    .border_color(hover_active_style.hovered().border)
            })
            .active(|e| {
                e.bg(hover_active_style.active().background)
                    .border_color(hover_active_style.active().border)
            })
        })
        .children(self.children)
    }
}

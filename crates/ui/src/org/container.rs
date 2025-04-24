use crate::{ActiveTheme, Disableable, InteractiveColor, Selectable};
use gpui::{
    AnyElement, App, Div, ElementId, FocusHandle, Hsla, Interactivity, Stateful, StyleRefinement,
    Window, div, prelude::*,
};
use smallvec::SmallVec;

pub fn container(style: ContainerStyle) -> Container {
    Container::new(style)
}

#[derive(IntoElement)]
pub struct Container {
    disabled: bool,

    base: Div,
    children: SmallVec<[AnyElement; 4]>,
    style: ContainerStyle,
}

impl Container {
    fn new(style: ContainerStyle) -> Self {
        Self { disabled: false, base: div(), children: SmallVec::new(), style }
    }
}

impl Disableable for Container {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl RenderOnce for Container {
    fn render(self, _w: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = match self.disabled {
            true => self.style.disabled(),
            false => self.style,
        };

        self.base
            .bg(style.background)
            .border_1()
            .border_color(style.border)
            .rounded(cx.theme().radius)
            .text_color(style.text_color)
            .overflow_hidden()
            .children(self.children)
    }
}

pub struct ContainerStyle {
    pub background: Hsla,
    pub border: Hsla,
    pub text_color: Hsla,
}

impl ContainerStyle {
    pub fn normal(w: &Window, cx: &App) -> Self {
        Self {
            background: cx.theme().colors.bg_secondary,
            border: cx.theme().colors.border,
            text_color: w.text_style().color,
        }
    }

    pub fn focused(w: &Window, cx: &App) -> Self {
        Self {
            background: cx.theme().colors.bg_focused,
            border: cx.theme().colors.border_focused,
            text_color: w.text_style().color,
        }
    }

    pub fn selected(w: &Window, cx: &App) -> Self {
        Self {
            background: cx.theme().colors.bg_selected,
            border: cx.theme().colors.border_selected,
            text_color: w.text_style().color,
        }
    }

    pub fn disabled(&self) -> Self {
        Self {
            background: self.background.muted(),
            border: self.border.muted(),
            text_color: self.text_color.muted(),
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
}

impl Disableable for InteractiveContainer {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for InteractiveContainer {
    fn selected(mut self, selected: bool) -> Self {
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
    fn render(self, w: &mut Window, cx: &mut App) -> impl IntoElement {
        let focused = self.focus_handle.as_ref().is_some_and(|fh| fh.is_focused(w));

        let mut style = if focused {
            ContainerStyle::focused(w, cx)
        } else if self.selected {
            ContainerStyle::selected(w, cx)
        } else {
            ContainerStyle::normal(w, cx)
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
        .when(self.disabled, |e| e.cursor_not_allowed())
        .when(!self.disabled, |e| {
            e.hover(|e| e.bg(style.hovered().background).border_color(style.hovered().border))
                .active(|e| e.bg(style.active().background).border_color(style.active().border))
        })
        .overflow_hidden()
        .children(self.children)
    }
}

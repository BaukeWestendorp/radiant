use gpui::*;
use prelude::FluentBuilder;
use smallvec::SmallVec;

use crate::{
    Disableable,
    theme::{ActiveTheme, InteractiveColor},
};

/// An interactive, styled container that can hold other elements.
#[derive(IntoElement)]
pub struct InteractiveContainer {
    disabled: bool,
    inset: Option<Pixels>,

    base: Stateful<Div>,
    children: SmallVec<[AnyElement; 2]>,
    focus_handle: FocusHandle,
}

impl InteractiveContainer {
    pub fn new(id: impl Into<ElementId>, focus_handle: FocusHandle) -> Self {
        Self {
            disabled: false,
            inset: None,
            base: div().id(id.into()),
            children: SmallVec::new(),
            focus_handle,
        }
    }

    pub fn inset(mut self, inset: Pixels) -> Self {
        self.inset = Some(inset);
        self
    }
}

impl Disableable for InteractiveContainer {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Focusable for InteractiveContainer {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
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
        self.base.interactivity()
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
        let focused = self.focus_handle.is_focused(window);

        self.base
            .bg(ContainerKind::Element.bg(cx))
            .border_1()
            .border_color(ContainerKind::Element.border_color(cx))
            .rounded(cx.theme().radius)
            .when(self.disabled, |e| {
                e.bg(cx.theme().element_background.muted())
                    .border_color(cx.theme().border.muted())
                    .text_color(cx.theme().text_primary.muted())
                    .cursor_not_allowed()
                    .when(focused, |e| e.border_color(cx.theme().border_focused.muted()))
            })
            .when(!self.disabled, |e| {
                e.hover(|e| e.bg(cx.theme().element_background.hovered())).when(focused, |e| {
                    e.bg(cx.theme().element_background_focused)
                        .border_color(cx.theme().border_focused)
                })
            })
            .children(self.children)
    }
}

pub enum ContainerKind {
    Regular,
    Element,
}

impl ContainerKind {
    fn bg(&self, cx: &App) -> Hsla {
        match self {
            Self::Regular => cx.theme().background,
            Self::Element => cx.theme().element_background,
        }
    }

    fn border_color(&self, cx: &App) -> Hsla {
        match self {
            Self::Regular => cx.theme().border,
            Self::Element => cx.theme().border,
        }
    }
}

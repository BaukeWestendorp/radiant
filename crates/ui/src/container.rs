use gpui::*;
use prelude::FluentBuilder;
use smallvec::SmallVec;

use crate::{z_stack, ActiveTheme};

/// A styled container that can hold other elements.
#[derive(IntoElement)]
pub struct Container {
    kind: ContainerKind,
    inset: Option<Pixels>,
    interactivity: Interactivity,
    children: SmallVec<[AnyElement; 2]>,
}

impl Container {
    pub fn new(kind: ContainerKind) -> Self {
        Self {
            kind,
            inset: None,
            interactivity: Interactivity::default(),
            children: SmallVec::new(),
        }
    }

    pub fn inset(mut self, inset: Pixels) -> Self {
        self.inset = Some(inset);
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
        &mut self.interactivity.base_style
    }
}

impl From<Container> for AnyElement {
    fn from(container: Container) -> Self {
        container.into_any_element()
    }
}

impl InteractiveElement for Container {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl RenderOnce for Container {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let container = base_container(self.kind, cx);

        if let Some(inset) = self.inset {
            z_stack([div()
                .size_full()
                .child(container)
                .child(div().size_full().children(self.children))])
            .p(inset)
        } else {
            container
        }
    }
}

pub enum ContainerKind {
    Regular,
    Element,
    Surface,
}

impl ContainerKind {
    fn bg(&self, cx: &AppContext) -> Hsla {
        match self {
            Self::Regular => cx.theme().background,
            Self::Element => cx.theme().element_background,
            Self::Surface => cx.theme().surface_background,
        }
    }

    fn border_color(&self, cx: &AppContext) -> Hsla {
        match self {
            Self::Regular => cx.theme().border,
            Self::Element => cx.theme().border,
            Self::Surface => cx.theme().border,
        }
    }
}

/// An interactive, styled container that can hold other elements..
#[derive(IntoElement)]
pub struct InteractiveContainer {
    id: ElementId,
    disabled: bool,
    focused: bool,
    interactivity: Interactivity,
    children: SmallVec<[AnyElement; 2]>,
    inset: Option<Pixels>,
}

impl InteractiveContainer {
    pub fn new(id: impl Into<ElementId>, disabled: bool, focused: bool) -> Self {
        Self {
            inset: None,
            id: id.into(),
            disabled,
            focused,
            interactivity: Interactivity::default(),
            children: SmallVec::new(),
        }
    }

    pub fn inset(mut self, inset: Pixels) -> Self {
        self.inset = Some(inset);
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
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for InteractiveContainer {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl StatefulInteractiveElement for InteractiveContainer {}

impl From<InteractiveContainer> for AnyElement {
    fn from(container: InteractiveContainer) -> Self {
        container.into_any_element()
    }
}

impl RenderOnce for InteractiveContainer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        base_container(ContainerKind::Element, cx)
            .id(self.id)
            .when(self.focused, |e| e.bg(cx.theme().element_selected))
            .when(self.disabled, |e| {
                e.bg(cx.theme().element_disabled)
                    .border_color(cx.theme().border_disabled)
            })
            .when(!self.disabled, |e| {
                e.hover(|e| e.bg(cx.theme().element_hover))
                    .active(|e| e.bg(cx.theme().element_active))
            })
            .cursor_pointer()
            .children(self.children)
    }
}

fn base_container(kind: ContainerKind, cx: &AppContext) -> Div {
    div()
        .size_full()
        .bg(kind.bg(cx))
        .border_1()
        .border_color(kind.border_color(cx))
        .rounded(cx.theme().radius)
}

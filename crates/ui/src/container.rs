use gpui::*;
use prelude::FluentBuilder;
use smallvec::SmallVec;

use crate::ActiveTheme;

/// A styled container that can hold other elements.
#[derive(IntoElement)]
pub struct Container {
    base: Div,
    kind: ContainerKind,
    inset: Option<Pixels>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Container {
    pub fn new(kind: ContainerKind) -> Self {
        Self {
            base: div(),
            kind,
            inset: None,
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
        self.base.style()
    }
}

impl From<Container> for AnyElement {
    fn from(container: Container) -> Self {
        container.into_any_element()
    }
}

impl InteractiveElement for Container {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Container {
    fn render(mut self, cx: &mut WindowContext) -> impl IntoElement {
        let style = self.base.style().clone();
        self.base
            .max_w(style.size.width.unwrap_or_default())
            .max_h(style.size.height.unwrap_or_default())
            .when_some(self.inset, |e, inset| e.p(inset))
            .child(
                base_container(style, &self.kind, cx)
                    .size_full()
                    .children(self.children),
            )
    }
}

pub enum ContainerKind {
    Regular,
    Element,
    Surface,
    Custom { bg: Hsla, border_color: Hsla },
}

impl ContainerKind {
    pub fn bg(&self, cx: &AppContext) -> Hsla {
        match self {
            Self::Regular => cx.theme().background,
            Self::Element => cx.theme().element_background,
            Self::Surface => cx.theme().surface_background,
            Self::Custom { bg, .. } => *bg,
        }
    }

    pub fn border_color(&self, cx: &AppContext) -> Hsla {
        match self {
            Self::Regular => cx.theme().border,
            Self::Element => cx.theme().border,
            Self::Surface => cx.theme().border,
            Self::Custom { border_color, .. } => *border_color,
        }
    }
}

/// An interactive, styled container that can hold other elements..
#[derive(IntoElement)]
pub struct InteractiveContainer {
    kind: ContainerKind,
    base: Div,
    id: ElementId,
    disabled: bool,
    focused: bool,
    children: SmallVec<[AnyElement; 2]>,
    inset: Option<Pixels>,
}

impl InteractiveContainer {
    pub fn new(
        kind: ContainerKind,
        id: impl Into<ElementId>,
        disabled: bool,
        focused: bool,
    ) -> Self {
        Self {
            kind,
            base: div(),
            inset: None,
            id: id.into(),
            disabled,
            focused,
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
        self.base.style()
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
    fn render(mut self, cx: &mut WindowContext) -> impl IntoElement {
        let style = self.base.style().clone();
        self.base
            .id(self.id.clone())
            .max_w(style.size.width.unwrap_or_default())
            .max_h(style.size.height.unwrap_or_default())
            .when_some(self.inset, |e, inset| e.p(inset))
            .group("interactive-container")
            .child(
                base_container(style, &self.kind, cx)
                    .id(ElementId::Name(
                        format!("{}-base", self.id.to_string()).into(),
                    ))
                    .size_full()
                    .cursor_pointer()
                    .when(self.focused, |e| e.bg(cx.theme().element_selected))
                    .when(self.disabled, |e| {
                        e.bg(cx.theme().element_disabled)
                            .border_color(cx.theme().border_disabled)
                    })
                    .when(!self.disabled, |e| {
                        e.group_active("interactive-container", |e| e.bg(cx.theme().element_active))
                            .group_hover("interactive-container", |e| {
                                e.bg(cx.theme().element_hover)
                            })
                    })
                    .children(self.children),
            )
    }
}

fn base_container(base_style: StyleRefinement, kind: &ContainerKind, cx: &AppContext) -> Div {
    let mut base = div();
    base.style().size = base_style.size;

    base.border_1()
        .bg(kind.bg(cx))
        .border_color(kind.border_color(cx))
        .rounded(cx.theme().radius)
}

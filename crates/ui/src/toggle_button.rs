use crate::{ActiveTheme, Disableable, InteractiveColor};
use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;

/// An interactive, styled container that can hold other elements.
#[derive(IntoElement)]
pub struct ToggleButton {
    disabled: bool,
    toggled: bool,

    base: Stateful<Div>,
    children: SmallVec<[AnyElement; 2]>,

    disabled_interactivity: Interactivity,
}

impl ToggleButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            disabled: false,
            toggled: false,
            base: div().id(id.into()).px_2().py_1(),
            children: SmallVec::new(),

            disabled_interactivity: Interactivity::default(),
        }
    }

    pub fn toggled(mut self, active: bool) -> Self {
        self.toggled = active;
        self
    }

    pub fn is_toggled(&self) -> bool {
        self.toggled
    }
}

impl Disableable for ToggleButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl ParentElement for ToggleButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for ToggleButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity().base_style
    }
}

impl InteractiveElement for ToggleButton {
    fn interactivity(&mut self) -> &mut Interactivity {
        if self.disabled { &mut self.disabled_interactivity } else { self.base.interactivity() }
    }
}

impl StatefulInteractiveElement for ToggleButton {}

impl From<ToggleButton> for AnyElement {
    fn from(container: ToggleButton) -> Self {
        container.into_any_element()
    }
}

impl RenderOnce for ToggleButton {
    fn render(self, _w: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .bg(cx.theme().element_background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .when(self.disabled, |e| {
                e.bg(cx.theme().element_background.muted())
                    .border_color(cx.theme().border.muted())
                    .text_color(cx.theme().text_primary.muted())
                    .cursor_not_allowed()
                    .when(self.toggled, |e| e.border_color(cx.theme().border_selected.muted()))
            })
            .when(!self.disabled, |e| {
                e.when(self.toggled, |e| {
                    e.bg(cx.theme().element_background_selected)
                        .border_color(cx.theme().border_selected)
                })
                .hover(|e| {
                    if self.toggled {
                        e.bg(cx.theme().element_background_selected.hovered())
                            .border_color(cx.theme().border_selected.hovered())
                    } else {
                        e.bg(cx.theme().element_background.hovered())
                            .border_color(cx.theme().border.hovered())
                    }
                })
                .cursor_pointer()
            })
            .children(self.children)
    }
}

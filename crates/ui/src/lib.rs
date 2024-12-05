#![allow(clippy::type_complexity)]
#![allow(clippy::option_as_ref_deref)]

pub mod input;
pub mod theme;

use gpui::*;
use prelude::FluentBuilder;
use theme::{ActiveTheme, Theme};

pub fn init(cx: &mut AppContext) {
    cx.set_global(Theme::default());
    input::text_field::init(cx);
}

/// Extends [`Styled`].
pub trait StyledExt: Styled + Sized {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    ///
    /// Sets `flex()`, `flex_col()`
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    /// Centers elements.
    ///
    /// Sets `flex()`, `items_center()`, `justify_center()`
    fn center_flex(self) -> Self {
        self.flex().items_center().justify_center()
    }
}

impl<E: Styled> StyledExt for E {}

/// Stack elements on top of each other.
pub fn z_stack(children: impl IntoIterator<Item = impl IntoElement>) -> Div {
    let children = children
        .into_iter()
        .map(|child| div().size_full().child(child).absolute());
    div().relative().children(children)
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

pub fn container(kind: ContainerKind, cx: &AppContext) -> Div {
    div()
        .bg(kind.bg(cx))
        .border_1()
        .border_color(kind.border_color(cx))
        .rounded(cx.theme().radius)
}

pub fn interactive_container(
    id: ElementId,
    disabled: bool,
    focused: bool,
    cx: &AppContext,
) -> Stateful<Div> {
    if !disabled {
        container(ContainerKind::Element, cx)
            .id(id)
            .hover(|e| e.bg(cx.theme().element_hover))
            .active(|e| e.bg(cx.theme().element_active))
            .when(focused, |e| e.bg(cx.theme().element_selected))
            .cursor_pointer()
    } else {
        container(ContainerKind::Element, cx)
            .id(id)
            .bg(cx.theme().element_disabled)
            .border_1()
            .border_color(cx.theme().border_disabled)
            .rounded(cx.theme().radius)
            .cursor_not_allowed()
    }
}

pub trait InteractiveElementExt: InteractiveElement {
    /// Set the listener for a double click event.
    fn on_double_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().on_click(move |event, context| {
            if event.up.click_count == 2 {
                listener(event, context);
            }
        });
        self
    }
}

impl<E: InteractiveElement> InteractiveElementExt for Focusable<E> {}

pub fn bounds_updater<V: 'static>(
    view: View<V>,
    f: impl FnOnce(&mut V, Bounds<Pixels>, &mut ViewContext<V>) + 'static,
) -> impl IntoElement {
    let view = view.clone();
    canvas(
        move |bounds, cx| view.update(cx, |view, cx| f(view, bounds, cx)),
        |_, _, _| {},
    )
    .size_full()
}

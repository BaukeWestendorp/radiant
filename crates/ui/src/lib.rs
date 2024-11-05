#![allow(clippy::type_complexity)]
#![allow(clippy::option_as_ref_deref)]

pub mod input;
pub mod theme;

use gpui::*;

pub fn init(cx: &mut AppContext) {
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
}

impl<E: Styled> StyledExt for E {}

/// Stack elements on top of each other.
pub fn z_stack(children: impl IntoIterator<Item = impl IntoElement>) -> Div {
    let children = children
        .into_iter()
        .map(|child| div().size_full().child(child).absolute());
    div().relative().children(children)
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

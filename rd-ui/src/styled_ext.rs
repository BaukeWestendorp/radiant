// From gpui-component:crates/ui/src/styled.rs

use gpui::{
    Action, App, Div, Edges, Empty, Refineable, StatefulInteractiveElement, StyleRefinement,
    Styled, Window, div,
};
use gpui::{Pixels, prelude::*, px};

use crate::{ActiveTheme, Binding};

/// Returns a `Div` as horizontal flex layout.
#[inline(always)]
pub fn h_flex() -> Div {
    div().h_flex()
}

/// Returns a `Div` as vertical flex layout.
#[inline(always)]
pub fn v_flex() -> Div {
    div().v_flex()
}

pub trait StyledExt: Styled + Sized {
    /// Apply self into a horizontal flex layout.
    #[inline(always)]
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Apply self into a vertical flex layout.
    #[inline(always)]
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    /// Refine the style of this element, applying the given style refinement.
    fn refine_style(mut self, style: &StyleRefinement) -> Self {
        self.style().refine(style);
        self
    }
}

impl<E: Styled> StyledExt for E {}

pub trait StatefulInteractiveElementExt: StatefulInteractiveElement + Sized {
    /// Refine the style of this element, applying the given style refinement.
    fn action_tooltip(self, action: Box<dyn Action>) -> Self {
        self.tooltip(move |window, cx| {
            struct BindingView(Binding);
            impl Render for BindingView {
                fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                    div().child(self.0.clone())
                }
            }

            let binding = Binding::binding_for_action(action.as_ref(), None, window);
            match binding {
                Some(binding) => cx.new(|_| BindingView(binding)).into(),
                None => cx.new(|_| Empty).into(),
            }
        })
    }
}

impl<E: StatefulInteractiveElement> StatefulInteractiveElementExt for E {}

pub(crate) trait FocusableExt<T: ParentElement + Styled + Sized> {
    /// Add focus ring to the element.
    fn focus_ring(self, is_focused: bool, margins: Pixels, window: &Window, cx: &App) -> Self;
}

impl<T: ParentElement + Styled + Sized> FocusableExt<T> for T {
    fn focus_ring(mut self, is_focused: bool, margins: Pixels, window: &Window, cx: &App) -> Self {
        if !is_focused {
            return self;
        }

        const RING_BORDER_WIDTH: Pixels = px(1.5);
        let rem_size = window.rem_size();
        let style = self.style();

        let border_widths = Edges::<Pixels> {
            top: style.border_widths.top.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            bottom: style.border_widths.bottom.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            left: style.border_widths.left.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            right: style.border_widths.right.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
        };

        let inset = RING_BORDER_WIDTH + margins;

        self.child(
            div()
                .flex_none()
                .absolute()
                .top(-(inset + border_widths.top))
                .left(-(inset + border_widths.left))
                .right(-(inset + border_widths.right))
                .bottom(-(inset + border_widths.bottom))
                .rounded(cx.theme().radius)
                .border(RING_BORDER_WIDTH)
                .border_color(cx.theme().border_focus.alpha(0.8)),
        )
    }
}

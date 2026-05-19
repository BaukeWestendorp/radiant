// From gpui-component:crates/ui/src/styled.rs

use gpui::prelude::*;
use gpui::{
    Action, Div, Empty, Refineable, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    div,
};

use crate::Binding;

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

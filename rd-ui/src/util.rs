use gpui::{
    Action, App, Div, Edges, Empty, Hsla, Pixels, StyleRefinement, Styled, Window, div, px,
};
use gpui::{ClickEvent, prelude::*};

use crate::{ActiveTheme, Binding};

pub fn todo(cx: &App) -> Div {
    div()
        .size_full()
        .border_1()
        .border_color(cx.theme().indicate.warning)
        .bg(cx.theme().indicate.warning.opacity(0.2))
        .text_color(cx.theme().indicate.warning)
        .flex()
        .justify_center()
        .items_center()
        .child("TODO")
}

pub fn z_stack(children: impl IntoIterator<Item = impl IntoElement>) -> Div {
    let children = children.into_iter().map(|child| div().size_full().child(child).absolute());
    div().relative().children(children)
}

#[inline(always)]
pub fn h_flex() -> Div {
    div().h_flex()
}

#[inline(always)]
pub fn v_flex() -> Div {
    div().v_flex()
}

pub trait StatefulInteractiveElementExt {
    fn on_double_click(
        self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self;

    fn action_tooltip(self, action: Box<dyn Action>) -> Self;
}

impl<E: StatefulInteractiveElement> StatefulInteractiveElementExt for E {
    fn on_double_click(
        self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click(move |event, window, cx| {
            if event.click_count() == 2 {
                listener(event, window, cx);
            }
        })
    }

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

pub trait StyledExt {
    fn h_flex(self) -> Self;

    fn v_flex(self) -> Self;

    fn refine_style(self, style: &StyleRefinement) -> Self;
}

impl<E: Styled> StyledExt for E {
    #[inline(always)]
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    #[inline(always)]
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    #[inline(always)]
    fn refine_style(mut self, style: &StyleRefinement) -> Self {
        self.style().refine(style);
        self
    }
}

pub trait StyledParentExt {
    fn ring(self, color: impl Into<Hsla>, window: &Window, cx: &App) -> Self;
}

impl<E: Styled + ParentElement> StyledParentExt for E {
    fn ring(mut self, color: impl Into<Hsla>, window: &Window, cx: &App) -> Self {
        const RING_BORDER_WIDTH: Pixels = px(1.5);
        let rem_size = window.rem_size();
        let style = self.style();

        let border_widths = Edges::<Pixels> {
            top: style.border_widths.top.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            bottom: style.border_widths.bottom.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            left: style.border_widths.left.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            right: style.border_widths.right.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
        };

        let inset = RING_BORDER_WIDTH + px(1.0);

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
                .border_color(color),
        )
    }
}

pub trait FocusableExt<T: ParentElement + Styled + Sized> {
    fn focus_ring(self, is_focused: bool, window: &Window, cx: &App) -> Self;
}

impl<T: ParentElement + Styled + Sized> FocusableExt<T> for T {
    fn focus_ring(self, is_focused: bool, window: &Window, cx: &App) -> Self {
        if !is_focused {
            return self;
        }

        self.ring(cx.theme().border_focus.alpha(0.8), window, cx)
    }
}

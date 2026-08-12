use gpui::{
    Action, App, ClickEvent, Div, Edges, Empty, FocusHandle, FontWeight, Hsla, Pixels, ReadGlobal,
    StyleRefinement, Window, deferred, div, prelude::*, px,
};

use crate::{ActiveTheme, Emphasis, FocusGlobal, HslaExt, comp::Binding};

#[inline(always)]
pub fn h_flex() -> Div {
    div().h_flex()
}

#[inline(always)]
pub fn v_flex() -> Div {
    div().v_flex()
}

#[inline(always)]
pub fn c_flex() -> Div {
    div().c_flex()
}

pub trait StyledExt {
    fn h_flex(self) -> Self;
    fn v_flex(self) -> Self;
    fn c_flex(self) -> Self;

    fn font_thin(self) -> Self;
    fn font_extra_light(self) -> Self;
    fn font_light(self) -> Self;
    fn font_normal(self) -> Self;
    fn font_medium(self) -> Self;
    fn font_semibold(self) -> Self;
    fn font_bold(self) -> Self;
    fn font_extra_bold(self) -> Self;
    fn font_black(self) -> Self;

    fn emphasis(self, emphasis: Emphasis, cx: &App) -> Self;
    fn disabled_emphasis(self, emphasis: Emphasis, cx: &App) -> Self;
    fn emphasis_bordered(self, emphasis: Emphasis, cx: &App) -> Self;
    fn disabled_emphasis_bordered(self, emphasis: Emphasis, cx: &App) -> Self;

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
    fn c_flex(self) -> Self {
        self.v_flex().items_center().justify_center()
    }

    #[inline(always)]
    fn font_thin(self) -> Self {
        self.font_weight(FontWeight::THIN)
    }

    #[inline(always)]
    fn font_extra_light(self) -> Self {
        self.font_weight(FontWeight::EXTRA_LIGHT)
    }

    #[inline(always)]
    fn font_light(self) -> Self {
        self.font_weight(FontWeight::LIGHT)
    }

    #[inline(always)]
    fn font_normal(self) -> Self {
        self.font_weight(FontWeight::NORMAL)
    }

    #[inline(always)]
    fn font_medium(self) -> Self {
        self.font_weight(FontWeight::MEDIUM)
    }

    #[inline(always)]
    fn font_semibold(self) -> Self {
        self.font_weight(FontWeight::SEMIBOLD)
    }

    #[inline(always)]
    fn font_bold(self) -> Self {
        self.font_weight(FontWeight::BOLD)
    }

    #[inline(always)]
    fn font_extra_bold(self) -> Self {
        self.font_weight(FontWeight::EXTRA_BOLD)
    }

    #[inline(always)]
    fn font_black(self) -> Self {
        self.font_weight(FontWeight::BLACK)
    }

    #[inline(always)]
    fn emphasis(self, emphasis: Emphasis, cx: &App) -> Self {
        self.bg(emphasis.bg_color(cx)).text_color(emphasis.fg_color(cx))
    }

    #[inline(always)]
    fn disabled_emphasis(self, emphasis: Emphasis, cx: &App) -> Self {
        self.bg(emphasis.bg_color(cx).disabled()).text_color(emphasis.fg_color(cx).disabled())
    }

    #[inline(always)]
    fn emphasis_bordered(self, emphasis: Emphasis, cx: &App) -> Self {
        self.emphasis(emphasis, cx)
            .border_1()
            .border_color(emphasis.border_color(cx))
            .rounded(cx.theme().radius)
    }

    #[inline(always)]
    fn disabled_emphasis_bordered(self, emphasis: Emphasis, cx: &App) -> Self {
        self.disabled_emphasis(emphasis, cx)
            .border_1()
            .border_color(emphasis.border_color(cx).disabled())
            .rounded(cx.theme().radius)
    }

    #[inline(always)]
    fn refine_style(mut self, style: &StyleRefinement) -> Self {
        self.style().refine(style);
        self
    }
}

pub trait StyledParentExt {
    fn ring(self, color: impl Into<Hsla>, window: &Window, cx: &App) -> Self;

    fn focus_ring(self, focus_handle: &FocusHandle, window: &Window, cx: &App) -> Self;
}

impl<E: Styled + ParentElement> StyledParentExt for E {
    fn ring(mut self, color: impl Into<Hsla>, window: &Window, cx: &App) -> Self {
        const RING_BORDER_WIDTH: Pixels = px(1.0);
        let rem_size = window.rem_size();
        let style = self.style();

        let border_widths = Edges::<Pixels> {
            top: style.border_widths.top.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            bottom: style.border_widths.bottom.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            left: style.border_widths.left.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
            right: style.border_widths.right.map(|v| v.to_pixels(rem_size)).unwrap_or_default(),
        };

        let inset = RING_BORDER_WIDTH / 2.0;

        self.child(deferred(
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
        ))
    }

    fn focus_ring(self, focus_handle: &FocusHandle, window: &Window, cx: &App) -> Self {
        if focus_handle.is_focused(window) && FocusGlobal::global(cx).last_focus_was_keyboard {
            self.ring(cx.theme().border_focused.opacity(0.75), window, cx)
        } else {
            self
        }
    }
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

pub trait StyledStatefulInteractiveElementExt {
    fn interactive_emphasis(self, emphasis: Emphasis, cx: &App) -> Self;

    fn interactive_emphasis_bordered(self, emphasis: Emphasis, cx: &App) -> Self;
}

impl<E: Styled + StatefulInteractiveElement> StyledStatefulInteractiveElementExt for E {
    #[inline(always)]
    fn interactive_emphasis(self, emphasis: Emphasis, cx: &App) -> Self {
        match emphasis {
            Emphasis::Ghost => StyledExt::emphasis(self, emphasis, cx)
                .hover(|e| e.bg(cx.theme().bg_secondary))
                .active(|e| e.bg(cx.theme().bg_tertiary)),
            _ => StyledExt::emphasis(self, emphasis, cx)
                .hover(|e| e.bg(emphasis.bg_color(cx).hover()))
                .active(|e| e.bg(emphasis.bg_color(cx).active()).top(cx.theme().button_depression)),
        }
    }

    #[inline(always)]
    fn interactive_emphasis_bordered(self, emphasis: Emphasis, cx: &App) -> Self {
        match emphasis {
            Emphasis::Ghost => StyledExt::emphasis_bordered(self, emphasis, cx)
                .hover(|e| e.bg(cx.theme().bg_secondary))
                .active(|e| e.bg(cx.theme().bg_tertiary)),
            _ => StyledExt::emphasis_bordered(self, emphasis, cx)
                .hover(|e| {
                    e.bg(emphasis.bg_color(cx).hover())
                        .border_color(emphasis.border_color(cx).hover())
                })
                .active(|e| {
                    e.bg(emphasis.bg_color(cx).active())
                        .border_color(emphasis.border_color(cx).active())
                        .top(cx.theme().button_depression)
                }),
        }
    }
}

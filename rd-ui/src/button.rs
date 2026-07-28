use gpui::{
    App, ClickEvent, ElementId, FocusHandle, Focusable, Hsla, SharedString, StyleRefinement,
    Window, div, prelude::*,
};

use crate::{ActiveTheme, FocusableExt, HslaExt, Icon, StyledExt, h_flex};

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: Option<SharedString>,
    icon: Option<Icon>,
    focus_handle: FocusHandle,
    style: StyleRefinement,
    variant: ButtonVariant,
    is_disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, focus_handle: FocusHandle) -> Self {
        Self {
            id: id.into(),
            label: None,
            icon: None,
            focus_handle,
            style: StyleRefinement::default(),
            variant: Default::default(),
            is_disabled: false,
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = self.variant.style(cx);

        let mut bg = style.bg;
        let mut text_color = style.text_color;
        let mut border_color = style.border_color;

        if self.is_disabled {
            bg = bg.disabled();
            text_color = text_color.disabled();
            border_color = border_color.disabled();
        }

        let shadow = cx.theme().shadow && style.shadow;

        div()
            .id(self.id)
            .track_focus(&self.focus_handle)
            .min_size_2()
            .flex()
            .items_center()
            .justify_center()
            .refine_style(&self.style)
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded(cx.theme().radius)
            .text_color(text_color)
            .when(self.is_disabled, |e| e.cursor_not_allowed())
            .when(!self.is_disabled, |e| {
                e.hover(|e| e.bg(style.bg_hover))
                    .active(|e| e.bg(style.bg_active).top(cx.theme().button_depression))
                    .on_click(move |event, window, cx| {
                        if let Some(on_click) = &self.on_click {
                            (on_click)(event, window, cx)
                        }
                    })
            })
            .when(shadow, |e| e.shadow_xs())
            .block_mouse_except_scroll()
            .focus_ring(self.focus_handle.is_focused(window), window, cx)
            .when_some(self.icon, |e, icon| {
                e.child(
                    h_flex()
                        .justify_center()
                        .h_full()
                        .px_1()
                        .border_r_1()
                        .border_color(border_color)
                        .child(icon),
                )
            })
            .when_some(self.label, |e, label| {
                e.child(div().w_full().px_2().text_center().child(label))
            })
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Focusable for Button {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
    Warning,
}

struct VariantStyle {
    bg: Hsla,
    bg_hover: Hsla,
    bg_active: Hsla,
    border_color: Hsla,
    text_color: Hsla,
    shadow: bool,
}

impl ButtonVariant {
    fn style(&self, cx: &App) -> VariantStyle {
        let theme = cx.theme();

        match self {
            Self::Primary => {
                let mut bg = theme.accent;
                bg.fade_out(0.5);

                VariantStyle {
                    bg,
                    bg_hover: bg.hover(),
                    bg_active: bg.active(),
                    border_color: theme.accent,
                    text_color: theme.fg_primary,
                    shadow: true,
                }
            }
            Self::Secondary => {
                let bg = theme.bg_tertiary;

                VariantStyle {
                    bg,
                    bg_hover: bg.hover(),
                    bg_active: bg.active(),
                    border_color: theme.border_tertiary,
                    text_color: theme.fg_primary,
                    shadow: true,
                }
            }
            Self::Ghost => VariantStyle {
                bg: theme.bg_primary.opacity(0.0),
                bg_hover: theme.bg_secondary,
                bg_active: theme.bg_tertiary,
                border_color: theme.border_primary.opacity(0.0),
                text_color: theme.fg_secondary,
                shadow: false,
            },
            Self::Danger => {
                let bg = theme.indicate.danger;
                let mut border_color = bg;
                border_color.l -= 0.2;

                VariantStyle {
                    bg,
                    bg_hover: bg.hover(),
                    bg_active: bg.active(),
                    border_color,
                    text_color: theme.fg_primary,
                    shadow: true,
                }
            }
            Self::Warning => {
                let bg = theme.indicate.warning;
                let mut border_color = bg;
                border_color.l -= 0.2;

                VariantStyle {
                    bg,
                    bg_hover: bg.hover(),
                    bg_active: bg.active(),
                    border_color,
                    text_color: theme.fg_primary,
                    shadow: true,
                }
            }
        }
    }
}

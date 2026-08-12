use gpui::{
    Action, App, ClickEvent, ElementId, FocusHandle, Focusable, SharedString, StyleRefinement,
    Window, div, prelude::*,
};

use crate::{
    ActiveTheme, Emphasis, HslaExt, StatefulInteractiveElementExt, StyledExt, StyledParentExt,
    StyledStatefulInteractiveElementExt,
    comp::{Disableable, FocusableComponent, Icon, IconSize, IconVariant, Identifiable, Labelled},
    h_flex,
};

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    focus_handle: FocusHandle,
    label: Option<SharedString>,
    icon: Option<IconVariant>,
    style: StyleRefinement,
    variant: ButtonVariant,
    center_text: bool,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    action: Option<Box<dyn Action>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut App) -> Self {
        let id = id.into();
        let focus_handle = window
            .use_keyed_state(id.clone(), cx, |_window, cx| cx.focus_handle().tab_stop(true))
            .read(cx)
            .clone();

        Self {
            id,
            focus_handle,

            label: None,
            icon: None,
            style: StyleRefinement::default(),
            variant: Default::default(),
            center_text: true,
            disabled: false,
            on_click: None,
            action: None,
        }
    }

    pub fn variant(&self) -> ButtonVariant {
        self.variant
    }

    pub fn set_variant(&mut self, variant: ButtonVariant) {
        self.variant = variant;
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn center_text(&self) -> bool {
        self.center_text
    }

    pub fn set_center_text(&mut self, center_text: bool) {
        self.center_text = center_text;
    }

    pub fn with_center_text(mut self, center_text: bool) -> Self {
        self.center_text = center_text;
        self
    }

    pub fn with_action<A: Action>(mut self, action: A) -> Self {
        self.action = Some(Box::new(action));
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
        let Button {
            id,
            label,
            icon,
            focus_handle,
            style,
            center_text,
            variant,
            disabled,
            on_click,
            action,
        } = self;

        let border_color = variant.border_color(cx, disabled);
        let shadow = cx.theme().shadow && variant.shadow();
        let tooltip_action = action.as_ref().map(|action| action.boxed_clone());
        let click_action = action.as_ref().map(|action| action.boxed_clone());

        div()
            .id(id)
            .when(!disabled, |e| e.track_focus(&focus_handle).focus_ring(&focus_handle, window, cx))
            .min_w(crate::comp::INPUT_SIZE)
            .h(crate::comp::INPUT_SIZE)
            .flex()
            .items_center()
            .justify_center()
            .when(disabled, |e| variant.disabled_style(e, cx).cursor_not_allowed())
            .when(!disabled, |e| {
                variant.enabled_style(e, cx).on_click(move |event, window, cx| {
                    if let Some(on_click) = &on_click {
                        (on_click)(event, window, cx);
                    }

                    if let Some(action) = &click_action {
                        window.dispatch_action(action.boxed_clone(), cx);
                    }
                })
            })
            .when_some(tooltip_action, |e, action| e.action_tooltip(action))
            .when(shadow, |e| e.shadow_xs())
            .when_some(icon, |e, icon| {
                e.child(
                    h_flex()
                        .justify_center()
                        .h_full()
                        .px_1()
                        .when(label.is_some(), |e| e.border_r_1().border_color(border_color))
                        .child(Icon::new(icon, IconSize::ExtraSmall)),
                )
            })
            .when_some(label, |e, label| {
                e.child(div().w_full().px_2().when(center_text, |e| e.text_center()).child(label))
            })
            .block_mouse_except_scroll()
            .refine_style(&style)
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Disableable for Button {
    fn disabled(&self, _cx: &App) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool, _cx: &mut App) {
        self.disabled = disabled;
    }
}

impl Identifiable for Button {
    fn id(&self, _cx: &App) -> &ElementId {
        &self.id
    }
}

impl Labelled for Button {
    fn label(&self) -> Option<&SharedString> {
        self.label.as_ref()
    }

    fn set_label(&mut self, label: impl Into<Option<SharedString>>) {
        self.label = label.into();
    }

    fn icon(&self) -> Option<IconVariant> {
        self.icon
    }

    fn set_icon(&mut self, icon: impl Into<Option<IconVariant>>) {
        self.icon = icon.into();
    }
}

impl Focusable for Button {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl FocusableComponent for Button {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = focus_handle;
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

impl ButtonVariant {
    fn shadow(&self) -> bool {
        !matches!(self, Self::Ghost)
    }

    fn border_color(&self, cx: &App, disabled: bool) -> gpui::Hsla {
        let color = match self {
            Self::Primary => Emphasis::Selected.border_color(cx),
            Self::Secondary => Emphasis::Tertiary.border_color(cx),
            Self::Ghost => Emphasis::Ghost.border_color(cx),
            Self::Danger => Emphasis::Danger.border_color(cx),
            Self::Warning => Emphasis::Warning.border_color(cx),
        };

        if disabled { color.disabled() } else { color }
    }

    fn enabled_style<E: Styled + StatefulInteractiveElement>(self, e: E, cx: &App) -> E {
        match self {
            Self::Primary => e.interactive_emphasis_bordered(Emphasis::Selected, cx),
            Self::Secondary => e.interactive_emphasis_bordered(Emphasis::Secondary, cx),
            Self::Ghost => e.interactive_emphasis_bordered(Emphasis::Ghost, cx),
            Self::Danger => e.interactive_emphasis_bordered(Emphasis::Danger, cx),
            Self::Warning => e.interactive_emphasis_bordered(Emphasis::Warning, cx),
        }
    }

    fn disabled_style<E: Styled>(self, e: E, cx: &App) -> E {
        match self {
            Self::Primary => e.disabled_emphasis_bordered(Emphasis::Selected, cx),
            Self::Secondary => e.disabled_emphasis_bordered(Emphasis::Tertiary, cx),
            Self::Ghost => e.disabled_emphasis_bordered(Emphasis::Ghost, cx),
            Self::Danger => e.disabled_emphasis_bordered(Emphasis::Danger, cx),
            Self::Warning => e.disabled_emphasis_bordered(Emphasis::Warning, cx),
        }
    }
}

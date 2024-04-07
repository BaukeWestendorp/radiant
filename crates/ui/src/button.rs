use gpui::prelude::FluentBuilder;
use gpui::{
    div, AnyElement, AppContext, ClickEvent, Div, ElementId, Hsla, InteractiveElement, IntoElement,
    MouseButton, MouseDownEvent, MouseUpEvent, ParentElement, RenderOnce,
    StatefulInteractiveElement, StyleRefinement, Styled, WindowContext,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

use crate::disableable::Disableable;
use crate::selectable::Selectable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
}

impl ButtonStyle {
    pub fn border(&self, cx: &AppContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().colors().border,
            ButtonStyle::Secondary => cx.theme().colors().border,
        }
    }

    pub fn border_selected(&self, cx: &AppContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().colors().border_selected,
            ButtonStyle::Secondary => cx.theme().colors().border_selected,
        }
    }

    pub fn border_disabled(&self, cx: &AppContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().colors().border,
            ButtonStyle::Secondary => cx.theme().colors().border,
        }
    }

    pub fn background(&self, cx: &AppContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().colors().element_background,
            ButtonStyle::Secondary => cx.theme().colors().element_background_secondary,
        }
    }

    pub fn background_hover(&self, cx: &AppContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().colors().element_background_hover,
            ButtonStyle::Secondary => cx.theme().colors().element_background_hover_secondary,
        }
    }

    pub fn background_active(&self, cx: &AppContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().colors().element_background_active,
            ButtonStyle::Secondary => cx.theme().colors().element_background_active_secondary,
        }
    }
}

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Button {
    base: Div,
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    on_press: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
    on_release: Option<Box<dyn Fn(&MouseUpEvent, &mut WindowContext) + 'static>>,
    disabled: bool,
    selected: bool,
    style: ButtonStyle,
}

impl Button {
    pub fn new(style: ButtonStyle, id: impl Into<ElementId>, cx: &AppContext) -> Self {
        let base = div()
            .border()
            .border_color(style.border(cx))
            .rounded_md()
            .bg(style.background(cx));

        Self {
            base,
            id: id.into(),
            children: SmallVec::new(),
            on_click: None,
            on_press: None,
            on_release: None,
            disabled: false,
            selected: false,
            style,
        }
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }

    pub fn on_press(
        mut self,
        listener: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_press = Some(Box::new(listener));
        self
    }

    pub fn on_release(
        mut self,
        listener: impl Fn(&MouseUpEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_release = Some(Box::new(listener));
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for Button {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .id(self.id.clone())
            .when(self.selected, |this| {
                this.border_color(self.style.border_selected(cx))
            })
            .when(self.disabled, |this| {
                this.cursor_not_allowed()
                    .border_color(self.style.border_disabled(cx))
            })
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|hover| hover.bg(self.style.background_hover(cx)))
                    .active(|active| active.bg(self.style.background_active(cx)))
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, move |_event, cx| cx.prevent_default())
                        .on_click(move |event, cx| {
                            cx.stop_propagation();
                            (on_click)(event, cx)
                        })
                },
            )
            .when_some(
                self.on_press.filter(|_| !self.disabled),
                |this, on_press| {
                    this.on_mouse_down(MouseButton::Left, move |event, cx| {
                        cx.prevent_default();
                        cx.stop_propagation();
                        (on_press)(event, cx)
                    })
                },
            )
            .when_some(
                self.on_release.filter(|_| !self.disabled),
                |this, on_release| {
                    this.on_mouse_up(MouseButton::Left, move |event, cx| {
                        cx.prevent_default();
                        cx.stop_propagation();
                        (on_release)(event, cx)
                    })
                },
            )
            .children(self.children)
    }
}

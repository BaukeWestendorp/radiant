use gpui::{AnyElement, App, ElementId, SharedString, Window, div, prelude::*};

use crate::{
    ActiveTheme, Emphasis, StyledExt,
    comp::{Icon, IconSize, IconVariant, Identifiable, Labelled},
};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(IntoElement)]
pub struct Label {
    id: ElementId,

    label: Option<SharedString>,
    icon: Option<IconVariant>,
    content: AnyElement,
    direction: LabelDirection,
    error: Result<(), SharedString>,
}

impl Label {
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),

            label: None,
            icon: None,
            content: content.into_any_element(),
            direction: LabelDirection::default(),
            error: Ok(()),
        }
    }

    pub fn content(&self) -> &AnyElement {
        &self.content
    }

    pub fn set_content(&mut self, content: impl IntoElement) {
        self.content = content.into_any_element();
    }

    pub fn direction(&self) -> LabelDirection {
        self.direction
    }

    pub fn set_direction(&mut self, direction: LabelDirection) {
        self.direction = direction;
    }

    pub fn with_direction(mut self, direction: LabelDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn error_message(&self) -> Result<&(), &SharedString> {
        self.error.as_ref()
    }

    pub fn set_error_message(&mut self, error: impl Into<Result<(), SharedString>>) {
        self.error = error.into();
    }

    pub fn with_error_message(mut self, error: impl Into<Result<(), SharedString>>) -> Self {
        self.error = error.into();
        self
    }
}

impl Identifiable for Label {
    fn id(&self, _cx: &App) -> &ElementId {
        &self.id
    }
}

impl Labelled for Label {
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

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let has_error = self.error.is_err();
        let label = match (self.icon, self.label) {
            (None, None) => None,
            (icon, label) => Some(
                div()
                    .h_flex()
                    .gap_1()
                    .items_center()
                    .text_color(Emphasis::Secondary.fg_color(cx))
                    .when(has_error, |e| e.italic().text_color(cx.theme().indicate.danger))
                    .when_some(icon, |e, icon| e.child(Icon::new(icon, IconSize::ExtraSmall)))
                    .when_some(label, |e, label| e.child(label)),
            ),
        };

        let label = match self.direction {
            LabelDirection::Vertical => div()
                .v_flex()
                .gap_1()
                .when_some(label, |e, label| e.child(label))
                .child(self.content),
            LabelDirection::Horizontal => div()
                .h_flex()
                .justify_between()
                .gap_2()
                .when_some(label, |e, label| e.child(label))
                .child(self.content),
        };

        label.id(self.id.clone()).when_some(self.error.err(), |e, error| {
            e.text_color(cx.theme().indicate.danger).hoverable_tooltip(move |_, cx| {
                struct Error(SharedString);
                impl Render for Error {
                    fn render(
                        &mut self,
                        _window: &mut Window,
                        cx: &mut Context<Self>,
                    ) -> impl IntoElement {
                        div()
                            .emphasis_bordered(Emphasis::Secondary, cx)
                            .px_2()
                            .py_1()
                            .text_color(cx.theme().indicate.danger)
                            .child(self.0.clone())
                    }
                }
                cx.new(|_| Error(error.clone())).into()
            })
        })
    }
}

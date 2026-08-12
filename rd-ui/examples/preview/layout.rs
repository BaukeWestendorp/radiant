use rd_ui::{
    ActiveTheme, Emphasis, StyledExt,
    gpui::{
        AnyElement, AnyView, App, RenderOnce, SharedString, StyleRefinement, Styled, Window, div,
        prelude::*,
    },
    v_flex,
};

pub struct PreviewPage {
    title: SharedString,
    description: Option<SharedString>,
    content: AnyView,
    style: StyleRefinement,
}

impl PreviewPage {
    pub fn new(title: impl Into<SharedString>, content: impl Into<AnyView>) -> Self {
        Self {
            title: title.into(),
            description: None,
            content: content.into(),
            style: StyleRefinement::default(),
        }
    }

    pub fn with_description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl Render for PreviewPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_2()
            .p_2()
            .refine_style(&self.style)
            .child(
                v_flex()
                    .w_full()
                    .gap_1()
                    .child(div().font_bold().child(self.title.clone()))
                    .when_some(self.description.clone(), |e, description| {
                        e.child(div().text_color(cx.theme().fg_secondary).child(description))
                    }),
            )
            .child(preview_inset(self.content.clone(), cx))
    }
}

impl Styled for PreviewPage {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

#[derive(IntoElement)]
pub struct PreviewStory {
    title: SharedString,
    content: AnyElement,
    style: StyleRefinement,
}

impl PreviewStory {
    pub fn new(title: impl Into<SharedString>, content: impl IntoElement) -> Self {
        Self {
            title: title.into(),
            content: content.into_any_element(),
            style: StyleRefinement::default(),
        }
    }
}

impl RenderOnce for PreviewStory {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_2()
            .refine_style(&self.style)
            .child(div().font_bold().child(self.title))
            .child(preview_inset(self.content, cx))
    }
}

impl Styled for PreviewStory {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

pub fn preview_inset(content: impl IntoElement, cx: &mut App) -> impl IntoElement {
    v_flex().size_full().emphasis_bordered(Emphasis::Primary, cx).child(content)
}

use gpui::{
    div, IntoElement, Model, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::geo::Bounds;

pub mod attribute_editor;

pub struct Window {
    pub id: usize,
    pub bounds: Bounds,
    pub kind: WindowKind,
}

pub struct WindowView<D: WindowDelegate> {
    window: Window,
    delegate: D,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum WindowKind {
    AttributeEditor,
}

impl<D: WindowDelegate + 'static> WindowView<D> {
    pub fn build(window: Window, delegate: D, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { window, delegate })
    }

    fn render_content(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let background = div()
            .size_full()
            .border()
            .border_color(gpui::white())
            .rounded_md();

        div()
            .size_full()
            .relative()
            .child(div().absolute().size_full().child(background))
            .child(
                div()
                    .absolute()
                    .size_full()
                    .child(self.delegate.render_content(cx)),
            )
    }
}

impl<D: WindowDelegate + 'static> Render for WindowView<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = self.render_content(cx);
        let header = self.delegate.render_header(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .children(header)
            .child(content)
    }
}

pub trait WindowDelegate {
    fn title(&mut self, cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString>
    where
        Self: Sized;

    fn render_content(&mut self, cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement
    where
        Self: Sized;

    fn header_items(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Vec<impl IntoElement>
    where
        Self: Sized,
    {
        Vec::<gpui::Empty>::new()
    }

    fn render_header(&mut self, cx: &mut ViewContext<WindowView<Self>>) -> Option<impl IntoElement>
    where
        Self: Sized,
    {
        let main = div()
            .border()
            .border_color(gpui::blue())
            .rounded_md()
            .size_full()
            .flex()
            .items_center()
            .px_2()
            .children(self.title(cx));

        Some(
            div()
                .w_full()
                .h_10()
                .flex()
                .gap_1()
                .child(main)
                .children(self.header_items(cx)),
        )
    }
}

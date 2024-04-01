use gpui::{
    div, IntoElement, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};
use ui::container::{Container, ContainerStyle};

pub struct Window<D: WindowDelegate> {
    delegate: D,
}

impl<D: WindowDelegate + 'static> Window<D> {
    pub fn build(delegate: D, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { delegate })
    }

    pub fn render_header(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let main = Container::new(cx)
            .container_style(ContainerStyle {
                background: gpui::rgb(0x0000a0).into(),
                border: gpui::rgb(0x0000ff).into(),
            })
            .size_full();
        let close_button = Container::new(cx).w_10().h_full();

        div()
            .w_full()
            .h_10()
            .flex()
            .gap_1()
            .children([main, close_button])
    }

    pub fn render_content(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let background = Container::new(cx).size_full();

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

impl<D: WindowDelegate + 'static> Render for Window<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let header = self.render_header(cx);
        let content = self.render_content(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_1()
            .child(header)
            .child(content)
    }
}

pub trait WindowDelegate {
    fn render_content(&mut self, cx: &mut ViewContext<Window<Self>>) -> impl IntoElement
    where
        Self: Sized;
}

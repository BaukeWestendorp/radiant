use gpui::{
    div, IntoElement, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use ui::container::{Container, ContainerStyle};

pub mod group_pool;

pub struct WindowView<D: WindowViewDelegate> {
    delegate: D,
}

impl<D: WindowViewDelegate + 'static> WindowView<D> {
    pub fn build(delegate: D, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { delegate })
    }

    pub fn close(&self) {
        todo!("Close window");
    }

    fn render_content(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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

impl<D: WindowViewDelegate + 'static> Render for WindowView<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = self.render_content(cx);
        let header = self.delegate.render_header(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_1()
            .children(header)
            .child(content)
    }
}

pub trait WindowViewDelegate {
    fn title(&self, cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString>
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
        let main = Container::new(cx)
            .container_style(ContainerStyle {
                background: gpui::rgb(0x0000a0).into(),
                border: gpui::rgb(0x0000ff).into(),
            })
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

use gpui::{
    div, IntoElement, Model, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use ui::container::{Container, ContainerStyle};

use crate::showfile::Window;

pub mod executors;
pub mod fixture_sheet;

pub mod all_pool;
pub mod beam_pool;
pub mod color_pool;
pub mod dimmer_pool;
pub mod focus_pool;
pub mod gobo_pool;
pub mod group_pool;
pub mod pool;
pub mod position_pool;

pub struct WindowView<D: WindowDelegate> {
    window: Model<Window>,
    delegate: D,
}

impl<D: WindowDelegate + 'static> WindowView<D> {
    pub fn build(window: Model<Window>, delegate: D, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { window, delegate })
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

impl<D: WindowDelegate + 'static> Render for WindowView<D> {
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

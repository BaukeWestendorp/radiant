use gpui::{
    div, px, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};
use theme::ActiveTheme;

use crate::showfile::{self, Layout, PoolWindowKind, WindowKind};
use crate::window::group_pool::GroupPoolWindowViewDelegate;
use crate::window::{WindowView, WindowViewDelegate};

pub const GRID_SIZE: gpui::Pixels = px(80.0);

pub struct LayoutView {
    layout: Model<Layout>,
}

impl LayoutView {
    pub fn build(layout: Model<Layout>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { layout })
    }

    fn render_grid(&self, cx: &mut WindowContext) -> impl IntoElement {
        let grid_size = self.layout.read(cx).size;
        let dot_size = 2.0;

        let mut dots = vec![];
        for x in 0..grid_size.width + 1 {
            for y in 0..grid_size.height + 1 {
                let dot = div()
                    .absolute()
                    .size(px(dot_size))
                    .bg(cx.theme().colors().accent)
                    .top(y as f32 * GRID_SIZE - px(dot_size / 2.0))
                    .left(x as f32 * GRID_SIZE - px(dot_size / 2.0));
                dots.push(dot);
            }
        }

        div()
            .w(grid_size.width as f32 * GRID_SIZE)
            .h(grid_size.height as f32 * GRID_SIZE)
            .children(dots)
    }

    fn render_content(&self, cx: &mut WindowContext) -> impl IntoElement {
        let windows = self.layout.read(cx).windows.clone();
        let window_views = windows
            .into_iter()
            .map(|window| get_window_view(window, cx));

        div().size_full().absolute().children(window_views)
    }
}

fn get_window_view(
    window: showfile::Window,
    cx: &mut WindowContext,
) -> View<WindowView<impl WindowViewDelegate>> {
    let delegate = match window.kind {
        WindowKind::Pool(pool_window) => match pool_window.kind {
            PoolWindowKind::ColorPreset => todo!(),
            PoolWindowKind::Group => GroupPoolWindowViewDelegate::new(),
        },
        WindowKind::Executors => todo!(),
        WindowKind::FixtureSheet => todo!(),
    };

    WindowView::build(delegate, cx)
}

impl Render for LayoutView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .relative()
            .child(div().absolute().inset_0().child(self.render_grid(cx)))
            .child(div().absolute().inset_0().child(self.render_content(cx)))
    }
}

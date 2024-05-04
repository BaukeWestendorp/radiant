use backstage::show::FixtureId;
use gpui::{
    div, px, AnyView, IntoElement, Model, ParentElement, Pixels, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::{
    showfile::{Layout, PoolWindowKind, Window, WindowKind},
    theme::THEME,
    window::{
        attribute_editor::AttributeEditorWindowDelegate,
        graph_editor::GraphEditorWindowDelegate,
        pool::{GroupPoolWindowDelegate, PoolWindowDelegate, PresetPoolWindowDelegate},
        WindowView,
    },
};

pub const GRID_SIZE: Pixels = px(80.0);

pub struct LayoutView {
    pub layout: Model<Layout>,
    window_views: Vec<(usize, AnyView)>,
}

impl LayoutView {
    pub fn build(
        layout: Model<Layout>,
        selected_fixtures: Model<Vec<FixtureId>>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let window_views = get_window_views(layout.clone(), selected_fixtures.clone(), cx);

            cx.observe(&layout, {
                let selected_fixtures = selected_fixtures.clone();
                move |this: &mut Self, layout, cx| {
                    this.window_views =
                        get_window_views(layout.clone(), selected_fixtures.clone(), cx);
                    cx.notify();
                }
            })
            .detach();

            Self {
                layout,
                window_views,
            }
        })
    }

    // FIXME: We could make this a lot faster with a canvas.
    fn render_grid(&self, cx: &mut WindowContext) -> impl IntoElement {
        let grid_size = self.layout.read(cx).size;
        let dot_size = 2.0;

        let mut dots = vec![];
        for x in 0..grid_size.width + 1 {
            for y in 0..grid_size.height + 1 {
                let dot = div()
                    .absolute()
                    .size(px(dot_size))
                    .bg(THEME.accent)
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
        let window_views = self.window_views.iter().map(|(id, window_view)| {
            let Some(window) = self.layout.read(cx).window(*id) else {
                return div();
            };

            div()
                .absolute()
                .top(window.bounds.origin.y as f32 * GRID_SIZE)
                .left(window.bounds.origin.x as f32 * GRID_SIZE)
                .w(window.bounds.size.width as f32 * GRID_SIZE)
                .h(window.bounds.size.height as f32 * GRID_SIZE)
                .child(window_view.clone())
        });

        div().size_full().relative().children(window_views)
    }
}

fn get_window_views(
    layout: Model<Layout>,
    selected_fixtures: Model<Vec<FixtureId>>,
    cx: &mut WindowContext,
) -> Vec<(usize, AnyView)> {
    let windows = layout.read(cx).windows.clone();
    windows
        .into_iter()
        .map(move |window| {
            let window_id = window.id;
            (
                window_id,
                get_window_view(window, selected_fixtures.clone(), cx),
            )
        })
        .collect()
}

fn get_window_view(
    window: Window,
    selected_fixtures: Model<Vec<FixtureId>>,
    cx: &mut WindowContext,
) -> AnyView {
    match &window.kind {
        WindowKind::Pool(pool_window) => match pool_window.kind {
            PoolWindowKind::Group => {
                let delegate = PoolWindowDelegate::new(GroupPoolWindowDelegate::new(), window);
                WindowView::build(delegate, cx).into()
            }
            PoolWindowKind::Preset(kind) => {
                let delegate =
                    PoolWindowDelegate::new(PresetPoolWindowDelegate::new(kind.into()), window);
                WindowView::build(delegate, cx).into()
            }
        },
        WindowKind::AttributeEditor => {
            let delegate = AttributeEditorWindowDelegate::new(selected_fixtures, cx);
            WindowView::build(delegate, cx).into()
        }
        WindowKind::GraphEditor(graph_id) => {
            let delegate = GraphEditorWindowDelegate::new(Some(*graph_id), cx);
            WindowView::build(delegate, cx).into()
        }
    }
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

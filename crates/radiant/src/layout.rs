use gpui::{
    div, px, AnyView, Context, IntoElement, Model, ParentElement, Render, Styled, View,
    ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;

use crate::showfile::{Layout, PoolWindowKind, Window, WindowKind};
use crate::window::all_pool::AllPoolWindowDelegate;
use crate::window::attribute_editor::AttributeEditorWindowDelegate;
use crate::window::beam_pool::BeamPoolWindowDelegate;
use crate::window::color_pool::ColorPoolWindowDelegate;
use crate::window::dimmer_pool::DimmerPoolWindowDelegate;
use crate::window::executors::ExecutorsWindowDelegate;
use crate::window::fixture_sheet::FixtureSheetWindowDelegate;
use crate::window::focus_pool::FocusPoolWindowDelegate;
use crate::window::gobo_pool::GoboPoolWindowDelegate;
use crate::window::group_pool::GroupPoolWindowDelegate;
use crate::window::position_pool::PositionPoolWindowDelegate;
use crate::window::sequence_pool::SequencePoolWindowDelegate;
use crate::window::WindowView;

pub const GRID_SIZE: gpui::Pixels = px(80.0);

pub struct LayoutView {
    layout: Model<Layout>,
    window_views: Vec<(usize, AnyView)>,
}

impl LayoutView {
    pub fn build(layout: Model<Layout>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let window_models = get_window_models(layout.clone(), cx);
            let window_views = get_window_views(window_models.clone(), layout.clone(), cx);

            Self {
                layout,
                window_views,
            }
        })
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
        let window_views = self.window_views.iter().map(|(id, window)| {
            let bounds = self.layout.read(cx).window(*id).unwrap().bounds;

            div()
                .absolute()
                .top(bounds.origin.y as f32 * GRID_SIZE)
                .left(bounds.origin.x as f32 * GRID_SIZE)
                .w(bounds.size.width as f32 * GRID_SIZE)
                .h(bounds.size.height as f32 * GRID_SIZE)
                .child(window.clone())
        });

        div().size_full().relative().children(window_views)
    }
}

fn get_window_models(
    layout: Model<Layout>,
    cx: &mut ViewContext<LayoutView>,
) -> Vec<Model<Window>> {
    let windows = layout.read(cx).windows.clone();

    windows
        .into_iter()
        .map(|w| {
            let window_model = cx.new_model(|_cx| w);

            cx.observe(&window_model, {
                let window_model = window_model.clone();
                move |layout, window, cx| {
                    let window_id = window.read(cx).id;
                    if let Some((_id, window)) =
                        layout.window_views.iter_mut().find(|w| w.0 == window_id)
                    {
                        *window = get_window_view(window_model.clone(), cx);
                        cx.notify();
                    } else {
                        // FIXME: Debug assertion
                    }
                }
            })
            .detach();

            window_model
        })
        .collect::<Vec<_>>()
}

fn get_window_views(
    window_models: Vec<Model<Window>>,
    layout: Model<Layout>,
    cx: &mut WindowContext,
) -> Vec<(usize, AnyView)> {
    let windows = layout.read(cx).windows.clone();
    windows
        .into_iter()
        .zip(window_models)
        .map(|(window, window_model)| {
            let window_id = window.id;
            (window_id, get_window_view(window_model, cx))
        })
        .collect()
}

fn get_window_view(window: Model<Window>, cx: &mut WindowContext) -> AnyView {
    match window.read(cx).kind {
        WindowKind::Pool(pool_window) => match pool_window.kind {
            PoolWindowKind::Group => {
                let delegate = GroupPoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
            PoolWindowKind::Sequence => {
                let delegate = SequencePoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
            PoolWindowKind::BeamPreset => {
                let delegate = BeamPoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
            PoolWindowKind::ColorPreset => {
                let delegate = ColorPoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
            PoolWindowKind::DimmerPreset => {
                let delegate = DimmerPoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
            PoolWindowKind::FocusPreset => {
                let delegate = FocusPoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
            PoolWindowKind::GoboPreset => {
                let delegate = GoboPoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
            PoolWindowKind::PositionPreset => {
                let delegate = PositionPoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
            PoolWindowKind::AllPreset => {
                let delegate = AllPoolWindowDelegate::new(window.clone());
                WindowView::build(window, delegate, cx).into()
            }
        },
        WindowKind::Executors => {
            let delegate = ExecutorsWindowDelegate::new(cx, window.clone());
            WindowView::build(window, delegate, cx).into()
        }
        WindowKind::FixtureSheet => {
            let delegate = FixtureSheetWindowDelegate::new(cx);
            WindowView::build(window, delegate, cx).into()
        }
        WindowKind::AttributeEditor => {
            let delegate = AttributeEditorWindowDelegate::new(cx);
            WindowView::build(window, delegate, cx).into()
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

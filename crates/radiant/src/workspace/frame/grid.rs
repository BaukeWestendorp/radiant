use gpui::*;
use show::{FrameKind, PoolKind, Show, WindowInstance};
use ui::{theme::ActiveTheme, z_stack};

use super::{
    EffectGraphEditorFrameDelegate, EffectGraphPoolFrameDelegate, EffectPoolFrameDelegate,
    FrameView, GroupPoolFrameDelegate, PoolFrameDelegate,
};

pub const GRID_SIZE: f32 = 80.0;

pub struct FrameGridView {
    width: u32,
    height: u32,

    frames: Vec<AnyView>,
}

impl FrameGridView {
    pub fn build(
        show: Model<Show>,
        window_instance: WindowInstance,
        cx: &mut WindowContext,
    ) -> View<FrameGridView> {
        cx.new_view(|cx| {
            let window = show.read(cx).layout.window(window_instance);
            let frames = window
                .read(cx)
                .frames
                .clone()
                .into_iter()
                .map(|frame| frame_to_view(frame, window_instance, show.clone(), cx))
                .collect();

            FrameGridView {
                width: 16,
                height: 10,

                frames,
            }
        })
    }
}

impl Render for FrameGridView {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        let background = canvas(|_, _| {}, {
            let width = self.width;
            let height = self.height;
            move |_, _, cx| {
                for x in 0..(width + 1) {
                    for y in 0..(height + 1) {
                        cx.paint_quad(fill(
                            Bounds::centered_at(
                                point(px(x as f32 * GRID_SIZE), px(y as f32 * GRID_SIZE)),
                                size(px(2.0), px(2.0)),
                            ),
                            cx.theme().accent.opacity(0.5),
                        ));
                    }
                }
            }
        })
        .size_full()
        .into_any_element();

        let frame_elements = self
            .frames
            .clone()
            .into_iter()
            .map(|frame| frame.into_any_element());

        z_stack([background].into_iter().chain(frame_elements)).size_full()
    }
}

pub fn frame_to_view(
    frame: show::Frame,
    window_instance: WindowInstance,
    show: Model<Show>,
    cx: &mut WindowContext,
) -> AnyView {
    let window = show.read(cx).layout.window(window_instance).clone();
    let assets = show.read(cx).assets.clone();

    match frame.kind {
        FrameKind::EffectGraphEditor => {
            let selected_graph_id = window.read(cx).selected_effect_graph.clone();
            let id = selected_graph_id.read(cx).unwrap();
            let graph_model =
                cx.new_model(|cx| assets.effect_graphs.read(cx).get(&id).unwrap().clone());

            cx.observe(&selected_graph_id, {
                let graph_model = graph_model.clone();
                move |selected_graph_id, cx| {
                    let id = selected_graph_id.read(cx).unwrap();
                    log::debug!("Window's selected graph id changed to {id}");
                    let effect_graph = assets.effect_graphs.read(cx).get(&id).unwrap().clone();
                    graph_model.update(cx, |graph, cx| {
                        *graph = effect_graph;
                        cx.notify();
                    });
                }
            })
            .detach();

            FrameView::build(
                frame.clone(),
                EffectGraphEditorFrameDelegate::new(graph_model, cx),
                cx,
            )
            .into()
        }
        FrameKind::Pool(kind) => match kind {
            PoolKind::EffectGraph => FrameView::build(
                frame.clone(),
                PoolFrameDelegate::new(
                    frame.width,
                    frame.height,
                    EffectGraphPoolFrameDelegate::new(window.clone(), assets.effect_graphs.clone()),
                ),
                cx,
            )
            .into(),
            PoolKind::Effect => FrameView::build(
                frame.clone(),
                PoolFrameDelegate::new(
                    frame.width,
                    frame.height,
                    EffectPoolFrameDelegate::new(assets.effects.clone()),
                ),
                cx,
            )
            .into(),
            PoolKind::Group => FrameView::build(
                frame.clone(),
                PoolFrameDelegate::new(
                    frame.width,
                    frame.height,
                    GroupPoolFrameDelegate::new(assets.groups.clone()),
                ),
                cx,
            )
            .into(),
        },
    }
}

use gpui::*;
use show::{FrameKind, PoolKind, Show, WindowEvent, WindowInstance};
use ui::{theme::ActiveTheme, z_stack};

use super::{
    CueEditorFrameDelegate, CuePoolFrameDelegate, EffectGraphEditorFrameDelegate,
    EffectGraphPoolFrameDelegate, FrameView, GroupPoolFrameDelegate, PoolFrameDelegate,
};

pub const GRID_SIZE: f32 = 80.0;

pub struct FrameGridView {
    size: Size<u32>,
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
                size: size(16, 10),
                frames,
            }
        })
    }
}

impl Render for FrameGridView {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        let background = canvas(|_, _| {}, {
            let width = self.size.width;
            let height = self.size.height;
            move |_, _, cx| {
                for x in 0..(width + 1) {
                    for y in 0..(height + 1) {
                        cx.paint_quad(fill(
                            Bounds::centered_at(
                                point(px(x as f32 * GRID_SIZE), px(y as f32 * GRID_SIZE)),
                                size(px(2.0), px(2.0)),
                            ),
                            cx.theme().border,
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

    match &frame.kind {
        FrameKind::EffectGraphEditor { settings } => {
            let graph_model = cx.new_model(|cx| {
                window
                    .read(cx)
                    .selected_effect_graph(&assets.effect_graphs, cx)
                    .unwrap()
                    .clone()
            });

            cx.observe(&assets.effect_graphs, {
                let graph_model = graph_model.clone();
                move |pool, cx| {
                    graph_model.update(cx, |graph, cx| {
                        log::debug!("Updating effect graph model with new graph {}", graph.id);
                        *graph = pool.read(cx).get(&graph.id).unwrap().clone();
                        cx.notify();
                    });
                }
            })
            .detach();

            cx.subscribe(&window, {
                let graph_model = graph_model.clone();
                move |_, event, cx| match event {
                    WindowEvent::SelectedEffectGraphChanged(id) => {
                        log::debug!("Window's selected graph id changed to {id:?}");
                        if let Some(id) = id {
                            let effect_graph =
                                assets.effect_graphs.read(cx).get(id).unwrap().clone();
                            graph_model.update(cx, |graph, cx| {
                                *graph = effect_graph;
                                cx.notify();
                            });
                        }
                    }
                    _ => {}
                }
            })
            .detach();

            FrameView::build(
                frame.clone(),
                EffectGraphEditorFrameDelegate::new(
                    show.clone(),
                    graph_model,
                    settings.clone(),
                    cx,
                ),
                cx,
            )
            .into()
        }
        FrameKind::CueEditor => {
            let cue_model = cx.new_model(|cx| {
                window
                    .read(cx)
                    .selected_cue(&assets.cues, cx)
                    .unwrap()
                    .clone()
            });

            cx.observe(&assets.cues, {
                let cue_model = cue_model.clone();
                move |pool, cx| {
                    cue_model.update(cx, |cue, cx| {
                        log::debug!("Updating effect cue model with new cue {}", cue.id);
                        *cue = pool.read(cx).get(&cue.id).unwrap().clone();
                        cx.notify();
                    });
                }
            })
            .detach();

            cx.subscribe(&window, {
                let cue_model = cue_model.clone();
                move |_, event, cx| match event {
                    WindowEvent::SelectedCueChanged(id) => {
                        log::debug!("Window's selected cue id changed to {id:?}");
                        if let Some(id) = id {
                            let new_cue = assets.cues.read(cx).get(id).unwrap().clone();
                            cue_model.update(cx, |cue, cx| {
                                *cue = new_cue;
                                cx.notify();
                            });
                        }
                    }
                    _ => {}
                }
            })
            .detach();

            FrameView::build(
                frame.clone(),
                CueEditorFrameDelegate::new(show.clone(), cue_model.clone(), cx),
                cx,
            )
            .into()
        }
        FrameKind::Pool(kind) => match kind {
            PoolKind::EffectGraph => FrameView::build(
                frame.clone(),
                PoolFrameDelegate::new(
                    frame.bounds.size,
                    EffectGraphPoolFrameDelegate::new(window.clone(), assets.effect_graphs.clone()),
                ),
                cx,
            )
            .into(),
            PoolKind::Cue => FrameView::build(
                frame.clone(),
                PoolFrameDelegate::new(
                    frame.bounds.size,
                    CuePoolFrameDelegate::new(window.clone(), assets.cues.clone()),
                ),
                cx,
            )
            .into(),
            PoolKind::Group => FrameView::build(
                frame.clone(),
                PoolFrameDelegate::new(
                    frame.bounds.size,
                    GroupPoolFrameDelegate::new(assets.groups.clone()),
                ),
                cx,
            )
            .into(),
        },
    }
}

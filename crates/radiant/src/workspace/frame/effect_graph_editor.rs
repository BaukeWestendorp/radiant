use flow::{
    gpui::{GraphEditorView, GraphEvent},
    Point,
};
use gpui::*;
use prelude::FluentBuilder;
use show::{Asset, EffectGraph, EffectGraphDefinition, Show};
use ui::{container, theme::ActiveTheme, ContainerKind, StyledExt};

use super::{FrameDelegate, FrameView};

pub struct EffectGraphEditorFrameDelegate {
    show: Model<Show>,
    graph: Model<EffectGraph>,
    editor: View<GraphEditorView<EffectGraphDefinition>>,
    auto_save: Model<bool>,
    graph_offset: Model<Point>,
}

impl EffectGraphEditorFrameDelegate {
    pub fn new(
        show: Model<Show>,
        graph: Model<EffectGraph>,
        auto_save: Model<bool>,
        graph_offset: Model<Point>,
        cx: &mut WindowContext,
    ) -> Self {
        let graph_offset = cx.new_model(|cx| graph.read(cx).offset);
        let editor = cx.new_view(|cx| {
            // FIXME: We could create a helper for these 'model mappings'.
            let flow_graph = cx.new_model(|cx| graph.read(cx).graph.clone());
            cx.observe(&graph, {
                let flow_graph = flow_graph.clone();
                let graph_offset = graph_offset.clone();
                move |editor, graph, cx| {
                    log::debug!("Updating effect graph editor with new graph");
                    flow_graph.update(cx, |flow_graph, cx| {
                        *flow_graph = graph.read(cx).graph.clone();
                        cx.notify();
                    });
                    graph_offset.update(cx, |graph_offset, cx| {
                        *graph_offset = graph.read(cx).offset;
                        cx.notify();
                    });

                    *editor = GraphEditorView::new(flow_graph.clone(), graph_offset.clone(), cx);
                    cx.notify();
                }
            })
            .detach();

            GraphEditorView::new(flow_graph.clone(), graph_offset.clone(), cx)
        });

        Self {
            show,
            graph,
            auto_save,
            editor,
            graph_offset,
        }
    }

    fn save_graph(&self, cx: &mut WindowContext) {
        let new_graph = self.editor.read(cx).graph(cx).read(cx).clone();
        let offset = *self.graph_offset.read(cx);

        let effect_graph_pool = self.show.read(cx).assets.effect_graphs.clone();
        effect_graph_pool.update(cx, |pool, cx| {
            let id = self.graph.read(cx).id();
            if let Some(graph) = pool.get_mut(&id) {
                graph.graph = new_graph;
                graph.offset = offset;
                log::info!("Saved effect graph '{}' ({}).", graph.label, graph.id);
            }
        });
    }

    fn toggle_autosave(&self, cx: &mut WindowContext) {
        self.auto_save.update(cx, |auto_save, cx| {
            *auto_save = !*auto_save;
            cx.notify();
            log::info!(
                "Auto save is now {} for this Effect Graph Editor.",
                if *auto_save { "enabled" } else { "disabled" }
            );
        });
    }
}

impl FrameDelegate for EffectGraphEditorFrameDelegate {
    fn init(&mut self, cx: &mut ViewContext<FrameView<Self>>) {
        let flow_graph = self.editor.read(cx).graph(cx).clone();
        cx.subscribe(&flow_graph, {
            let auto_save = self.auto_save.clone();
            move |this, _graph, event, cx| match event {
                GraphEvent::ShouldSave => {
                    if *auto_save.read(cx) {
                        this.delegate.save_graph(cx);
                        log::info!("Auto-saved effect graph.");
                    }
                }
                _ => {}
            }
        })
        .detach();

        cx.observe(&self.graph_offset, {
            let auto_save = self.auto_save.clone();
            move |this, _, cx| {
                if *auto_save.read(cx) {
                    this.delegate.save_graph(cx);
                    log::info!("Auto-saved effect graph.");
                }
            }
        })
        .detach();
    }

    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String {
        format!(
            "Effect Graph Editor ({} [{}])",
            self.graph.read(cx).label,
            self.graph.read(cx).id,
        )
    }

    fn render_header_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        let auto_save_enabled = *self.auto_save.read(cx);

        div().h_flex().gap_2().children([
            div()
                .id("save-button")
                .on_click(cx.listener(|this, _, cx| this.delegate.save_graph(cx)))
                .border_1()
                .border_color(white())
                .p_1()
                .child("Save"),
            div()
                .id("autosave-button")
                .on_click(cx.listener(|this, _, cx| this.delegate.toggle_autosave(cx)))
                .border_1()
                .border_color(red())
                .when(auto_save_enabled, |e| e.border_color(green()))
                .p_1()
                .child("Auto Save"),
        ])
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        container(ContainerKind::Element, cx)
            .size_full()
            .border_color(cx.theme().frame_header_border)
            .child(self.editor.clone())
    }
}

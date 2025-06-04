use super::Frame;
use crate::show::{self, Show, effect_graph};
use crate::ui::FRAME_CELL_SIZE;
use gpui::{App, Entity, ReadGlobal, SharedString, Window, div, prelude::*};
use graph_editor::GraphEditorFrame;
use ui::{ContainerStyle, container, h6};

pub mod graph_editor;

pub struct WindowFrame {
    frame: Entity<Frame>,
    pub kind: WindowFrameKind,
}

impl WindowFrame {
    pub fn new(kind: WindowFrameKind, frame: Entity<Frame>) -> Self {
        Self { kind, frame }
    }

    fn title(&self, cx: &App) -> SharedString {
        self.kind.into_show(cx).to_string().into()
    }

    fn render_header(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = div()
            .size_full()
            .p_2()
            .flex()
            .items_center()
            .child(h6(self.title(cx)))
            .into_any_element();

        super::header_container(self.frame.clone(), content, window, cx)
            .w_full()
            .h(FRAME_CELL_SIZE / 2.0)
    }

    fn render_content(&mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let content = match &self.kind {
            WindowFrameKind::EffectGraphEditor(frame) => frame.clone(),
        };

        container(ContainerStyle::normal(window, cx)).size_full().child(content)
    }
}

impl Render for WindowFrame {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.render_header(window, cx))
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .size_full()
                    .p_px()
                    .child(self.render_content(window, cx)),
            )
            .overflow_hidden()
    }
}

pub enum WindowFrameKind {
    EffectGraphEditor(Entity<GraphEditorFrame<effect_graph::Def>>),
}

impl WindowFrameKind {
    pub fn into_show(&self, cx: &App) -> show::WindowFrameKind {
        match self {
            Self::EffectGraphEditor(graph_editor_frame) => {
                let asset = &graph_editor_frame.read(cx).asset;
                show::WindowFrameKind::EffectGraphEditor(asset.as_ref().map(|a| a.read(cx).id))
            }
        }
    }

    pub fn from_show(from: &show::WindowFrameKind, window: &mut Window, cx: &mut App) -> Self {
        match from {
            show::WindowFrameKind::EffectGraphEditor(asset_id) => {
                let editor_frame = cx.new(|cx| {
                    let asset = asset_id.as_ref().map(|asset_id| {
                        Show::global(cx).assets.effect_graphs.get(asset_id).unwrap()
                    });
                    GraphEditorFrame::new(asset.cloned(), window, cx)
                });

                WindowFrameKind::EffectGraphEditor(editor_frame)
            }
        }
    }
}

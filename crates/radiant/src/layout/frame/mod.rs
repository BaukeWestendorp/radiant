use crate::{
    show::{self, Show},
    ui::FRAME_CELL_SIZE,
};
use gpui::{App, Bounds, Entity, FocusHandle, Focusable, ReadGlobal, Window, div, prelude::*};
use pool::{PoolFrame, PoolFrameKind, PresetPoolFrameKind};
use window::{WindowFrame, WindowFrameKind, graph_editor::GraphEditorFrame};

mod pool;
mod window;

pub enum FrameKind {
    Window(Entity<WindowFrame>),
    Pool(Entity<PoolFrame>),
}

pub struct Frame {
    pub bounds: Bounds<u32>,
    pub kind: FrameKind,
    focus_handle: FocusHandle,
}

impl Render for Frame {
    fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .w(FRAME_CELL_SIZE * self.bounds.size.width as f32)
            .h(FRAME_CELL_SIZE * self.bounds.size.height as f32)
            .left(FRAME_CELL_SIZE * self.bounds.origin.x as f32)
            .top(FRAME_CELL_SIZE * self.bounds.origin.y as f32)
            .child(match &self.kind {
                FrameKind::Window(window_frame) => window_frame.clone().into_any_element(),
                FrameKind::Pool(pool_frame) => pool_frame.clone().into_any_element(),
            })
    }
}

impl Frame {
    pub fn into_show(&self, cx: &App) -> show::Frame {
        let kind = match &self.kind {
            FrameKind::Window(window_frame) => {
                let window_frame = window_frame.read(cx);
                show::FrameKind::Window(match &window_frame.kind {
                    WindowFrameKind::EffectGraphEditor(graph_editor_frame) => {
                        let asset = &graph_editor_frame.read(cx).asset;
                        show::WindowFrameKind::EffectGraphEditor(asset.read(cx).id)
                    }
                })
            }
            FrameKind::Pool(pool_frame) => show::FrameKind::Pool(match &pool_frame.read(cx).kind {
                PoolFrameKind::EffectGraphs => show::PoolFrameKind::EffectGraphs,
                PoolFrameKind::FixtureGroups => show::PoolFrameKind::FixtureGroups,
                PoolFrameKind::Cues => show::PoolFrameKind::Cues,
                PoolFrameKind::Sequences => show::PoolFrameKind::Sequences,
                PoolFrameKind::Executors => show::PoolFrameKind::Executors,
                PoolFrameKind::Preset(kind) => match kind {
                    PresetPoolFrameKind::Dimmer => show::PoolFrameKind::DimmerPresets,
                    PresetPoolFrameKind::Position => show::PoolFrameKind::PositionPresets,
                    PresetPoolFrameKind::Gobo => show::PoolFrameKind::GoboPresets,
                    PresetPoolFrameKind::Color => show::PoolFrameKind::ColorPresets,
                    PresetPoolFrameKind::Beam => show::PoolFrameKind::BeamPresets,
                    PresetPoolFrameKind::Focus => show::PoolFrameKind::FocusPresets,
                    PresetPoolFrameKind::Control => show::PoolFrameKind::ControlPresets,
                    PresetPoolFrameKind::Shapers => show::PoolFrameKind::ShapersPresets,
                    PresetPoolFrameKind::Video => show::PoolFrameKind::VideoPresets,
                },
            }),
        };

        show::Frame { bounds: self.bounds, kind }
    }

    pub fn from_show(from: &show::Frame, w: &mut Window, cx: &mut Context<Self>) -> Self {
        let frame = cx.entity();
        let kind = match &from.kind {
            show::FrameKind::Window(kind) => match kind {
                show::WindowFrameKind::EffectGraphEditor(asset_id) => {
                    let editor_frame = cx.new(|cx| {
                        let asset = Show::global(cx).assets.effect_graphs.get(asset_id).unwrap();
                        GraphEditorFrame::new(asset.clone(), w, cx)
                    });

                    let window_frame = cx.new(|_| {
                        WindowFrame::new(WindowFrameKind::EffectGraphEditor(editor_frame), frame)
                    });

                    FrameKind::Window(window_frame)
                }
            },
            show::FrameKind::Pool(kind) => {
                let pool_frame_kind = match kind {
                    show::PoolFrameKind::EffectGraphs => PoolFrameKind::EffectGraphs,
                    show::PoolFrameKind::FixtureGroups => PoolFrameKind::FixtureGroups,
                    show::PoolFrameKind::Cues => PoolFrameKind::Cues,
                    show::PoolFrameKind::Sequences => PoolFrameKind::Sequences,
                    show::PoolFrameKind::Executors => PoolFrameKind::Executors,
                    show::PoolFrameKind::DimmerPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Dimmer)
                    }
                    show::PoolFrameKind::PositionPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Position)
                    }
                    show::PoolFrameKind::GoboPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Gobo)
                    }
                    show::PoolFrameKind::ColorPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Color)
                    }
                    show::PoolFrameKind::BeamPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Beam)
                    }
                    show::PoolFrameKind::FocusPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Focus)
                    }
                    show::PoolFrameKind::ControlPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Control)
                    }
                    show::PoolFrameKind::ShapersPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Shapers)
                    }
                    show::PoolFrameKind::VideoPresets => {
                        PoolFrameKind::Preset(PresetPoolFrameKind::Video)
                    }
                };

                let pool_frame = cx.new(|_| PoolFrame::new(pool_frame_kind, frame));
                FrameKind::Pool(pool_frame)
            }
        };

        Self { kind, bounds: from.bounds, focus_handle: cx.focus_handle() }
    }
}

impl Focusable for Frame {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

use gpui::{AppContext, Context, EventEmitter, Model, ModelContext};

use crate::{showfile, AssetPool, EffectGraph, EffectGraphId};

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub main_window: Model<Window>,
    pub secondary_window: Model<Window>,
}

impl Layout {
    pub fn from_showfile(layout: showfile::Layout, cx: &mut AppContext) -> Self {
        Self {
            main_window: cx.new_model(|_| Window::from_showfile(layout.main_window)),
            secondary_window: cx.new_model(|_| Window::from_showfile(layout.secondary_window)),
        }
    }

    pub fn window(&self, instace: WindowInstance) -> &Model<Window> {
        match instace {
            WindowInstance::Main => &self.main_window,
            WindowInstance::Secondary => &self.secondary_window,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowInstance {
    Main,
    Secondary,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    selected_effect_graph: Option<EffectGraphId>,
    pub frames: Vec<Frame>,
}

impl Window {
    pub fn from_showfile(window: showfile::Window) -> Self {
        Self {
            selected_effect_graph: window.selected_effect_graph.map(EffectGraphId),
            frames: window.frames.into_iter().map(Frame::from).collect(),
        }
    }

    pub fn selected_effect_graph<'a>(
        &self,
        pool: &Model<AssetPool<EffectGraph>>,
        cx: &'a AppContext,
    ) -> Option<&'a EffectGraph> {
        self.selected_effect_graph
            .and_then(|id| pool.read(cx).get(&id))
    }

    pub fn set_selected_effect_graph(
        &mut self,
        id: Option<EffectGraphId>,
        cx: &mut ModelContext<Self>,
    ) {
        self.selected_effect_graph = id;
        cx.emit(WindowEvent::SelectedEffectGraphChanged(id));
        cx.notify();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEvent {
    SelectedEffectGraphChanged(Option<EffectGraphId>),
}

impl EventEmitter<WindowEvent> for Window {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub kind: FrameKind,
}

impl From<showfile::Frame> for Frame {
    fn from(frame: showfile::Frame) -> Self {
        Self {
            x: frame.x,
            y: frame.y,
            width: frame.width,
            height: frame.height,
            kind: frame.kind.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameKind {
    EffectGraphEditor,
    Pool(PoolKind),
}

impl From<showfile::FrameKind> for FrameKind {
    fn from(kind: showfile::FrameKind) -> Self {
        match kind {
            showfile::FrameKind::EffectGraphEditor => Self::EffectGraphEditor,
            showfile::FrameKind::Pool(kind) => Self::Pool(kind.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolKind {
    EffectGraph,
    Effect,
    Group,
}

impl From<showfile::PoolKind> for PoolKind {
    fn from(kind: showfile::PoolKind) -> Self {
        match kind {
            showfile::PoolKind::EffectGraph => Self::EffectGraph,
            showfile::PoolKind::Effect => Self::Effect,
            showfile::PoolKind::Group => Self::Group,
        }
    }
}

use gpui::{point, size, AppContext, Bounds, Context, EventEmitter, Model, ModelContext};

use crate::{showfile, AssetPool, CueList, CueListId, EffectGraph, EffectGraphId};

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub main_window: Model<Window>,
    pub secondary_window: Model<Window>,
}

impl Layout {
    pub fn window(&self, instace: WindowInstance) -> &Model<Window> {
        match instace {
            WindowInstance::Main => &self.main_window,
            WindowInstance::Secondary => &self.secondary_window,
        }
    }
}

impl Layout {
    pub(crate) fn from_showfile(layout: showfile::Layout, cx: &mut AppContext) -> Self {
        Self {
            main_window: cx.new_model(|cx| Window::from_showfile(layout.main_window, cx)),
            secondary_window: cx.new_model(|cx| Window::from_showfile(layout.secondary_window, cx)),
        }
    }

    pub(crate) fn to_showfile(&self, cx: &mut AppContext) -> showfile::Layout {
        showfile::Layout {
            main_window: self.main_window.read(cx).clone().to_showfile(cx),
            secondary_window: self.secondary_window.read(cx).clone().to_showfile(cx),
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
    selected_cuelist: Option<CueListId>,
    pub frames: Vec<Frame>,
}

impl Window {
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

    pub fn selected_cuelist<'a>(
        &self,
        pool: &Model<AssetPool<CueList>>,
        cx: &'a AppContext,
    ) -> Option<&'a CueList> {
        self.selected_cuelist.and_then(|id| pool.read(cx).get(&id))
    }

    pub fn set_selected_cuelist(&mut self, id: Option<CueListId>, cx: &mut ModelContext<Self>) {
        self.selected_cuelist = id;
        cx.emit(WindowEvent::SelectedCueListChanged(id));
        cx.notify();
    }
}

impl Window {
    pub(crate) fn from_showfile(window: showfile::Window, cx: &mut AppContext) -> Self {
        Self {
            selected_effect_graph: window.selected_effect_graph.map(EffectGraphId),
            selected_cuelist: window.selected_effect_graph.map(CueListId),
            frames: window
                .frames
                .into_iter()
                .map(|frame| Frame::from_showfile(frame, cx))
                .collect(),
        }
    }

    pub(crate) fn to_showfile(&self, cx: &mut AppContext) -> showfile::Window {
        showfile::Window {
            selected_effect_graph: self.selected_effect_graph.map(|id| id.0),
            selected_cue: self.selected_cuelist.map(|id| id.0),
            frames: self
                .frames
                .iter()
                .map(|frame| frame.to_showfile(cx))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEvent {
    SelectedEffectGraphChanged(Option<EffectGraphId>),
    SelectedCueListChanged(Option<CueListId>),
}

impl EventEmitter<WindowEvent> for Window {}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub bounds: Bounds<u32>,
    pub kind: FrameKind,
}

impl Frame {
    pub(crate) fn from_showfile(frame: showfile::Frame, cx: &mut AppContext) -> Self {
        Self {
            bounds: Bounds {
                origin: point(frame.x, frame.y),
                size: size(frame.width, frame.height),
            },
            kind: FrameKind::from_showfile(frame.kind, cx),
        }
    }

    pub(crate) fn to_showfile(&self, cx: &mut AppContext) -> showfile::Frame {
        showfile::Frame {
            x: self.bounds.origin.x,
            y: self.bounds.origin.y,
            width: self.bounds.size.width,
            height: self.bounds.size.height,
            kind: self.kind.to_showfile(cx),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameKind {
    EffectGraphEditor {
        settings: Model<EffectGraphEditorSettings>,
    },
    CuelistEditor,
    Pool(PoolKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectGraphEditorSettings {
    pub auto_save: bool,
}

impl EffectGraphEditorSettings {
    pub(crate) fn from_showfile(settings: showfile::EffectGraphEditorSettings) -> Self {
        Self {
            auto_save: settings.auto_save,
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::EffectGraphEditorSettings {
        showfile::EffectGraphEditorSettings {
            auto_save: self.auto_save,
        }
    }
}

impl FrameKind {
    pub(crate) fn from_showfile(kind: showfile::FrameKind, cx: &mut AppContext) -> Self {
        match kind {
            showfile::FrameKind::EffectGraphEditor { settings } => Self::EffectGraphEditor {
                settings: cx.new_model(|_| EffectGraphEditorSettings::from_showfile(settings)),
            },
            showfile::FrameKind::CuelistEditor => Self::CuelistEditor,
            showfile::FrameKind::Pool(kind) => Self::Pool(PoolKind::from_showfile(kind)),
        }
    }

    pub(crate) fn to_showfile(&self, cx: &mut AppContext) -> showfile::FrameKind {
        match self {
            Self::EffectGraphEditor { settings } => showfile::FrameKind::EffectGraphEditor {
                settings: settings.read(cx).to_showfile(),
            },
            Self::CuelistEditor => showfile::FrameKind::CuelistEditor,
            Self::Pool(kind) => showfile::FrameKind::Pool(kind.to_showfile()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolKind {
    EffectGraph,
    Cue,
    Group,
}

impl PoolKind {
    pub(crate) fn from_showfile(kind: showfile::PoolKind) -> Self {
        match kind {
            showfile::PoolKind::EffectGraph => Self::EffectGraph,
            showfile::PoolKind::Cue => Self::Cue,
            showfile::PoolKind::Group => Self::Group,
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::PoolKind {
        match self {
            Self::EffectGraph => showfile::PoolKind::EffectGraph,
            Self::Cue => showfile::PoolKind::Cue,
            Self::Group => showfile::PoolKind::Group,
        }
    }
}

use crate::{Graph, GraphDef, InputSocket, NodeId, OutputSocket};
use gpui::*;

pub mod editor;
pub(crate) mod graph;
pub(crate) mod node;

pub mod actions {
    use gpui::App;

    pub fn init(cx: &mut App) {
        super::editor::actions::init(cx);
    }
}

#[derive(Debug, Clone)]
pub enum GraphEvent {
    NodeAdded(NodeId),
    NodeRemoved(NodeId),
    EdgeAdded { target: InputSocket, source: OutputSocket },
    EdgeRemoved { target: InputSocket },
}

impl<D: GraphDef + 'static> EventEmitter<GraphEvent> for Graph<D> {}

pub struct ControlView {
    pub view: AnyView,
}

impl ControlView {
    pub fn new(
        cx: &mut App,
        build_view: impl FnOnce(&mut Context<Self>) -> AnyView,
    ) -> Entity<Self> {
        cx.new(|cx| Self { view: build_view(cx) })
    }
}

impl<D: GraphDef + 'static> EventEmitter<ControlEvent<D>> for ControlView {}

pub enum ControlEvent<D: GraphDef> {
    Change(D::Value),
}

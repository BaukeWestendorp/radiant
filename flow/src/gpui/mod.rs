use crate::{GraphDef, InputSocket, NodeId, OutputSocket};
use gpui::*;

pub mod editor;
mod graph;
mod node;

pub fn init(cx: &mut App) {
    node::init(cx);
    editor::init(cx);
}

#[derive(Debug, Clone)]
pub enum GraphEvent {
    NodeAdded(NodeId),
    NodeRemoved(NodeId),
    EdgeAdded { target: InputSocket, source: OutputSocket },
    EdgeRemoved { target: InputSocket },
}

impl<D: GraphDef + 'static> EventEmitter<GraphEvent> for crate::Graph<D> {}

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

use std::collections::HashMap;

use super::{Frontend, GraphEvent};
use crate::{Graph, GraphDef, NodeId};
use gpui::*;

pub use editor::GraphEditorView;

mod editor;
mod graph;
mod node;

pub type NodePosition = (f32, f32);

#[derive(Default)]
pub struct VisualState {
    pub node_positions: HashMap<NodeId, NodePosition>,
}

impl<'cx, E> super::Frontend for Context<'cx, E>
where
    E: EventEmitter<GraphEvent>,
{
    type VisualState = VisualState;

    fn emit_event(&mut self, event: GraphEvent) {
        self.emit(event);
    }
}

impl<D: GraphDef + 'static> EventEmitter<GraphEvent> for Graph<D> {}

use std::collections::HashMap;

use flow::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}

impl From<NodePosition> for gpui::Point<f32> {
    fn from(pos: NodePosition) -> Self {
        gpui::Point::new(pos.x, pos.y)
    }
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GpuiGraphState {
    pub node_positions: HashMap<NodeId, NodePosition>,
}

impl GpuiGraphState {
    pub fn node_position(&self, node_id: &NodeId) -> Option<&NodePosition> {
        self.node_positions.get(node_id)
    }

    pub fn set_node_position(&mut self, node_id: NodeId, position: NodePosition) {
        self.node_positions.insert(node_id, position);
    }
}

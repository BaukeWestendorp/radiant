use std::collections::HashMap;

use flow::NodeId;

#[derive(Default)]
pub struct GpuiGraphState {
    pub node_positions: HashMap<NodeId, (f32, f32)>,
}

impl GpuiGraphState {
    pub fn node_position(&self, node_id: &NodeId) -> Option<&(f32, f32)> {
        self.node_positions.get(node_id)
    }
}

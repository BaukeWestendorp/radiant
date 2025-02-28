use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::graph::Value;
use crate::socket::SocketId;
use crate::template::TemplateId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeId(pub u32);

impl NodeId {
    const COUNT: AtomicU32 = AtomicU32::new(0);

    pub fn next() -> Self {
        let id = Self::COUNT.fetch_add(1, Ordering::Relaxed);
        Self(id)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node {
    id: NodeId,
    template: TemplateId,
    pub input_constants: HashMap<SocketId, Value>,
    pub output_constants: HashMap<SocketId, Value>,
    x: f32,
    y: f32,
}

impl Node {
    pub fn new(template: TemplateId, x: f32, y: f32) -> Self {
        Node {
            id: NodeId::next(),
            template,
            input_constants: HashMap::new(),
            output_constants: HashMap::new(),
            x,
            y,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn template(&self) -> &TemplateId {
        &self.template
    }
}

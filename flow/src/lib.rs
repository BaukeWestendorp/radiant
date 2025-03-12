#[cfg(feature = "serde")]
pub mod serde;

pub mod export_prelude {
    pub use crate::{
        DataType, Edge, GraphDef, Input, Node, NodeId, Output, ProcessingContext, Socket,
        SocketValues, Template,
    };
}

pub(crate) mod graph;
pub(crate) mod node;
pub(crate) mod socket;
pub(crate) mod template;

pub use graph::*;
pub use node::*;
pub use socket::*;
pub use template::*;

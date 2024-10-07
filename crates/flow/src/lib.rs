pub mod error;
pub mod gpui;
pub mod graph;
pub mod node;
pub mod traits;

pub use error::*;
pub use graph::*;
pub use node::*;
pub use traits::*;

slotmap::new_key_type! {
    pub struct NodeId;
    pub struct InputId;
    pub struct OutputId;
}

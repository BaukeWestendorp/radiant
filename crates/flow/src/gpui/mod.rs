use crate::GraphDefinition;

use gpui::*;

pub mod editor;
pub mod graph;
pub mod node;

pub use editor::*;
pub use graph::*;
pub use node::*;

#[cfg(all(feature = "macros", feature = "gpui"))]
pub use flow_macros::NodeCategory;

pub fn init(cx: &mut AppContext) {
    editor::init(cx);
    node::init(cx);
}

pub trait NodeCategory: ToString + Copy + PartialEq {
    #[allow(opaque_hidden_inferred_bound)]
    fn all() -> impl Iterator<Item = Self>;
}

pub trait Control<Def: GraphDefinition> {
    fn view<View: EventEmitter<ControlEvent<Def>>>(
        &self,
        id: impl Into<ElementId>,
        initial_value: Def::Value,
        cx: &mut ViewContext<View>,
    ) -> AnyView;
}

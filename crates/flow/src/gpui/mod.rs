use crate::GraphDefinition;

use gpui::*;

pub mod editor;
pub mod graph;
pub mod node;

pub use editor::*;
pub use graph::*;
pub use node::*;

pub fn init(cx: &mut AppContext) {
    editor::init(cx);
    node::init(cx);
}

pub trait VisualNodeKind {
    type Category: NodeCategory;

    fn label(&self) -> &str;

    fn category(&self) -> Self::Category;

    #[allow(opaque_hidden_inferred_bound)]
    fn all() -> impl Iterator<Item = Self>;
}

pub trait NodeCategory: ToString + Copy + PartialEq {
    #[allow(opaque_hidden_inferred_bound)]
    fn all() -> impl Iterator<Item = Self>;
}

pub trait VisualDataType {
    fn color(&self) -> Hsla;
}

pub trait VisualNodeData: Default {
    fn position(&self) -> &geo::Point;

    fn set_position(&mut self, position: geo::Point);
}

pub trait VisualControl<Def: GraphDefinition + 'static> {
    fn view<View: EventEmitter<ControlEvent<Def>>>(
        &self,
        id: impl Into<ElementId>,
        initial_value: Def::Value,
        cx: &mut ViewContext<View>,
    ) -> AnyView;
}

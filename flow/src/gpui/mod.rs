use crate::GraphDefinition;

pub mod editor;
pub mod graph;
pub mod node;

pub use editor::*;
pub use graph::*;
pub use node::*;

use gpui::{AnyView, App, ElementId, EventEmitter, Hsla, Context};

pub fn init(cx: &mut App) {
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
    fn position(&self) -> &crate::Point;

    fn set_position(&mut self, position: crate::Point);

    fn snapped_position(&self, snap_grid_size: f32) -> crate::Point {
        crate::Point::new(
            (self.position().x / snap_grid_size).floor() * snap_grid_size,
            (self.position().y / snap_grid_size).floor() * snap_grid_size,
        )
    }
}

pub trait VisualControl<Def: GraphDefinition + 'static> {
    fn view<View: EventEmitter<ControlEvent<Def>>>(
        &self,
        id: impl Into<ElementId>,
        initial_value: Def::Value,
        cx: &mut Context<View>,
    ) -> AnyView;
}

use crate::node::ControlEvent;
use flow::graph_def::GraphDefinition;
use gpui::*;

pub mod editor;
pub mod graph;
pub mod node;

pub fn init(cx: &mut AppContext) {
    editor::init(cx);
    node::init(cx);
}

pub trait VisualNodeKind {
    fn label(&self) -> &str;
}

pub trait VisualDataType {
    fn color(&self) -> Hsla;
}

pub trait VisualNodeData: Default {
    fn position(&self) -> &Point<Pixels>;

    fn set_position(&mut self, position: Point<Pixels>);
}

pub trait VisualControl<Def: GraphDefinition + 'static> {
    fn view<View: EventEmitter<ControlEvent<Def>>>(
        &self,
        id: impl Into<ElementId>,
        initial_value: Def::Value,
        cx: &mut ViewContext<View>,
    ) -> AnyView;
}

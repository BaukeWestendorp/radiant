use gpui::*;

use crate::Graph;

pub struct GraphView<State: Default, Value: Clone> {
    pub graph: Entity<Graph<State, Value>>,
}

impl<State: Default + 'static, Value: Clone + 'static> GraphView<State, Value> {
    pub fn build(graph: Entity<Graph<State, Value>>, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self { graph })
    }

    pub fn graph(&self) -> &Entity<Graph<State, Value>> {
        &self.graph
    }
}

impl<State: Default + 'static, Value: Clone + 'static> Render for GraphView<State, Value> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child("Graph")
    }
}

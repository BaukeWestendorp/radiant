use crate::effect_graph::{
    EffectGraph, EffectGraphNodeData, EffectGraphNodeKind, EffectGraphValue,
};
use crate::fixture::{AttributeValue, Fixture, FixtureId};
use flow::OutputParameterKind;
use gpui::{point, px};

pub mod effect_graph;
pub mod fixture;

pub struct Show {
    fixtures: Vec<Fixture>,

    effect_graph: EffectGraph,
}

impl Show {
    pub fn fixtures(&self) -> impl Iterator<Item = &Fixture> {
        self.fixtures.iter()
    }

    pub fn fixture(&self, id: &FixtureId) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id() == id)
    }

    pub fn effect_graph(&self) -> &EffectGraph {
        &self.effect_graph
    }

    pub fn effect_graph_mut(&mut self) -> &mut EffectGraph {
        &mut self.effect_graph
    }
}

impl Default for Show {
    fn default() -> Self {
        Self {
            fixtures: vec![Fixture::new(FixtureId(0))],
            effect_graph: create_example_graph(),
        }
    }
}

fn create_example_graph() -> EffectGraph {
    let mut graph = EffectGraph::new();

    let attribute_value_node_id = graph.add_node(
        EffectGraphNodeKind::AttributeValue,
        EffectGraphNodeData {
            position: point(px(50.0), px(250.0)),
        },
    );

    let set_channel_value_node_id = graph.add_node(
        EffectGraphNodeKind::SetFixtureChannelValue,
        EffectGraphNodeData {
            position: point(px(350.0), px(150.0)),
        },
    );

    if let OutputParameterKind::Constant { value, .. } = &mut graph
        .output_mut(graph.node(attribute_value_node_id).output("value").id)
        .kind
    {
        *value = EffectGraphValue::AttributeValue(AttributeValue::new(0.5))
    }

    graph.add_edge(
        graph.node(attribute_value_node_id).output("value").id,
        graph.node(set_channel_value_node_id).input("value").id,
    );

    graph
}

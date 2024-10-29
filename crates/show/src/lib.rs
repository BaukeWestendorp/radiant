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
}

impl Default for Show {
    fn default() -> Self {
        Self {
            fixtures: {
                let mut fixtures = Vec::default();
                fixtures.push(Fixture::new(FixtureId(0)));
                fixtures
            },
            effect_graph: create_example_graph(),
        }
    }
}

fn create_example_graph() -> EffectGraph {
    let mut graph = EffectGraph::new();

    let fixture_id_node_id = graph.add_node(
        EffectGraphNodeKind::FixtureId,
        EffectGraphNodeData {
            position: point(px(50.0), px(50.0)),
        },
    );
    let attribute_value_node_id = graph.add_node(
        EffectGraphNodeKind::AttributeValue,
        EffectGraphNodeData {
            position: point(px(50.0), px(250.0)),
        },
    );
    let set_attribute_node_id = graph.add_node(
        EffectGraphNodeKind::SetFixtureAttribute,
        EffectGraphNodeData {
            position: point(px(350.0), px(150.0)),
        },
    );

    if let OutputParameterKind::Constant { value, .. } = &mut graph
        .output_mut(graph.node(fixture_id_node_id).output("id").id)
        .kind
    {
        *value = EffectGraphValue::FixtureId(FixtureId(42))
    }

    if let OutputParameterKind::Constant { value, .. } = &mut graph
        .output_mut(graph.node(attribute_value_node_id).output("value").id)
        .kind
    {
        *value = EffectGraphValue::AttributeValue(AttributeValue::new(0.5))
    }

    graph.add_edge(
        graph.node(fixture_id_node_id).output("id").id,
        graph.node(set_attribute_node_id).input("id").id,
    );

    graph.add_edge(
        graph.node(attribute_value_node_id).output("value").id,
        graph.node(set_attribute_node_id).input("ColorAdd_R").id,
    );

    graph.add_edge(
        graph.node(attribute_value_node_id).output("value").id,
        graph.node(set_attribute_node_id).input("ColorAdd_G").id,
    );

    graph
}

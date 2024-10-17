use graph::node::OutputValue;
use graph::{Graph, NodeKind, Value};
use slotmap::SlotMap;

slotmap::new_key_type! {
    pub struct FixtureId;
}

pub struct Fixture {
    id: FixtureId,
}

impl Fixture {
    pub fn new(id: FixtureId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> FixtureId {
        self.id
    }
}

pub struct Show {
    fixtures: SlotMap<FixtureId, Fixture>,

    effect_graph: Graph,
}

impl Show {
    pub fn fixtures(&self) -> impl Iterator<Item = (FixtureId, &Fixture)> {
        self.fixtures.iter()
    }

    pub fn fixture(&self, id: FixtureId) -> Option<&Fixture> {
        self.fixtures.get(id)
    }

    pub fn effect_graph(&self) -> &Graph {
        &self.effect_graph
    }
}

impl Default for Show {
    fn default() -> Self {
        Self {
            fixtures: {
                let mut fixtures = SlotMap::default();
                fixtures.insert_with_key(|id| Fixture::new(id));
                fixtures
            },
            effect_graph: create_example_graph(),
        }
    }
}

fn create_example_graph() -> Graph {
    let mut graph = Graph::new();

    let a_node_id = graph.add_node(NodeKind::NewInt, 50.0, 50.0);
    let b_node_id = graph.add_node(NodeKind::NewFloat, 50.0, 150.0);
    let new_string_node_id = graph.add_node(NodeKind::NewString, 50.0, 250.0);
    let add_node_id = graph.add_node(NodeKind::IntAdd, 300.0, 100.0);
    let output_node_id = graph.add_node(NodeKind::Output, 550.0, 100.0);

    if let OutputValue::Constant { value, .. } = &mut graph
        .output_mut(graph.node(a_node_id).output("value").unwrap())
        .value
    {
        *value = Value::Int(42);
    }

    if let OutputValue::Constant { value, .. } = &mut graph
        .output_mut(graph.node(b_node_id).output("value").unwrap())
        .value
    {
        *value = Value::Float(0.33);
    }

    if let OutputValue::Constant { value, .. } = &mut graph
        .output_mut(graph.node(new_string_node_id).output("value").unwrap())
        .value
    {
        *value = Value::String("Hello, world!".into());
    }

    graph.add_connection(
        graph.input(graph.node(add_node_id).input("a").unwrap()).id,
        graph
            .output(graph.node(a_node_id).output("value").unwrap())
            .id,
    );

    graph.add_connection(
        graph.input(graph.node(add_node_id).input("b").unwrap()).id,
        graph
            .output(graph.node(b_node_id).output("value").unwrap())
            .id,
    );

    graph.add_connection(
        graph
            .input(graph.node(output_node_id).input("value").unwrap())
            .id,
        graph
            .output(graph.node(add_node_id).output("sum").unwrap())
            .id,
    );
    graph
}

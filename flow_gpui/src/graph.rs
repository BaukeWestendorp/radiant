use std::collections::HashMap;

use flow::{
    Edge, GraphDef, Input, Node, NodeId, Output, ProcessingContext, Socket, Template, TemplateId,
};
use gpui::{Hsla, Pixels, Point};

use crate::GraphEvent;

#[derive(Clone)]
pub struct Graph<D: GraphDef> {
    pub(crate) graph: flow::Graph<D>,
    pub(crate) node_positions: HashMap<NodeId, Point<Pixels>>,
    pub(crate) dragged_node_position: Option<(NodeId, Point<Pixels>)>,
    pub(crate) offset: Point<Pixels>,
}

impl<D: GraphDef + 'static> Default for Graph<D>
where
    D::DataType: crate::DataType<D>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D: GraphDef + 'static> Graph<D> {
    pub fn new() -> Self {
        Self {
            graph: flow::Graph::new(),
            node_positions: HashMap::new(),
            dragged_node_position: None,
            offset: Point::default(),
        }
    }

    pub fn add_template(&mut self, template: Template<D>) {
        self.graph.add_template(template);
    }

    pub fn add_templates(&mut self, templates: impl IntoIterator<Item = Template<D>>) {
        self.graph.add_templates(templates);
    }

    pub fn template(&self, template_id: &TemplateId) -> &Template<D> {
        self.graph.template(template_id)
    }

    pub fn templates(&self) -> impl Iterator<Item = &Template<D>> {
        self.graph.templates()
    }

    pub fn add_node(
        &mut self,
        node: Node<D>,
        position: Point<Pixels>,
        cx: &mut gpui::Context<Self>,
    ) -> NodeId {
        let node_id = self.graph.add_node(node);
        self.node_positions.insert(node_id, position);
        cx.emit(GraphEvent::NodeAdded(node_id));
        node_id
    }

    pub fn remove_node(&mut self, node_id: &NodeId, cx: &mut gpui::Context<Self>) {
        self.graph.remove_node(node_id);
        self.node_positions.remove(node_id);
        cx.emit(GraphEvent::NodeRemoved(*node_id));
    }

    pub fn node(&self, node_id: &NodeId) -> &Node<D> {
        self.graph.node(node_id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &Node<D>)> {
        self.graph.nodes()
    }

    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.graph.node_ids()
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.graph.add_edge(edge, true)
    }

    pub fn remove_edge(&mut self, source: &Socket) {
        self.graph.remove_edge(source)
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.graph.edges()
    }

    pub fn input(&self, socket: &Socket) -> &Input<D> {
        let template_id = self.node(&socket.node_id).template_id();
        self.template(template_id)
            .inputs()
            .iter()
            .find(|i| i.id() == socket.id)
            .expect("should have found input")
    }

    pub fn output(&self, socket: &Socket) -> &Output<D> {
        let template_id = self.node(&socket.node_id).template_id();
        self.template(template_id)
            .outputs()
            .iter()
            .find(|o| o.id() == socket.id)
            .expect("should have found output")
    }

    pub fn process(&mut self, pcx: &mut ProcessingContext<D>) {
        self.graph.process(pcx)
    }
}

impl<D: GraphDef> Graph<D> {
    pub fn node_position(&self, node_id: &NodeId) -> &Point<Pixels> {
        self.node_positions.get(node_id).expect("should have a position for every NodeId")
    }

    pub fn set_node_position(&mut self, node_id: NodeId, position: Point<Pixels>) {
        self.node_positions.insert(node_id, position);
    }

    pub fn visual_node_position(&self, node_id: &NodeId) -> &Point<Pixels> {
        match &self.dragged_node_position {
            Some((dragged_node_id, position)) if dragged_node_id == node_id => position,
            _ => self.node_position(node_id),
        }
    }

    pub fn update_visual_node_position(&mut self, position: Option<(NodeId, Point<Pixels>)>) {
        self.dragged_node_position = position;
    }

    pub fn offset(&self) -> &Point<Pixels> {
        &self.offset
    }

    pub fn set_offset(&mut self, offset: Point<Pixels>) {
        self.offset = offset;
    }
}

pub trait DataType<D: GraphDef>: flow::DataType<D> {
    fn color(&self) -> Hsla;
}

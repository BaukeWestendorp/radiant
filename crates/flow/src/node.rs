use crate::{
    FlowError, Graph, GraphDefinition, InputId, NodeId, NodeKind, OutputId, ProcessingResult,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Node<Def: GraphDefinition> {
    pub id: NodeId,
    pub data: Def::NodeData,
    pub(crate) kind: Def::NodeKind,
    pub(crate) inputs: Vec<NodeInputParameter>,
    pub(crate) outputs: Vec<NodeOutputParameter>,
}

impl<Def: GraphDefinition> Node<Def> {
    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn kind(&self) -> &Def::NodeKind {
        &self.kind
    }

    pub fn input(&self, label: &str) -> &NodeInputParameter {
        self.inputs
            .iter()
            .find(|i| i.label == label)
            .unwrap_or_else(|| panic!("Tried to get input parameter with nonexistent label: '{label}' not found on node"))
    }

    pub fn inputs(&self) -> &[NodeInputParameter] {
        &self.inputs
    }

    pub fn input_ids(&self) -> impl Iterator<Item = InputId> + '_ {
        self.inputs.iter().map(|i| i.id)
    }

    pub fn output(&self, label: &str) -> &NodeOutputParameter {
        self.outputs
            .iter()
            .find(|o| o.label == label)
            .unwrap_or_else(|| panic!("Tgried to get output parameter with nonexistent label: '{label}' not found on node"))
    }

    pub fn outputs(&self) -> &[NodeOutputParameter] {
        &self.outputs
    }

    pub fn output_ids(&self) -> impl Iterator<Item = OutputId> + '_ {
        self.outputs.iter().map(|i| i.id)
    }

    pub fn process(
        &self,
        context: &mut <Def::NodeKind as NodeKind<Def>>::ProcessingContext,
        graph: &Graph<Def>,
        #[cfg(feature = "gpui")] cx: &mut gpui::AppContext,
    ) -> Result<ProcessingResult<Def>, FlowError> {
        self.kind.process(self.id, context, graph, cx)
    }
}

impl<Def: GraphDefinition> Node<Def> {
    pub fn new(id: NodeId, kind: Def::NodeKind, data: Def::NodeData) -> Self {
        Self {
            id,
            data,
            kind,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInputParameter {
    pub label: String,
    pub id: InputId,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeOutputParameter {
    pub label: String,
    pub id: OutputId,
}

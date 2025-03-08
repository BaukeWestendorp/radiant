use crate::{GraphDef, SocketValues, TemplateId};

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Clone)]
pub struct Node<D: GraphDef> {
    template_id: TemplateId,
    #[serde(default = "SocketValues::new")]
    input_values: SocketValues<D>,
}

impl<D: GraphDef> Node<D> {
    pub fn new(template_id: impl Into<TemplateId>) -> Self {
        Self { template_id: template_id.into(), input_values: SocketValues::new() }
    }

    pub fn template_id(&self) -> &TemplateId {
        &self.template_id
    }

    pub fn input_values(&self) -> &SocketValues<D> {
        &self.input_values
    }

    pub fn input_values_mut(&mut self) -> &mut SocketValues<D> {
        &mut self.input_values
    }
}

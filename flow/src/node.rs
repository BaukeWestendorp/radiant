use crate::{GraphDef, Template, TemplateId, Values};

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Clone)]
pub struct Node<D: GraphDef> {
    template_id: TemplateId,
    #[cfg_attr(feature = "serde", serde(default = "Values::new"))]
    input_values: Values<D>,
    #[cfg_attr(feature = "serde", serde(default = "Values::new"))]
    control_values: Values<D>,
}

impl<D: GraphDef> Node<D> {
    pub fn new(template: &Template<D>) -> Self {
        Self {
            template_id: template.id().clone(),
            input_values: template.default_input_values(),
            control_values: template.default_control_values(),
        }
    }

    pub fn template_id(&self) -> &TemplateId {
        &self.template_id
    }

    pub fn input_values(&self) -> &Values<D> {
        &self.input_values
    }

    pub fn input_values_mut(&mut self) -> &mut Values<D> {
        &mut self.input_values
    }

    pub fn control_values(&self) -> &Values<D> {
        &self.control_values
    }

    pub fn control_values_mut(&mut self) -> &mut Values<D> {
        &mut self.control_values
    }
}

impl<D: GraphDef> From<&Template<D>> for Node<D> {
    fn from(value: &Template<D>) -> Self {
        Self::new(value)
    }
}

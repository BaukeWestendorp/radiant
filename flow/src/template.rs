use crate::{GraphDef, Input, Output, ProcessingContext, SocketValues};

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TemplateId(pub String);

impl From<String> for TemplateId {
    fn from(id: String) -> Self {
        TemplateId(id)
    }
}

impl From<&str> for TemplateId {
    fn from(id: &str) -> Self {
        TemplateId(id.to_string())
    }
}

pub type Processor<D> = fn(&SocketValues<D>, &mut SocketValues<D>, &mut ProcessingContext<D>);

#[derive(Clone)]
pub struct Template<D: GraphDef> {
    id: TemplateId,

    label: String,

    inputs: Vec<Input<D>>,
    outputs: Vec<Output<D>>,

    processor: Box<Processor<D>>,
}

impl<D: GraphDef> Template<D> {
    pub fn new(
        id: impl Into<TemplateId>,
        label: impl Into<String>,
        inputs: Vec<Input<D>>,
        outputs: Vec<Output<D>>,
        processor: Box<Processor<D>>,
    ) -> Self {
        Self { id: id.into(), label: label.into(), inputs, outputs, processor }
    }

    pub fn id(&self) -> &TemplateId {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn input(&self, id: &str) -> &Input<D> {
        self.inputs
            .iter()
            .find(|i| i.id() == id)
            .expect("should get input from template for given id")
    }

    pub fn inputs(&self) -> &[Input<D>] {
        &self.inputs
    }

    pub fn output(&self, id: &str) -> &Output<D> {
        self.outputs
            .iter()
            .find(|i| i.id() == id)
            .expect("should get input from template for given id")
    }

    pub fn outputs(&self) -> &[Output<D>] {
        &self.outputs
    }

    pub fn default_input_values(&self) -> SocketValues<D> {
        let mut values = SocketValues::new();
        for input in &self.inputs {
            values.set_value(input.id(), input.default().clone());
        }
        values
    }

    pub fn process(
        &self,
        input_values: &SocketValues<D>,
        output_values: &mut SocketValues<D>,
        state: &mut ProcessingContext<D>,
    ) {
        (self.processor)(input_values, output_values, state)
    }
}

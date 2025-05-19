use crate::{GraphDef, Input, Output, ProcessingContext, Value as _, Values};

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

pub type Processor<D> =
    fn(&Values<D>, &Values<D>, &mut Values<D>, &mut ProcessingContext<D>, &mut gpui::App);

#[derive(Clone)]
pub struct Template<D: GraphDef> {
    id: TemplateId,

    label: String,

    inputs: Vec<Input<D>>,
    outputs: Vec<Output<D>>,
    controls: Vec<NodeControl<D>>,

    processor: Box<Processor<D>>,
}

impl<D: GraphDef> Template<D> {
    pub fn new(
        id: impl Into<TemplateId>,
        label: impl Into<String>,
        processor: Processor<D>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            controls: Vec::new(),
            processor: Box::new(processor),
        }
    }

    pub fn add_input(mut self, input: Input<D>) -> Self {
        self.inputs.push(input);
        self
    }

    pub fn add_output(mut self, output: Output<D>) -> Self {
        self.outputs.push(output);
        self
    }

    pub fn add_control(mut self, control: NodeControl<D>) -> Self {
        self.controls.push(control);
        self
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

    pub fn control(&self, id: &str) -> &NodeControl<D> {
        self.controls
            .iter()
            .find(|c| c.id() == id)
            .expect("should get control from template for given id")
    }

    pub fn controls(&self) -> &[NodeControl<D>] {
        &self.controls
    }

    pub fn default_input_values(&self) -> Values<D> {
        let mut values = Values::new();
        for input in &self.inputs {
            values.set_value(input.id(), input.default().clone());
        }
        values
    }

    pub fn default_control_values(&self) -> Values<D> {
        let mut values = Values::new();
        for control in &self.controls {
            values.set_value(control.id(), control.default().clone());
        }
        values
    }

    pub fn process(
        &self,
        input_values: &Values<D>,
        control_values: &Values<D>,
        output_values: &mut Values<D>,
        state: &mut ProcessingContext<D>,
        cx: &mut gpui::App,
    ) {
        (self.processor)(input_values, control_values, output_values, state, cx)
    }
}

#[derive(Debug, Clone)]
pub struct NodeControl<D: GraphDef> {
    id: String,
    label: String,
    default: D::Value,
    control: D::Control,
}

impl<D: GraphDef> NodeControl<D> {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        default: D::Value,
        control: D::Control,
    ) -> Self {
        Self { id: id.into(), label: label.into(), default, control }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn default(&self) -> &D::Value {
        &self.default
    }

    pub fn data_type(&self) -> D::DataType {
        self.default().data_type()
    }

    pub fn control(&self) -> &D::Control {
        &self.control
    }
}

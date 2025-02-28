use std::collections::HashMap;

use crate::{
    graph::NodeContext,
    socket::{Input, Output, SocketId},
};

pub type TemplateId = String;

pub type Processor = dyn Fn(&mut NodeContext) -> ();

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeTemplate {
    label: String,
    inputs: HashMap<SocketId, Input>,
    outputs: HashMap<SocketId, Output>,
    #[cfg_attr(feature = "serde", serde(skip))]
    processor: Option<Box<Processor>>,
}

impl std::fmt::Debug for NodeTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeTemplate")
            .field("label", &self.label)
            .field("inputs", &self.inputs)
            .field("outputs", &self.outputs)
            .field(
                "processor",
                &self.processor.is_some().then(|| "registered").unwrap_or("unregistered"),
            )
            .finish()
    }
}

impl NodeTemplate {
    pub fn new(
        label: String,
        inputs: HashMap<SocketId, Input>,
        outputs: HashMap<SocketId, Output>,
        processor: Option<Box<Processor>>,
    ) -> Self {
        NodeTemplate { label, inputs, outputs, processor }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn input(&self, id: &SocketId) -> &Input {
        self.inputs.get(id).expect("should always find the input")
    }

    pub fn inputs(&self) -> &HashMap<SocketId, Input> {
        &self.inputs
    }

    pub fn output(&self, id: &SocketId) -> &Output {
        self.outputs.get(id).expect("should always find the output")
    }

    pub fn outputs(&self) -> &HashMap<SocketId, Output> {
        &self.outputs
    }

    pub fn process(&self, cx: &mut NodeContext) {
        let processor = self
            .processor
            .as_ref()
            .expect("every node template should have a processor. has it been registered?");
        processor(cx);
    }

    pub fn has_processor(&self) -> bool {
        self.processor.is_some()
    }

    pub(crate) fn set_processor(&mut self, processor: Option<Box<Processor>>) {
        self.processor = processor;
    }
}

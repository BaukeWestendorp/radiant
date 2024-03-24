use dmx::{channel, DmxOutput};

#[derive(Debug, Clone, PartialEq)]
pub struct PlaybackEngine {}

impl PlaybackEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn determine_dmx_output(&self) -> DmxOutput {
        let mut output = DmxOutput::new();
        output.set_channel(&channel!(0, 0).unwrap(), 127).unwrap();
        output
    }
}

use crate::DmxChannel;

#[derive(Debug, Clone, Default)]
pub struct DmxUniverse(Vec<u8>);

impl DmxUniverse {
    pub fn new() -> Self {
        Self(vec![0; 512])
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn set_channel_value(&mut self, channel: DmxChannel, value: u8) {
        self.0[channel.value() as usize] = value;
    }

    pub fn get_channel_value(&self, channel: DmxChannel) -> u8 {
        self.0[channel.value() as usize]
    }
}

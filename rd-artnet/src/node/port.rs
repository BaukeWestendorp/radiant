use crate::PortProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDirection {
    Input,
    Output,
    Bidirectional,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub direction: PortDirection,
    pub protocol: PortProtocol,
    pub input_universe: Option<u8>,
    pub output_universe: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(transparent))]
#[repr(transparent)]
pub struct PortId(u16);

impl PortId {
    pub fn new(id: u16) -> Self {
        Self(id)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

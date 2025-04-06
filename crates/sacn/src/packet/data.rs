use crate::{ComponentIdentifier, Error, source::SourceConfig};

use super::{
    ACN_PACKET_IDENTIFIER, POSTAMBLE_SIZE, PREAMBLE_SIZE, VECTOR_DATA_PACKET,
    VECTOR_DMP_SET_PROPERTY,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPacket {
    root: RootLayer,
    framing: FramingLayer,
    dmp: DmpLayer,
}

impl DataPacket {
    pub fn new(
        config: &SourceConfig,
        cid: ComponentIdentifier,
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        let seq_number = 0;
        let stream_terminated = false;

        Ok(DataPacket {
            root: RootLayer::new(cid),
            framing: FramingLayer::new(
                &config.name,
                config.priority,
                config.sync_addr,
                seq_number,
                config.preview_data,
                stream_terminated,
                config.force_synchronization,
                config.universe.0,
            )?,
            dmp: DmpLayer::new(data)?,
        })
    }
}

impl super::Pdu for DataPacket {
    fn to_bytes(&self) -> Vec<u8> {
        let pdu_len = self.len();

        vec![
            self.root.to_bytes(pdu_len),
            self.framing.to_bytes(pdu_len),
            self.dmp.to_bytes(pdu_len),
        ]
        .concat()
    }

    fn len(&self) -> u16 {
        126 + self.dmp.data.len() as u16
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RootLayer {
    cid: ComponentIdentifier,
}

impl RootLayer {
    pub fn new(cid: ComponentIdentifier) -> Self {
        RootLayer { cid }
    }
}

impl RootLayer {
    fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(54);
        bytes.extend(PREAMBLE_SIZE.to_be_bytes());
        bytes.extend(POSTAMBLE_SIZE.to_be_bytes());
        bytes.extend(ACN_PACKET_IDENTIFIER);
        bytes.extend(flags_and_length(pdu_len).to_be_bytes());
        bytes.extend(VECTOR_DATA_PACKET.to_be_bytes());
        bytes.extend(self.cid.as_bytes());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FramingLayer {
    /// User Assigned Name of Source.
    source_name: [u8; 64],
    /// Data priority if multiple sources.
    priority: u8,
    /// Universe address on which sync packets will be sent.
    sync_address: u16,
    /// Sequence number.
    sequence_number: u8,

    /// Options
    options: u8,

    /// Universe number.
    universe: u16,
}

impl FramingLayer {
    pub fn new(
        source_name: &str,
        priority: u8,
        sync_address: u16,
        sequence_number: u8,
        preview_data: bool,
        stream_terminated: bool,
        force_synchronization: bool,
        universe: u16,
    ) -> Result<Self, Error> {
        // 6.2.2 E1.31 Data Packet: Source Name.
        if source_name.len() > 64 {
            return Err(Error::InvalidSourceNameLength(source_name.len()));
        }

        let bytes = source_name.as_bytes();
        let mut source_name = [0u8; 64];
        let len = bytes.len().min(64);
        source_name[..len].copy_from_slice(&bytes[..len]);

        // 6.2.3 E1.31 Data Packet: Priority.
        if !(0..200).contains(&priority) {
            return Err(Error::InvalidPriority(priority));
        }

        let options = (preview_data as u8) << 7
            | (stream_terminated as u8) << 6
            | (force_synchronization as u8) << 5;

        Ok(FramingLayer { source_name, priority, sync_address, sequence_number, options, universe })
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(flags_and_length(pdu_len).to_be_bytes());
        bytes.extend(VECTOR_DATA_PACKET.to_be_bytes());
        bytes.extend(self.source_name);
        bytes.push(self.priority);
        bytes.extend(self.sync_address.to_be_bytes());
        bytes.push(self.sequence_number);
        bytes.push(self.options);
        bytes.extend(self.universe.to_be_bytes());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DmpLayer {
    prop_value_count: u16,
    data: Vec<u8>,
}

impl DmpLayer {
    pub fn new(data: Vec<u8>) -> Result<Self, Error> {
        Ok(DmpLayer { prop_value_count: (data.len() + 1) as u16, data })
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        const START_CODE: u8 = 0x00;

        let mut bytes = Vec::new();
        bytes.extend(flags_and_length(pdu_len).to_be_bytes());
        bytes.push(VECTOR_DMP_SET_PROPERTY);
        bytes.push(0xa1);
        bytes.extend(0x0000u16.to_be_bytes());
        bytes.extend(0x0001u16.to_be_bytes());
        bytes.extend(self.prop_value_count.to_be_bytes());
        bytes.push(START_CODE);
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

fn flags_and_length(pdu_len: u16) -> u16 {
    // Low 12 bits = PDU length, high 4 bits = 0x7.
    let flags = 0x7 << 12;
    let length = pdu_len & 0xFFF;
    flags | length
}

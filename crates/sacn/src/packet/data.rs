use super::{RootLayer, flags_and_length, source_name_from_str};
use crate::{ComponentIdentifier, Error, source::SourceConfig};

pub const VECTOR_DMP_SET_PROPERTY: u8 = 0x02;
pub const VECTOR_DATA_PACKET: u32 = 0x00000002;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPacket {
    root: RootLayer,
    framing: FramingLayer,
    dmp: DmpLayer,
}

impl DataPacket {
    pub fn new(
        cid: ComponentIdentifier,
        source_name: &str,
        priority: u8,
        synchronization_address: u16,
        sequence_number: u8,
        preview_data: bool,
        stream_terminated: bool,
        force_synchronization: bool,
        universe: u16,
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        Ok(DataPacket {
            root: RootLayer::new(cid, false),
            framing: FramingLayer::new(
                source_name,
                priority,
                synchronization_address,
                sequence_number,
                preview_data,
                stream_terminated,
                force_synchronization,
                universe,
            )?,
            dmp: DmpLayer::new(data)?,
        })
    }

    pub fn from_source_config(
        config: &SourceConfig,
        sequence_number: u8,
        stream_terminated: bool,
        universe: u16,
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        Self::new(
            config.cid,
            &config.name,
            config.priority,
            config.synchronization_address,
            sequence_number,
            config.preview_data,
            stream_terminated,
            config.force_synchronization,
            universe,
            data,
        )
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

    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        todo!()
    }

    fn len(&self) -> u16 {
        126 + self.dmp.data.len() as u16
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FramingLayer {
    /// User Assigned Name of Source.
    source_name: [u8; 64],
    /// Data priority if multiple sources.
    priority: u8,
    /// Universe address on which sync packets will be sent.
    synchronization_address: u16,
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
        synchronization_address: u16,
        sequence_number: u8,
        preview_data: bool,
        stream_terminated: bool,
        force_synchronization: bool,
        universe: u16,
    ) -> Result<Self, Error> {
        let source_name = source_name_from_str(source_name)?;

        // 6.2.3 E1.31 Data Packet: Priority.
        if !(0..200).contains(&priority) {
            return Err(Error::InvalidPriority(priority));
        }

        let options = (preview_data as u8) << 7
            | (stream_terminated as u8) << 6
            | (force_synchronization as u8) << 5;

        Ok(FramingLayer {
            source_name,
            priority,
            synchronization_address,
            sequence_number,
            options,
            universe,
        })
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(77);
        bytes.extend(flags_and_length(pdu_len - 38).to_be_bytes());
        bytes.extend(VECTOR_DATA_PACKET.to_be_bytes());
        bytes.extend(self.source_name);
        bytes.push(self.priority);
        bytes.extend(self.synchronization_address.to_be_bytes());
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

        let mut bytes = Vec::with_capacity(self.prop_value_count as usize + 10);
        bytes.extend(flags_and_length(pdu_len - 115).to_be_bytes());
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

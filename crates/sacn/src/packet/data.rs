use super::{RootLayer, flags_and_length, source_name_from_str};
use crate::{ComponentIdentifier, Error, source::SourceConfig};

const VECTOR_DMP_SET_PROPERTY: u8 = 0x02;
const VECTOR_DATA_PACKET: u32 = 0x00000002;
const ADDRESS_INCREMENT: u16 = 0x0001;
const FIRST_PROPERTY_ADDRESS: u16 = 0x0000;
const ADDRESS_TYPE_AND_DATA_TYPE: u8 = 0xa1;

const PREVIEW_DATA_BIT: u8 = 0x80;
const STREAM_TERMINATED_BIT: u8 = 0x40;
const FORCE_SYNCHRONIZATION_BIT: u8 = 0x20;

/// Represents an E1.31 Data Packet.
///
/// This packet carries a DMX512-A payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPacket {
    root: RootLayer,
    framing: FramingLayer,
    dmp: DmpLayer,
}

impl DataPacket {
    /// Creates a new [DataPacket].
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

    /// Creates a new [DataPacket] from a [SourceConfig].
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

    /// Returns the [ComponentIdentifier] in this packet.
    pub fn cid(&self) -> &ComponentIdentifier {
        &self.root.cid
    }

    /// Returns the Source Name in this packet.
    pub fn source_name(&self) -> &str {
        core::str::from_utf8(&self.framing.source_name).unwrap()
    }

    /// Returns the Priority in this packet.
    pub fn priority(&self) -> u8 {
        self.framing.priority
    }

    /// Returns the synchronization address in this packet.
    pub fn synchronization_address(&self) -> u16 {
        self.framing.synchronization_address
    }

    /// Returns the Preview Data flag in this packet.
    pub fn preview_data(&self) -> bool {
        self.framing.options & PREVIEW_DATA_BIT == PREVIEW_DATA_BIT
    }

    /// Returns the Stream Terminated flag in this packet.
    pub fn stream_terminated(&self) -> bool {
        self.framing.options & STREAM_TERMINATED_BIT == STREAM_TERMINATED_BIT
    }

    /// Returns the Force Synchronization flag in this packet.
    pub fn force_synchronization(&self) -> bool {
        self.framing.options & FORCE_SYNCHRONIZATION_BIT == FORCE_SYNCHRONIZATION_BIT
    }

    /// Returns the Universe Number in this packet.
    pub fn universe(&self) -> u16 {
        self.framing.universe
    }

    /// Returns the DMX Start Code in this packet.
    pub fn start_code(&self) -> Option<u8> {
        self.dmp.property_values.get(0).copied()
    }

    /// Returns the DMX Data in this packet.
    pub fn data(&self) -> &[u8] {
        &self.dmp.property_values[1..]
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
        Ok(Self {
            root: RootLayer::from_bytes(&bytes, false)?,
            framing: FramingLayer::from_bytes(&bytes)?,
            dmp: DmpLayer::from_bytes(&bytes)?,
        })
    }

    fn len(&self) -> u16 {
        125 + self.dmp.property_values.len() as u16
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

        let mut options = 0;
        if preview_data {
            options |= PREVIEW_DATA_BIT;
        }
        if stream_terminated {
            options |= STREAM_TERMINATED_BIT;
        }
        if force_synchronization {
            options |= FORCE_SYNCHRONIZATION_BIT;
        }

        Ok(FramingLayer {
            source_name,
            priority,
            synchronization_address,
            sequence_number,
            options,
            universe,
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let source_name = bytes[44..108].try_into().unwrap();
        let priority = bytes[108];
        let synchronization_address = u16::from_be_bytes([bytes[109], bytes[110]]);
        let sequence_number = bytes[111];
        let options = bytes[112];
        let universe = u16::from_be_bytes([bytes[113], bytes[114]]);

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
    property_values: Vec<u8>,
}

impl DmpLayer {
    const DEFAULT_START_CODE: u8 = 0x00;

    pub fn new(data: Vec<u8>) -> Result<Self, Error> {
        let mut property_values = vec![Self::DEFAULT_START_CODE];
        property_values.extend(data);
        Ok(DmpLayer { property_values })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        // E1.13 7.2 DMP Layer: Vector
        let vector = bytes[117];
        if vector != VECTOR_DMP_SET_PROPERTY {
            return Err(Error::InvalidDmpVector(vector));
        }

        // E1.13 7.3 Address Type and Data Type
        let address_type_and_data_type = bytes[118];
        if address_type_and_data_type != ADDRESS_TYPE_AND_DATA_TYPE {
            return Err(Error::InvalidDmpAddressType(address_type_and_data_type));
        }

        // E1.13 7.4 First Property Address
        let first_property_address = u16::from_be_bytes([bytes[119], bytes[120]]);
        if first_property_address != FIRST_PROPERTY_ADDRESS {
            return Err(Error::InvalidDmpFirstPropertyAddress(first_property_address));
        }

        // E1.13 7.5 Address Increment
        let address_increment = u16::from_be_bytes([bytes[121], bytes[122]]);
        if address_increment != ADDRESS_INCREMENT {
            return Err(Error::InvalidDmpAddressIncrement(address_increment));
        }

        let property_value_count = u16::from_be_bytes([bytes[123], bytes[124]]);

        let property_values = bytes[125..125 + property_value_count as usize].to_vec();

        Ok(DmpLayer { property_values })
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let property_value_count = self.property_values.len();

        let mut bytes = Vec::with_capacity(10 + property_value_count);
        bytes.extend(flags_and_length(pdu_len - 115).to_be_bytes());
        bytes.push(VECTOR_DMP_SET_PROPERTY);
        bytes.push(ADDRESS_TYPE_AND_DATA_TYPE);
        bytes.extend(FIRST_PROPERTY_ADDRESS.to_be_bytes());
        bytes.extend(ADDRESS_INCREMENT.to_be_bytes());
        bytes.extend((property_value_count as u16).to_be_bytes());
        bytes.extend_from_slice(&self.property_values);
        bytes
    }
}

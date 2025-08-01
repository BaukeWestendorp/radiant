use super::super::{MAX_UNIVERSE_SIZE, Slot, acn, source::SourceConfig};
use super::{PacketError, flags_and_length, source_name_from_str};
use arrayvec::ArrayVec;

const PREVIEW_DATA_BIT: u8 = 0x80;
const STREAM_TERMINATED_BIT: u8 = 0x40;
const FORCE_SYNCHRONIZATION_BIT: u8 = 0x20;

/// An E1.31 Data Packet Framing Layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFraming {
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

    // DMP
    dmp: Dmp,
}

impl DataFraming {
    const VECTOR: [u8; 4] = [0x00, 0x00, 0x00, 0x02];

    /// Creates a new [DataFraming] layer.
    pub fn new(
        source_name: &str,
        priority: u8,
        synchronization_address: u16,
        sequence_number: u8,
        preview_data: bool,
        stream_terminated: bool,
        force_synchronization: bool,
        universe: u16,
        dmp: Dmp,
    ) -> Result<Self, PacketError> {
        // E1.31 6.2.2 Data Packet: Source Name
        let source_name = source_name_from_str(source_name)?;

        // 6.2.3 E1.31 Data Packet: Priority.
        if !(0..200).contains(&priority) {
            return Err(PacketError::InvalidPriority(priority));
        }

        // E1.31 6.2.6 Data Packet: Options
        let mut options = 0;
        options |= (preview_data as u8) * PREVIEW_DATA_BIT;
        options |= (stream_terminated as u8) * STREAM_TERMINATED_BIT;
        options |= (force_synchronization as u8) * FORCE_SYNCHRONIZATION_BIT;

        Ok(DataFraming {
            source_name,
            priority,
            synchronization_address,
            sequence_number,
            options,
            universe,
            dmp,
        })
    }

    /// Creates a new [DataFraming] layer from a [SourceConfig].
    pub fn from_source_config(
        config: &SourceConfig,
        sequence_number: u8,
        stream_terminated: bool,
        universe: u16,
        dmp: Dmp,
    ) -> Result<Self, PacketError> {
        Self::new(
            &config.name,
            config.priority,
            config.synchronization_address,
            sequence_number,
            config.preview_data,
            stream_terminated,
            config.force_synchronization,
            universe,
            dmp,
        )
    }

    /// Returns the source name in this layer.
    pub fn source_name(&self) -> &str {
        core::str::from_utf8(&self.source_name).unwrap()
    }

    /// Returns the priority in this layer.
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Returns the synchronization address in this layer.
    pub fn synchronization_address(&self) -> u16 {
        self.synchronization_address
    }

    /// Returns the preview data flag in this layer.
    pub fn preview_data(&self) -> bool {
        self.options & PREVIEW_DATA_BIT == PREVIEW_DATA_BIT
    }

    /// Returns the stream terminated flag in this layer.
    pub fn stream_terminated(&self) -> bool {
        self.options & STREAM_TERMINATED_BIT == STREAM_TERMINATED_BIT
    }

    /// Returns the force synchronization flag in this layer.
    pub fn force_synchronization(&self) -> bool {
        self.options & FORCE_SYNCHRONIZATION_BIT == FORCE_SYNCHRONIZATION_BIT
    }

    /// Returns the universe number in this layer.
    pub fn universe(&self) -> u16 {
        self.universe
    }

    /// Returns the DMP PDU in this layer.
    pub fn dmp(&self) -> &Dmp {
        &self.dmp
    }
}

impl acn::Pdu for DataFraming {
    type DecodeError = PacketError;

    fn encode(&self) -> impl Into<Vec<u8>> {
        let flags_and_length = flags_and_length(self.size()).to_be_bytes();

        let mut bytes = Vec::with_capacity(77 + self.dmp.size());
        bytes.extend(flags_and_length);
        bytes.extend(Self::VECTOR);
        bytes.extend(self.source_name);
        bytes.push(self.priority);
        bytes.extend(self.synchronization_address.to_be_bytes());
        bytes.push(self.sequence_number);
        bytes.push(self.options);
        bytes.extend(self.universe.to_be_bytes());
        bytes.extend(self.dmp.encode().into());
        bytes
    }

    fn decode(bytes: &[u8]) -> Result<Self, Self::DecodeError> {
        if bytes.len() < 77 {
            return Err(PacketError::InvalidLength(bytes.len()));
        }

        // E1.31 6.2.1 Data Packet: Vector
        let vector = [bytes[2], bytes[3], bytes[4], bytes[5]];
        if vector != Self::VECTOR {
            return Err(PacketError::InvalidFramingLayerVector(vector.to_vec()));
        }

        // E1.31 6.2.2 Data Packet: Source Name
        let source_name = bytes[6..70].try_into().unwrap();

        // E1.31 6.2.3 Data Packet: Priority
        let priority = bytes[70];

        // E1.31 6.2.4 Data Packet: Synchronization Address
        let synchronization_address = u16::from_be_bytes([bytes[71], bytes[72]]);

        // E1.31 6.2.5 Data Packet: Sequence Number
        let sequence_number = bytes[73];

        // E1.31 6.2.6 Data Packet: Options
        let options = bytes[74];

        // E1.31 6.2.7 Data Packet: Universe
        let universe = u16::from_be_bytes([bytes[75], bytes[76]]);

        let dmp = Dmp::decode(&bytes[77..])?;

        Ok(DataFraming {
            source_name,
            priority,
            synchronization_address,
            sequence_number,
            options,
            universe,
            dmp,
        })
    }

    fn size(&self) -> usize {
        77 + self.dmp.size()
    }
}

/// The DMP (Device Management Protocol) Layer of an E1.31 Data Packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dmp {
    property_values: ArrayVec<Slot, { 1 + MAX_UNIVERSE_SIZE }>,
}

impl Dmp {
    const VECTOR: u8 = 0x02;
    const ADDRESS_INCREMENT: [u8; 2] = [0x00, 0x01];
    const FIRST_PROPERTY_ADDRESS: [u8; 2] = [0x00, 0x00];
    const ADDRESS_TYPE_AND_DATA_TYPE: u8 = 0xa1;

    /// Creates a new [Dmp] layer.
    pub fn new(property_values: ArrayVec<Slot, { 1 + MAX_UNIVERSE_SIZE }>) -> Self {
        Dmp { property_values }
    }

    /// Returns the start code slot.
    pub fn start_code_slot(&self) -> Slot {
        self.property_values[0]
    }

    /// Returns the data slots.
    pub fn data_slots(&self) -> &[Slot] {
        &self.property_values[1..]
    }

    /// Returns all slots.
    pub fn slots(&self) -> &[Slot] {
        &self.property_values
    }
}

impl acn::Pdu for Dmp {
    type DecodeError = PacketError;

    fn encode(&self) -> impl Into<Vec<u8>> {
        let flags_and_length = flags_and_length(self.size()).to_be_bytes();
        let property_value_count = self.property_values.len();

        let mut bytes = Vec::with_capacity(10 + property_value_count);
        bytes.extend(flags_and_length);
        bytes.push(Self::VECTOR);
        bytes.push(Self::ADDRESS_TYPE_AND_DATA_TYPE);
        bytes.extend(Self::FIRST_PROPERTY_ADDRESS);
        bytes.extend(Self::ADDRESS_INCREMENT);
        bytes.extend((property_value_count as u16).to_be_bytes());
        bytes.extend_from_slice(&self.property_values);
        bytes
    }

    fn decode(bytes: &[u8]) -> Result<Self, Self::DecodeError> {
        // E1.13 7.2 DMP Layer: Vector
        let vector = bytes[2];
        if vector != Self::VECTOR {
            return Err(PacketError::InvalidDmpLayerVector(vec![vector]));
        }

        // E1.13 7.3 Address Type and Data Type
        let address_type_and_data_type = bytes[3];
        if address_type_and_data_type != Self::ADDRESS_TYPE_AND_DATA_TYPE {
            return Err(PacketError::InvalidDmpAddressType(address_type_and_data_type));
        }

        // E1.13 7.4 First Property Address
        let first_property_address = [bytes[4], bytes[5]];
        if first_property_address != Self::FIRST_PROPERTY_ADDRESS {
            return Err(PacketError::InvalidDmpFirstPropertyAddress(u16::from_be_bytes(
                first_property_address,
            )));
        }

        // E1.13 7.5 Address Increment
        let address_increment = [bytes[6], bytes[7]];
        if address_increment != Self::ADDRESS_INCREMENT {
            return Err(PacketError::InvalidDmpAddressIncrement(u16::from_be_bytes(
                address_increment,
            )));
        }

        // E1.13 7.6 Property Value Count
        let property_value_count = u16::from_be_bytes([bytes[8], bytes[9]]);

        // E1.13 7.7 Property Values (DMX512-A Data)
        let mut property_values = ArrayVec::new();
        property_values.extend(bytes[10..10 + property_value_count as usize].to_vec());

        Ok(Dmp { property_values })
    }

    fn size(&self) -> usize {
        10 + self.property_values.len()
    }
}

use std::{io, net::Ipv4Addr};

use bytes::{BufMut, Bytes, BytesMut};

use crate::{FixedString, PortAddress};

pub const PACKET_ID: [u8; 8] = *b"Art-Net\0";
pub const PROTOCOL_VERSION: u16 = 14;

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
pub struct Packet {
    /// Byte representation of "Art-Net\0"
    id: [u8; 8],
    /// The OpCode defines the class of data following ArtPoll within this UDP packet.
    opcode: Opcode,

    /// The payload of this packet.
    pub(crate) payload: PacketPayload,
}

impl Packet {
    pub fn new(payload: PacketPayload) -> Self {
        Self { id: PACKET_ID, opcode: payload.opcode(), payload }
    }

    pub fn id(&self) -> &[u8; 8] {
        &self.id
    }

    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn payload(&self) -> &PacketPayload {
        &self.payload
    }

    pub fn encode(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(128);
        buf.put_slice(&self.id);
        buf.put_u16_le(self.opcode as u16);

        match &self.payload {
            PacketPayload::ArtPoll(p) => {
                buf.put_u16_le(p.prot_ver);
                buf.put_u8(p.flags);
                buf.put_u8(p.diag_priority);
                buf.put_slice(&p.target_port_address_top.get().to_be_bytes());
                buf.put_slice(&p.target_port_address_bottom.get().to_be_bytes());
                buf.put_u16_le(p.esta_man);
                buf.put_u16_le(p.oem);
            }
            PacketPayload::ArtPollReply(p) => {
                buf.put_slice(&p.ip_address.octets());
                buf.put_u16_le(p.port);
                buf.put_u16_le(p.vers_info);
                buf.put_u8(p.net_switch);
                buf.put_u8(p.sub_switch);
                buf.put_u16_le(p.oem);
                buf.put_u8(p.ubea_version);
                buf.put_u8(p.status1);
                buf.put_u16_le(p.esta_man);
                buf.put_slice(p.port_name.as_bytes());
                buf.put_slice(p.long_name.as_bytes());
                buf.put_slice(p.node_report.as_bytes());
                buf.put_u16_le(p.num_ports);
                buf.put_slice(&p.port_types);
                buf.put_slice(&p.good_input);
                buf.put_slice(&p.good_output_a);
                buf.put_slice(&p.sw_in);
                buf.put_slice(&p.sw_out);
                buf.put_u8(p.acn_priority);
                buf.put_u8(p.sw_macro);
                buf.put_u8(p.sw_remote);
                buf.put_slice(&p._spare);
                buf.put_u8(p.style);
                buf.put_slice(&p.mac);
                buf.put_slice(&p.bind_ip.octets());
                buf.put_u8(p.bind_index);
                buf.put_u8(p.status2);
                buf.put_slice(&p.good_output_b);
                buf.put_u8(p.status3);
                buf.put_slice(&p.default_resp_uid);
                buf.put_u16_le(p.user);
                buf.put_u16_le(p.refresh_rate);
                buf.put_u8(p.background_queue_policy);
                buf.put_slice(&p._filler);
            }
        }

        buf.freeze()
    }

    pub fn decode(bytes: &Bytes) -> crate::Result<Self> {
        let mut cursor = io::Cursor::new(bytes.as_ref());
        Self::from_reader(&mut cursor)
    }

    pub fn from_reader<R: io::Read>(mut reader: R) -> crate::Result<Self> {
        let mut header = [0u8; 10];
        if reader.read_exact(&mut header).is_err() {
            return Err(crate::Error::InvalidPacketLength(0));
        }

        let id: [u8; 8] = header[0..8].try_into().unwrap();
        if id != PACKET_ID {
            return Err(crate::Error::InvalidPacketId);
        }

        let opcode = Opcode::try_from(u16::from_le_bytes([header[8], header[9]])).unwrap();

        let payload = match opcode {
            Opcode::OpPoll => {
                let mut poll_data = [0u8; 12];
                let mut bytes_read = 0;

                while bytes_read < 12 {
                    match reader.read(&mut poll_data[bytes_read..]).unwrap_or(0) {
                        0 => break,
                        n => bytes_read += n,
                    }
                }

                PacketPayload::ArtPoll(ArtPoll {
                    prot_ver: u16::from_be_bytes([poll_data[0], poll_data[1]]),
                    flags: poll_data[2],
                    diag_priority: poll_data[3],
                    target_port_address_top: PortAddress::from_raw(u16::from_be_bytes([
                        poll_data[4],
                        poll_data[5],
                    ]))?,
                    target_port_address_bottom: PortAddress::from_raw(u16::from_be_bytes([
                        poll_data[6],
                        poll_data[7],
                    ]))?,
                    esta_man: u16::from_be_bytes([poll_data[8], poll_data[9]]),
                    oem: u16::from_be_bytes([poll_data[10], poll_data[11]]),
                })
            }
            Opcode::OpPollReply => {
                let mut reply_data = [0u8; 229];
                let mut bytes_read = 0;

                while bytes_read < 229 {
                    match reader.read(&mut reply_data[bytes_read..]).unwrap_or(0) {
                        0 => break,
                        n => bytes_read += n,
                    }
                }

                PacketPayload::ArtPollReply(ArtPollReply {
                    ip_address: Ipv4Addr::new(
                        reply_data[0],
                        reply_data[1],
                        reply_data[2],
                        reply_data[3],
                    ),
                    port: u16::from_le_bytes([reply_data[4], reply_data[5]]),
                    vers_info: u16::from_be_bytes([reply_data[6], reply_data[7]]),
                    net_switch: reply_data[8],
                    sub_switch: reply_data[9],
                    oem: u16::from_be_bytes([reply_data[10], reply_data[11]]),
                    ubea_version: reply_data[12],
                    status1: reply_data[13],
                    esta_man: u16::from_be_bytes([reply_data[14], reply_data[15]]),
                    port_name: reply_data[16..34].try_into()?,
                    long_name: reply_data[34..98].try_into()?,
                    node_report: reply_data[98..162].try_into()?,
                    num_ports: u16::from_be_bytes([reply_data[162], reply_data[163]]),
                    port_types: reply_data[164..168].try_into().unwrap(),
                    good_input: reply_data[168..172].try_into().unwrap(),
                    good_output_a: reply_data[172..176].try_into().unwrap(),
                    sw_in: reply_data[176..180].try_into().unwrap(),
                    sw_out: reply_data[180..184].try_into().unwrap(),
                    acn_priority: reply_data[184],
                    sw_macro: reply_data[185],
                    sw_remote: reply_data[186],
                    _spare: reply_data[187..190].try_into().unwrap(),
                    style: reply_data[190],
                    mac: reply_data[191..197].try_into().unwrap(),
                    bind_ip: Ipv4Addr::new(
                        reply_data[197],
                        reply_data[198],
                        reply_data[199],
                        reply_data[200],
                    ),
                    bind_index: reply_data[201],
                    status2: reply_data[202],
                    good_output_b: reply_data[203..207].try_into().unwrap(),
                    status3: reply_data[207],
                    default_resp_uid: reply_data[208..214].try_into().unwrap(),
                    user: u16::from_be_bytes([reply_data[214], reply_data[215]]),
                    refresh_rate: u16::from_be_bytes([reply_data[216], reply_data[217]]),
                    background_queue_policy: reply_data[218],
                    _filler: reply_data[219..229].try_into().unwrap(),
                })
            }
            opcode => {
                log::warn!("Received unimplemented {opcode:?} packet");
                return Err(crate::Error::UnimplementedOpcode(opcode));
            }
        };

        Ok(Self { id, opcode, payload })
    }
}

impl From<PacketPayload> for Packet {
    fn from(payload: PacketPayload) -> Self {
        Self::new(payload)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
#[repr(C)]
pub enum PacketPayload {
    ArtPoll(ArtPoll),
    ArtPollReply(ArtPollReply),
}

impl PacketPayload {
    pub fn opcode(&self) -> Opcode {
        match self {
            PacketPayload::ArtPoll(_) => Opcode::OpPoll,
            PacketPayload::ArtPollReply(_) => Opcode::OpPollReply,
        }
    }
}

impl From<ArtPoll> for PacketPayload {
    fn from(p: ArtPoll) -> Self {
        PacketPayload::ArtPoll(p)
    }
}

impl From<ArtPollReply> for PacketPayload {
    fn from(p: ArtPollReply) -> Self {
        PacketPayload::ArtPollReply(p)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
pub struct ArtPoll {
    /// Controllers should ignore communication with nodes using a protocol version lower than 14.
    prot_ver: u16,
    /// Set behaviour of Node.
    flags: u8,
    /// The lowest priority of diagnostics message that should be sent.
    diag_priority: u8,
    /// Top of the range of [`PortAddress`]es to be tested if Targeted Mode is active.
    target_port_address_top: PortAddress,
    /// Bottom of the range of [`PortAddress`]es to be tested if Targeted Mode is active.
    target_port_address_bottom: PortAddress,
    /// The ESTA Manufacturer Code is assigned by ESTA and uniquely identifies the manufacturer that generated this packet.
    esta_man: u16,
    /// The Oem code uniquely identifies the product sending this packet.
    oem: u16,
}

impl ArtPoll {
    pub fn new() -> Self {
        Self {
            prot_ver: 14,
            flags: 0,
            diag_priority: 0,
            target_port_address_top: PortAddress::MAX,
            target_port_address_bottom: PortAddress::MIN,
            esta_man: 0,
            oem: 0,
        }
    }

    pub fn prot_ver(&self) -> u16 {
        self.prot_ver
    }

    pub fn set_prot_ver(&mut self, prot_ver: u16) {
        self.prot_ver = prot_ver;
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    pub fn diag_priority(&self) -> u8 {
        self.diag_priority
    }

    pub fn set_diag_priority(&mut self, diag_priority: u8) {
        self.diag_priority = diag_priority;
    }

    pub fn target_port_address_top(&self) -> PortAddress {
        self.target_port_address_top
    }

    pub fn set_target_port_address_top(&mut self, target_port_address_top: PortAddress) {
        self.target_port_address_top = target_port_address_top;
    }

    pub fn target_port_address_bottom(&self) -> PortAddress {
        self.target_port_address_bottom
    }

    pub fn set_target_port_address_bottom(&mut self, target_port_address_bottom: PortAddress) {
        self.target_port_address_bottom = target_port_address_bottom;
    }

    pub fn esta_man(&self) -> u16 {
        self.esta_man
    }

    pub fn set_esta_man(&mut self, esta_man: u16) {
        self.esta_man = esta_man;
    }

    pub fn oem(&self) -> u16 {
        self.oem
    }

    pub fn set_oem(&mut self, oem: u16) {
        self.oem = oem;
    }

    pub fn is_targeted_mode_enabled(&self) -> bool {
        (self.flags & (1 << 5)) != 0
    }

    pub fn set_targeted_mode_enabled(&mut self, enabled: bool) {
        if enabled {
            self.flags |= 1 << 5;
        } else {
            self.flags &= !(1 << 5);
        }
    }

    pub fn is_vlc_transmission_disabled(&self) -> bool {
        (self.flags & (1 << 4)) != 0
    }

    pub fn set_vlc_transmission_disabled(&mut self, disabled: bool) {
        if disabled {
            self.flags |= 1 << 4;
        } else {
            self.flags &= !(1 << 4);
        }
    }

    pub fn is_diagnostics_unicast(&self) -> bool {
        (self.flags & (1 << 3)) != 0
    }

    pub fn set_diagnostics_unicast(&mut self, unicast: bool) {
        if unicast {
            self.flags |= 1 << 3;
        } else {
            self.flags &= !(1 << 3);
        }
    }

    pub fn is_diagnostics_requested(&self) -> bool {
        (self.flags & (1 << 2)) != 0
    }

    pub fn set_diagnostics_requested(&mut self, requested: bool) {
        if requested {
            self.flags |= 1 << 2;
        } else {
            self.flags &= !(1 << 2);
        }
    }

    pub fn is_reply_on_change_enabled(&self) -> bool {
        (self.flags & (1 << 1)) != 0
    }

    pub fn set_reply_on_change_enabled(&mut self, enabled: bool) {
        if enabled {
            self.flags |= 1 << 1;
        } else {
            self.flags &= !(1 << 1);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
pub struct ArtPollReply {
    /// Array containing the Node's IP address.
    ip_address: Ipv4Addr,
    /// The Port is always [`crate::PORT`].
    /// When binding is implemented, bound nodes may share the root node's
    /// IP Address and the BindIndex is used to differentiate the nodes.
    port: u16,
    /// Node's firmware revision number.
    /// The Controller should only use this field to decide if a firmware update should proceed.
    /// The convention is that a higher number is a more recent release of firmware.
    vers_info: u16,
    /// Bits 14-8 of the 15 bit Port-Address are encoded into the bottom 7 bits of this field.
    net_switch: u8,
    /// Bits 7-4 of the 15 bit Port-Address are encoded into the bottom 4 bits of this field.
    sub_switch: u8,
    /// The Oem code uniquely identifies the product.
    oem: u16,
    /// Firmware version of the User Bios Extension Area (UBEA). Zero if not programmed.
    ubea_version: u8,
    /// General Status register 1.
    status1: u8,
    /// The ESTA manufacturer code.
    esta_man: u16,
    /// Null terminated name for each port of the node. Max length is 17 characters plus the null.
    port_name: FixedString<18>,
    /// Null terminated long name for the Node. Max length is 63 characters plus the null.
    long_name: FixedString<64>,
    /// Textual report of the Node's operating status or operational errors.
    /// Formatted as: "#xxxx [yyyy] zzzzz..."
    node_report: FixedString<64>,
    /// Number of input or output ports. Maximum value is 4.
    num_ports: u16,
    /// Defines the operation and protocol of each channel.
    port_types: [u8; 4],
    /// Defines input status of the node.
    good_input: [u8; 4],
    /// Defines output status of the node.
    good_output_a: [u8; 4],
    /// Bits 3-0 of the 15 bit Port-Address for each of the 4 possible input ports.
    sw_in: [u8; 4],
    /// Bits 3-0 of the 15 bit Port-Address for each of the 4 possible output ports.
    sw_out: [u8; 4],
    /// The sACN priority value that will be used when any received DMX is converted to sACN.
    acn_priority: u8,
    /// If the Node supports macro key inputs, this byte represents the trigger values.
    sw_macro: u8,
    /// If the Node supports remote trigger inputs, this byte represents the trigger values.
    sw_remote: u8,
    /// Not used, set to zero.
    _spare: [u8; 3],
    /// The Style code defines the equipment style of the device.
    style: u8,
    /// MAC Address.
    mac: [u8; 6],
    /// If this unit is part of a larger or modular product, this is the IP of the root device.
    bind_ip: Ipv4Addr,
    /// This number represents the order of bound devices.
    bind_index: u8,
    /// General Status register 2.
    status2: u8,
    /// Defines output status of the node.
    good_output_b: [u8; 4],
    /// General Status register 3.
    status3: u8,
    /// RDMnet & LLRP Default Responder UID.
    default_resp_uid: [u8; 6],
    /// Available for user specific data.
    user: u16,
    /// Allows the device to specify the maximum refresh rate, expressed in Hz.
    refresh_rate: u16,
    /// Defines the method by which the node retrieves STATUS_MESSAGE and QUEUED_MESSAGE pids.
    background_queue_policy: u8,
    /// Transmit as zero. For future expansion.
    _filler: [u8; 10],
}

impl ArtPollReply {
    pub fn new() -> Self {
        Self {
            ip_address: Ipv4Addr::new(0, 0, 0, 0),
            port: crate::PORT,
            vers_info: 0,
            net_switch: 0,
            sub_switch: 0,
            oem: 0,
            ubea_version: 0,
            status1: 0,
            esta_man: 0,
            port_name: FixedString::default(),
            long_name: FixedString::default(),
            node_report: FixedString::default(),
            num_ports: 0,
            port_types: [0; 4],
            good_input: [0; 4],
            good_output_a: [0; 4],
            sw_in: [0; 4],
            sw_out: [0; 4],
            acn_priority: 0,
            sw_macro: 0,
            sw_remote: 0,
            _spare: [0; 3],
            style: 0,
            mac: [0; 6],
            bind_ip: Ipv4Addr::new(0, 0, 0, 0),
            bind_index: 0,
            status2: 0,
            good_output_b: [0; 4],
            status3: 0,
            default_resp_uid: [0; 6],
            user: 0,
            refresh_rate: 0,
            background_queue_policy: 0,
            _filler: [0; 10],
        }
    }

    pub fn ip_address(&self) -> &Ipv4Addr {
        &self.ip_address
    }

    pub fn set_ip_address(&mut self, ip_address: Ipv4Addr) {
        self.ip_address = ip_address;
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn vers_info(&self) -> u16 {
        self.vers_info
    }

    pub fn set_vers_info(&mut self, vers_info: u16) {
        self.vers_info = vers_info;
    }

    pub fn net_switch(&self) -> u8 {
        self.net_switch
    }

    pub fn set_net_switch(&mut self, net_switch: u8) {
        self.net_switch = net_switch;
    }

    pub fn sub_switch(&self) -> u8 {
        self.sub_switch
    }

    pub fn set_sub_switch(&mut self, sub_switch: u8) {
        self.sub_switch = sub_switch;
    }

    pub fn oem(&self) -> u16 {
        self.oem
    }

    pub fn set_oem(&mut self, oem: u16) {
        self.oem = oem;
    }

    pub fn ubea_version(&self) -> u8 {
        self.ubea_version
    }

    pub fn set_ubea_version(&mut self, ubea_version: u8) {
        self.ubea_version = ubea_version;
    }

    pub fn status1(&self) -> u8 {
        self.status1
    }

    pub fn set_status1(&mut self, status1: u8) {
        self.status1 = status1;
    }

    pub fn esta_man(&self) -> u16 {
        self.esta_man
    }

    pub fn set_esta_man(&mut self, esta_man: u16) {
        self.esta_man = esta_man;
    }

    pub fn port_name(&self) -> &str {
        self.port_name.as_str()
    }

    pub fn set_port_name(&mut self, port_name: FixedString<18>) {
        self.port_name = port_name;
    }

    pub fn long_name(&self) -> &str {
        self.long_name.as_str()
    }

    pub fn set_long_name(&mut self, long_name: FixedString<64>) {
        self.long_name = long_name;
    }

    pub fn node_report(&self) -> &str {
        self.node_report.as_str()
    }

    pub fn set_node_report(&mut self, node_report: FixedString<64>) {
        self.node_report = node_report;
    }

    pub fn num_ports(&self) -> u16 {
        self.num_ports
    }

    pub fn set_num_ports(&mut self, num_ports: u16) {
        self.num_ports = num_ports;
    }

    pub fn port_types(&self) -> &[u8; 4] {
        &self.port_types
    }

    pub fn set_port_types(&mut self, port_types: [u8; 4]) {
        self.port_types = port_types;
    }

    pub fn good_input(&self) -> &[u8; 4] {
        &self.good_input
    }

    pub fn set_good_input(&mut self, good_input: [u8; 4]) {
        self.good_input = good_input;
    }

    pub fn good_output_a(&self) -> &[u8; 4] {
        &self.good_output_a
    }

    pub fn set_good_output_a(&mut self, good_output_a: [u8; 4]) {
        self.good_output_a = good_output_a;
    }

    pub fn sw_in(&self) -> &[u8; 4] {
        &self.sw_in
    }

    pub fn set_sw_in(&mut self, sw_in: [u8; 4]) {
        self.sw_in = sw_in;
    }

    pub fn sw_out(&self) -> &[u8; 4] {
        &self.sw_out
    }

    pub fn set_sw_out(&mut self, sw_out: [u8; 4]) {
        self.sw_out = sw_out;
    }

    pub fn acn_priority(&self) -> u8 {
        self.acn_priority
    }

    pub fn set_acn_priority(&mut self, acn_priority: u8) {
        self.acn_priority = acn_priority;
    }

    pub fn sw_macro(&self) -> u8 {
        self.sw_macro
    }

    pub fn set_sw_macro(&mut self, sw_macro: u8) {
        self.sw_macro = sw_macro;
    }

    pub fn sw_remote(&self) -> u8 {
        self.sw_remote
    }

    pub fn set_sw_remote(&mut self, sw_remote: u8) {
        self.sw_remote = sw_remote;
    }

    pub fn style(&self) -> u8 {
        self.style
    }

    pub fn set_style(&mut self, style: u8) {
        self.style = style;
    }

    pub fn mac(&self) -> &[u8; 6] {
        &self.mac
    }

    pub fn set_mac(&mut self, mac: [u8; 6]) {
        self.mac = mac;
    }

    pub fn bind_ip(&self) -> &Ipv4Addr {
        &self.bind_ip
    }

    pub fn set_bind_ip(&mut self, bind_ip: Ipv4Addr) {
        self.bind_ip = bind_ip;
    }

    pub fn bind_index(&self) -> u8 {
        self.bind_index
    }

    pub fn set_bind_index(&mut self, bind_index: u8) {
        self.bind_index = bind_index;
    }

    pub fn status2(&self) -> u8 {
        self.status2
    }

    pub fn set_status2(&mut self, status2: u8) {
        self.status2 = status2;
    }

    pub fn good_output_b(&self) -> &[u8; 4] {
        &self.good_output_b
    }

    pub fn set_good_output_b(&mut self, good_output_b: [u8; 4]) {
        self.good_output_b = good_output_b;
    }

    pub fn status3(&self) -> u8 {
        self.status3
    }

    pub fn set_status3(&mut self, status3: u8) {
        self.status3 = status3;
    }

    pub fn default_resp_uid(&self) -> &[u8; 6] {
        &self.default_resp_uid
    }

    pub fn set_default_resp_uid(&mut self, default_resp_uid: [u8; 6]) {
        self.default_resp_uid = default_resp_uid;
    }

    pub fn user(&self) -> u16 {
        self.user
    }

    pub fn set_user(&mut self, user: u16) {
        self.user = user;
    }

    pub fn refresh_rate(&self) -> u16 {
        self.refresh_rate
    }

    pub fn set_refresh_rate(&mut self, refresh_rate: u16) {
        self.refresh_rate = refresh_rate;
    }

    pub fn background_queue_policy(&self) -> u8 {
        self.background_queue_policy
    }

    pub fn set_background_queue_policy(&mut self, background_queue_policy: u8) {
        self.background_queue_policy = background_queue_policy;
    }

    pub fn indicator_state(&self) -> u8 {
        (self.status1 >> 6) & 0b11
    }

    pub fn set_indicator_state(&mut self, state: u8) {
        self.status1 = (self.status1 & !(0b11 << 6)) | ((state & 0b11) << 6);
    }

    pub fn port_address_programming_authority(&self) -> u8 {
        (self.status1 >> 4) & 0b11
    }

    pub fn set_port_address_programming_authority(&mut self, authority: u8) {
        self.status1 = (self.status1 & !(0b11 << 4)) | ((authority & 0b11) << 4);
    }

    pub fn is_booted_from_rom(&self) -> bool {
        (self.status1 & (1 << 2)) != 0
    }

    pub fn set_booted_from_rom(&mut self, booted: bool) {
        if booted {
            self.status1 |= 1 << 2;
        } else {
            self.status1 &= !(1 << 2);
        }
    }

    pub fn is_rdm_capable(&self) -> bool {
        (self.status1 & (1 << 1)) != 0
    }

    pub fn set_rdm_capable(&mut self, capable: bool) {
        if capable {
            self.status1 |= 1 << 1;
        } else {
            self.status1 &= !(1 << 1);
        }
    }

    pub fn is_ubea_present(&self) -> bool {
        (self.status1 & (1 << 0)) != 0
    }

    pub fn set_ubea_present(&mut self, present: bool) {
        if present {
            self.status1 |= 1 << 0;
        } else {
            self.status1 &= !(1 << 0);
        }
    }

    pub fn supports_rdm_artaddress(&self) -> bool {
        (self.status2 & (1 << 7)) != 0
    }

    pub fn set_supports_rdm_artaddress(&mut self, supports: bool) {
        if supports {
            self.status2 |= 1 << 7;
        } else {
            self.status2 &= !(1 << 7);
        }
    }

    pub fn supports_switching_output_style(&self) -> bool {
        (self.status2 & (1 << 6)) != 0
    }

    pub fn set_supports_switching_output_style(&mut self, supports: bool) {
        if supports {
            self.status2 |= 1 << 6;
        } else {
            self.status2 &= !(1 << 6);
        }
    }

    pub fn is_squawking(&self) -> bool {
        (self.status2 & (1 << 5)) != 0
    }

    pub fn set_squawking(&mut self, squawking: bool) {
        if squawking {
            self.status2 |= 1 << 5;
        } else {
            self.status2 &= !(1 << 5);
        }
    }

    pub fn supports_sacn_switch(&self) -> bool {
        (self.status2 & (1 << 4)) != 0
    }

    pub fn set_supports_sacn_switch(&mut self, supports: bool) {
        if supports {
            self.status2 |= 1 << 4;
        } else {
            self.status2 &= !(1 << 4);
        }
    }

    pub fn supports_15bit_port_address(&self) -> bool {
        (self.status2 & (1 << 3)) != 0
    }

    pub fn set_supports_15bit_port_address(&mut self, supports: bool) {
        if supports {
            self.status2 |= 1 << 3;
        } else {
            self.status2 &= !(1 << 3);
        }
    }

    pub fn is_dhcp_capable(&self) -> bool {
        (self.status2 & (1 << 2)) != 0
    }

    pub fn set_dhcp_capable(&mut self, capable: bool) {
        if capable {
            self.status2 |= 1 << 2;
        } else {
            self.status2 &= !(1 << 2);
        }
    }

    pub fn is_ip_dhcp_configured(&self) -> bool {
        (self.status2 & (1 << 1)) != 0
    }

    pub fn set_ip_dhcp_configured(&mut self, configured: bool) {
        if configured {
            self.status2 |= 1 << 1;
        } else {
            self.status2 &= !(1 << 1);
        }
    }

    pub fn supports_web_browser_config(&self) -> bool {
        (self.status2 & (1 << 0)) != 0
    }

    pub fn set_supports_web_browser_config(&mut self, supports: bool) {
        if supports {
            self.status2 |= 1 << 0;
        } else {
            self.status2 &= !(1 << 0);
        }
    }

    pub fn failsafe_state(&self) -> u8 {
        (self.status3 >> 6) & 0b11
    }

    pub fn set_failsafe_state(&mut self, state: u8) {
        self.status3 = (self.status3 & !(0b11 << 6)) | ((state & 0b11) << 6);
    }

    pub fn supports_programmable_failsafe(&self) -> bool {
        (self.status3 & (1 << 5)) != 0
    }

    pub fn set_supports_programmable_failsafe(&mut self, supports: bool) {
        if supports {
            self.status3 |= 1 << 5;
        } else {
            self.status3 &= !(1 << 5);
        }
    }

    pub fn supports_llrp(&self) -> bool {
        (self.status3 & (1 << 4)) != 0
    }

    pub fn set_supports_llrp(&mut self, supports: bool) {
        if supports {
            self.status3 |= 1 << 4;
        } else {
            self.status3 &= !(1 << 4);
        }
    }

    pub fn supports_switching_port_direction(&self) -> bool {
        (self.status3 & (1 << 3)) != 0
    }

    pub fn set_supports_switching_port_direction(&mut self, supports: bool) {
        if supports {
            self.status3 |= 1 << 3;
        } else {
            self.status3 &= !(1 << 3);
        }
    }

    pub fn supports_rdmnet(&self) -> bool {
        (self.status3 & (1 << 2)) != 0
    }

    pub fn set_supports_rdmnet(&mut self, supports: bool) {
        if supports {
            self.status3 |= 1 << 2;
        } else {
            self.status3 &= !(1 << 2);
        }
    }

    pub fn is_background_queue_supported(&self) -> bool {
        (self.status3 & (1 << 1)) != 0
    }

    pub fn set_background_queue_supported(&mut self, supported: bool) {
        if supported {
            self.status3 |= 1 << 1;
        } else {
            self.status3 &= !(1 << 1);
        }
    }

    pub fn background_discovery_disable_by_artaddress(&self) -> bool {
        (self.status3 & (1 << 0)) != 0
    }

    pub fn set_background_discovery_disable_by_artaddress(&mut self, can_disable: bool) {
        if can_disable {
            self.status3 |= 1 << 0;
        } else {
            self.status3 &= !(1 << 0);
        }
    }
}

/// Legal OpCode values used in Art-Net packets:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(facet::Facet)]
#[repr(u16)]
pub enum Opcode {
    /// This is an ArtPoll packet, no other data is contained in this UDP packet.
    OpPoll = 0x2000,
    /// This is an ArtPollReply Packet. It contains device status information.
    OpPollReply = 0x2100,
    /// Diagnostics and data logging packet.
    OpDiagData = 0x2300,
    /// This is an ArtCommand packet. It is used to send text based parameter commands.
    OpCommand = 0x2400,
    /// This is an ArtDataRequest packet. It is used to request data such as products URLs.
    OpDataRequest = 0x2700,
    /// This is an ArtDataReply packet. It is used to reply to ArtDataRequest packets.
    OpDataReply = 0x2800,
    /// This is an ArtDmx data packet. It contains zero start code DMX512 information for a single Universe.
    /// (Also referred to as OpOutput in the protocol specification).
    OpDmx = 0x5000,
    /// This is an ArtNzs data packet. It contains non-zero start code (except RDM) DMX512 information for a single Universe.
    OpNzs = 0x5100,
    /// This is an ArtSync data packet. It is used to force synchronous transfer of ArtDmx packets to a node's output.
    OpSync = 0x5200,
    /// This is an ArtAddress packet. It contains remote programming information for a Node.
    OpAddress = 0x6000,
    /// This is an ArtInput packet. It contains enable/disable data for DMX inputs.
    OpInput = 0x7000,
    /// This is an ArtTodRequest packet. It is used to request a Table of Devices (ToD) for RDM discovery.
    OpTodRequest = 0x8000,
    /// This is an ArtTodData packet. It is used to send a Table of Devices (ToD) for RDM discovery.
    OpTodData = 0x8100,
    /// This is an ArtTodControl packet. It is used to send RDM discovery control messages.
    OpTodControl = 0x8200,
    /// This is an ArtRdm packet. It is used to send all non discovery RDM messages.
    OpRdm = 0x8300,
    /// This is an ArtRdmSub packet. It is used to send compressed, RDM Sub-Device data.
    OpRdmSub = 0x8400,
    /// This is an ArtVideoSetup packet. It contains video screen setup information for nodes that implement the extended video features.
    OpVideoSetup = 0xa010,
    /// This is an ArtVideoPalette packet. It contains colour palette setup information for nodes that implement the extended video features.
    OpVideoPalette = 0xa020,
    /// This is an ArtVideoData packet. It contains display data for nodes that implement the extended video features.
    OpVideoData = 0xa040,
    /// This packet is deprecated.
    OpMacMaster = 0xf000,
    /// This packet is deprecated.
    OpMacSlave = 0xf100,
    /// This is an ArtFirmwareMaster packet. It is used to upload new firmware or firmware extensions to the Node.
    OpFirmwareMaster = 0xf200,
    /// This is an ArtFirmwareReply packet. It is returned by the node to acknowledge receipt of an ArtFirmwareMaster packet or ArtFileTnMaster packet.
    OpFirmwareReply = 0xf300,
    /// Uploads user file to node.
    OpFileTnMaster = 0xf400,
    /// Downloads user file from node.
    OpFileFnMaster = 0xf500,
    /// Server to Node acknowledge for download packets.
    OpFileFnReply = 0xf600,
    /// This is an ArtIpProg packet. It is used to re-programme the IP address and Mask of the Node.
    OpIpProg = 0xf800,
    /// This is an ArtIpProgReply packet. It is returned by the node to acknowledge receipt of an ArtIpProg packet.
    OpIpProgReply = 0xf900,
    /// This is an ArtMedia packet. It is Unicast by a Media Server and acted upon by a Controller.
    OpMedia = 0x9000,
    /// This is an ArtMediaPatch packet. It is Unicast by a Controller and acted upon by a Media Server.
    OpMediaPatch = 0x9100,
    /// This is an ArtMediaControl packet. It is Unicast by a Controller and acted upon by a Media Server.
    OpMediaControl = 0x9200,
    /// This is an ArtMediaControlReply packet. It is Unicast by a Media Server and acted upon by a Controller.
    OpMediaControlReply = 0x9300,
    /// This is an ArtTimeCode packet. It is used to transport time code over the network.
    OpTimeCode = 0x9700,
    /// Used to synchronise real time date and clock.
    OpTimeSync = 0x9800,
    /// Used to send trigger macros.
    OpTrigger = 0x9900,
    /// Requests a node's file list.
    OpDirectory = 0x9a00,
    /// Replies to OpDirectory with file list.
    OpDirectoryReply = 0x9b00,
}

impl TryFrom<u16> for Opcode {
    type Error = crate::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x2000 => Ok(Opcode::OpPoll),
            0x2100 => Ok(Opcode::OpPollReply),
            0x2300 => Ok(Opcode::OpDiagData),
            0x2400 => Ok(Opcode::OpCommand),
            0x2700 => Ok(Opcode::OpDataRequest),
            0x2800 => Ok(Opcode::OpDataReply),
            0x5000 => Ok(Opcode::OpDmx),
            0x5100 => Ok(Opcode::OpNzs),
            0x5200 => Ok(Opcode::OpSync),
            0x6000 => Ok(Opcode::OpAddress),
            0x7000 => Ok(Opcode::OpInput),
            0x8000 => Ok(Opcode::OpTodRequest),
            0x8100 => Ok(Opcode::OpTodData),
            0x8200 => Ok(Opcode::OpTodControl),
            0x8300 => Ok(Opcode::OpRdm),
            0x8400 => Ok(Opcode::OpRdmSub),
            0xa010 => Ok(Opcode::OpVideoSetup),
            0xa020 => Ok(Opcode::OpVideoPalette),
            0xa040 => Ok(Opcode::OpVideoData),
            0xf000 => Ok(Opcode::OpMacMaster),
            0xf100 => Ok(Opcode::OpMacSlave),
            0xf200 => Ok(Opcode::OpFirmwareMaster),
            0xf300 => Ok(Opcode::OpFirmwareReply),
            0xf400 => Ok(Opcode::OpFileTnMaster),
            0xf500 => Ok(Opcode::OpFileFnMaster),
            0xf600 => Ok(Opcode::OpFileFnReply),
            0xf800 => Ok(Opcode::OpIpProg),
            0xf900 => Ok(Opcode::OpIpProgReply),
            0x9000 => Ok(Opcode::OpMedia),
            0x9100 => Ok(Opcode::OpMediaPatch),
            0x9200 => Ok(Opcode::OpMediaControl),
            0x9300 => Ok(Opcode::OpMediaControlReply),
            0x9700 => Ok(Opcode::OpTimeCode),
            0x9800 => Ok(Opcode::OpTimeSync),
            0x9900 => Ok(Opcode::OpTrigger),
            0x9a00 => Ok(Opcode::OpDirectory),
            0x9b00 => Ok(Opcode::OpDirectoryReply),
            _ => Err(crate::Error::InvalidOpcode(value)),
        }
    }
}

/// Defines generic error, advisory and status messages for both Nodes and Controllers. The NodeReport is returned in [`ArtPollReply`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(facet::Facet)]
#[repr(u16)]
pub enum NodeReport {
    /// Booted in debug mode (Only used in development)
    RcDebug = 0x0000,
    /// Power On Tests successful
    RcPowerOk = 0x0001,
    /// Hardware tests failed at Power On
    RcPowerFail = 0x0002,
    /// Last UDP from Node failed due to truncated length, most likely caused by a collision.
    RcSocketWr1 = 0x0003,
    /// Unable to identify last UDP transmission. Check OpCode and packet length.
    RcParseFail = 0x0004,
    /// Unable to open Udp Socket in last transmission attempt
    RcUdpFail = 0x0005,
    /// Confirms that Port Name programming via ArtAddress,was successful.
    RcShNameOk = 0x0006,
    /// Confirms that Long Name programming via ArtAddress, was successful.
    RcLoNameOk = 0x0007,
    /// DMX512 receive errors detected.
    RcDmxError = 0x0008,
    /// Ran out of internal DMX transmit buffers.
    RcDmxUdpFull = 0x0009,
    /// Ran out of internal DMX Rx buffers.
    RcDmxRxFull = 0x000a,
    /// Rx Universe switches conflict.
    RcSwitchErr = 0x000b,
    /// Product configuration does not match firmware.
    RcConfigErr = 0x000c,
    /// DMX output short detected. See GoodOutput field.
    RcDmxShort = 0x000d,
    /// Last attempt to upload new firmware failed.
    RcFirmwareFail = 0x000e,
    /// User changed switch settings when address locked by remote programming. User changes ignored.
    RcUserFail = 0x000f,
    /// Factory reset has occurred.
    RcFactoryRes = 0x0010,
}

impl TryFrom<u16> for NodeReport {
    type Error = crate::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0000 => Ok(NodeReport::RcDebug),
            0x0001 => Ok(NodeReport::RcPowerOk),
            0x0002 => Ok(NodeReport::RcPowerFail),
            0x0003 => Ok(NodeReport::RcSocketWr1),
            0x0004 => Ok(NodeReport::RcParseFail),
            0x0005 => Ok(NodeReport::RcUdpFail),
            0x0006 => Ok(NodeReport::RcShNameOk),
            0x0007 => Ok(NodeReport::RcLoNameOk),
            0x0008 => Ok(NodeReport::RcDmxError),
            0x0009 => Ok(NodeReport::RcDmxUdpFull),
            0x000a => Ok(NodeReport::RcDmxRxFull),
            0x000b => Ok(NodeReport::RcSwitchErr),
            0x000c => Ok(NodeReport::RcConfigErr),
            0x000d => Ok(NodeReport::RcDmxShort),
            0x000e => Ok(NodeReport::RcFirmwareFail),
            0x000f => Ok(NodeReport::RcUserFail),
            0x0010 => Ok(NodeReport::RcFactoryRes),
            _ => Err(crate::Error::InvalidNodeReport(value)),
        }
    }
}

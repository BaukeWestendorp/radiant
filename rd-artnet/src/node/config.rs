use std::net::Ipv4Addr;

use crate::{
    BackgroundQueuePolicy, FixedString, NetId, PortProtocol, StyleCode, SubNetId, SwMacro,
    SwRemote, UniverseId,
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NodeConfig {
    pub bound_node_config: BoundNodeConfig,

    pub long_name: FixedString<64>,
    pub esta_man: u16,
    pub oem_code: u16,
    pub version_info: u16,
    pub ueba_version: u8,
    pub bound_nodes: Vec<BoundNodeConfig>,
    pub network: NodeNetworkConfig,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            bound_node_config: BoundNodeConfig::default(),

            long_name: FixedString::try_from_str("Art-Net Node")
                .expect("String should be less than 64 bytes"),
            esta_man: 0x7FFF,
            oem_code: 0x0000,
            ueba_version: 0x00,
            version_info: 0x00,
            network: NodeNetworkConfig::default(),
            bound_nodes: Vec::new(),
        }
    }
}

impl std::ops::DerefMut for NodeConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bound_node_config
    }
}

impl std::ops::Deref for NodeConfig {
    type Target = BoundNodeConfig;

    fn deref(&self) -> &Self::Target {
        &self.bound_node_config
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[repr(C)]
pub enum RefreshRate {
    /// Using `Internal` will make the Node use it's own internal timer.
    Internal { frequency_hz: u16 },
    /// Using `External` expects the user to write the DMX data to the correct ports
    /// themselves at the provided frequency. This frequency will be put into the ArtPolLReply
    /// packet to tell the other end what frequency to expect.
    External { frequency_hz: u16 },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BoundNodeConfig {
    pub bind_index: u8,
    pub net: NetId,
    pub sub_net: SubNetId,
    pub ports: [Option<PortConfig>; 4],
    pub port_name: FixedString<18>,
    pub node_report: FixedString<64>,
    pub default_resp_uid: [u8; 6],
    pub style: StyleCode,
    pub macros: SwMacro,
    pub remote: SwRemote,
    pub refresh_rate: RefreshRate,
    pub bg_queue_policy: BackgroundQueuePolicy,
}

impl Default for BoundNodeConfig {
    fn default() -> Self {
        Self {
            bind_index: 1,
            net: NetId::default(),
            sub_net: SubNetId::default(),
            ports: [None, None, None, None],
            port_name: FixedString::try_from_str("Default Port")
                .expect("String should be less than 18 bytes"),
            node_report: FixedString::try_from_str("#0000 [0000] Normal Boot")
                .expect("String should be less than 64 bytes"),
            default_resp_uid: [0; 6],
            style: StyleCode::StController,
            macros: SwMacro::new(),
            remote: SwRemote::new(),
            refresh_rate: RefreshRate::Internal { frequency_hz: 44 },
            bg_queue_policy: BackgroundQueuePolicy::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[repr(C)]
pub enum MergeMode {
    Htp,
    Ltp,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[repr(C)]
pub enum OutputStyle {
    Continuous,
    Delta,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PortConfig {
    pub input: Option<UniverseId>,
    pub output: Option<UniverseId>,
    pub protocol: PortProtocol,
    pub sacn_priority: u8,

    pub merge_mode: MergeMode,
    pub output_style: OutputStyle,
    pub rdm_enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[repr(C)]
pub enum NodeNetworkConfig {
    Interface {
        /// When set to `None` it will try to select the first non-loopback interface.
        interface_name: Option<String>,
        dhcp_enabled: bool,
    },
    Custom {
        ip: Ipv4Addr,
        mask: Ipv4Addr,
        mac_address: [u8; 6],
        default_gateway: Ipv4Addr,
        dhcp_enabled: bool,
    },
}

impl Default for NodeNetworkConfig {
    fn default() -> Self {
        Self::Interface { interface_name: None, dhcp_enabled: true }
    }
}

use std::net::Ipv4Addr;

use crate::FixedString;

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
pub struct NodeConfig {
    pub name: FixedString<64>,
    pub esta_man: u16,
    pub oem_code: u16,
    pub version_info: u16,
    pub ueba_version: u8,
    pub bound_nodes: Vec<()>,
    pub network: NodeNetworkConfig,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            name: FixedString::try_from_str("Art-Net Node")
                .expect("String should be less than 64 bytes"),
            esta_man: 0x7FFF,
            oem_code: 0x0000,
            ueba_version: 0x00,
            version_info: 0x00,
            bound_nodes: Default::default(),
            network: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
#[repr(C)]
pub enum NodeNetworkConfig {
    Interface {
        /// When set to `None` it will try to select the first non-loopback interface.
        interface_name: Option<String>,
    },
    Custom {
        ip: Ipv4Addr,
        mask: Ipv4Addr,
        mac_address: [u8; 6],
    },
}

impl Default for NodeNetworkConfig {
    fn default() -> Self {
        Self::Interface { interface_name: None }
    }
}

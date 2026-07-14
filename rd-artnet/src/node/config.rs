use std::{net::Ipv4Addr, sync::Arc};

use crate::{
    BackgroundQueuePolicy, FixedString, FrameScheduler, NetId, PortAddress, StyleCode, SubNetId,
    SwMacro, SwRemote, Universe,
};

#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NodeConfig {
    long_name: FixedString<64>,
    esta_man: u16,
    oem_code: u16,
    version_info: u16,
    ueba_version: u8,
    supports_web_browser_configuration: bool,

    network_config: NodeNetworkConfig,
    bound_node_config: BoundNodeConfig,

    bound_nodes: Vec<BoundNodeConfig>,
}

impl NodeConfig {
    pub fn new(
        short_name: FixedString<18>,
        long_name: FixedString<64>,
        dmx_provider: Arc<
            impl Fn(&mut Universe, PortAddress) -> crate::Result<()> + Send + Sync + 'static,
        >,
    ) -> Self {
        Self {
            long_name,
            esta_man: 0x7FFF,
            oem_code: 0x0000,
            version_info: 0x00,
            ueba_version: 0x00,
            supports_web_browser_configuration: false,

            network_config: NodeNetworkConfig::Interface {
                name: None,
                assignment: IpAssignment::Dhcp,
            },
            bound_node_config: BoundNodeConfig::new(
                short_name,
                NetId::default(),
                SubNetId::default(),
                dmx_provider,
            )
            .with_frame_scheduler(FrameScheduler::Internal { refresh_rate: 40 }),

            bound_nodes: Vec::new(),
        }
    }

    pub fn long_name(&self) -> &FixedString<64> {
        &self.long_name
    }

    pub fn set_long_name(&mut self, long_name: FixedString<64>) {
        self.long_name = long_name;
    }

    pub fn with_long_name(mut self, long_name: FixedString<64>) -> Self {
        self.set_long_name(long_name);
        self
    }

    pub fn esta_man(&self) -> u16 {
        self.esta_man
    }

    pub fn set_esta_man(&mut self, esta_man: u16) {
        self.esta_man = esta_man;
    }

    pub fn with_esta_man(mut self, esta_man: u16) -> Self {
        self.set_esta_man(esta_man);
        self
    }

    pub fn oem_code(&self) -> u16 {
        self.oem_code
    }

    pub fn set_oem_code(&mut self, oem_code: u16) {
        self.oem_code = oem_code;
    }

    pub fn with_oem_code(mut self, oem_code: u16) -> Self {
        self.set_oem_code(oem_code);
        self
    }

    pub fn version_info(&self) -> u16 {
        self.version_info
    }

    pub fn set_version_info(&mut self, version_info: u16) {
        self.version_info = version_info;
    }

    pub fn with_version_info(mut self, version_info: u16) -> Self {
        self.set_version_info(version_info);
        self
    }

    pub fn ueba_version(&self) -> u8 {
        self.ueba_version
    }

    pub fn set_ueba_version(&mut self, ueba_version: u8) {
        self.ueba_version = ueba_version;
    }

    pub fn with_ueba_version(mut self, ueba_version: u8) -> Self {
        self.set_ueba_version(ueba_version);
        self
    }

    pub fn supports_web_browser_configuration(&self) -> bool {
        self.supports_web_browser_configuration
    }

    pub fn set_supports_web_browser_configuration(&mut self, supported: bool) {
        self.supports_web_browser_configuration = supported;
    }

    pub fn with_supports_web_browser_configuration(mut self, supported: bool) -> Self {
        self.set_supports_web_browser_configuration(supported);
        self
    }

    pub fn network_config(&self) -> &NodeNetworkConfig {
        &self.network_config
    }

    pub fn set_network_config(&mut self, network: NodeNetworkConfig) {
        self.network_config = network;
    }

    pub fn with_network_config(mut self, network: NodeNetworkConfig) -> Self {
        self.set_network_config(network);
        self
    }

    pub fn bind_index(&self) -> u8 {
        debug_assert_eq!(self.bound_node_config.bind_index, BoundNodeConfig::ROOT_BIND_INDEX);
        self.bound_node_config.bind_index
    }

    pub fn net(&self) -> NetId {
        self.bound_node_config.net
    }

    pub fn set_net(&mut self, net: NetId) {
        self.bound_node_config.net = net;
    }

    pub fn with_net(mut self, net: NetId) -> Self {
        self.bound_node_config.set_net(net);
        self
    }

    pub fn sub_net(&self) -> SubNetId {
        self.bound_node_config.sub_net
    }

    pub fn set_sub_net(&mut self, sub_net: SubNetId) {
        self.bound_node_config.sub_net = sub_net;
    }

    pub fn with_sub_net(mut self, sub_net: SubNetId) -> Self {
        self.bound_node_config.set_sub_net(sub_net);
        self
    }

    pub fn ports(&self) -> impl Iterator<Item = (&BoundNodeConfig, &PortConfig)> {
        self.bound_node_config
            .ports()
            .iter()
            .filter_map(|p| p.as_ref().map(|port| (&self.bound_node_config, port)))
            .chain(self.bound_nodes.iter().flat_map(|node| {
                node.ports().iter().filter_map(move |p| p.as_ref().map(|port| (node, port)))
            }))
    }

    pub fn add_port(&mut self, port: PortConfig) {
        let (net, sub_net) = if let Some(out_addr) = port.output() {
            (out_addr.net(), out_addr.sub_net())
        } else if let Some(in_addr) = port.input() {
            (in_addr.net(), in_addr.sub_net())
        } else {
            return; // Ignore ports with no direction configured.
        };

        // Try to place it in the root bound node.
        if self.bound_node_config.net() == net
            && self.bound_node_config.sub_net() == sub_net
            && self.bound_node_config.port_count() < 4
        {
            if let Some(slot) = self.bound_node_config.ports_mut().iter_mut().find(|p| p.is_none())
            {
                *slot = Some(port);
                return;
            }
        }

        // Try to place it in an existing extended bound node.
        for bound_node in &mut self.bound_nodes {
            if bound_node.net() == net
                && bound_node.sub_net() == sub_net
                && bound_node.port_count() < 4
            {
                if let Some(slot) = bound_node.ports_mut().iter_mut().find(|p| p.is_none()) {
                    *slot = Some(port);
                    return;
                }
            }
        }

        // If no matching slots exist, create new BoundNode.
        let mut new_bound_config = self.bound_node_config.clone();
        new_bound_config.set_net(net);
        new_bound_config.set_sub_net(sub_net);

        new_bound_config.bind_index =
            BoundNodeConfig::ROOT_BIND_INDEX + self.bound_nodes.len() as u8 + 1;

        *new_bound_config.ports_mut() = [Some(port), None, None, None];
        self.bound_nodes.push(new_bound_config);
    }

    pub fn with_port(mut self, port: PortConfig) -> Self {
        self.add_port(port);
        self
    }

    pub fn remove_port(&mut self) -> Option<PortConfig> {
        todo!();
    }

    pub fn port_name(&self) -> &FixedString<18> {
        &self.bound_node_config.port_name
    }

    pub fn set_port_name(&mut self, port_name: FixedString<18>) {
        self.bound_node_config.port_name = port_name;
    }

    pub fn with_port_name(mut self, port_name: FixedString<18>) -> Self {
        self.bound_node_config.set_port_name(port_name);
        self
    }

    pub fn default_resp_uid(&self) -> [u8; 6] {
        self.bound_node_config.default_resp_uid
    }

    pub fn set_default_resp_uid(&mut self, default_resp_uid: [u8; 6]) {
        self.bound_node_config.default_resp_uid = default_resp_uid;
    }

    pub fn with_default_resp_uid(mut self, default_resp_uid: [u8; 6]) -> Self {
        self.bound_node_config.set_default_resp_uid(default_resp_uid);
        self
    }

    pub fn style(&self) -> StyleCode {
        self.bound_node_config.style
    }

    pub fn set_style(&mut self, style: StyleCode) {
        self.bound_node_config.style = style;
    }

    pub fn with_style(mut self, style: StyleCode) -> Self {
        self.bound_node_config.set_style(style);
        self
    }

    pub fn macros(&self) -> SwMacro {
        self.bound_node_config.macros
    }

    pub fn macros_mut(&mut self) -> &mut SwMacro {
        &mut self.bound_node_config.macros
    }

    pub fn with_macros(mut self, macros: SwMacro) -> Self {
        self.bound_node_config.macros = macros;
        self
    }

    pub fn remotes(&self) -> SwRemote {
        self.bound_node_config.remotes
    }

    pub fn remotes_mut(&mut self) -> &mut SwRemote {
        &mut self.bound_node_config.remotes
    }

    pub fn with_remotes(mut self, remotes: SwRemote) -> Self {
        self.bound_node_config.remotes = remotes;
        self
    }

    pub fn bg_queue_policy(&self) -> BackgroundQueuePolicy {
        self.bound_node_config.bg_queue_policy
    }

    pub fn bg_queue_policy_mut(&mut self) -> &mut BackgroundQueuePolicy {
        &mut self.bound_node_config.bg_queue_policy
    }

    pub fn with_bg_queue_policy(mut self, bg_queue_policy: BackgroundQueuePolicy) -> Self {
        self.bound_node_config.bg_queue_policy = bg_queue_policy;
        self
    }

    pub fn frame_scheduler(&self) -> &FrameScheduler {
        &self.bound_node_config.frame_scheduler
    }

    pub fn set_frame_scheduler(&mut self, frame_scheduler: FrameScheduler) {
        self.bound_node_config.frame_scheduler = frame_scheduler;
    }

    pub fn with_frame_scheduler(mut self, frame_scheduler: FrameScheduler) -> Self {
        self.bound_node_config.set_frame_scheduler(frame_scheduler);
        self
    }

    pub fn dmx_provider(
        &self,
    ) -> &Arc<dyn Fn(&mut Universe, PortAddress) -> Result<(), crate::Error> + Send + Sync + 'static>
    {
        &self.bound_node_config.dmx_provider
    }

    pub fn set_dmx_provider(
        &mut self,
        dmx_provider: Arc<dyn Fn(&mut Universe, PortAddress) -> crate::Result<()> + Send + Sync>,
    ) {
        self.bound_node_config.dmx_provider = dmx_provider;
    }

    pub fn with_dmx_provider(
        mut self,
        dmx_provider: Arc<dyn Fn(&mut Universe, PortAddress) -> crate::Result<()> + Send + Sync>,
    ) -> Self {
        self.bound_node_config.set_dmx_provider(dmx_provider);
        self
    }

    pub fn bound_nodes(&self) -> &[BoundNodeConfig] {
        &self.bound_nodes
    }

    pub fn bound_nodes_mut(&mut self) -> &mut Vec<BoundNodeConfig> {
        &mut self.bound_nodes
    }

    pub(crate) fn bound_node_config(&self) -> &BoundNodeConfig {
        &self.bound_node_config
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(tag = "type"))]
#[repr(C)]
pub enum IpAssignment {
    Static { dhcp_capable: bool },
    Dhcp,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(tag = "type"))]
#[repr(C)]
pub enum NodeNetworkConfig {
    Interface {
        /// When set to `None` it will try to select the first non-loopback interface.
        name: Option<String>,
        // Explicitly define the DHCP state of this system interface.
        #[cfg_attr(feature = "facet", facet(default = IpAssignment::Dhcp))]
        assignment: IpAssignment,
    },
    Custom {
        ip: Ipv4Addr,
        mask: Ipv4Addr,
        mac_address: [u8; 6],
        default_gateway: Ipv4Addr,
        // NOTE: Let's assume 'DHCP Capable', as the most common use for this crate
        // probably will be software built on top of an OS, which pretty much always
        // supports DHCP.
        #[cfg_attr(feature = "facet", facet(default = true))]
        dhcp_capable: bool,
    },
}

impl NodeNetworkConfig {
    pub fn dhcp_capable(&self) -> bool {
        match self {
            NodeNetworkConfig::Interface { assignment, .. } => match assignment {
                IpAssignment::Static { dhcp_capable } => *dhcp_capable,
                IpAssignment::Dhcp => true,
            },
            NodeNetworkConfig::Custom { .. } => false,
        }
    }
}

impl Default for NodeNetworkConfig {
    fn default() -> Self {
        Self::Interface { name: None, assignment: IpAssignment::Dhcp }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BoundNodeConfig {
    bind_index: u8,
    net: NetId,
    sub_net: SubNetId,
    ports: [Option<PortConfig>; 4],
    port_name: FixedString<18>,
    default_resp_uid: [u8; 6],
    user_data: u16,
    sacn_priority: u8,
    style: StyleCode,
    macros: SwMacro,
    remotes: SwRemote,
    bg_queue_policy: BackgroundQueuePolicy,
    frame_scheduler: FrameScheduler,
    #[cfg_attr(feature = "facet", facet(opaque))]
    dmx_provider: Arc<dyn Fn(&mut Universe, PortAddress) -> crate::Result<()> + Send + Sync>,
}

impl BoundNodeConfig {
    pub const ROOT_BIND_INDEX: u8 = 0x01;

    pub fn new(
        short_name: FixedString<18>,
        net: NetId,
        sub_net: SubNetId,
        dmx_provider: Arc<
            impl Fn(&mut Universe, PortAddress) -> crate::Result<()> + Send + Sync + 'static,
        >,
    ) -> Self {
        Self {
            bind_index: BoundNodeConfig::ROOT_BIND_INDEX,
            net,
            sub_net,
            ports: [None, None, None, None],
            port_name: short_name,
            default_resp_uid: [0x00; 6],
            user_data: 0x00,
            sacn_priority: 100,
            style: StyleCode::StController,
            macros: SwMacro::new(),
            remotes: SwRemote::new(),
            bg_queue_policy: BackgroundQueuePolicy::new(),
            frame_scheduler: FrameScheduler::Internal { refresh_rate: 40 },
            dmx_provider,
        }
    }

    pub fn bind_index(&self) -> u8 {
        self.bind_index
    }

    pub fn net(&self) -> NetId {
        self.net
    }

    pub fn set_net(&mut self, net: NetId) {
        self.net = net;
    }

    pub fn with_net(mut self, net: NetId) -> Self {
        self.set_net(net);
        self
    }

    pub fn sub_net(&self) -> SubNetId {
        self.sub_net
    }

    pub fn set_sub_net(&mut self, sub_net: SubNetId) {
        self.sub_net = sub_net;
    }

    pub fn with_sub_net(mut self, sub_net: SubNetId) -> Self {
        self.set_sub_net(sub_net);
        self
    }

    pub fn ports(&self) -> &[Option<PortConfig>; 4] {
        &self.ports
    }

    pub fn ports_mut(&mut self) -> &mut [Option<PortConfig>; 4] {
        &mut self.ports
    }

    pub fn with_ports(mut self, ports: [Option<PortConfig>; 4]) -> Self {
        self.ports = ports;
        self
    }

    pub fn port_count(&self) -> usize {
        self.ports.iter().filter(|p| p.is_some()).count()
    }

    pub fn port_name(&self) -> &FixedString<18> {
        &self.port_name
    }

    pub fn set_port_name(&mut self, port_name: FixedString<18>) {
        self.port_name = port_name;
    }

    pub fn with_port_name(mut self, port_name: FixedString<18>) -> Self {
        self.set_port_name(port_name);
        self
    }

    pub fn default_resp_uid(&self) -> [u8; 6] {
        self.default_resp_uid
    }

    pub fn set_default_resp_uid(&mut self, default_resp_uid: [u8; 6]) {
        self.default_resp_uid = default_resp_uid;
    }

    pub fn with_default_resp_uid(mut self, default_resp_uid: [u8; 6]) -> Self {
        self.set_default_resp_uid(default_resp_uid);
        self
    }

    pub fn user_data(&self) -> u16 {
        self.user_data
    }

    pub fn set_user_data(&mut self, user_data: u16) {
        self.user_data = user_data;
    }

    pub fn with_user_data(mut self, user_data: u16) -> Self {
        self.set_user_data(user_data);
        self
    }

    pub fn sacn_priority(&self) -> u8 {
        self.sacn_priority
    }

    pub fn set_sacn_priority(&mut self, sacn_priority: u8) {
        self.sacn_priority = sacn_priority;
    }

    pub fn with_sacn_priority(mut self, sacn_priority: u8) -> Self {
        self.set_sacn_priority(sacn_priority);
        self
    }

    pub fn style(&self) -> StyleCode {
        self.style
    }

    pub fn set_style(&mut self, style: StyleCode) {
        self.style = style;
    }

    pub fn with_style(mut self, style: StyleCode) -> Self {
        self.set_style(style);
        self
    }

    pub fn macros(&self) -> SwMacro {
        self.macros
    }

    pub fn macros_mut(&mut self) -> &mut SwMacro {
        &mut self.macros
    }

    pub fn with_macros(mut self, macros: SwMacro) -> Self {
        self.macros = macros;
        self
    }

    pub fn remotes(&self) -> SwRemote {
        self.remotes
    }

    pub fn remotes_mut(&mut self) -> &mut SwRemote {
        &mut self.remotes
    }

    pub fn with_remotes(mut self, remotes: SwRemote) -> Self {
        self.remotes = remotes;
        self
    }

    pub fn bg_queue_policy(&self) -> BackgroundQueuePolicy {
        self.bg_queue_policy
    }

    pub fn bg_queue_policy_mut(&mut self) -> &mut BackgroundQueuePolicy {
        &mut self.bg_queue_policy
    }

    pub fn with_bg_queue_policy(mut self, bg_queue_policy: BackgroundQueuePolicy) -> Self {
        self.bg_queue_policy = bg_queue_policy;
        self
    }

    pub fn frame_scheduler(&self) -> &FrameScheduler {
        &self.frame_scheduler
    }

    pub fn set_frame_scheduler(&mut self, frame_scheduler: FrameScheduler) {
        self.frame_scheduler = frame_scheduler;
    }

    pub fn with_frame_scheduler(mut self, frame_scheduler: FrameScheduler) -> Self {
        self.set_frame_scheduler(frame_scheduler);
        self
    }

    pub fn dmx_provider(
        &self,
    ) -> &Arc<dyn Fn(&mut Universe, PortAddress) -> Result<(), crate::Error> + Send + Sync + 'static>
    {
        &self.dmx_provider
    }

    pub fn set_dmx_provider(
        &mut self,
        dmx_provider: Arc<dyn Fn(&mut Universe, PortAddress) -> crate::Result<()> + Send + Sync>,
    ) {
        self.dmx_provider = dmx_provider;
    }

    pub fn with_dmx_provider(
        mut self,
        dmx_provider: Arc<dyn Fn(&mut Universe, PortAddress) -> crate::Result<()> + Send + Sync>,
    ) -> Self {
        self.set_dmx_provider(dmx_provider);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PortConfig {
    direction: PortDirection,
    physical: u8,
}

impl PortConfig {
    pub fn new(direction: PortDirection) -> Self {
        Self { direction, physical: 0 }
    }

    pub fn direction(&self) -> &PortDirection {
        &self.direction
    }

    pub fn set_direction(&mut self, direction: PortDirection) {
        self.direction = direction;
    }

    pub fn with_direction(mut self, direction: PortDirection) -> Self {
        self.set_direction(direction);
        self
    }

    pub fn input(&self) -> Option<PortAddress> {
        match &self.direction {
            PortDirection::Input(addr) | PortDirection::Bidirectional { input: addr, .. } => {
                Some(*addr)
            }
            _ => None,
        }
    }

    pub fn output(&self) -> Option<PortAddress> {
        match &self.direction {
            PortDirection::Output(addr) | PortDirection::Bidirectional { output: addr, .. } => {
                Some(*addr)
            }
            _ => None,
        }
    }

    pub fn physical(&self) -> u8 {
        self.physical
    }

    pub fn set_physical(&mut self, physical: u8) {
        self.physical = physical;
    }

    pub fn with_physical(mut self, physical: u8) -> Self {
        self.set_physical(physical);
        self
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
#[repr(C)]
pub enum PortDirection {
    Input(PortAddress),
    Output(PortAddress),
    Bidirectional { input: PortAddress, output: PortAddress },
}

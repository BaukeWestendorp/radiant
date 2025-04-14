use gpui::SharedString;
use std::net::IpAddr;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// DMX IO preferences.
pub struct DmxIo {
    /// The interface to use for DMX IO.
    pub interface: Interface,
    /// sACN DMX IO preferences.
    pub sacn: Sacn,
}

/// Preferences about the interface to use for DMX IO.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Interface {
    /// The name of the interface to use (e.g. 'en0').
    name: SharedString,
}

/// sACN DMX IO preferences.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sacn {
    pub outputs: Vec<SacnOutput>,
}

/// sACN DMX Output preferences.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SacnOutput {
    /// The name of this sACN source.
    pub name: SharedString,
    /// The local universes to send over this sACN source.
    pub local_universes: Vec<sacn::UniverseNumber>,
    /// The destination universe for this source.
    pub destination_universe: sacn::UniverseNumber,
    /// The priority of the packets for this source.
    pub priority: u8,
    /// Whether to send the packets as preview data for this source.
    pub preview_data: bool,
    /// The type of sACN output for this source.
    pub r#type: SacnOutputType,
}

/// The type of sACN output to use.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SacnOutputType {
    /// Sends sACN packets using Unicast UDP.
    Unicast { destination_ip: Option<IpAddr> },
}

impl Default for SacnOutputType {
    fn default() -> Self {
        Self::Unicast { destination_ip: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let json = r#"{
            "interface": {
                "name": "lo"
            },
            "sacn": {
                "outputs": [
                    {
                        "name": "Test sACN Input",
                        "local_universes": [1, 3, 4],
                        "destination_universe": 1,
                        "priority": 142,
                        "preview_data": false,
                        "type": {
                            "unicast": {
                                "destination_ip": "127.0.0.1"
                            }
                        }
                    }
                ]
            }
        }"#;

        let dmx_io: DmxIo = serde_json::from_str(json).unwrap();

        assert_eq!(
            dmx_io,
            DmxIo {
                interface: Interface { name: "lo".into() },
                sacn: Sacn {
                    outputs: vec![SacnOutput {
                        name: "Test sACN Input".into(),
                        local_universes: vec![1, 3, 4],
                        destination_universe: 1,
                        priority: 142,
                        preview_data: false,
                        r#type: SacnOutputType::Unicast {
                            destination_ip: Some("127.0.0.1".parse().unwrap())
                        }
                    }]
                }
            }
        )
    }

    #[test]
    fn deserialize() {
        let dmx_io = DmxIo {
            interface: Interface { name: "lo".into() },
            sacn: Sacn {
                outputs: vec![SacnOutput {
                    name: "Test sACN Input".into(),
                    local_universes: vec![1, 3, 4],
                    destination_universe: 1,
                    priority: 142,
                    preview_data: false,
                    r#type: SacnOutputType::Unicast {
                        destination_ip: Some("127.0.0.1".parse().unwrap()),
                    },
                }],
            },
        };

        let json = serde_json::to_string(&dmx_io).unwrap();

        assert_eq!(
            json,
            r#"{"interface":{"name":"lo"},"sacn":{"outputs":[{"name":"Test sACN Input","local_universes":[1,3,4],"destination_universe":1,"priority":142,"preview_data":false,"type":{"unicast":{"destination_ip":"127.0.0.1"}}}]}}"#
        )
    }
}

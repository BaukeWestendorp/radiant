use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};

use dmx::{DmxChannel, DmxValue};

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    #[serde(default = "Default::default")]
    pub patch_list: PatchList,

    #[serde(default = "Default::default")]
    pub programmer: Programmer,

    #[serde(default = "Default::default")]
    pub data: Data,

    #[serde(default = "Default::default")]
    pub executors: Vec<Executor>,

    #[serde(default = "Default::default")]
    pub dmx_protocols: Vec<DmxArtnetProtocol>,
}

impl Showfile {
    pub fn from_file(file: File) -> io::Result<Self> {
        let reader = BufReader::new(file);
        let showfile: Showfile = serde_json::from_reader(reader)?;
        Ok(showfile)
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct PatchList {
    #[serde(default = "Default::default")]
    pub fixtures: Vec<Fixture>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    pub id: usize,
    pub label: String,
    pub gdtf_share_revision_id: i32,
    pub mode: String,
    pub channel: DmxChannel,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Programmer {
    pub selection: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub groups: Vec<Group>,
    pub sequences: Vec<Sequence>,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub id: usize,
    pub label: String,
    pub fixtures: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    pub id: usize,
    pub label: String,
    pub cues: Vec<Cue>,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Cue {
    pub groups: Vec<usize>,
    pub label: String,
    pub attribute_values: HashMap<String, DmxValue>,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Executor {
    pub id: usize,
    pub sequence: Option<usize>,
    pub current_index: Option<usize>,
    pub r#loop: bool,
    pub button_1: ExecutorButton,
    pub button_2: ExecutorButton,
    pub button_3: ExecutorButton,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct ExecutorButton {
    pub action: ExecutorButtonAction,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ExecutorButtonAction {
    #[default]
    Go,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct DmxArtnetProtocol {
    pub target_ip: String,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use dmx::{DmxChannel, DmxValue};

    use crate::showfile::{
        Cue, Data, DmxArtnetProtocol, Executor, ExecutorButton, ExecutorButtonAction, Fixture,
        Group, PatchList, Programmer, Sequence, Showfile,
    };

    macro_rules! check_showfile {
        ($json:expr, $show_file:expr) => {
            let show_file: Showfile = serde_json::from_str($json).unwrap();
            assert_eq!(show_file, $show_file)
        };
    }

    #[test]
    fn default_showfile() {
        check_showfile!(
            r#"{}"#,
            Showfile {
                patch_list: PatchList {
                    fixtures: Vec::new()
                },
                programmer: Programmer {
                    selection: Vec::new(),
                },
                data: Data {
                    groups: Vec::new(),
                    sequences: Vec::new(),
                },
                executors: Vec::new(),
                dmx_protocols: Vec::new(),
            }
        );
    }

    #[test]
    fn filled() {
        check_showfile!(
            r#"{
                "patch_list": {
                    "fixtures": [
                        {
                            "id": 420,
                            "gdtf_share_revision_id": 42,
                            "label": "Front Wash 1",
                            "mode": "Basic",
                            "channel": {
                                "address": 1,
                                "universe": 2
                            }
                        },
                        {
                            "id": 12,
                            "label": "Wash 2",
                            "gdtf_share_revision_id": 23,
                            "mode": "Turned to 11",
                            "channel": {
                                "address": 5,
                                "universe": 8
                            }
                        }
                    ]
                },
                "data": {
                    "groups": [
                        {
                            "id": 1,
                            "label": "Even",
                            "fixtures": [1]
                        },
                        {
                            "id": 2,
                            "label": "Odd",
                            "fixtures": [2]
                        },
                        {
                            "id": 3,
                            "label": "All",
                            "fixtures": [1, 2]
                        }
                    ],
                    "sequences": [
                        {
                            "id": 1,
                            "label": "Switch",
                            "cues": [
                                {
                                    "groups": [1],
                                    "label": "Cue 1",
                                    "attribute_values": {
                                        "ColorAdd_R": 255,
                                        "ColorAdd_G": 16,
                                        "ColorAdd_B": 127
                                    }
                                },
                                {
                                    "groups": [2],
                                    "label": "Cue 2",
                                    "attribute_values": {
                                        "ColorAdd_R": 32,
                                        "ColorAdd_G": 255,
                                        "ColorAdd_B": 16
                                    }
                                },
                                {
                                    "groups": [3],
                                    "label": "Cue 3",
                                    "attribute_values": {
                                        "ColorAdd_R": 255,
                                        "ColorAdd_G": 255,
                                        "ColorAdd_B": 255
                                    }
                                }
                            ]
                        }
                    ]
                },
                "executors": [
                    {
                        "id": 1,
                        "sequence": 1,
                        "current_index": null,
                        "loop": true,
                        "button_1": {
                            "action": "Go"
                        },
                        "button_2": {
                            "action": "Go"
                        },
                        "button_3": {
                            "action": "Go"
                        }
                    }
                ],
                "dmx_protocols": [
                    {
                        "target_ip": "0.0.0.0"
                    }
                ]
            }"#,
            Showfile {
                patch_list: PatchList {
                    fixtures: vec![
                        Fixture {
                            id: 420,
                            label: "Front Wash 1".to_string(),
                            gdtf_share_revision_id: 42,
                            mode: "Basic".to_string(),
                            channel: DmxChannel {
                                address: 1,
                                universe: 2
                            }
                        },
                        Fixture {
                            id: 12,
                            label: "Wash 2".to_string(),
                            gdtf_share_revision_id: 23,
                            mode: "Turned to 11".to_string(),
                            channel: DmxChannel {
                                address: 5,
                                universe: 8
                            }
                        }
                    ]
                },
                programmer: Programmer::default(),
                data: Data {
                    groups: vec![
                        Group {
                            id: 1,
                            label: "Even".to_string(),
                            fixtures: vec![1]
                        },
                        Group {
                            id: 2,
                            label: "Odd".to_string(),
                            fixtures: vec![2]
                        },
                        Group {
                            id: 3,
                            label: "All".to_string(),
                            fixtures: vec![1, 2]
                        }
                    ],
                    sequences: vec![Sequence {
                        id: 1,
                        label: "Switch".to_string(),
                        cues: vec![
                            Cue {
                                groups: vec![1],
                                label: "Cue 1".to_string(),
                                attribute_values: {
                                    let mut map = HashMap::new();
                                    map.insert("ColorAdd_R".to_string(), DmxValue::new(255));
                                    map.insert("ColorAdd_G".to_string(), DmxValue::new(16));
                                    map.insert("ColorAdd_B".to_string(), DmxValue::new(127));
                                    map
                                }
                            },
                            Cue {
                                groups: vec![2],
                                label: "Cue 2".to_string(),
                                attribute_values: {
                                    let mut map = HashMap::new();
                                    map.insert("ColorAdd_R".to_string(), DmxValue::new(32));
                                    map.insert("ColorAdd_G".to_string(), DmxValue::new(255));
                                    map.insert("ColorAdd_B".to_string(), DmxValue::new(16));
                                    map
                                }
                            },
                            Cue {
                                groups: vec![3],
                                label: "Cue 3".to_string(),
                                attribute_values: {
                                    let mut map = HashMap::new();
                                    map.insert("ColorAdd_R".to_string(), DmxValue::new(255));
                                    map.insert("ColorAdd_G".to_string(), DmxValue::new(255));
                                    map.insert("ColorAdd_B".to_string(), DmxValue::new(255));
                                    map
                                }
                            }
                        ]
                    },]
                },
                executors: vec![Executor {
                    id: 1,
                    sequence: Some(1),
                    current_index: None,
                    r#loop: true,
                    button_1: ExecutorButton {
                        action: ExecutorButtonAction::Go,
                    },
                    button_2: ExecutorButton {
                        action: ExecutorButtonAction::Go,
                    },
                    button_3: ExecutorButton {
                        action: ExecutorButtonAction::Go,
                    }
                }],
                dmx_protocols: vec![DmxArtnetProtocol {
                    target_ip: "0.0.0.0".to_string()
                }]
            }
        );
    }
}

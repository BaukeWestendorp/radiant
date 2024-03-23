use dmx::DmxChannel;

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    #[serde(default = "Default::default")]
    pub patch_list: PatchList,
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

#[cfg(test)]
mod tests {
    use dmx::DmxChannel;

    use crate::showfile::{Fixture, PatchList, Showfile};

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
                }
            }
        );
    }

    #[test]
    fn empty_patch_list() {
        check_showfile!(
            r#"{
                "patch_list": {}
            }"#,
            Showfile {
                patch_list: PatchList {
                    fixtures: Vec::new()
                }
            }
        );
    }

    #[test]
    fn with_fixtures() {
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
                }
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
                }
            }
        );
    }
}

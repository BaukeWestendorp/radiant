use std::cell::Cell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use dmx::{DmxChannel, DmxValue};
use gdtf::GdtfDescription;
use gdtf_share::GdtfShare;
use lazy_static::lazy_static;

use crate::dmx_protocols;
use crate::playback_engine::PlaybackEngine;
use crate::show::{self, AttributeValues, Show};

lazy_static! {
    static ref BASE_DIRS: xdg::BaseDirectories = xdg::BaseDirectories::new().unwrap();
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    #[serde(default = "Default::default")]
    pub patchlist: Patchlist,

    #[serde(default = "Default::default")]
    pub programmer: Programmer,

    #[serde(default = "Default::default")]
    pub data: Data,

    #[serde(default = "Default::default")]
    pub presets: Presets,

    #[serde(default = "Default::default")]
    pub executors: Vec<Executor>,

    #[serde(default = "Default::default")]
    pub dmx_protocols: Vec<ArtnetDmxProtocol>,
}

impl Showfile {
    pub async fn try_into_show(self, gdtf_share: Option<GdtfShare>) -> Result<Show, Error> {
        Ok(Show {
            patchlist: self.patchlist.try_into_show_patchlist(gdtf_share).await?,
            programmer: self.programmer.into_show_programmer(),
            playback_engine: PlaybackEngine::new(),
            data: self.data.into(),
            presets: self.presets.into(),
            executors: self
                .executors
                .into_iter()
                .map(|executor| executor.into())
                .collect(),
            dmx_protocols: self
                .dmx_protocols
                .into_iter()
                .map(|dmx_protocol| dmx_protocol.into())
                .collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Patchlist {
    #[serde(default = "Default::default")]
    pub fixtures: Vec<Fixture>,
}

impl Patchlist {
    pub async fn try_into_show_patchlist(
        self,
        gdtf_share: Option<GdtfShare>,
    ) -> Result<show::Patchlist, Error> {
        let mut patchlist = show::Patchlist::new();

        for fixture in self.fixtures {
            // FIXME: This should be done in parallel.
            let f = fixture
                .into_show_fixture(&mut patchlist, gdtf_share.as_ref())
                .await;
            patchlist.patch_fixture(f);
        }

        Ok(patchlist)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    pub id: usize,
    pub label: String,
    pub gdtf_share_revision_id: i32,
    pub mode: String,
    pub channel: DmxChannel,
}

impl Fixture {
    pub async fn into_show_fixture(
        self,
        patchlist: &mut show::Patchlist,
        gdtf_share: Option<&GdtfShare>,
    ) -> show::Fixture {
        let rid = self.gdtf_share_revision_id;
        let gdtf_description = match patchlist.get_gdtf_description(rid) {
            Some(description) => description,
            None => {
                let cached_file_path = BASE_DIRS
                    .place_cache_file(Path::new(&format!("radiant/fixtures/{rid}.gdtf")))
                    .unwrap();
                if let Ok(cached_file) = std::fs::read(cached_file_path.clone()) {
                    let cached_description =
                        GdtfDescription::from_archive_bytes(&cached_file).unwrap();
                    log::debug!("Using cached GDTF file '{rid}.gdtf'");
                    patchlist.register_gdtf_description(rid, cached_description)
                } else {
                    let Some(gdtf_share) = gdtf_share else {
                        log::error!("Could not download uncached GDTF file.");
                        todo!();
                    };

                    let description_file = gdtf_share.download_file(rid).await.unwrap();
                    let reader = std::io::Cursor::new(description_file.clone());
                    let description = GdtfDescription::from_archive_reader(reader).unwrap();

                    let mut file = File::create_new(cached_file_path).unwrap();
                    file.write_all(&description_file).unwrap();
                    log::debug!("Cached GDTF file '{rid}.gdtf'");

                    patchlist.register_gdtf_description(rid, description)
                }
            }
        };

        show::Fixture {
            id: self.id,
            label: self.label,
            description: gdtf_description,
            channel: self.channel,
            mode: self.mode,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Programmer {
    pub selection: Vec<usize>,
    pub changes: HashMap<DmxChannel, u8>,
}

impl Programmer {
    pub fn into_show_programmer(self) -> show::Programmer {
        show::Programmer {
            selection: self.selection,
            changes: self.changes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub groups: Vec<Group>,
    pub sequences: Vec<Sequence>,
}

impl Into<show::Data> for Data {
    fn into(self) -> show::Data {
        show::Data {
            groups: self.groups.into_iter().map(|group| group.into()).collect(),
            sequences: self
                .sequences
                .into_iter()
                .map(|sequence| sequence.into())
                .collect(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub id: usize,
    pub label: String,
    pub fixtures: Vec<usize>,
}

impl Into<show::Group> for Group {
    fn into(self) -> show::Group {
        show::Group {
            id: self.id,
            label: self.label,
            fixtures: self.fixtures,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    pub id: usize,
    pub label: String,
    pub cues: Vec<Cue>,
}

impl Into<show::Sequence> for Sequence {
    fn into(self) -> show::Sequence {
        show::Sequence {
            id: self.id,
            label: self.label,
            cues: self.cues.into_iter().map(|cue| cue.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Cue {
    pub groups: Vec<usize>,
    pub label: String,
    pub attribute_values: HashMap<String, DmxValue>,
}

impl Into<show::Cue> for Cue {
    fn into(self) -> show::Cue {
        show::Cue {
            groups: self.groups,
            label: self.label,
            attribute_values: self.attribute_values,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Presets {
    pub colors: Vec<Preset>,
}

impl Into<show::Presets> for Presets {
    fn into(self) -> show::Presets {
        show::Presets {
            colors: self
                .colors
                .into_iter()
                .map(|preset| preset.into())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Preset {
    pub id: usize,
    pub label: String,
    pub attribute_values: AttributeValues,
}

impl Into<show::ColorPreset> for Preset {
    fn into(self) -> show::ColorPreset {
        show::ColorPreset {
            id: self.id,
            label: self.label,
            attribute_values: self.attribute_values,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Executor {
    pub id: usize,
    pub sequence: Option<usize>,
    pub current_index: Option<usize>,
    pub r#loop: bool,
    pub fader_value: f32,
    pub button_1: ExecutorButton,
    pub button_2: ExecutorButton,
    pub button_3: ExecutorButton,
}

impl Into<show::Executor> for Executor {
    fn into(self) -> show::Executor {
        show::Executor {
            id: self.id,
            sequence: self.sequence,
            current_index: Cell::new(self.current_index),
            r#loop: self.r#loop,
            fader_value: self.fader_value,
            button_1: self.button_1.into(),
            button_2: self.button_2.into(),
            button_3: self.button_3.into(),
            flash: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct ExecutorButton {
    pub action: ExecutorButtonAction,
}

impl Into<show::ExecutorButton> for ExecutorButton {
    fn into(self) -> show::ExecutorButton {
        show::ExecutorButton {
            action: self.action.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ExecutorButtonAction {
    #[default]
    Go,
    Top,
    Flash,
}

impl Into<show::ExecutorButtonAction> for ExecutorButtonAction {
    fn into(self) -> show::ExecutorButtonAction {
        match self {
            ExecutorButtonAction::Go => show::ExecutorButtonAction::Go,
            ExecutorButtonAction::Top => show::ExecutorButtonAction::Top,
            ExecutorButtonAction::Flash => show::ExecutorButtonAction::Flash,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct ArtnetDmxProtocol {
    pub target_ip: String,
}

impl Into<dmx_protocols::ArtnetDmxProtocol> for ArtnetDmxProtocol {
    fn into(self) -> dmx_protocols::ArtnetDmxProtocol {
        dmx_protocols::ArtnetDmxProtocol::new(self.target_ip.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use dmx::{DmxChannel, DmxValue};

    use crate::showfile::{
        ArtnetDmxProtocol, Cue, Data, Executor, ExecutorButton, ExecutorButtonAction, Fixture,
        Group, Patchlist, Preset, Presets, Programmer, Sequence, Showfile,
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
                patchlist: Patchlist {
                    fixtures: Vec::new()
                },
                programmer: Programmer {
                    selection: Vec::new(),
                    changes: HashMap::new(),
                },
                data: Data {
                    groups: Vec::new(),
                    sequences: Vec::new(),
                },
                presets: Presets { colors: Vec::new() },
                executors: Vec::new(),
                dmx_protocols: Vec::new(),
            }
        );
    }

    #[test]
    fn filled() {
        check_showfile!(
            r#"{
                "patchlist": {
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
                                }
                            ]
                        }
                    ]
                },
                "presets": {
                    "colors": [
                        {
                            "id": 1,
                            "label": "Red",
                            "attribute_values": {
                                "ColorAdd_R": 255,
                                "ColorAdd_G": 0,
                                "ColorAdd_B": 0
                            }
                        }
                    ]
                },
                "executors": [
                    {
                        "id": 1,
                        "sequence": 1,
                        "current_index": null,
                        "loop": true,
                        "fader_value": 0.2,
                        "button_1": {
                            "action": "Top"
                        },
                        "button_2": {
                            "action": "Go"
                        },
                        "button_3": {
                            "action": "Flash"
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
                patchlist: Patchlist {
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
                        cues: vec![Cue {
                            groups: vec![1],
                            label: "Cue 1".to_string(),
                            attribute_values: {
                                let mut map = HashMap::new();
                                map.insert("ColorAdd_R".to_string(), DmxValue::new(255));
                                map.insert("ColorAdd_G".to_string(), DmxValue::new(16));
                                map.insert("ColorAdd_B".to_string(), DmxValue::new(127));
                                map
                            }
                        }]
                    },]
                },
                presets: Presets {
                    colors: vec![Preset {
                        id: 1,
                        label: "Red".to_string(),
                        attribute_values: {
                            let mut map = HashMap::new();
                            map.insert("ColorAdd_R".to_string(), DmxValue::new(255));
                            map.insert("ColorAdd_G".to_string(), DmxValue::new(0));
                            map.insert("ColorAdd_B".to_string(), DmxValue::new(0));
                            map
                        }
                    }],
                },
                executors: vec![Executor {
                    id: 1,
                    sequence: Some(1),
                    current_index: None,
                    r#loop: true,
                    fader_value: 0.2,
                    button_1: ExecutorButton {
                        action: ExecutorButtonAction::Top,
                    },
                    button_2: ExecutorButton {
                        action: ExecutorButtonAction::Go,
                    },
                    button_3: ExecutorButton {
                        action: ExecutorButtonAction::Flash,
                    }
                }],
                dmx_protocols: vec![ArtnetDmxProtocol {
                    target_ip: "0.0.0.0".to_string()
                }]
            }
        );
    }
}

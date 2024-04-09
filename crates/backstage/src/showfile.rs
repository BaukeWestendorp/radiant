use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use anyhow::{anyhow, Error};
use dmx::{DmxChannel, DmxOutput};
use gdtf::GdtfDescription;
use gdtf_share::GdtfShare;
use lazy_static::lazy_static;

use crate::playback_engine::PlaybackEngine;
use crate::show::{self, AttributeValues, Show};
use crate::{dmx_protocols, preset};

const FIXTURE_CACHE_PATH: &str = "radiant/fixtures";

lazy_static! {
    static ref BASE_DIRS: xdg::BaseDirectories = xdg::BaseDirectories::new().unwrap();
    static ref GDTF_SHARE: Result<GdtfShare, Error> = {
        let user = env::var("GDTF_SHARE_API_USER")?;
        let password = env::var("GDTF_SHARE_API_PASSWORD")?;
        match futures_lite::future::block_on(GdtfShare::auth(user, password)) {
            Ok(gdtf_share) => {
                log::info!("Authenticated with GDTF Share API");
                Ok(gdtf_share)
            }
            Err(_) => {
                Err(anyhow!("Failed to authenticate with GDTF Share. Trying to load showfile without a connection..."))
            }
        }
    };
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
}

impl Showfile {
    pub async fn try_into_show(self) -> Result<Show, Error> {
        Ok(Show {
            patchlist: self.patchlist.try_into_show_patchlist().await?,
            programmer: self.programmer.into_show_programmer(),
            playback_engine: PlaybackEngine::new(),
            data: self.data.into(),
            presets: self.presets.into(),
            executors: self
                .executors
                .into_iter()
                .map(|executor| executor.into())
                .collect(),
            current_command: None,
            stage_output: Rc::new(RefCell::new(DmxOutput::default())),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Patchlist {
    #[serde(default = "Default::default")]
    pub fixtures: Vec<Fixture>,
}

impl Patchlist {
    pub async fn try_into_show_patchlist(self) -> Result<show::Patchlist, Error> {
        let mut patchlist = show::Patchlist::new();

        for fixture in self.fixtures {
            // FIXME: This should be done in parallel.
            let f = fixture.into_show_fixture(&mut patchlist).await?;
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
    ) -> Result<show::Fixture, Error> {
        let rid = self.gdtf_share_revision_id;
        let gdtf_description = match patchlist.get_gdtf_description(rid) {
            Some(description) => description,
            None => {
                let cached_file_path = BASE_DIRS
                    .place_cache_file(Path::new(FIXTURE_CACHE_PATH).join(format!("{rid}.gdtf")))
                    .unwrap();

                if let Ok(cached_file) = std::fs::read(cached_file_path.clone()) {
                    let cached_description =
                        GdtfDescription::from_archive_bytes(&cached_file).unwrap();
                    log::info!("Using cached GDTF file '{}'", cached_file_path.display());
                    patchlist.register_gdtf_description(rid, cached_description)
                } else {
                    let Ok(gdtf_share) = GDTF_SHARE.as_ref() else {
                        return Err(anyhow!("Could not download uncached GDTF file."));
                    };

                    let description_file = gdtf_share.download_file(rid).await.unwrap();
                    let reader = std::io::Cursor::new(description_file.clone());
                    let description = GdtfDescription::from_archive_reader(reader).unwrap();

                    let mut file = File::create_new(cached_file_path.clone()).unwrap();
                    file.write_all(&description_file).unwrap();
                    log::info!("Cached GDTF file '{}'", cached_file_path.display());

                    patchlist.register_gdtf_description(rid, description)
                }
            }
        };

        Ok(show::Fixture {
            id: self.id,
            label: self.label,
            description: gdtf_description,
            channel: self.channel,
            mode: self.mode,
        })
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

impl From<Data> for show::Data {
    fn from(val: Data) -> Self {
        show::Data {
            groups: val.groups.into_iter().map(|group| group.into()).collect(),
            sequences: val
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

impl From<Group> for show::Group {
    fn from(val: Group) -> Self {
        show::Group {
            id: val.id,
            label: val.label,
            fixtures: val.fixtures,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    pub id: usize,
    pub label: String,
    pub cues: Vec<Cue>,
}

impl From<Sequence> for show::Sequence {
    fn from(val: Sequence) -> Self {
        show::Sequence {
            id: val.id,
            label: val.label,
            cues: val.cues.into_iter().map(|cue| cue.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Cue {
    pub label: String,
    pub changes: HashMap<usize, AttributeValues>,
}

impl From<Cue> for show::Cue {
    fn from(val: Cue) -> Self {
        show::Cue {
            label: val.label,
            changes: val.changes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Presets {
    #[serde(default)]
    pub beam: Vec<Preset>,
    #[serde(default)]
    pub color: Vec<Preset>,
    #[serde(default)]
    pub dimmer: Vec<Preset>,
    #[serde(default)]
    pub focus: Vec<Preset>,
    #[serde(default)]
    pub gobo: Vec<Preset>,
    #[serde(default)]
    pub position: Vec<Preset>,
    #[serde(default)]
    pub all: Vec<Preset>,
}

impl From<Presets> for preset::Presets {
    fn from(val: Presets) -> Self {
        preset::Presets {
            beam: val.beam.into_iter().map(|preset| preset.into()).collect(),
            color: val.color.into_iter().map(|preset| preset.into()).collect(),
            dimmer: val.dimmer.into_iter().map(|preset| preset.into()).collect(),
            focus: val.focus.into_iter().map(|preset| preset.into()).collect(),
            gobo: val.gobo.into_iter().map(|preset| preset.into()).collect(),
            position: val
                .position
                .into_iter()
                .map(|preset| preset.into())
                .collect(),
            all: val.all.into_iter().map(|preset| preset.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Preset {
    pub id: usize,
    pub label: String,
    pub attribute_values: AttributeValues,
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

impl From<Executor> for show::Executor {
    fn from(val: Executor) -> Self {
        show::Executor {
            id: val.id,
            sequence: val.sequence,
            current_index: Cell::new(val.current_index),
            r#loop: val.r#loop,
            fader_value: val.fader_value,
            button_1: val.button_1.into(),
            button_2: val.button_2.into(),
            button_3: val.button_3.into(),
            flash: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct ExecutorButton {
    pub action: ExecutorButtonAction,
}

impl From<ExecutorButton> for show::ExecutorButton {
    fn from(val: ExecutorButton) -> Self {
        show::ExecutorButton {
            action: val.action.into(),
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

impl From<ExecutorButtonAction> for show::ExecutorButtonAction {
    fn from(val: ExecutorButtonAction) -> Self {
        match val {
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

impl From<ArtnetDmxProtocol> for dmx_protocols::ArtnetDmxProtocol {
    fn from(val: ArtnetDmxProtocol) -> Self {
        dmx_protocols::ArtnetDmxProtocol::new(val.target_ip.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::showfile::{Data, Patchlist, Presets, Programmer, Showfile};

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
                presets: Presets {
                    beam: Vec::new(),
                    color: Vec::new(),
                    dimmer: Vec::new(),
                    focus: Vec::new(),
                    gobo: Vec::new(),
                    position: Vec::new(),
                    all: Vec::new()
                },
                executors: Vec::new(),
            }
        );
    }
}

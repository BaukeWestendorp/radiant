use anyhow::Result;
use itertools::Itertools;
use std::cell::Cell;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;

use dmx::{DmxChannel, DmxOutput, DmxValue};
use gdtf::{ActivationGroup, Attribute, FeatureGroup, FixtureType, GdtfDescription};
use gdtf_share::GdtfShare;

use crate::command::{Command, Instruction, Object};
use crate::dmx_protocols::ArtnetDmxProtocol;
use crate::playback_engine::PlaybackEngine;
use crate::showfile::{self, Showfile};

#[derive(Debug)]
pub struct Show {
    patchlist: Patchlist,

    programmer: Programmer,

    playback_engine: PlaybackEngine,

    data: Data,

    executors: Vec<Executor>,

    dmx_protocols: Vec<ArtnetDmxProtocol>,
}

impl Show {
    pub async fn new(showfile: Showfile, gdtf_share: GdtfShare) -> Self {
        let mut patchlist = Patchlist::new();

        for f in showfile.patch_list.fixtures.into_iter() {
            let rid = f.gdtf_share_revision_id;
            let gdtf_description = match patchlist.get_gdtf_description(rid) {
                Some(description) => description,
                None => {
                    let description_file = gdtf_share.download_file(rid).await.unwrap();
                    let reader = Cursor::new(description_file);
                    let description = GdtfDescription::from_archive_reader(reader).unwrap();
                    patchlist.register_gdtf_description(rid, description)
                }
            };

            let fixture = Fixture {
                id: f.id,
                label: f.label,
                description: gdtf_description,
                channel: f.channel,
                mode: f.mode,
            };

            patchlist.patch_fixture(fixture);
        }

        let programmer = Programmer {
            selection: showfile.programmer.selection,
        };

        let data = Data {
            groups: showfile
                .data
                .groups
                .into_iter()
                .map(|group| Group {
                    id: group.id,
                    label: group.label,
                    fixtures: group.fixtures,
                })
                .collect(),
            sequences: showfile
                .data
                .sequences
                .into_iter()
                .map(|sequence| Sequence {
                    id: sequence.id,
                    label: sequence.label,
                    cues: sequence
                        .cues
                        .into_iter()
                        .map(|cue| Cue {
                            groups: cue.groups,
                            label: cue.label,
                            attribute_values: cue.attribute_values,
                        })
                        .collect(),
                })
                .collect(),
        };

        let executors = showfile
            .executors
            .into_iter()
            .map(|executor| Executor {
                id: executor.id,
                sequence: executor.sequence,
                current_index: Cell::new(executor.current_index),
                r#loop: executor.r#loop,
                flash: false,
                fader_value: executor.fader_value,
                button_1: ExecutorButton {
                    action: match executor.button_1.action {
                        showfile::ExecutorButtonAction::Go => ExecutorButtonAction::Go,
                        showfile::ExecutorButtonAction::Top => ExecutorButtonAction::Top,
                        showfile::ExecutorButtonAction::Flash => ExecutorButtonAction::Flash,
                    },
                },
                button_2: ExecutorButton {
                    action: match executor.button_2.action {
                        showfile::ExecutorButtonAction::Go => ExecutorButtonAction::Go,
                        showfile::ExecutorButtonAction::Top => ExecutorButtonAction::Top,
                        showfile::ExecutorButtonAction::Flash => ExecutorButtonAction::Flash,
                    },
                },
                button_3: ExecutorButton {
                    action: match executor.button_3.action {
                        showfile::ExecutorButtonAction::Go => ExecutorButtonAction::Go,
                        showfile::ExecutorButtonAction::Top => ExecutorButtonAction::Top,
                        showfile::ExecutorButtonAction::Flash => ExecutorButtonAction::Flash,
                    },
                },
            })
            .collect();

        // FIXME: Get the dmx protocols from the showfile.
        let dmx_protocols = showfile
            .dmx_protocols
            .into_iter()
            .map(|protocol| ArtnetDmxProtocol::new(protocol.target_ip.as_str()).unwrap())
            .collect();

        Self {
            patchlist,
            programmer,
            playback_engine: PlaybackEngine::new(),
            data,
            executors,
            dmx_protocols,
        }
    }

    pub fn execute_command_str(&mut self, s: &str) -> Result<()> {
        let command = Command::parse(s)?;
        self.execute_command(command)?;
        Ok(())
    }

    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        // FIXME: This command execution is quite ad-hoc for now.
        match command.instructions.get(0) {
            Some(instr) => {
                match instr {
                    Instruction::Clear => {}
                    Instruction::Select(object) => match object {
                        Object::Fixture(id) => {
                            if !self.is_fixture_selected(*id) {
                                if self.fixture_exists(*id) {
                                    self.programmer.selection.push(*id);
                                    log::info!("Selected Fixture {id}");
                                } else {
                                    log::error!("Failed to select Fixture {id}: Fixture not found");
                                    // FIXME: Return a useful error.
                                }
                            }
                        }
                        Object::Group(id) => {
                            for fixture_id in self
                                .fixtures_in_group(*id)
                                .iter()
                                .map(|f| f.id)
                                .collect_vec()
                            {
                                if let Err(err) =
                                    self.execute_command(Command::new([Instruction::Select(
                                        Object::Fixture(fixture_id),
                                    )]))
                                {
                                    log::error!("Failed to Select Group {id}: {err}");
                                    // FIXME: Return a useful error.
                                    return Ok(());
                                }
                            }
                        }
                        Object::Executor(id) => {
                            let Some(next_instruction) = command.instructions.get(1) else {
                                log::error!("Expected instruction after executor selection");
                                // FIXME: Return a useful error.
                                return Ok(());
                            };

                            match next_instruction {
                                Instruction::Go => {
                                    let Some(executor) = self.executor(*id) else {
                                        log::error!("Failed to Go executor: Executor with id '{id}' not found.");
                                        // FIXME: Return a useful error.
                                        return Ok(());
                                    };

                                    executor.go(self);
                                }
                                Instruction::Top => {
                                    let Some(executor) = self.executor(*id) else {
                                        log::error!("Failed to Top executor: Executor with id '{id}' not found.");
                                        // FIXME: Return a useful error.
                                        return Ok(());
                                    };

                                    executor.top(self);
                                }
                                instr => {
                                    log::error!(
                                        "Invalid instruction after executor selection: {instr}"
                                    );
                                }
                            }
                        }
                    },
                    Instruction::Go => {
                        log::error!("The Go command should be used after selecting a executor with 'Select Executor #'!");
                    }
                    Instruction::Top => {
                        log::error!("The Top command should be used after selecting a executor with 'Select Executor #'!");
                    }
                }
            }
            None => {}
        }
        Ok(())
    }

    pub fn fixture(&self, fixture_id: usize) -> Option<&Fixture> {
        self.patchlist.fixtures.iter().find(|f| f.id == fixture_id)
    }

    pub fn fixture_mut(&mut self, fixture_id: usize) -> Option<&mut Fixture> {
        self.patchlist
            .fixtures
            .iter_mut()
            .find(|e| e.id == fixture_id)
    }

    pub fn fixtures(&self) -> &Vec<Fixture> {
        &self.patchlist.fixtures
    }

    pub fn selected_fixtures(&self) -> Vec<&Fixture> {
        self.programmer
            .selection
            .iter()
            .filter_map(|id| self.fixture(*id))
            .collect()
    }

    pub fn fixtures_in_group(&self, group_id: usize) -> Vec<&Fixture> {
        let Some(group) = self.group(group_id) else {
            return Vec::new();
        };

        group
            .fixtures
            .iter()
            .filter_map(|f| self.fixture(*f))
            .collect()
    }

    pub fn fixtures_in_groups(&self, group_ids: &[usize]) -> Vec<&Fixture> {
        group_ids
            .into_iter()
            .flat_map(|id| self.fixtures_in_group(*id))
            .collect()
    }

    pub fn is_fixture_selected(&self, id: usize) -> bool {
        self.programmer.selection.contains(&id)
    }

    pub fn are_fixtures_selected(&self, fixtures: &[usize]) -> bool {
        !fixtures
            .into_iter()
            .any(|id| !self.is_fixture_selected(*id))
    }

    pub fn fixture_exists(&self, id: usize) -> bool {
        self.fixture(id).is_some()
    }

    pub fn executor(&self, id: usize) -> Option<&Executor> {
        self.executors.iter().find(|e| e.id == id)
    }

    pub fn executor_mut(&mut self, id: usize) -> Option<&mut Executor> {
        self.executors.iter_mut().find(|e| e.id == id)
    }

    pub fn executors(&self) -> &Vec<Executor> {
        &self.executors
    }

    pub fn used_universes(&self) -> Vec<u16> {
        self.patchlist
            .fixtures
            .iter()
            .map(|f| f.channel.universe)
            .unique()
            .collect()
    }

    pub fn all_attributes(&self) -> Vec<&Attribute> {
        self.patchlist
            .fixtures
            .iter()
            .flat_map(|f| f.attributes())
            .collect()
    }

    pub fn attributes_with_channel(&self) -> Vec<&Attribute> {
        self.patchlist
            .fixtures
            .iter()
            .flat_map(|f| f.attributes_with_channels())
            .collect()
    }

    pub fn group(&self, group_id: usize) -> Option<&Group> {
        self.data.groups.iter().find(|g| g.id == group_id)
    }

    pub fn group_mut(&mut self, group_id: usize) -> Option<&mut Group> {
        self.data.groups.iter_mut().find(|g| g.id == group_id)
    }

    pub fn groups(&self) -> &Vec<Group> {
        &self.data.groups
    }

    pub fn sequence(&self, sequence_id: usize) -> Option<&Sequence> {
        self.data.sequences.iter().find(|s| s.id == sequence_id)
    }

    pub fn sequence_mut(&mut self, sequence_id: usize) -> Option<&mut Sequence> {
        self.data.sequences.iter_mut().find(|s| s.id == sequence_id)
    }

    pub fn sequences(&self) -> &Vec<Sequence> {
        &self.data.sequences
    }

    pub fn stage_output(&mut self) -> DmxOutput {
        let mut playback = self.playback_engine.determine_dmx_output(self);
        for universe in self.used_universes().iter() {
            if let Err(err) = playback.add_universe_if_absent(*universe) {
                log::error!("Failed to add universe with id '{universe}': {err}",)
            }
        }
        playback
    }

    pub fn stage_output_dmx_value_for_channel(&mut self, channel: DmxChannel) -> Option<u8> {
        // FIXME: We should cache the current stage output.
        self.stage_output().get_channel(channel)
    }

    pub fn send_stage_output_to_dmx_protocols(&mut self) {
        let dmx_output = self.stage_output();
        for dmx_protocol in self.dmx_protocols.iter() {
            dmx_protocol.send_dmx_output(&dmx_output);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Patchlist {
    fixtures: Vec<Fixture>,
    gdtf_descriptions: HashMap<i32, Rc<GdtfDescription>>,
}

impl Patchlist {
    pub fn new() -> Self {
        Self {
            fixtures: Vec::new(),
            gdtf_descriptions: HashMap::new(),
        }
    }

    pub fn patch_fixture(&mut self, fixture: Fixture) {
        self.fixtures.push(fixture);
    }

    pub fn register_gdtf_description(
        &mut self,
        rid: i32,
        gdtf_description: GdtfDescription,
    ) -> Rc<GdtfDescription> {
        let gdtf_description = Rc::new(gdtf_description);
        self.gdtf_descriptions.insert(rid, gdtf_description.clone());
        gdtf_description
    }

    pub fn get_gdtf_description(&self, rid: i32) -> Option<Rc<GdtfDescription>> {
        self.gdtf_descriptions.get(&rid).cloned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fixture {
    pub id: usize,
    pub label: String,
    pub description: Rc<GdtfDescription>,
    pub channel: DmxChannel,
    pub mode: String,
}

impl Fixture {
    pub fn r#type(&self) -> &FixtureType {
        &self.description.fixture_type
    }

    pub fn activation_groups(&self) -> &Vec<ActivationGroup> {
        &self.r#type().attribute_definitions.activation_groups
    }

    pub fn feature_groups(&self) -> &Vec<FeatureGroup> {
        &self.r#type().attribute_definitions.feature_groups
    }

    pub fn attributes(&self) -> &Vec<gdtf::Attribute> {
        &self.r#type().attribute_definitions.attributes
    }

    pub fn attribute_offset_for_current_mode(&self, attribute_name: &str) -> Option<Vec<i32>> {
        self.get_dmx_channels_using_attribute(attribute_name)
            .first()
            .and_then(|channel| channel.offset.clone())
    }

    pub fn get_dmx_channels_using_attribute(&self, attribute_name: &str) -> Vec<&gdtf::DmxChannel> {
        self.current_dmx_mode()
            .dmx_channels
            .iter()
            .filter(|channel| {
                channel
                    .all_channel_functions()
                    .iter()
                    .any(|cf| cf.attribute(self.attributes()).name == attribute_name)
            })
            .collect()
    }

    pub fn get_channel_functions_using_attribute(
        &self,
        attribute_name: &str,
    ) -> Vec<&gdtf::ChannelFunction> {
        self.current_dmx_mode()
            .all_channel_functions()
            .iter()
            .filter(|cf| cf.attribute(self.attributes()).name == attribute_name)
            .map(|cf| *cf)
            .collect()
    }

    pub fn current_dmx_mode(&self) -> &gdtf::DmxMode {
        self.r#type()
            .dmx_modes
            .iter()
            .find(|mode| mode.name == self.mode)
            .expect("fixture mode should always be set to a value in the GDTF file")
    }

    pub fn channel_resolution_for_attribute(&self, attribute_name: &str) -> Option<u8> {
        let Some(offset) = self
            .get_dmx_channels_using_attribute(attribute_name)
            .first()
            .and_then(|c| c.offset.clone())
        else {
            return None;
        };

        Some(offset.len().clamp(u8::MIN as usize, u8::MAX as usize) as u8)
    }

    fn attributes_with_channels(&self) -> Vec<&Attribute> {
        self.current_dmx_mode()
            .dmx_channels
            .iter()
            .flat_map(|channel| {
                if channel.offset.as_ref().is_some_and(|o| !o.is_empty()) {
                    Some(
                        channel
                            .logical_channels
                            .get(0)
                            .unwrap()
                            .attribute(self.attributes()),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn dmx_channels_for_attribute(&self, attribute_name: &str) -> Option<Vec<DmxChannel>> {
        let offset = self.attribute_offset_for_current_mode(attribute_name)?;

        let mut channels = vec![];
        for o in offset.iter() {
            // Because the offset in the GDTF files starts at 1, we need to
            // compensate for our zero-based array.
            let offset = o.saturating_sub(1);
            let address = self.channel.address + offset as u16;

            if let Ok(channel) = DmxChannel::new(self.channel.universe, address) {
                channels.push(channel);
            } else {
                return None;
            }
        }
        Some(channels)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Programmer {
    selection: Vec<usize>,
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
    pub current_index: Cell<Option<usize>>,
    pub r#loop: bool,
    pub flash: bool,
    pub fader_value: f32,
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
    Top,
    Flash,
}

impl std::fmt::Display for ExecutorButtonAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutorButtonAction::Go => write!(f, "Go"),
            ExecutorButtonAction::Top => write!(f, "Top"),
            ExecutorButtonAction::Flash => write!(f, "Flash"),
        }
    }
}

impl Executor {
    pub fn is_running(&self) -> bool {
        self.current_index.get().is_some()
    }

    pub fn go(&self, show: &Show) {
        match self.current_index.get() {
            None => self.go_to_cue(0, show),
            Some(index) => {
                let Some(sequence) = self.sequence(show) else {
                    log::error!("Sequence not found for Executor {}", self.id);
                    return;
                };

                if index + 1 < sequence.cues.len() {
                    self.current_index.set(Some(index + 1));
                } else {
                    if self.r#loop {
                        self.current_index.set(Some(0))
                    } else {
                        self.current_index.set(None)
                    }
                }
            }
        }
    }

    pub fn top(&self, show: &Show) {
        self.go_to_cue(0, show);
    }

    pub fn go_to_cue(&self, index: usize, show: &Show) {
        let Some(sequence) = self.sequence(show) else {
            log::error!("Sequence not found for Executor {}", self.id);
            return;
        };

        if index < sequence.cues.len() {
            self.current_index.set(Some(index));
        }
    }

    fn sequence<'a>(&'a self, show: &'a Show) -> Option<&Sequence> {
        let Some(id) = self.sequence else {
            return None;
        };
        show.sequence(id)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use gdtf_share::GdtfShare;

    use crate::showfile::Showfile;

    use super::Show;

    #[tokio::test]
    async fn from_empty_showfile() {
        dotenv::dotenv().ok();
        let user = env::var("GDTF_SHARE_API_USER").unwrap();
        let password = env::var("GDTF_SHARE_API_PASSWORD").unwrap();

        let showfile: Showfile = serde_json::from_str(r#"{}"#).unwrap();
        let gdtf_share = GdtfShare::auth(user, password).await.unwrap();

        Show::new(showfile, gdtf_share).await;
    }

    #[tokio::test]
    async fn from_showfile_with_fixture() {
        dotenv::dotenv().ok();
        let user = env::var("GDTF_SHARE_API_USER").unwrap();
        let password = env::var("GDTF_SHARE_API_PASSWORD").unwrap();

        let showfile: Showfile = serde_json::from_str(
            r#"{
            "patch_list": {
                "fixtures": [
                    {
                        "id": 420,
                        "gdtf_share_revision_id": 60124,
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
                        "gdtf_share_revision_id": 60124,
                        "mode": "Turned to 11",
                        "channel": {
                            "address": 5,
                            "universe": 8
                        }
                    }
                ]
            }
        }"#,
        )
        .unwrap();
        let gdtf_share = GdtfShare::auth(user, password).await.unwrap();

        let show = Show::new(showfile, gdtf_share).await;

        assert_eq!(
            show.patchlist.fixtures[0].description.fixture_type.id,
            "DB42C9F0-3236-4251-8436-D9CBE92F4021".to_string()
        );

        assert_eq!(show.patchlist.gdtf_descriptions.len(), 1);
    }

    #[tokio::test]
    async fn test_selecting_fixture_with_command() {
        dotenv::dotenv().ok();
        let user = env::var("GDTF_SHARE_API_USER").unwrap();
        let password = env::var("GDTF_SHARE_API_PASSWORD").unwrap();

        let showfile: Showfile = serde_json::from_str(
            r#"{
            "patch_list": {
                "fixtures": [
                    {
                        "id": 420,
                        "gdtf_share_revision_id": 60124,
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
                        "gdtf_share_revision_id": 60124,
                        "mode": "Turned to 11",
                        "channel": {
                            "address": 5,
                            "universe": 8
                        }
                    }
                ]
            }
        }"#,
        )
        .unwrap();
        let gdtf_share = GdtfShare::auth(user, password).await.unwrap();

        let mut show = Show::new(showfile, gdtf_share).await;
        show.execute_command_str("Select Fixture 420").unwrap();

        assert_eq!(*show.programmer.selection.first().unwrap(), 420);
    }
}

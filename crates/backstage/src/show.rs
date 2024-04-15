use std::cell::Cell;
use std::collections::HashMap;
use std::fs::File;
use std::rc::Rc;

use anyhow::{anyhow, Result};
use dmx::{DmxChannel, DmxOutput, DmxValue};
use gdtf::{Attribute, FeatureGroup, FixtureType, GdtfDescription};
use itertools::Itertools;

use crate::command::Command;
use crate::playback_engine::PlaybackEngine;
use crate::showfile::Showfile;
use crate::{Preset, Presets};

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub values: HashMap<usize, AttributeValues>,
}

impl Output {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn set(&mut self, fixture_id: &usize, attribute_name: &str, value: DmxValue) {
        if let Some(attribute_values) = self.values.get_mut(fixture_id) {
            attribute_values.insert(attribute_name.to_string(), value);
        } else {
            let mut attribute_values = AttributeValues::new();
            attribute_values.insert(attribute_name.to_string(), value);
            self.values.insert(*fixture_id, attribute_values);
        }
    }

    pub fn attribute_value(&self, fixture_id: &usize, attribute_name: &str) -> Option<&DmxValue> {
        self.values
            .get(fixture_id)
            .and_then(|values| values.get(attribute_name))
    }

    pub fn combine(&mut self, other: Output) {
        for (fixture_id, attribute_values) in other.values {
            for (attribute_name, value) in attribute_values {
                self.set(&fixture_id, &attribute_name, value);
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct Show {
    pub current_command: Option<Command>,

    pub(crate) patchlist: Patchlist,
    pub(crate) programmer: Programmer,
    pub(crate) playback_engine: PlaybackEngine,
    pub(crate) data: Data,
    pub(crate) presets: Presets,
    pub(crate) executors: Vec<Executor>,
    pub(crate) stage_output: Output,
}

impl Show {
    pub async fn from_file(file: File) -> Result<Self> {
        let reader = std::io::BufReader::new(file);
        let showfile: Showfile = serde_json::from_reader(reader)?;
        Self::from_showfile(showfile).await
    }

    pub async fn from_showfile_str(s: &str) -> Result<Self> {
        let showfile: Showfile = serde_json::from_str(s)?;
        showfile.try_into_show().await
    }

    pub(crate) async fn from_showfile(showfile: Showfile) -> Result<Self> {
        let mut show = showfile.try_into_show().await?;
        show.recalculate_stage_output();
        Ok(show)
    }

    pub fn to_json(self) -> Result<String> {
        serde_json::to_string_pretty(&Showfile::from(self))
            .map_err(|err| anyhow!("Failed to serialize showfile: {err}"))
    }

    pub fn execute_current_command(&mut self) -> Result<()> {
        if let Some(command) = self.current_command.as_ref().cloned() {
            self.current_command = None;
            self.execute_command(&command)
        } else {
            Err(anyhow!("No current command to execute"))
        }
    }

    pub fn execute_command_str(&mut self, s: &str) -> Result<()> {
        let command = Command::parse(s)?;
        self.execute_command(&command)?;
        Ok(())
    }

    pub fn apply_preset(&mut self, preset: &impl Preset) -> Result<()> {
        for fixture_id in self.selected_fixtures().to_vec() {
            let fixture = self.fixture(fixture_id).unwrap().clone();

            let values = preset
                .attribute_values()
                .iter()
                .filter(|(attribute_name, _)| fixture.has_attribute_with_name(attribute_name))
                .map(|(attribute_name, value)| (attribute_name.clone(), value.clone()))
                .collect();

            self.programmer
                .apply_attribute_values_for_fixture(fixture.id, values);
        }

        Ok(())
    }

    pub fn attribute_values_in_programmer(
        &self,
        attribute_values: &HashMap<String, DmxValue>,
    ) -> bool {
        self.programmer_changes()
            .values
            .iter()
            .any(|(fixture_id, changes)| {
                let fixture = self.fixture(*fixture_id).unwrap();
                let matching_attributes = attribute_values
                    .iter()
                    .filter(|(attribute_name, _)| {
                        fixture
                            .attributes()
                            .iter()
                            .any(|a| a.name == **attribute_name)
                    })
                    .collect::<Vec<_>>();

                if matching_attributes.is_empty() {
                    return false;
                }

                matching_attributes.iter().all(|(attribute_name, value)| {
                    match changes.get(*attribute_name) {
                        Some(existing_value) => existing_value == *value,
                        None => false,
                    }
                })
            })
    }

    pub fn feature_groups_in_selected_fixtures(&self) -> Vec<Rc<FeatureGroup>> {
        self.selected_fixtures()
            .iter()
            .filter_map(|fixture_id| {
                self.fixture(*fixture_id).map(|fixture| {
                    fixture
                        .description
                        .fixture_type
                        .attribute_definitions
                        .feature_groups
                        .clone()
                })
            })
            .flatten()
            .unique()
            .collect::<Vec<_>>()
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

    pub fn fixtures(&self) -> &[Fixture] {
        &self.patchlist.fixtures
    }

    pub fn selected_fixtures(&self) -> &[usize] {
        &self.programmer.selection
    }

    pub fn programmer_changes(&self) -> &Output {
        &self.programmer.changes
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
            .iter()
            .flat_map(|id| self.fixtures_in_group(*id))
            .collect()
    }

    pub fn is_fixture_selected(&self, id: usize) -> bool {
        self.programmer.selection.contains(&id)
    }

    pub fn are_fixtures_selected(&self, fixtures: &[usize]) -> bool {
        !fixtures.iter().any(|id| !self.is_fixture_selected(*id))
    }

    pub fn fixture_exists(&self, id: usize) -> bool {
        self.fixture(id).is_some()
    }

    pub fn group(&self, group_id: usize) -> Option<&Group> {
        self.data.groups.iter().find(|g| g.id == group_id)
    }

    pub fn group_mut(&mut self, group_id: usize) -> Option<&mut Group> {
        self.data.groups.iter_mut().find(|g| g.id == group_id)
    }

    pub fn groups(&self) -> &[Group] {
        &self.data.groups
    }

    pub fn sequence(&self, sequence_id: usize) -> Option<&Sequence> {
        self.data.sequences.iter().find(|s| s.id == sequence_id)
    }

    pub fn sequence_mut(&mut self, sequence_id: usize) -> Option<&mut Sequence> {
        self.data.sequences.iter_mut().find(|s| s.id == sequence_id)
    }

    pub fn sequences(&self) -> &[Sequence] {
        &self.data.sequences
    }

    pub fn cue(&self, sequence_id: usize, cue_ix: usize) -> Option<&Cue> {
        self.sequence(sequence_id).and_then(|s| s.cues.get(cue_ix))
    }

    pub fn cue_mut(&mut self, sequence_id: usize, cue_ix: usize) -> Option<&mut Cue> {
        self.sequence_mut(sequence_id)
            .and_then(|s| s.cues.get_mut(cue_ix))
    }

    pub fn executor(&self, id: usize) -> Option<&Executor> {
        self.executors.iter().find(|e| e.id == id)
    }

    pub fn executor_mut(&mut self, id: usize) -> Option<&mut Executor> {
        self.executors.iter_mut().find(|e| e.id == id)
    }

    pub fn executors(&self) -> &[Executor] {
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

    pub fn all_attributes(&self) -> Vec<&Rc<Attribute>> {
        self.patchlist
            .fixtures
            .iter()
            .flat_map(|f| f.attributes())
            .collect()
    }

    pub fn stage_output(&self) -> &Output {
        &self.stage_output
    }

    pub fn default_stage_output(&self) -> Output {
        let mut output = Output::new();
        for fixture in self.fixtures().iter() {
            for attribute in fixture.attributes().iter() {
                output.set(&fixture.id, &attribute.name, DmxValue::new(0));
            }
        }
        output
    }

    pub fn recalculate_stage_output(&mut self) {
        let mut stage_output = self.default_stage_output();
        stage_output.combine(self.playback_engine.determine_output(self));
        stage_output.combine(self.programmer_changes().clone());
        self.stage_output = stage_output;
    }

    pub(crate) fn first_free_sequence_id(&self) -> usize {
        self.data.sequences.iter().map(|s| s.id).max().unwrap_or(0) + 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Patchlist {
    pub(crate) fixtures: Vec<Fixture>,
    pub(crate) gdtf_descriptions: HashMap<i32, Rc<GdtfDescription>>,
}

impl Default for Patchlist {
    fn default() -> Self {
        Self::new()
    }
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
    pub rid: i32,
}

impl Fixture {
    pub fn r#type(&self) -> &FixtureType {
        &self.description.fixture_type
    }

    pub fn attributes(&self) -> &[Rc<gdtf::Attribute>] {
        &self.r#type().attribute_definitions.attributes
    }

    fn has_attribute_with_name(&self, attribute_name: &str) -> bool {
        self.attributes()
            .iter()
            .any(|attribute| attribute.name == attribute_name)
    }

    pub fn attributes_for_feature_group(
        &self,
        feature_group_name: &str,
    ) -> Vec<&Rc<gdtf::Attribute>> {
        self.attributes()
            .iter()
            .filter(|attribute| attribute.feature_group.name == feature_group_name)
            .collect()
    }

    pub fn attributes_for_feature(
        &self,
        feature_group_name: &str,
        feature_name: &str,
    ) -> Vec<&Rc<Attribute>> {
        self.attributes()
            .iter()
            .filter(|attribute| {
                attribute.feature.name == feature_name
                    && attribute.feature_group.name == feature_group_name
            })
            .collect()
    }

    pub fn attribute_offset_for_current_mode(&self, attribute_name: &str) -> Option<Vec<i32>> {
        self.dmx_channels_using_attribute(attribute_name)
            .first()
            .and_then(|channel| channel.offset.clone())
    }

    pub fn dmx_channels_using_attribute(&self, attribute_name: &str) -> Vec<&gdtf::DmxChannel> {
        self.current_dmx_mode()
            .dmx_channels
            .iter()
            .filter(|channel| {
                channel
                    .initial_function
                    .attribute
                    .as_ref()
                    .map(|f| f.name.as_str())
                    == Some(attribute_name)
            })
            .collect()
    }

    pub fn current_dmx_mode(&self) -> &gdtf::DmxMode {
        self.r#type()
            .dmx_modes
            .iter()
            .find(|mode| mode.name == self.mode)
            .expect("Fixture mode should always be set to a value in the GDTF file")
    }

    pub fn channel_resolution_for_attribute(&self, attribute_name: &str) -> Option<u8> {
        let offset = self
            .dmx_channels_using_attribute(attribute_name)
            .first()
            .and_then(|c| c.offset.clone())?;

        Some(offset.len().clamp(u8::MIN as usize, u8::MAX as usize) as u8 * 8)
    }

    pub fn dmx_channels_for_attribute(&self, attribute_name: &str) -> Option<Vec<DmxChannel>> {
        let offset = self.attribute_offset_for_current_mode(attribute_name)?;

        let mut channels = vec![];
        for o in offset.iter() {
            if let Ok(channel) = self.dmx_channel_from_offset(*o) {
                channels.push(channel);
            } else {
                return None;
            }
        }
        Some(channels)
    }

    fn dmx_channel_from_offset(&self, offset: i32) -> Result<DmxChannel> {
        // Because the offset in the GDTF files starts at 1, we need to
        // compensate for our zero-based array.
        let offset = offset.saturating_sub(1);
        let address = self.channel.address + offset as u16;

        DmxChannel::new(self.channel.universe, address)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Programmer {
    pub(crate) selection: Vec<usize>,
    pub(crate) changes: Output,
}

impl Programmer {
    pub fn apply_attribute_values_for_fixture(
        &mut self,
        fixture_id: usize,
        attribute_values: AttributeValues,
    ) {
        self.changes.values.insert(fixture_id, attribute_values);
    }

    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    pub fn clear_changes(&mut self) {
        self.changes.values.clear();
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }
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
    pub label: String,
    pub changes: Output,
}

pub type AttributeValues = HashMap<String, DmxValue>;

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

impl Executor {
    pub fn new(id: usize, sequence: Option<usize>) -> Self {
        Self {
            id,
            sequence,
            current_index: Cell::new(None),
            r#loop: false,
            flash: false,
            fader_value: 1.0,
            button_1: ExecutorButton {
                action: crate::ExecutorButtonAction::Top,
            },
            button_2: ExecutorButton {
                action: crate::ExecutorButtonAction::Go,
            },
            button_3: ExecutorButton {
                action: crate::ExecutorButtonAction::Flash,
            },
        }
    }
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
                    log::error!("Sequence not found for executor {}", self.id);
                    return;
                };

                if index + 1 < sequence.cues.len() {
                    self.current_index.set(Some(index + 1));
                } else if self.r#loop {
                    self.current_index.set(Some(0))
                } else {
                    self.current_index.set(None)
                }
            }
        }
    }

    pub fn top(&self, show: &Show) {
        self.go_to_cue(0, show);
    }

    pub fn go_to_cue(&self, index: usize, show: &Show) {
        let Some(sequence) = self.sequence(show) else {
            log::error!("Sequence not found for executor {}", self.id);
            return;
        };

        if index < sequence.cues.len() {
            self.current_index.set(Some(index));
        }
    }

    fn sequence<'a>(&'a self, show: &'a Show) -> Option<&Sequence> {
        let id = self.sequence?;
        show.sequence(id)
    }
}

pub fn update_dmx_output_with_attribute_values(
    fixture: &Fixture,
    attribute_values: &HashMap<String, DmxValue>,
    output: &mut DmxOutput,
) {
    for (attribute_name, attribute_value) in attribute_values.iter() {
        let Some(dmx_channels) = fixture.dmx_channels_for_attribute(attribute_name) else {
            continue;
        };

        let Some(channel_resolution) = fixture.channel_resolution_for_attribute(attribute_name)
        else {
            continue;
        };

        let raw_dmx_values = attribute_value.raw_values_for_channel_resolution(channel_resolution);

        for (channel, value) in dmx_channels.iter().zip(raw_dmx_values) {
            if let Err(err) = output.set_channel(channel, value) {
                log::error!("Failed to set channel output: {}", err.to_string())
            }
        }
    }
}

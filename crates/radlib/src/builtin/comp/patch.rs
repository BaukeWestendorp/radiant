use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
use std::{fmt, str};

use eyre::{Context, ContextCompat};
use gdtf::GdtfFile;
use gdtf::attribute::FeatureGroup;
use gdtf::dmx_mode::DmxMode;
use gdtf::fixture_type::FixtureType;
use uuid::Uuid;

use crate::attr::{Attribute, AttributeValue};
use crate::comp::Component;
use crate::engine::Engine;
use crate::error::Result;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    engine.register_component::<Patch>()?;
    Ok(())
}

#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Patch {
    content: PatchContent,
    editable_content: Option<PatchContent>,
}

impl Patch {
    pub fn is_edited(&self) -> bool {
        Some(&self.content) == self.editable_content.as_ref()
    }

    pub(crate) fn start_edit(&mut self) {
        self.editable_content = Some(self.content.clone());
    }

    pub(crate) fn save_edit(&mut self) -> Result<()> {
        let Some(editable_content) = &self.editable_content else { return Ok(()) };

        if !editable_content.validate() {
            eyre::bail!("patch content is not valid");
        }

        self.content = editable_content.clone();

        self.editable_content = None;

        Ok(())
    }

    pub(crate) fn discard_edit(&mut self) {
        self.editable_content = None;
    }

    pub(crate) fn fixture_mut(
        &mut self,
        fixture_ref: impl Into<FixtureReference>,
    ) -> Result<Option<&mut Fixture>> {
        let Some(editable_content) = &mut self.editable_content else {
            eyre::bail!("patch is not in edit mode");
        };

        let fixture_ref = fixture_ref.into();
        let fixture = editable_content.fixtures.iter_mut().find(|f| match fixture_ref {
            FixtureReference::FixtureId(fid) => f.fid == Some(fid),
            FixtureReference::Uuid(uuid) => f.uuid() == uuid,
            FixtureReference::Address(address) => f.address == Some(address),
        });

        Ok(fixture)
    }

    pub(crate) fn add_fixture(&mut self, fixture: Fixture) -> Result<()> {
        if self.editable_content.is_none() {
            eyre::bail!("patch is not in edit mode");
        }

        if let Some(fid) = fixture.fid
            && let Some(fixture) = self.fixture_mut(fid)?
        {
            fixture.fid = None;
        }

        let Some(fixture_type) = self.fixture_types.get(&fixture.fixture_type_id) else {
            eyre::bail!(
                "fixture type with id '{}' not found",
                fixture.fid.map_or("None".to_string(), |f| f.to_string())
            );
        };

        if fixture_type.dmx_mode(&fixture.dmx_mode).is_none() {
            eyre::bail!(
                "DMX mode with name '{}' on fixture type '{}' not found",
                fixture.dmx_mode,
                fixture_type.long_name
            );
        }

        let Some(editable_content) = &mut self.editable_content else {
            eyre::bail!("patch is not in edit mode");
        };

        editable_content.fixtures.push(fixture);

        Ok(())
    }

    pub(crate) fn remove_fixture(
        &mut self,
        fixture_ref: impl Into<FixtureReference>,
    ) -> Result<()> {
        let Some(editable_content) = &mut self.editable_content else {
            eyre::bail!("patch is not in edit mode");
        };

        let fixture_ref = fixture_ref.into();
        editable_content.fixtures.retain(|f| match fixture_ref {
            FixtureReference::FixtureId(fid) => f.fid != Some(fid),
            FixtureReference::Uuid(uuid) => f.uuid() != uuid,
            FixtureReference::Address(address) => f.address != Some(address),
        });

        Ok(())
    }
}

impl Deref for Patch {
    type Target = PatchContent;

    fn deref(&self) -> &Self::Target {
        match &self.editable_content {
            Some(editable_content) => editable_content,
            None => &self.content,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PatchContent {
    #[serde(skip, default)]
    fixture_types: HashMap<GdtfFixtureTypeId, FixtureType>,

    #[serde(default)]
    fixtures: Vec<Fixture>,
}

impl PatchContent {
    pub fn fixture_type(&self, fixture_type_id: &GdtfFixtureTypeId) -> Option<&FixtureType> {
        self.fixture_types.get(fixture_type_id)
    }

    pub fn fixture_types(&self) -> &HashMap<GdtfFixtureTypeId, FixtureType> {
        &self.fixture_types
    }

    pub fn fixture(&self, fixture_ref: impl Into<FixtureReference>) -> Option<&Fixture> {
        let fixture_ref = fixture_ref.into();
        self.fixtures.iter().find(|f| match fixture_ref {
            FixtureReference::FixtureId(fid) => f.fid == Some(fid),
            FixtureReference::Uuid(uuid) => f.uuid() == uuid,
            FixtureReference::Address(address) => f.address != Some(address),
        })
    }

    pub fn validate(&self) -> bool {
        self.fixtures().iter().all(|f| f.validate())
    }

    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    pub fn fixture_ids(&self) -> impl IntoIterator<Item = &FixtureId> {
        self.fixtures.iter().filter_map(|f| f.fid.as_ref())
    }

    pub fn has_fixture(&self, fixture_ref: impl Into<FixtureReference>) -> bool {
        let fixture_ref = fixture_ref.into();
        self.fixtures.iter().any(|f| match fixture_ref {
            FixtureReference::FixtureId(fid) => f.fid == Some(fid),
            FixtureReference::Uuid(uuid) => f.uuid() == uuid,
            FixtureReference::Address(address) => f.address == Some(address),
        })
    }

    pub fn first_unbounded_address(&self) -> dmx::Address {
        let mut fixtures = self.fixtures.clone();
        fixtures.retain(|f| f.address.is_some());
        fixtures.sort_by(|a, b| a.address.cmp(&b.address));

        let Some(last_fixture) = fixtures.last() else {
            return dmx::Address::default();
        };

        let channel_count = crate::gdtf::channel_count(last_fixture.dmx_mode(self));

        last_fixture
            .address
            .expect("fixture should have an address as we just removed all fixtures without them")
            .with_channel_offset(channel_count)
    }
}

impl Component for Patch {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "patch.yaml"
    }

    fn after_load_from_showfile(&mut self, showfile_path: &Path) -> Result<()> {
        const GDTF_FOLDER: &str = "gdtf_files";
        let path = showfile_path.join(GDTF_FOLDER);

        let entries = path.read_dir().wrap_err_with(|| {
            format!("failed to read gdtf_files folder at path: {}", path.display())
        })?;

        for entry in entries {
            let entry = entry.wrap_err_with(|| {
                format!("failed to read directory entry in folder: {}", path.display())
            })?;

            if !entry.file_name().as_os_str().to_string_lossy().ends_with(".gdtf") {
                continue;
            }

            let file = File::open(entry.path())
                .wrap_err_with(|| format!("failed to open gdtf file {}", entry.path().display()))?;
            let gdtf_file = GdtfFile::new(file)
                .wrap_err_with(|| format!("failed to read gdtf file {}", entry.path().display()))?;

            self.start_edit();
            for fixture_type in gdtf_file.description.fixture_types {
                self.editable_content
                    .as_mut()
                    .expect("patch should be in edit mode")
                    .fixture_types
                    .insert(fixture_type.fixture_type_id, fixture_type);
            }
            self.save_edit()?;
        }

        Ok(())
    }

    fn save_to_showfile(&self, showfile_path: &Path) -> Result<()> {
        let file_path = showfile_path.join(Self::relative_file_path());
        let mut file = File::create(&file_path)?;
        let yaml = serde_yaml::to_string(self)?;
        file.write_all(yaml.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    #[serde(default = "Uuid::new_v4")]
    uuid: Uuid,

    pub name: String,
    pub fid: Option<FixtureId>,
    pub fixture_type_id: GdtfFixtureTypeId,
    pub address: Option<dmx::Address>,
    pub dmx_mode: String,
}

impl Fixture {
    pub fn new(
        fid: Option<FixtureId>,
        fixture_type_id: GdtfFixtureTypeId,
        address: Option<dmx::Address>,
        dmx_mode: String,
        name: String,
    ) -> Self {
        Self { uuid: Uuid::new_v4(), fid, fixture_type_id, address, dmx_mode, name }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn fixture_type<'a>(&self, patch: &'a PatchContent) -> &'a FixtureType {
        patch
            .fixture_types
            .get(&self.fixture_type_id)
            .expect("every fixture should have a valid GDTF Fixture Type")
    }

    pub fn feature_groups<'a>(&self, patch: &'a PatchContent) -> &'a [FeatureGroup] {
        &self.fixture_type(patch).attribute_definitions.feature_groups
    }

    pub fn dmx_mode<'a>(&self, patch: &'a PatchContent) -> &'a DmxMode {
        self.fixture_type(patch)
            .dmx_mode(&self.dmx_mode)
            .expect("fixture should always have a valid dmx mode index")
    }

    pub fn has_attribute(&self, attribute: &Attribute, patch: &PatchContent) -> bool {
        let fixture_type = self.fixture_type(patch);
        let dmx_mode = fixture_type.dmx_mode(&self.dmx_mode);
        if dmx_mode.is_none() {
            return false;
        }
        let dmx_mode = dmx_mode.unwrap();
        for dmx_channel in &dmx_mode.dmx_channels {
            for logical_channel in &dmx_channel.logical_channels {
                if let Some(attr) = logical_channel.attribute(fixture_type)
                    && let Some(name) = attr.name.as_ref()
                    && **name == attribute.to_string()
                {
                    return true;
                }
                for channel_function in &logical_channel.channel_functions {
                    if let Some(attr) = channel_function.attribute(fixture_type)
                        && let Some(name) = attr.name.as_ref()
                        && **name == attribute.to_string()
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_channel_values(
        &self,
        attribute: &Attribute,
        value: &AttributeValue,
        patch: &PatchContent,
    ) -> Result<Vec<(dmx::Channel, dmx::Value)>> {
        let channels = self.channels_for_attribute(attribute, patch)?;
        let mut values = Vec::new();
        for (ix, channel) in channels.into_iter().enumerate() {
            let int_value = (value.as_f32() * u32::MAX as f32) as u32;
            let bytes: [u8; 4] = int_value.to_be_bytes();
            let value = dmx::Value(bytes[ix]);
            values.push((channel, value));
        }
        Ok(values)
    }

    /// Returns attributes supported by this fixture along with
    /// their corresponding default [AttributeValue]s as
    /// defined in the fixture's GDTF definition.
    pub fn get_default_attribute_values(
        &self,
        patch: &PatchContent,
    ) -> Vec<(Attribute, AttributeValue)> {
        let fixture_type = self.fixture_type(patch);

        let mut values = Vec::new();
        for dmx_channel in &self.dmx_mode(patch).dmx_channels {
            let Some((_, channel_function)) = dmx_channel.initial_function() else {
                continue;
            };
            let Some(mut attribute) = channel_function.attribute(fixture_type) else {
                continue;
            };
            if let Some(main_attribute) =
                attribute.main_attribute(&fixture_type.attribute_definitions)
            {
                attribute = main_attribute;
            };

            let Some(attribute_name) = attribute.name.as_ref() else { continue };
            let attribute = Attribute::from_str(attribute_name).unwrap();

            values.push((attribute, channel_function.default.into()));
        }
        values
    }

    /// Get the [dmx::Channel]s for a given attribute on this fixture.
    pub fn channels_for_attribute(
        &self,
        attribute: &Attribute,
        patch: &PatchContent,
    ) -> Result<Vec<dmx::Channel>> {
        let Some(address) = self.address else {
            eyre::bail!("fixture does not have an address");
        };

        let dmx_channel = &self
            .dmx_mode(patch)
            .dmx_channels
            .iter()
            .find(|dmx_channel| {
                dmx_channel.logical_channels.iter().any(|logical_channel| {
                    let fixture_type = self.fixture_type(patch);
                    if logical_channel.attribute(fixture_type).is_some_and(|attr| {
                        attr.name.as_ref().is_some_and(|name| **name == attribute.to_string())
                    }) {
                        true
                    } else {
                        logical_channel.channel_functions.iter().any(|channel_function| {
                            channel_function.attribute(fixture_type).is_some_and(|attr| {
                                attr.name
                                    .as_ref()
                                    .is_some_and(|name| **name == attribute.to_string())
                            })
                        })
                    }
                })
            })
            .wrap_err_with(|| format!("channel not found for attribute {attribute}"))?;

        let offsets = dmx_channel
            .offset
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|offset| (offset - 1).clamp(u16::MIN as i32, u16::MAX as i32) as u16);

        let channels = offsets
            .map(|offset| {
                dmx::Channel::new(u16::from(address.channel) + offset)
                    .expect("channel should always be in range of universe")
            })
            .collect();

        Ok(channels)
    }

    pub fn validate(&self) -> bool {
        self.fid.is_some() && self.address.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureId(pub NonZeroU32);

impl Default for FixtureId {
    fn default() -> Self {
        Self(NonZeroU32::new(1).unwrap())
    }
}

impl From<FixtureId> for u32 {
    fn from(value: FixtureId) -> Self {
        value.0.into()
    }
}

impl fmt::Display for FixtureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for FixtureId {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let id = s.parse::<u32>()?;
        let nonzero =
            NonZeroU32::new(id).ok_or_else(|| eyre::eyre!("FixtureId must be non-zero"))?;
        Ok(FixtureId(nonzero))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixtureReference {
    FixtureId(FixtureId),
    Uuid(Uuid),
    Address(dmx::Address),
}

impl fmt::Display for FixtureReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixtureReference::FixtureId(fixture_id) => write!(f, "{fixture_id}"),
            FixtureReference::Uuid(uuid) => write!(f, "{uuid}"),
            FixtureReference::Address(address) => write!(f, "{address}"),
        }
    }
}

impl From<FixtureId> for FixtureReference {
    fn from(fid: FixtureId) -> Self {
        Self::FixtureId(fid)
    }
}

impl From<Uuid> for FixtureReference {
    fn from(uuid: Uuid) -> Self {
        Self::Uuid(uuid)
    }
}

impl From<dmx::Address> for FixtureReference {
    fn from(address: dmx::Address) -> Self {
        Self::Address(address)
    }
}

pub type GdtfFixtureTypeId = uuid::Uuid;

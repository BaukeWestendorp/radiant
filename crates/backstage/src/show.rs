//! # Show
//!
//! The show module contains the show struct and its sub-structs.

use crate::dmx::{self, DmxChannel, DmxOutput};
use gdtf::{Attribute, DmxMode, Feature, FeatureGroup, GdtfDescription};
use gdtf_share::GdtfShare;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display, io::Write, path::PathBuf, rc::Rc};

lazy_static! {
    static ref BASE_DIRS: xdg::BaseDirectories = xdg::BaseDirectories::new().unwrap();
    static ref FIXTURE_CACHE_PATH: PathBuf = {
        match std::env::var("BACKSTAGE_FIXTURE_CACHE_LOCATION") {
            Ok(path) => PathBuf::from(path),
            Err(_) => BASE_DIRS
                .place_cache_file("radiant/fixtures")
                .map_err(|_| Error::GdtfFileCacheFailed)
                .expect("Failed to get fixture cache path"),
        }
    };
}

/// An id that uniquely identifies a fixture.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct FixtureId(usize);

impl FixtureId {
    /// Create a new fixture id.
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl Display for FixtureId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The revision id of a GDTF file.
pub type RevisionId = i32;

/// # Show
///
/// The show struct contains all information related to a show.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Show {
    patchlist: Patchlist,
    programmer: Programmer,
    data: Data,
    selected_fixtures: Vec<FixtureId>,
}

impl Show {
    /// Create a new show.
    pub fn new() -> Self {
        Self {
            patchlist: Patchlist::new(),
            programmer: Programmer::new(),
            data: Data::new(),
            selected_fixtures: Vec::new(),
        }
    }

    /// Initialize the show. This will get all GDTF descriptions from the GDTF share.
    pub async fn initialize(
        &mut self,
        gdtf_share_username: String,
        gdtf_share_password: String,
    ) -> Result<(), Error> {
        self.patchlist
            .initialize(gdtf_share_username, gdtf_share_password)
            .await?;
        Ok(())
    }

    /// Get the patchlist.
    pub fn patchlist(&self) -> &Patchlist {
        &self.patchlist
    }

    /// Get the patchlist mutably.
    pub fn patchlist_mut(&mut self) -> &mut Patchlist {
        &mut self.patchlist
    }

    /// Get the programmer.
    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    /// Get the programmer mutably.
    pub fn programmer_mut(&mut self) -> &mut Programmer {
        &mut self.programmer
    }

    /// Get the data collection.
    pub fn data(&self) -> &Data {
        &self.data
    }

    /// Get the calculated DMX output for the current show state.
    pub fn get_dmx_output(&self) -> &DmxOutput {
        self.programmer().get_dmx_output()
    }

    /// Get the selected fixtures.
    pub fn selected_fixtures(&self) -> Vec<&Fixture> {
        self.selected_fixtures
            .iter()
            .map(|fixture_id| self.patchlist().fixture(&fixture_id).unwrap())
            .collect()
    }

    /// Get the id's of the selected fixtures
    pub fn selected_fixture_ids(&self) -> &[FixtureId] {
        &self.selected_fixtures
    }

    /// Add a fixture id to the currently selected fixtures.
    pub fn select_fixture(&mut self, fixture_id: FixtureId) {
        if !self.selected_fixtures.contains(&fixture_id) {
            self.selected_fixtures.push(fixture_id);
        }
    }

    /// Remove all fixtures from the selection.
    pub fn clear_selection(&mut self) {
        self.selected_fixtures.clear();
    }
}

impl<'de> serde::Deserialize<'de> for Show {
    fn deserialize<D>(deserializer: D) -> Result<Show, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let intermediate = ShowIntermediate::deserialize(deserializer)?;
        intermediate.try_into().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct ShowIntermediate {
    #[serde(default = "Default::default")]
    patchlist: PatchlistIntermediate,
    #[serde(default = "Default::default")]
    programmer: Programmer,
    #[serde(default = "Default::default")]
    data: Data,
    #[serde(default = "Default::default")]
    selected_fixtures: Vec<FixtureId>,
}

impl TryInto<Show> for ShowIntermediate {
    type Error = Error;

    fn try_into(self) -> Result<Show, Error> {
        Ok(Show {
            patchlist: self.patchlist.try_into()?,
            programmer: self.programmer,
            data: self.data,
            selected_fixtures: self.selected_fixtures,
        })
    }
}

/// # Patchlist
///
/// The patchlist struct contains all information about the fixtures in the show.
#[derive(Debug, Clone, Default)]
pub struct Patchlist {
    fixtures: Vec<Fixture>,
    gdtf_descriptions: HashMap<RevisionId, Rc<GdtfDescription>>,
    gdtf_share: Option<GdtfShare>,
}

impl PartialEq for Patchlist {
    fn eq(&self, other: &Self) -> bool {
        self.fixtures == other.fixtures && self.gdtf_descriptions == other.gdtf_descriptions
    }
}

impl Patchlist {
    /// Create a new patchlist.
    pub fn new() -> Self {
        Self {
            fixtures: Vec::new(),
            gdtf_descriptions: HashMap::new(),
            gdtf_share: None,
        }
    }

    pub(crate) async fn initialize(
        &mut self,
        gdtf_share_username: String,
        gdtf_share_password: String,
    ) -> Result<(), Error> {
        self.authenticate_gdtf_share(gdtf_share_username, gdtf_share_password)
            .await
            .ok();

        for fixture in self.fixtures.clone() {
            self.patch_fixture(
                fixture.id,
                fixture.label,
                fixture.revision_id,
                fixture.channel,
                fixture.mode,
            )
            .await?;
        }

        Ok(())
    }

    pub(crate) async fn authenticate_gdtf_share(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        let gdtf_share = GdtfShare::new(username, password);
        gdtf_share.auth().await?;
        self.gdtf_share = Some(gdtf_share);
        Ok(())
    }

    /// Get the fixtures.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    /// Get a fixture with the given id.
    pub fn fixture(&self, id: &FixtureId) -> Option<&Fixture> {
        self.fixtures().iter().find(|fixture| fixture.id == *id)
    }

    /// Patch a new fixture into the patchlist.
    ///
    /// # Errors
    ///
    /// This function can error if the GDTF file cannot be downloaded or if the GDTF file is invalid.
    pub async fn patch_fixture(
        &mut self,
        id: FixtureId,
        label: String,
        revision_id: RevisionId,
        channel: DmxChannel,
        mode: String,
    ) -> Result<(), Error> {
        let description = self.get_gdtf_description(revision_id).await?;

        let new_fixture = Fixture {
            id,
            label,
            revision_id,
            description: Some(description),
            channel,
            mode,
        };

        if let Some(fixture) = self.fixtures.iter_mut().find(|f| f.id == id) {
            *fixture = new_fixture;
        } else {
            self.fixtures.push(new_fixture);
        }

        Ok(())
    }

    async fn get_gdtf_description(
        &mut self,
        revision_id: RevisionId,
    ) -> Result<Rc<GdtfDescription>, Error> {
        if let Some(description) = self.gdtf_descriptions.get(&revision_id) {
            return Ok(description.clone());
        }

        let cached_file_path = FIXTURE_CACHE_PATH.join(format!("{}.gdtf", revision_id));

        let cached_description = match std::fs::read(&cached_file_path) {
            Ok(cached_file) => {
                let cached_description = GdtfDescription::from_archive_bytes(&cached_file)
                    .map_err(|err| Error::GdtfFileInvalid(err.to_string()))?;
                log::info!("Using cached GDTF file '{}'", cached_file_path.display());
                Some(cached_description)
            }
            _ => None,
        };

        let description = match cached_description {
            Some(cached_description) => cached_description,
            None => {
                let gdtf_share = match self.gdtf_share {
                    Some(ref gdtf_share) => gdtf_share,
                    None => return Err(Error::GdtfShareNotAuthenticated),
                };

                let description_file = gdtf_share.download_file(revision_id).await?;
                let reader = std::io::Cursor::new(description_file.clone());
                let description = GdtfDescription::from_archive_reader(reader)
                    .map_err(|err| Error::GdtfFileInvalid(err.to_string()))?;

                let mut file = std::fs::File::create_new(cached_file_path.clone())
                    .map_err(|_| Error::GdtfFileCacheFailed)?;
                file.write_all(&description_file)
                    .map_err(|_| Error::GdtfFileCacheFailed)?;
                log::info!("Cached GDTF file '{}'", cached_file_path.display());
                description
            }
        };

        let description = Rc::new(description);
        self.gdtf_descriptions
            .insert(revision_id, description.clone());

        Ok(description)
    }
}

impl<'de> serde::Deserialize<'de> for Patchlist {
    fn deserialize<D>(deserializer: D) -> Result<Patchlist, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let intermediate = PatchlistIntermediate::deserialize(deserializer)?;
        intermediate.try_into().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
struct PatchlistIntermediate {
    pub fixtures: Vec<FixtureIntermediate>,
}

impl TryInto<Patchlist> for PatchlistIntermediate {
    type Error = Error;

    fn try_into(self) -> Result<Patchlist, Error> {
        let mut patchlist = Patchlist::new();
        for fixture in self.fixtures {
            patchlist.fixtures.push(Fixture {
                id: fixture.id,
                label: fixture.label,
                revision_id: fixture.revision_id,
                description: None,
                channel: fixture.channel,
                mode: fixture.mode,
            })
        }
        Ok(patchlist)
    }
}

/// # Fixture
///
/// The fixture struct contains all information about a fixture in the show.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fixture {
    /// The fixture ID.
    pub id: FixtureId,
    /// A custom label for the fixture.
    pub label: String,
    /// The revision id of the fixture. This should correspond to the revision id of the GDTF description.
    pub revision_id: i32,
    /// The GDTF description of the fixture. This is `None` if the show has not been initialized yet.
    description: Option<Rc<GdtfDescription>>,
    /// The channel on which the fixture is patched.
    pub channel: DmxChannel,
    /// The DMX mode used. The mode must be one of the modes in the GDTF description.
    pub mode: String,
}

impl Fixture {
    /// Get the GDTF description of the fixture.
    ///
    /// # Panics
    ///
    /// This function will panic if the show has not been initialized yet.
    pub fn description(&self) -> Rc<GdtfDescription> {
        self.description.clone().unwrap()
    }

    /// Get all feature groups of the fixture type.
    ///
    /// # Panics
    ///
    /// This function will panic if the show has not been initialized yet.
    pub fn feature_groups(&self) -> &[Rc<FeatureGroup>] {
        &self
            .description
            .as_ref()
            .expect("Show not initialized")
            .fixture_type
            .attribute_definitions
            .feature_groups
    }

    /// Get a feature group by name.
    ///
    /// # Panics
    ///
    /// This function will panic if the show has not been initialized yet.
    pub fn feature_group(&self, feature_group_name: &str) -> Option<&Rc<FeatureGroup>> {
        self.feature_groups()
            .iter()
            .find(|fg| fg.name == *feature_group_name)
    }

    /// Get all attributes of the fixture type.
    ///
    /// # Panics
    ///
    /// This function will panic if the show has not been initialized yet.
    pub fn attributes(&self) -> &[Rc<Attribute>] {
        &self
            .description
            .as_ref()
            .expect("Show not initialized")
            .fixture_type
            .attribute_definitions
            .attributes
    }

    /// Get all attributes for the current mode of the fixture.
    ///
    /// # Panics
    ///
    /// This function will panic if the show has not been initialized yet.
    pub fn attributes_for_current_mode(&self) -> Vec<&Rc<Attribute>> {
        self.dmx_mode()
            .dmx_channels
            .iter()
            .flat_map(|channel| {
                channel
                    .logical_channels
                    .iter()
                    .filter_map(|logical_channel| self.attribute(&logical_channel.attribute.name))
            })
            .collect()
    }

    /// Get an attribute by name.
    ///
    /// # Panics
    ///
    /// This function will panic if the show has not been initialized yet.
    pub fn attribute(&self, attribute_name: &str) -> Option<&Rc<Attribute>> {
        self.description
            .as_ref()
            .expect("Show not initialized")
            .fixture_type
            .attribute_definitions
            .attributes
            .iter()
            .find(|attribute| attribute.name == *attribute_name)
    }

    /// Get all attributes for a feature in the current mode of the fixture.
    ///
    /// # Panics
    ///
    /// This function will panic if the show has not been initialized yet.
    pub fn attributes_for_feature_in_current_mode(&self, feature: &Feature) -> Vec<&Rc<Attribute>> {
        self.attributes_for_current_mode()
            .iter()
            .filter(|attribute| *attribute.feature == *feature)
            .cloned()
            .collect()
    }

    /// Get the current DMX mode of the fixture.
    ///
    /// # Panics
    ///
    /// This function will panic if the show has not been initialized yet,
    /// or if the name of the mode name in the fixture is not found in the GDTF Description.
    pub fn dmx_mode(&self) -> &DmxMode {
        let mode = self
            .description
            .as_ref()
            .expect("Show not initialized")
            .fixture_type
            .dmx_modes
            .iter()
            .find(|m| m.name == self.mode)
            .expect("Mode not found");
        mode
    }

    /// Get the offset of the channel for the given attribute.
    /// These could be multiple bytes if the attribute has a higher channel resolution.
    pub fn channel_offset_for_attribute(&self, attribute_name: &str) -> Option<&[i32]> {
        self.dmx_mode()
            .channel_with_attribute(attribute_name)
            .and_then(|channel| channel.offset.as_ref().map(|offset| offset.as_slice()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct FixtureIntermediate {
    pub id: FixtureId,
    pub label: String,
    pub revision_id: i32,
    pub channel: DmxChannel,
    pub mode: String,
}

/// # Programmer
///
/// The programmer contains changes made to the output of the show. These, for example can be used to make presets.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Programmer {
    changes: Changes,
    #[serde(skip)]
    dmx_output: DmxOutput,
}

impl Programmer {
    /// Create a new programmer.
    pub fn new() -> Self {
        Programmer {
            changes: Changes::new(),
            dmx_output: DmxOutput::new(),
        }
    }

    /// Set the value of an attribute of a fixture.
    ///
    /// # Errors
    ///
    /// This function will return an error if the fixture or attribute is not found.
    pub fn set_attribute_value(
        &mut self,
        fixture: &Fixture,
        attribute_name: String,
        value: AttributeValue,
    ) -> Result<(), Error> {
        if !self.changes.contains_key(&fixture.id) {
            self.changes.insert(fixture.id.clone(), HashMap::new());
        }

        let Some(channel_offset) = fixture.channel_offset_for_attribute(&attribute_name) else {
            return Err(Error::AttributeNotFound(attribute_name));
        };
        let value_bytes =
            value.to_bytes(ChannelResolution::try_from(channel_offset.len() as u8 * 8)?);

        let channel_offset = channel_offset
            .iter()
            // NOTE: We subtract 1 from the offset because the GDTF spec uses 1-based indexing for the offsets.
            .map(|&offset| offset as u16 - 1)
            .collect::<Vec<_>>();

        self.dmx_output
            .set_values(&fixture.channel, &channel_offset, value_bytes.as_slice())?;

        let fixture_changes = self
            .changes
            .get_mut(&fixture.id)
            .ok_or(Error::FixtureNotFound(fixture.id))?;
        fixture_changes.insert(attribute_name, value);

        Ok(())
    }

    /// Get the value of an attribute of a fixture.
    /// Returns `None` if the attribute is not found.
    pub fn get_attribute_value(
        &self,
        fixture_id: &FixtureId,
        attribute_name: &str,
    ) -> Option<&AttributeValue> {
        self.changes
            .get(fixture_id)
            .and_then(|fixture_changes| fixture_changes.get(attribute_name))
    }

    /// Get the DMX output of the programmer.
    pub fn get_dmx_output(&self) -> &DmxOutput {
        &self.dmx_output
    }

    /// Clear all changes in the programmer.
    pub fn clear_changes(&mut self) {
        self.changes.clear();
        self.dmx_output.clear();
    }
}

/// A map with the changed attributes per fixture.
pub type Changes = HashMap<FixtureId, HashMap<String, AttributeValue>>;

/// The value of an attribute.
///
/// The value is a floating point number between 0.0 and 1.0.
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct AttributeValue(f32);

impl AttributeValue {
    /// Create a new attribute value.
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Get the value of the attribute.
    pub fn value(&self) -> f32 {
        self.0.clamp(0.0, 1.0)
    }

    /// Get the value of the attribute from the raw bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use backstage::show::{AttributeValue, ChannelResolution};
    /// let value = AttributeValue::from_bytes(&[0xFF, 0xFF, 0xFF, 0xFF], ChannelResolution::Bit32).unwrap();
    /// assert_eq!(value.value(), 1.0);
    ///
    /// let value = AttributeValue::from_bytes(&[0x00, 0x00], ChannelResolution::Bit16).unwrap();
    /// assert_eq!(value.value(), 0.0);
    ///
    /// let value = AttributeValue::from_bytes(&[0x00, 0x00, 0x00, 0x00], ChannelResolution::Bit32).unwrap();
    /// assert_eq!(value.value(), 0.0);
    ///
    ///
    /// let value = AttributeValue::from_bytes(&[0x7F], ChannelResolution::Bit8).unwrap();
    /// assert_eq!(value.value(), 127.0/255.0);
    ///
    /// let value = AttributeValue::from_bytes(&[0x7F, 0xFF], ChannelResolution::Bit16).unwrap();
    /// assert_eq!(value.value(), 32767.0/65535.0);
    ///
    /// let value = AttributeValue::from_bytes(&[0x7F, 0xFF, 0xFF], ChannelResolution::Bit24).unwrap();
    /// assert_eq!(value.value(), 8388607.0/16777215.0);
    ///
    /// let value = AttributeValue::from_bytes(&[0x7F, 0xFF, 0xFF, 0xFF], ChannelResolution::Bit32).unwrap();
    /// assert_eq!(value.value(), 2147483647.0/4294967295.0);
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the bytes do not match the channel resolution.
    pub fn from_bytes(bytes: &[u8], channel_resolution: ChannelResolution) -> Result<Self, Error> {
        if bytes.len() != channel_resolution as usize / 8 {
            return Err(Error::MismatchedChannelResolution {
                found: ChannelResolution::try_from(bytes.len() as u8 * 8)?,
                expected: channel_resolution,
            });
        }

        let value = match channel_resolution {
            ChannelResolution::Bit8 => bytes[0] as f32 / 255.0,
            ChannelResolution::Bit16 => u16::from_be_bytes([bytes[0], bytes[1]]) as f32 / 65535.0,
            ChannelResolution::Bit24 => {
                u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]) as f32 / 16777215.0
            }
            ChannelResolution::Bit32 => {
                u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f32 / 4294967295.0
            }
        };

        Ok(Self::new(value))
    }

    /// Convert the attribute value to raw bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use backstage::show::{AttributeValue, ChannelResolution};
    /// let value = AttributeValue::new(0.0);
    /// assert_eq!(value.to_bytes(ChannelResolution::Bit8), vec![0x00]);
    ///
    /// let value = AttributeValue::new(0.0);
    /// assert_eq!(value.to_bytes(ChannelResolution::Bit32), vec![0x00, 0x00, 0x00, 0x00]);
    ///
    /// let value = AttributeValue::new(1.0);
    /// assert_eq!(value.to_bytes(ChannelResolution::Bit8), vec![0xFF]);
    ///
    /// let value = AttributeValue::new(1.0);
    /// assert_eq!(value.to_bytes(ChannelResolution::Bit32), vec![0xFF, 0xFF, 0xFF, 0xFF]);
    ///
    /// let value = AttributeValue::new(0.5);
    /// assert_eq!(value.to_bytes(ChannelResolution::Bit8), vec![0x80]);
    ///
    /// let value = AttributeValue::new(0.5);
    /// assert_eq!(value.to_bytes(ChannelResolution::Bit16), vec![0x80, 0x00]);
    ///
    /// let value = AttributeValue::new(0.5);
    /// assert_eq!(value.to_bytes(ChannelResolution::Bit24), vec![0x80, 0x00, 0x00]);
    ///
    /// let value = AttributeValue::new(0.5);
    /// assert_eq!(value.to_bytes(ChannelResolution::Bit32), vec![0x80, 0x00, 0x00, 0x00]);
    /// ```
    pub fn to_bytes(&self, channel_resolution: ChannelResolution) -> Vec<u8> {
        let max_value = 2u64.pow(channel_resolution as u32) - 1;
        let scaled_value = (self.0 * max_value as f32).round() as u32;

        match channel_resolution {
            ChannelResolution::Bit8 => vec![scaled_value as u8],
            ChannelResolution::Bit16 => (scaled_value as u32).to_be_bytes()[2..].to_vec(),
            ChannelResolution::Bit24 => (scaled_value as u32).to_be_bytes()[1..].to_vec(),
            ChannelResolution::Bit32 => scaled_value.to_be_bytes().to_vec(),
        }
    }
}

/// The resolution of a DMX channel.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ChannelResolution {
    /// 8-bit resolution.
    Bit8 = 8,
    /// 16-bit resolution.
    Bit16 = 16,
    /// 24-bit resolution.
    Bit24 = 24,
    /// 32-bit resolution.
    Bit32 = 32,
}

impl TryFrom<u8> for ChannelResolution {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            8 => Ok(Self::Bit8),
            16 => Ok(Self::Bit16),
            24 => Ok(Self::Bit24),
            32 => Ok(Self::Bit32),
            _ => Err(Error::InvalidChannelResolution(value)),
        }
    }
}

/// A collection of different types of data used in a show.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Data {
    /// All groups in the show.
    groups: Vec<Group>,
}

impl Data {
    /// Creates a new data collection.
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    /// Get the groups.
    pub fn groups(&self) -> &[Group] {
        &self.groups
    }

    /// Get a group with the given id.
    pub fn group(&self, id: usize) -> Option<&Group> {
        self.groups().iter().find(|group| group.id == id)
    }
}

/// An ordered group of fixtures.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Group {
    /// The group id.
    pub id: usize,
    /// The label of the group.
    pub label: String,
    /// An ordered list of all fixtures in the group.
    pub fixtures: Vec<FixtureId>,
}

/// Error type for errors related to the show.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// A GDTF Share error.
    #[error("{0}")]
    GdtfShareError(#[from] gdtf_share::Error),
    /// A DMX error.
    #[error("{0}")]
    DmxError(#[from] dmx::Error),

    /// Error when the GDTF share is not authenticated.
    #[error("GDTF share is not authenticated failed")]
    GdtfShareNotAuthenticated,
    /// Error when trying to cache a GDTF file.
    #[error("GDTF file caching failed")]
    GdtfFileCacheFailed,
    /// Error when trying to download a GDTF file.
    #[error("GDTF file download failed")]
    GdtfFileDownloadFailed,
    /// Error when trying to parse a GDTF file.
    #[error("GDTF file invalid: {0}")]
    GdtfFileInvalid(String),
    /// Error when a fixture is not found.
    #[error("Fixture not found with id {0}")]
    FixtureNotFound(FixtureId),
    /// Error when an attribute is not found.
    #[error("Attribute not found with name {0}")]
    AttributeNotFound(String),
    /// Invalid channel resolution.
    #[error("Invalid channel resolution: {0}")]
    InvalidChannelResolution(u8),
    /// Error when the channel resolution is invalid.
    #[error("Mismatched channel resolution: found {found:?}, expected {expected:?}")]
    MismatchedChannelResolution {
        /// The found channel resolution.
        found: ChannelResolution,
        /// The expected channel resolution.
        expected: ChannelResolution,
    },
}

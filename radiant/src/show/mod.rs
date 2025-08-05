use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use eyre::Context;
use gdtf::GdtfFile;

use crate::error::Result;
use crate::showfile::{RELATIVE_GDTF_FILE_FOLDER_PATH, Showfile};

mod object;
mod patch;
mod preset;

pub use object::*;
pub use patch::*;
pub use preset::*;

pub struct Show {
    path: Option<PathBuf>,

    pub groups: ObjectPool<Group>,
    pub sequences: ObjectPool<Sequence>,
    pub executors: ObjectPool<Executor>,
    pub presets_dimmer: ObjectPool<PresetDimmer>,
    pub presets_position: ObjectPool<PresetPosition>,
    pub presets_gobo: ObjectPool<PresetGobo>,
    pub presets_color: ObjectPool<PresetColor>,
    pub presets_beam: ObjectPool<PresetBeam>,
    pub presets_focus: ObjectPool<PresetFocus>,
    pub presets_control: ObjectPool<PresetControl>,
    pub presets_shapers: ObjectPool<PresetShapers>,
    pub presets_video: ObjectPool<PresetVideo>,

    pub(crate) patch: Patch,
    pub(crate) programmer: Programmer,
    pub(crate) selected_fixtures: Vec<FixtureId>,
}

impl Show {
    pub fn new(showfile: Showfile) -> Result<Self> {
        let mut patch = Patch::default();
        for gdtf_file_name in &showfile.patch.gdtf_file_names {
            let path = showfile
                .path()
                .expect("we cannot load new showfiles without a showfile path yet")
                .join(RELATIVE_GDTF_FILE_FOLDER_PATH)
                .join(gdtf_file_name);
            let file = File::open(path).wrap_err("failed to open gdtf file")?;
            let gdtf_file = GdtfFile::new(file).wrap_err("failed to load gdtf file")?;
            let fixture_type = gdtf_file.description.fixture_types[0].clone();
            patch.fixture_types.push(fixture_type);
        }
        for fixture in &showfile.patch.fixtures {
            let address = dmx::Address::new(
                dmx::UniverseId::new(fixture.universe)?,
                dmx::Channel::new(fixture.channel)?,
            );
            patch.insert_fixture(
                fixture.fid.into(),
                address,
                fixture.gdtf_type_id,
                fixture.dmx_mode.clone(),
            );
        }

        let mut this = Self {
            path: showfile.path().cloned(),
            patch,

            groups: ObjectPool::new(),
            sequences: ObjectPool::new(),
            executors: ObjectPool::new(),
            presets_dimmer: ObjectPool::new(),
            presets_position: ObjectPool::new(),
            presets_gobo: ObjectPool::new(),
            presets_color: ObjectPool::new(),
            presets_beam: ObjectPool::new(),
            presets_focus: ObjectPool::new(),
            presets_control: ObjectPool::new(),
            presets_shapers: ObjectPool::new(),
            presets_video: ObjectPool::new(),

            programmer: Programmer::default(),
            selected_fixtures: vec![FixtureId(101), FixtureId(102), FixtureId(103), FixtureId(104)],
        };

        for obj in &showfile.objects.groups {
            this.groups.insert(obj.clone());
        }
        for obj in &showfile.objects.sequences {
            this.sequences.insert(obj.clone());
        }
        for obj in &showfile.objects.executors {
            this.executors.insert(obj.clone());
        }
        for obj in &showfile.objects.dimmer_presets {
            this.presets_dimmer.insert(obj.clone());
        }
        for obj in &showfile.objects.position_presets {
            this.presets_position.insert(obj.clone());
        }
        for obj in &showfile.objects.gobo_presets {
            this.presets_gobo.insert(obj.clone());
        }
        for obj in &showfile.objects.color_presets {
            this.presets_color.insert(obj.clone());
        }
        for obj in &showfile.objects.beam_presets {
            this.presets_beam.insert(obj.clone());
        }
        for obj in &showfile.objects.focus_presets {
            this.presets_focus.insert(obj.clone());
        }
        for obj in &showfile.objects.control_presets {
            this.presets_control.insert(obj.clone());
        }
        for obj in &showfile.objects.shapers_presets {
            this.presets_shapers.insert(obj.clone());
        }
        for obj in &showfile.objects.video_presets {
            this.presets_video.insert(obj.clone());
        }

        Ok(this)
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    pub fn any_object(&self, id: impl Into<AnyObjectId>) -> Option<AnyObject> {
        match id.into() {
            AnyObjectId::Group(id) => self.groups.get(id).cloned().map(Into::into),
            AnyObjectId::Sequence(id) => self.sequences.get(id).cloned().map(Into::into),
            AnyObjectId::Executor(id) => self.executors.get(id).cloned().map(Into::into),
            AnyObjectId::PresetDimmer(id) => self.presets_dimmer.get(id).cloned().map(Into::into),
            AnyObjectId::PresetPosition(id) => {
                self.presets_position.get(id).cloned().map(Into::into)
            }
            AnyObjectId::PresetGobo(id) => self.presets_gobo.get(id).cloned().map(Into::into),
            AnyObjectId::PresetColor(id) => self.presets_color.get(id).cloned().map(Into::into),
            AnyObjectId::PresetBeam(id) => self.presets_beam.get(id).cloned().map(Into::into),
            AnyObjectId::PresetFocus(id) => self.presets_focus.get(id).cloned().map(Into::into),
            AnyObjectId::PresetControl(id) => self.presets_control.get(id).cloned().map(Into::into),
            AnyObjectId::PresetShapers(id) => self.presets_shapers.get(id).cloned().map(Into::into),
            AnyObjectId::PresetVideo(id) => self.presets_video.get(id).cloned().map(Into::into),
        }
    }

    pub fn any_preset(&self, id: impl Into<AnyPresetId>) -> Option<AnyPreset> {
        match id.into() {
            AnyPresetId::Dimmer(id) => self.presets_dimmer.get(id).cloned().map(Into::into),
            AnyPresetId::Position(id) => self.presets_position.get(id).cloned().map(Into::into),
            AnyPresetId::Gobo(id) => self.presets_gobo.get(id).cloned().map(Into::into),
            AnyPresetId::Color(id) => self.presets_color.get(id).cloned().map(Into::into),
            AnyPresetId::Beam(id) => self.presets_beam.get(id).cloned().map(Into::into),
            AnyPresetId::Focus(id) => self.presets_focus.get(id).cloned().map(Into::into),
            AnyPresetId::Control(id) => self.presets_control.get(id).cloned().map(Into::into),
            AnyPresetId::Shapers(id) => self.presets_shapers.get(id).cloned().map(Into::into),
            AnyPresetId::Video(id) => self.presets_video.get(id).cloned().map(Into::into),
        }
    }

    /// The path at which the [Showfile][crate::showfile::Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn selected_fixtures(&self) -> &[FixtureId] {
        &self.selected_fixtures
    }
}

/// Contains 'work in progress' values that can be stored into presets.
#[derive(Debug, Default)]
pub struct Programmer {
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl Programmer {
    /// Sets an [AttributeValue] for a given (main) attribute [Attribute] on the
    /// fixture with the given [FixtureId]. The attribute has to be a main
    /// attribute to prevent double channel assignments.
    pub fn set_value(&mut self, fid: FixtureId, main_attribute: Attribute, value: AttributeValue) {
        self.values.insert((fid, main_attribute), value);
    }

    /// Gets an [AttributeValue] for the given (main) [Attribute] on the
    /// fixture with the given [FixtureId].
    pub fn value(&self, fid: FixtureId, main_attribute: Attribute) -> Option<AttributeValue> {
        self.values.get(&(fid, main_attribute)).copied()
    }

    /// Gets an iterator over all values.
    pub fn values(&self) -> impl IntoIterator<Item = (&FixtureId, &Attribute, &AttributeValue)> {
        self.values.iter().map(|((fid, attr), value)| (fid, attr, value))
    }

    /// Clears all values.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

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

macro_rules! generate_object_methods {
    [$(($getter:ident, $mut_getter:ident, $all_getter:ident, $variant:ident, $ty:ty)),* $(,)?] => {
        $(
            pub fn $getter(&self, id: &ObjectId) -> Option<&$ty> {
                match self.object(id) {
                    Some(AnyObject::$variant(obj)) => Some(obj),
                    _ => None,
                }
            }

            pub(crate) fn $mut_getter(&mut self, id: &ObjectId) -> Option<&mut $ty> {
                match self.objects.get_mut(id) {
                    Some(AnyObject::$variant(obj)) => Some(obj),
                    _ => None,
                }
            }

            pub fn $all_getter(&self) -> impl Iterator<Item = &$ty> {
                self.objects.values().filter_map(|obj| {
                    if let AnyObject::$variant(o) = obj {
                        Some(o)
                    } else {
                        None
                    }
                })
            }
        )*
    }
}

pub struct Show {
    path: Option<PathBuf>,

    pub(crate) objects: HashMap<ObjectId, AnyObject>,

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

            objects: HashMap::new(),

            programmer: Programmer::default(),
            selected_fixtures: vec![FixtureId(101), FixtureId(102), FixtureId(103), FixtureId(104)],
        };

        for obj in &showfile.objects.groups {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.sequences {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.executors {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.dimmer_presets {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.position_presets {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.gobo_presets {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.color_presets {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.beam_presets {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.focus_presets {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.control_presets {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.shapers_presets {
            this.insert_object(obj.clone());
        }
        for obj in &showfile.objects.video_presets {
            this.insert_object(obj.clone());
        }

        Ok(this)
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    generate_object_methods!(
        (group, _group_mut, groups, Group, Group),
        (sequence, sequence_mut, sequences, Sequence, Sequence),
        (executor, _executor_mut, executors, Executor, Executor),
        (preset_dimmer, _preset_dimmer_mut, dimmer_presets, PresetDimmer, PresetDimmer),
        (preset_position, _preset_position_mut, position_presets, PresetPosition, PresetPosition),
        (preset_gobo, _preset_gobo_mut, gobo_presets, PresetGobo, PresetGobo),
        (preset_color, _preset_color_mut, color_presets, PresetColor, PresetColor),
        (preset_beam, _preset_beam_mut, beam_presets, PresetBeam, PresetBeam),
        (preset_focus, _preset_focus_mut, focus_presets, PresetFocus, PresetFocus),
        (preset_control, _preset_control_mut, control_presets, PresetControl, PresetControl),
        (preset_shapers, _preset_shapers_mut, shapers_presets, PresetShapers, PresetShapers),
        (preset_video, _preset_video_mut, video_presets, PresetVideo, PresetVideo),
    );

    pub fn object(&self, id: &ObjectId) -> Option<&AnyObject> {
        self.objects.get(id)
    }

    pub fn preset(&self, id: &ObjectId) -> Option<AnyPreset> {
        match self.object(id) {
            Some(AnyObject::PresetDimmer(preset)) => Some(AnyPreset::Dimmer(preset.clone())),
            Some(AnyObject::PresetPosition(preset)) => Some(AnyPreset::Position(preset.clone())),
            Some(AnyObject::PresetGobo(preset)) => Some(AnyPreset::Gobo(preset.clone())),
            Some(AnyObject::PresetColor(preset)) => Some(AnyPreset::Color(preset.clone())),
            Some(AnyObject::PresetBeam(preset)) => Some(AnyPreset::Beam(preset.clone())),
            Some(AnyObject::PresetFocus(preset)) => Some(AnyPreset::Focus(preset.clone())),
            Some(AnyObject::PresetControl(preset)) => Some(AnyPreset::Control(preset.clone())),
            Some(AnyObject::PresetShapers(preset)) => Some(AnyPreset::Shapers(preset.clone())),
            Some(AnyObject::PresetVideo(preset)) => Some(AnyPreset::Video(preset.clone())),
            _ => None,
        }
    }

    pub fn object_id_from_pool_id<T>(&self, pool_id: PoolId<T>) -> Option<ObjectId>
    where
        T: Object,
    {
        self.objects.iter().find_map(|(id, any_object)| {
            T::try_from(any_object.clone()).ok().filter(|obj| obj.pool_id() == pool_id).map(|_| *id)
        })
    }

    pub(crate) fn insert_object(&mut self, object: impl Into<AnyObject>) {
        let object = object.into();
        self.objects.insert(object.id(), object);
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

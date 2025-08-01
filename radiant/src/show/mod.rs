use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Mutex;

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

    objects: Mutex<HashMap<AnyObjectId, AnyObject>>,
    patch: Patch,
    programmer: Programmer,
    selected_fixtures: Vec<FixtureId>,
}

impl Show {
    pub fn new(showfile: Showfile) -> Result<Self> {
        let mut objects = HashMap::new();
        for obj in &showfile.objects.groups {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.sequences {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.dimmer_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.position_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.gobo_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.color_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.beam_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.focus_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.control_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.shapers_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }
        for obj in &showfile.objects.video_presets {
            objects.insert(obj.id.into(), obj.clone().into());
        }

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

        Ok(Self {
            path: showfile.path().cloned(),
            objects: Mutex::new(objects),
            patch,
            programmer: Programmer::default(),
            selected_fixtures: vec![],
        })
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    pub fn insert_object(&self, object: impl Into<AnyObject>) {
        let object: AnyObject = object.into();
        self.objects.lock().unwrap().insert(object.id(), object);
    }

    pub fn group(&self, id: impl Into<ObjectId<Group>>) -> Option<Group> {
        Some(
            self.objects
                .lock()
                .unwrap()
                .get(&AnyObjectId::Group(*id.into()))?
                .clone()
                .try_into()
                .expect("objects map should always contain a matching id and object types"),
        )
    }

    pub fn sequence(&self, id: impl Into<ObjectId<Sequence>>) -> Option<Sequence> {
        Some(
            self.objects
                .lock()
                .unwrap()
                .get(&AnyObjectId::Sequence(*id.into()))?
                .clone()
                .try_into()
                .expect("objects map should always contain a matching id and object types"),
        )
    }

    pub fn any_preset(&self, id: impl Into<AnyPresetId>) -> Option<AnyPreset> {
        Some(
            self.objects
                .lock()
                .unwrap()
                .get(&(id.into()).into())?
                .clone()
                .try_into()
                .expect("objects map should always contain a matching id and object types"),
        )
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
    values: Mutex<HashMap<(FixtureId, Attribute), AttributeValue>>,
}

impl Programmer {
    /// Sets an [AttributeValue] for a given (main) attribute [Attribute] on the
    /// fixture with the given [FixtureId]. The attribute has to be a main
    /// attribute to prevent double channel assignments.
    pub fn set_value(&self, fid: FixtureId, main_attribute: Attribute, value: AttributeValue) {
        self.values.lock().unwrap().insert((fid, main_attribute), value);
    }

    /// Gets an [AttributeValue] for the given (main) [Attribute] on the
    /// fixture with the given [FixtureId].
    pub fn value(&self, fid: FixtureId, main_attribute: Attribute) -> Option<AttributeValue> {
        self.values.lock().unwrap().get(&(fid, main_attribute)).copied()
    }

    /// Gets an iterator over all values.
    pub fn values(&self) -> Vec<(FixtureId, Attribute, AttributeValue)> {
        self.values
            .lock()
            .unwrap()
            .iter()
            .map(|((fid, attr), value)| (*fid, attr.clone(), *value))
            .collect()
    }

    /// Clears all values.
    pub fn clear(&self) {
        self.values.lock().unwrap().clear();
    }
}

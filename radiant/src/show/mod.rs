use std::fs::File;
use std::path::PathBuf;

use eyre::Context;
use gdtf::GdtfFile;

use crate::error::Result;
use crate::showfile::{RELATIVE_GDTF_FILE_FOLDER_PATH, Showfile};

mod object;
mod patch;
mod programmer;

pub use object::*;
pub use patch::*;
pub use programmer::*;

pub struct Show {
    path: Option<PathBuf>,

    pub(crate) objects: ObjectContainer,
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

        let mut objects = ObjectContainer::new();
        showfile.objects.groups.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.executors.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.sequences.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.dimmer_presets.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.position_presets.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.gobo_presets.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.color_presets.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.beam_presets.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.focus_presets.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.control_presets.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.shapers_presets.iter().for_each(|o| objects.insert(o.clone()));
        showfile.objects.video_presets.iter().for_each(|o| objects.insert(o.clone()));

        Ok(Self {
            path: showfile.path().cloned(),

            patch,

            objects,

            programmer: Programmer::default(),
            selected_fixtures: vec![],
        })
    }

    pub fn objects(&self) -> &ObjectContainer {
        &self.objects
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
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

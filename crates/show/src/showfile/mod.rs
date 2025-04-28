pub mod assets;
pub mod dmx_io;
pub mod layout;
pub mod patch;

pub use assets::*;
pub use dmx_io::*;
pub use layout::*;
pub use patch::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Showfile {
    pub dmx_io_settings: DmxIoSettings,
    pub assets: Assets,
    pub layout: Layout,
    pub patch: Patch,
}

impl Showfile {
    pub fn open_from_file(path: &std::path::PathBuf) -> ron::Result<Self> {
        let file = std::fs::File::open(path)?;
        let showfile: Self = ron::de::from_reader(file)?;
        Ok(showfile)
    }

    pub fn save_to_file(&self, path: &std::path::PathBuf) -> std::io::Result<()> {
        let extensions = ron::extensions::Extensions::UNWRAP_NEWTYPES;
        let config = ron::ser::PrettyConfig::default().compact_arrays(true).extensions(extensions);
        let serialized = ron::ser::to_string_pretty(self, config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        std::fs::write(path, serialized)
    }
}

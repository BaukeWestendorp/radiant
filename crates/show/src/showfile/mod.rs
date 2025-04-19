pub use assets::*;
pub use dmx_io::*;
pub use layout::*;

pub mod assets;
pub mod dmx_io;
pub mod layout;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Showfile {
    pub dmx_io_settings: DmxIoSettings,
    pub assets: Assets,
    pub layout: Layout,
}

pub fn open_from_file(path: &std::path::PathBuf) -> ron::Result<Showfile> {
    let file = std::fs::File::open(path)?;
    let showfile: Showfile = ron::de::from_reader(file)?;
    Ok(showfile)
}

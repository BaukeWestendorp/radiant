use std::path::PathBuf;

use dmx_io::DmxIo;
use effect_graph::EffectGraph;
use layout::Layout;

pub mod dmx_io;
pub mod effect_graph;
pub mod layout;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Showfile {
    pub dmx_io: DmxIo,
    pub effect_graph: EffectGraph,
    pub layout: Layout,
}

pub fn open_from_file(path: &PathBuf) -> Result<Showfile, std::io::Error> {
    let file = std::fs::File::open(path)?;
    let showfile: Showfile = serde_json::from_reader(file)?;
    Ok(showfile)
}

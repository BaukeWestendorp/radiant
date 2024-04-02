use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use backstage::show::Show;

use crate::layout::Layout;

const LAYOUT_PATH: &str = "layout.json";
const SHOW_PATH: &str = "show.json";

#[derive(Debug, Clone)]
pub struct Showfile {
    pub layouts: Vec<Layout>,
    pub show: Show,
}

impl Showfile {
    pub fn from_dir(path: &Path) -> anyhow::Result<Self> {
        let layout_file = File::open(path.join(LAYOUT_PATH))?;
        let layout_reader = BufReader::new(layout_file);
        let layouts: LayoutsFile = serde_json::from_reader(layout_reader)?;

        let show_file = File::open(path.join(SHOW_PATH))?;
        let show = futures_lite::future::block_on(Show::from_file(show_file))?;

        Ok(Showfile {
            layouts: layouts.layouts,
            show,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct LayoutsFile {
    pub layouts: Vec<Layout>,
}

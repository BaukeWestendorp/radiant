use gpui::{Bounds, Size};
use radiant::showfile::ShowfileComponent;

#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pages {
    pub current_page: Page,
    pub pages: Vec<Pages>,
}

impl ShowfileComponent for Pages {
    const RELATIVE_FILE_PATH: &str = "pages.yaml";
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Page {
    pub name: String,
    pub panels: Vec<Panel>,
}

impl Default for Page {
    fn default() -> Self {
        Self { name: "New Page".to_string(), panels: Vec::default() }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Panel {
    pub kind: PanelKind,
    pub bounds: Bounds<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum PanelKind {
    Window(WindowPanelKind),
    Pool(PoolPanelKind),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum WindowPanelKind {
    Executors,
    AttributeEditor,
    FixturesTable,
    CommandLine,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum PoolPanelKind {
    Group,
    Sequence,
    PresetDimmer,
    PresetPosition,
    PresetGobo,
    PresetColor,
    PresetBeam,
    PresetFocus,
    PresetControl,
    PresetShapers,
    PresetVideo,
}

impl Page {
    pub const SIZE: Size<u32> = Size { width: 20, height: 12 };
}

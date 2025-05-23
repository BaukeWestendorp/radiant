pub mod frame;

pub use frame::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Layout {
    pub main_window: MainWindow,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct MainWindow {
    pub pages: Vec<Page>,
    pub loaded_page: Page,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self { pages: Vec::default(), loaded_page: Page::default() }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Page {
    pub label: String,
    pub frames: Vec<Frame>,
}

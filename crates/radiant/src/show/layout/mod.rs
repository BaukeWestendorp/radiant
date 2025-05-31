pub mod frame;

pub use frame::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Layout {
    pub main_window: MainWindow,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
#[derive(Default)]
pub struct MainWindow {
    pub pages: Vec<Page>,
    pub loaded_page: Page,
}


#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Page {
    pub label: String,
    pub frames: Vec<Frame>,
}

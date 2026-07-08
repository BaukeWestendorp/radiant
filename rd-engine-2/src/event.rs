use std::path::PathBuf;

pub enum Event {
    HighlightChanged { highlight: bool },

    Saved { path: PathBuf },
}

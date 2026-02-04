use std::{borrow::Cow, fs, path::PathBuf};

use gpui::{AssetSource, SharedString};

pub struct Assets {
    base: PathBuf,
}

impl Default for Assets {
    fn default() -> Self {
        Self { base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets") }
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        fs::read(self.base.join(path)).map(|data| Some(Cow::Owned(data))).map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

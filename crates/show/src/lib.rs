use std::path::PathBuf;

use anyhow::Context;
use showfile::Showfile;

pub mod asset;
pub mod dmx_io;
pub mod layout;
pub mod patch;

#[derive(Clone)]
pub struct Show {
    pub path: Option<PathBuf>,

    pub dmx_io_settings: dmx_io::DmxIoSettings,
    pub assets: asset::Assets,
    pub layout: gpui::Entity<layout::Layout>,
    pub patch: gpui::Entity<patch::Patch>,
}

impl Show {
    pub fn new(cx: &mut gpui::App) -> Self {
        Showfile::default()
            .try_into_show(None, cx)
            .expect("should create show from default showfile")
    }

    pub fn init(cx: &mut gpui::App, showfile_path: Option<&PathBuf>) -> anyhow::Result<()> {
        let show = match showfile_path {
            Some(path) => Show::open_from_file(path.clone(), cx).expect("should open showfile"),
            None => Show::new(cx),
        };

        cx.set_global(show);

        Ok(())
    }

    pub fn open_from_file(path: PathBuf, cx: &mut gpui::App) -> anyhow::Result<Show> {
        let showfile = Showfile::open_from_file(&path).context("open showfile")?;
        showfile.try_into_show(Some(path), cx)
    }

    pub fn save_to_file(&mut self, path: &PathBuf, cx: &gpui::App) -> Result<(), std::io::Error> {
        let showfile = Showfile::from_show(self.clone(), cx);
        self.path = Some(path.clone());
        showfile.save_to_file(path)
    }
}

impl gpui::Global for Show {}

pub(crate) mod showfile {
    use gpui::AppContext as _;

    use super::{asset, dmx_io, layout, patch};

    #[derive(Default)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Showfile {
        pub dmx_io_settings: dmx_io::showfile::DmxIoSettings,
        pub assets: asset::showfile::Assets,
        pub layout: layout::Layout,
        pub patch: patch::Patch,
    }

    impl Showfile {
        pub fn open_from_file(path: &std::path::PathBuf) -> ron::Result<Self> {
            let file = std::fs::File::open(path)?;
            let showfile: Self = ron::de::from_reader(file)?;
            Ok(showfile)
        }

        pub fn save_to_file(&self, path: &std::path::PathBuf) -> std::io::Result<()> {
            let extensions = ron::extensions::Extensions::UNWRAP_NEWTYPES;
            let config =
                ron::ser::PrettyConfig::default().compact_arrays(true).extensions(extensions);
            let serialized = ron::ser::to_string_pretty(self, config)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            std::fs::write(path, serialized)
        }

        pub fn try_into_show(
            self,
            path: Option<std::path::PathBuf>,
            cx: &mut gpui::App,
        ) -> anyhow::Result<super::Show> {
            Ok(super::Show {
                path,

                dmx_io_settings: self.dmx_io_settings.into_show(cx),
                assets: self.assets.into_show(cx),
                layout: cx.new(|_| self.layout),
                patch: cx.new(|_| self.patch),
            })
        }

        pub fn from_show(from: super::Show, cx: &gpui::App) -> Self {
            Self {
                dmx_io_settings: super::dmx_io::showfile::DmxIoSettings::from_show(
                    from.dmx_io_settings,
                    cx,
                ),
                assets: super::asset::showfile::Assets::from_show(from.assets, cx),
                layout: from.layout.read(cx).clone(),
                patch: from.patch.read(cx).clone(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FloatingDmxValue(pub f32);

impl From<FloatingDmxValue> for dmx::Value {
    fn from(value: FloatingDmxValue) -> Self {
        dmx::Value((value.0 * (u8::MAX as f32)).clamp(0.0, 1.0) as u8)
    }
}

impl ui::NumberFieldImpl for FloatingDmxValue {
    type Value = Self;

    const MIN: Option<Self> = Some(FloatingDmxValue(0.0));
    const MAX: Option<Self> = Some(FloatingDmxValue(1.0));
    const STEP: Option<f32> = None;

    fn from_str_or_default(s: &str) -> Self::Value {
        Self(s.parse().unwrap_or_default())
    }

    fn to_shared_string(value: &Self::Value) -> gpui::SharedString {
        value.0.to_string().into()
    }

    fn as_f32(value: &Self::Value) -> f32 {
        value.0
    }

    fn from_f32(v: f32) -> Self::Value {
        Self(v)
    }
}

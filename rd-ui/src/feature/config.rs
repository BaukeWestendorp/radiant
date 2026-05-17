use gpui::{App, Global, ReadGlobal};

struct AppConfig {
    pub(crate) config: config::Config,
}

impl Global for AppConfig {}

pub fn init(config: config::Config, cx: &mut App) {
    cx.set_global(AppConfig { config });
}

pub trait ConfigAppExt {
    fn config(&self) -> &config::Config;
}

impl ConfigAppExt for App {
    fn config(&self) -> &config::Config {
        &AppConfig::global(self).config
    }
}

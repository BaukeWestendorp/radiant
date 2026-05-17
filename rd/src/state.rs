use anyhow::Context as _;
use gpui::{App, AppContext as _, Entity, Global, ReadGlobal as _};
use rd_core::Engine;

use crate::layout::Layout;

pub(crate) fn init(engine: Engine, cx: &mut App) -> anyhow::Result<()> {
    let app_state = AppState::new(engine, cx).context("failed to create app state")?;
    cx.set_global(app_state);
    Ok(())
}

pub mod action {
    use gpui::{App, KeyBinding, actions};

    actions!([ToggleHighlight]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("h", ToggleHighlight, None)]);

        cx.on_action::<ToggleHighlight>(|_, _| {
            todo!("Implement enabling highlight in rd-core");
        });
    }
}

pub struct AppState {
    engine: Engine,
    layout: Entity<Layout>,
}

impl AppState {
    pub fn new(engine: Engine, cx: &mut App) -> anyhow::Result<Self> {
        let layout = match engine.showfile_path() {
            Some(path) => Layout::load_from_showfile_dir(path)?,
            None => Layout::default(),
        };

        engine.start();

        Ok(Self { engine, layout: cx.new(|_| layout) })
    }

    pub fn engine(cx: &App) -> &Engine {
        &Self::global(cx).engine
    }

    pub fn layout(cx: &App) -> &Entity<Layout> {
        &Self::global(cx).layout
    }

    pub fn save(cx: &App) -> anyhow::Result<()> {
        let engine = Self::engine(cx);
        let showfile_path = match engine.showfile_path() {
            Some(path) => path,
            None => todo!("Implement saving new projects"),
        };

        engine.save_to_showfile_dir(showfile_path).context("failed to save showfile")?;

        Self::layout(cx)
            .read(cx)
            .save_to_showfile_dir(showfile_path)
            .context("failed to save layout file")?;

        Ok(())
    }
}

impl Global for AppState {}

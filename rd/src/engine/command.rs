use std::{fs, path::PathBuf, sync::Arc};

use gpui::{App, prelude::*};
use zeevonk::{Zeevonk, project::ProjectFile as ZeevonkProjectFile};

use crate::engine::{
    CueList, Effect, EffectAgent, Engine, FixtureCollection, Group, LayoutPage, Object,
    ObjectRegistry, OutputAgent, Programmer, Selection,
};

const LAYOUT_FILE_NAME: &str = "layout.json";

#[derive(Debug, Clone, PartialEq)]
#[derive(derive_more::From)]
pub enum Command {
    #[from]
    Selection(SelectionCommand),
    #[from]
    Highlight(HighlightCommand),
    Save(PathBuf),
    Load(PathBuf),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionCommand {
    All,
    Clear,
    Add(FixtureCollection),
    Remove(FixtureCollection),
    Overwrite(FixtureCollection),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HighlightCommand {
    Toggle,
    Enable,
    Disable,
}

pub fn execute(command: Command, engine: &mut Engine, cx: &mut App) -> anyhow::Result<()> {
    match command {
        Command::Selection(cmd) => {
            let fixtures = engine.selection(cx).fixtures();
            fixtures.update(cx, |selection, cx| {
                match cmd {
                    SelectionCommand::All => {
                        let all =
                            engine.zeevonk().project().stage().fixtures().keys().cloned().collect();
                        *selection = all;
                    }
                    SelectionCommand::Clear => {
                        selection.clear();
                    }
                    SelectionCommand::Add(fixtures) => {
                        selection.extend_from_slice(fixtures.to_fixture_ids(engine.objects()));
                    }
                    SelectionCommand::Remove(fixtures) => {
                        let remove_ids = fixtures.to_fixture_ids(engine.objects());
                        selection.retain(|id| !remove_ids.contains(id));
                    }
                    SelectionCommand::Overwrite(fixtures) => {
                        let new_selection = fixtures.to_fixture_ids(engine.objects());
                        *selection = new_selection.to_vec();
                    }
                }
                cx.notify();
            });
        }
        Command::Highlight(cmd) => match cmd {
            HighlightCommand::Toggle => todo!(),
            HighlightCommand::Enable => todo!(),
            HighlightCommand::Disable => todo!(),
        },
        Command::Save(path) => {
            fn save_objects_to_file<T>(
                obj_registry: &ObjectRegistry,
                file: PathBuf,
            ) -> anyhow::Result<()>
            where
                T: Object + Clone + serde::Serialize + 'static,
            {
                if let Some(parent) = file.parent() {
                    fs::create_dir_all(parent)?;
                }

                let items: Vec<_> = obj_registry.get_all::<T>().into_iter().cloned().collect();
                let json = serde_json::to_string_pretty(&items)?;
                fs::write(file, json)?;
                Ok(())
            }

            fs::create_dir_all(&path)?;

            // Save object files.
            let obj = engine.objects();
            save_objects_to_file::<Effect>(obj, path.join("obj/effects.json"))?;
            save_objects_to_file::<Group>(obj, path.join("obj/groups.json"))?;
            save_objects_to_file::<CueList>(obj, path.join("obj/cue_lists.json"))?;
            save_objects_to_file::<LayoutPage>(obj, path.join("obj/layout_pages.json"))?;

            // Save zeevonk project file.
            engine.zeevonk().project().file().save_to_folder(&path.join("zv/"))?;
        }
        Command::Load(path) => {
            fn load_objects_from_file<T>(
                obj_registry: &mut ObjectRegistry,
                file: PathBuf,
            ) -> anyhow::Result<()>
            where
                T: Object + serde::de::DeserializeOwned + 'static,
            {
                if !file.exists() {
                    return Ok(());
                }

                let content = fs::read_to_string(&file)?;
                let objects: Vec<T> = serde_json::from_str(&content)?;

                for object in objects {
                    obj_registry.insert(object);
                }
                Ok(())
            }

            // Load objects.
            let mut objects = ObjectRegistry::default();
            load_objects_from_file::<Effect>(&mut objects, path.join("obj/effects.json"))?;
            load_objects_from_file::<Group>(&mut objects, path.join("obj/groups.json"))?;
            load_objects_from_file::<CueList>(&mut objects, path.join("obj/cue_lists.json"))?;
            load_objects_from_file::<LayoutPage>(&mut objects, path.join("obj/layout_pages.json"))?;

            // Load zeevonk project file.
            let zv_project_file = ZeevonkProjectFile::load_from_folder(&path.join("zv/"))?;

            let objects = Arc::new(objects);
            let effect_agent = Arc::new(EffectAgent::new(Arc::clone(&objects), Some(path.clone())));
            let programmer = Arc::new(Programmer::new());
            let zeevonk = Arc::new(Zeevonk::new(zv_project_file)?);
            let output_agent = Arc::new(OutputAgent::new(
                Arc::clone(&objects),
                Arc::clone(&programmer),
                Arc::clone(&effect_agent),
                Arc::clone(&zeevonk),
            ));

            let selection = cx.new(|cx| Selection::new(cx));

            *engine = Engine {
                showfile_path: Some(path),
                objects,
                programmer,
                output_agent,
                effect_agent,
                zeevonk,

                selection,
            };
        }
    }

    Ok(())
}

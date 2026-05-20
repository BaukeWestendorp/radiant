use std::{fs, path::PathBuf};

use crate::{
    CueList, Effect, Event, Executor, FixtureCollection, Group, LayoutPage, Object, ObjectRegistry,
    RadiantEngine,
};

#[derive(Debug, Clone, PartialEq)]
#[derive(derive_more::From)]
pub enum Command {
    #[from]
    Selection(SelectionCommand),
    #[from]
    Highlight(HighlightCommand),
    Save(PathBuf),
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
    Set(bool),
}

pub fn execute(command: Command, engine: &RadiantEngine, should_emit: bool) -> anyhow::Result<()> {
    let emit = |event| {
        if should_emit {
            engine.emit(event);
        }
    };

    match command {
        Command::Selection(SelectionCommand::All) => {
            let mut selection = engine.selection.write().unwrap();
            let all = engine.zeevonk.project().stage().fixtures().keys().cloned().collect();
            *selection = all;
            emit(Event::SelectionChanged(selection.clone()));
        }
        Command::Selection(SelectionCommand::Clear) => {
            let mut selection = engine.selection.write().unwrap();
            selection.clear();
            emit(Event::SelectionChanged(selection.clone()));
        }
        Command::Selection(SelectionCommand::Add(fixtures)) => {
            let mut selection = engine.selection.write().unwrap();
            selection.extend_from_slice(fixtures.to_fixture_ids(engine.objects()));
            emit(Event::SelectionChanged(selection.clone()));
        }
        Command::Selection(SelectionCommand::Remove(fixtures)) => {
            let mut selection = engine.selection.write().unwrap();
            let remove_ids = fixtures.to_fixture_ids(engine.objects());
            selection.retain(|id| !remove_ids.contains(id));
            emit(Event::SelectionChanged(selection.clone()));
        }
        Command::Selection(SelectionCommand::Overwrite(fixtures)) => {
            let new_selection = fixtures.to_fixture_ids(engine.objects());
            *engine.selection.write().unwrap() = new_selection.to_vec();
            emit(Event::SelectionChanged(new_selection.to_vec()));
        }
        Command::Highlight(HighlightCommand::Toggle) => {
            let mut highlight = engine.highlight.write().unwrap();
            *highlight = !*highlight;
            emit(Event::HighlightChanged(*highlight));
        }
        Command::Highlight(HighlightCommand::Set(highlight)) => {
            *engine.highlight.write().unwrap() = highlight;
            emit(Event::HighlightChanged(highlight));
        }
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
            save_objects_to_file::<Executor>(obj, path.join("obj/executors.json"))?;

            // Save zeevonk project file.
            engine.zeevonk.project().file().save_to_folder(&path.join("zv/"))?;
        }
    }

    Ok(())
}

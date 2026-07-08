use std::{thread, time::Duration};

use rd_engine_2::{Command, Engine, Project};

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let path = std::env::args().nth(1).expect("Please provide a path to the project folder");

    let project = Project::load_from_folder(path)?;
    let mut engine = Engine::new();
    engine.load_project(project)?;
    engine.execute({
        let path = engine.with_project(|project| {
            project.path.as_ref().expect("Project should have a path").to_path_buf()
        });
        Command::Save { path }
    })?;

    thread::sleep(Duration::from_secs_f64(3.0));

    Ok(())
}

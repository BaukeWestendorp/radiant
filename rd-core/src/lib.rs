use std::{path::PathBuf, sync::Arc, thread};

use crate::{
    object::ObjectRegistry, output::OutputAgent, programmer::Programmer, showfile::Showfile,
};

pub mod compositor;
pub mod object;
pub mod output;
pub mod parameter;
pub mod programmer;
pub mod showfile;

mod error;

pub use error::*;
use zeevonk::{Zeevonk, project::ProjectFile};

pub struct Engine {
    showfile: Arc<Showfile>,
    objects: Arc<ObjectRegistry>,
    programmer: Arc<Programmer>,
    output_agent: Arc<OutputAgent>,

    zeevonk: Arc<Zeevonk>,
}

impl Engine {
    pub fn new(showfile_path: Option<PathBuf>) -> Result<Self, crate::Error> {
        let (showfile, objects, zv_project_file) = match showfile_path {
            Some(showfile_path) => Showfile::load_from_dir(showfile_path)?,
            None => (Showfile::default(), ObjectRegistry::default(), ProjectFile::default()),
        };

        let showfile = Arc::new(showfile);
        let objects = Arc::new(objects);
        let programmer = Arc::new(Programmer::new());
        let zeevonk = Arc::new(Zeevonk::new(zv_project_file)?);
        let output_agent = Arc::new(OutputAgent::new(
            Arc::clone(&objects),
            Arc::clone(&programmer),
            Arc::clone(&zeevonk),
        ));

        Ok(Self { showfile, objects, programmer, output_agent, zeevonk })
    }

    pub fn showfile(&self) -> &Showfile {
        &self.showfile
    }

    pub fn objects(&self) -> &ObjectRegistry {
        &self.objects
    }

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    pub fn output_agent(&self) -> &OutputAgent {
        &self.output_agent
    }

    pub fn zeevonk(&self) -> &Zeevonk {
        &self.zeevonk
    }

    pub fn start(&self) {
        self.zeevonk().start();
        self.output_agent().start();

        let _ = thread::Builder::new().name("rd_engine_main".to_string()).spawn({
            let output_agent = Arc::clone(&self.output_agent);
            move || {
                let _ = output_agent;
            }
        });
    }
}

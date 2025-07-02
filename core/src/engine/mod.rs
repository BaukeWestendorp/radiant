use std::time::Duration;

use eyre::{Context, ContextCompat};

use super::pipeline::Pipeline;
use crate::showfile::Showfile;
use crate::{Command, DmxMode, FixtureId, PatchCommand, Result, Show};

mod adapters;
mod cmd;
mod dmx_resolver;
mod protocols;

pub const DMX_OUTPUT_UPDATE_INTERVAL: Duration = Duration::from_millis(40);

/// The [Engine] controls the flow of output data,
/// and is the interface between the user interface
/// (including a headless app) and
/// the show.
pub struct Engine {
    show: Show,

    /// The final output that will be sent to the DMX sources.
    output_pipeline: Pipeline,

    /// Handles all DMX protocol interaction.
    protocols: protocols::Protocols,

    adapters: adapters::Adapters,
}

impl Engine {
    /// Creates a new [Engine] and internally converts the provided [Showfile] into a [Show].
    pub fn new(showfile: Showfile) -> Result<Self> {
        let show = Show::new(showfile.path().cloned());

        let output_pipeline = Pipeline::new();

        let mut protocols = protocols::Protocols::new();
        for config in showfile.protocols().sacn().sources() {
            protocols.add_sacn_source(config).wrap_err("failed to add sACN source to engine")?;
        }

        let adapters = adapters::Adapters::new(&showfile.adapters().midi())
            .wrap_err("failed to create adapter handler")?;

        let mut this = Self { show, output_pipeline, protocols, adapters };

        this.show.patch.gdtf_file_names = showfile.patch().gdtf_files().to_vec();

        // Initialize patch.
        for fixture in showfile.patch().fixtures() {
            let id = FixtureId(fixture.id());

            let address = dmx::Address::new(
                dmx::UniverseId::new(fixture.universe())?,
                dmx::Channel::new(fixture.channel())?,
            );

            let mode = DmxMode::new(fixture.dmx_mode());

            let gdtf_file_name = showfile
                .patch()
                .gdtf_files()
                .get(fixture.gdtf_file_index())
                .wrap_err("failed to generate patch: tried to reference GDTF file index that is out of bounds")?
                .to_string();

            this.exec_cmd(Command::Patch(PatchCommand::Add { id, address, mode, gdtf_file_name }))?;
        }

        // Run init commands
        for cmd in showfile.init_commands() {
            this.exec_cmd(cmd.clone()).context("failed to run init command")?;
        }

        this.output_pipeline.clear_unresolved();

        Ok(this)
    }

    /// Execute a [Command] to interface with the backend.
    pub fn exec_cmd(&mut self, cmd: Command) -> Result<()> {
        cmd::exec_cmd(self, cmd)
    }

    /// Do a single iteration of DMX resolving and executor state management.
    /// This should be called in a loop externally, with a delay of [DMX_OUTPUT_UPDATE_INTERVAL].
    pub fn resolve_dmx(&mut self) {
        // FIXME: Cloning the whole show is extremely cursed.
        let show = &self.show.clone();
        for executor in self.show.executors.values_mut() {
            executor.manage_state(&show);
        }

        self.output_pipeline = Pipeline::default();
        dmx_resolver::resolve(&mut self.output_pipeline, &mut self.show);

        self.protocols.update_dmx_output(self.output_pipeline.resolved_multiverse());
    }

    /// Gets the resolved output [Multiverse].
    pub fn output_multiverse(&self) -> &dmx::Multiverse {
        self.output_pipeline.resolved_multiverse()
    }

    /// Handles all adapter inputs like MIDI controllers controlling executors.
    pub fn handle_adapter_input(&mut self) -> Result<()> {
        let commands = self.adapters.handle_input()?;
        for cmd in commands {
            self.exec_cmd(cmd)?;
        }
        Ok(())
    }

    /// Gets the [Show] associated with this [Engine].
    pub fn show(&self) -> &Show {
        &self.show
    }
}

//! Core controller for Radiant.
//!
//! This module provides the [Engine] struct, which
//! acts as the central controller for Radiant. The [Engine]
//! manages the show lifecycle, executes incoming
//! [Command]s, resolves DMX output, and coordinates
//! protocol and adapter integration. It is the main entry point for
//! embedding Radiant's backend in an application.

use std::time::{Duration, Instant};

use eyre::{Context, ContextCompat};

use super::pipeline::Pipeline;
use crate::cmd::{Command, PatchCommand};
use crate::error::Result;
use crate::patch::{DmxMode, FixtureId};
use crate::show::Show;
use crate::showfile::Showfile;

mod adapters;
mod cmd;
mod dmx_resolver;
mod protocols;

/// The interval at which the host should update the DMX output.
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

    start_time: Instant,
    frame: usize,
    last_dmx_resolution: Instant,
}

impl Engine {
    /// Creates a new [Engine] and internally converts the provided [Showfile]
    /// into a [Show].
    pub fn new(showfile: Showfile) -> Result<Self> {
        let show = Show::new(showfile.path().cloned());

        let output_pipeline = Pipeline::new();

        let mut protocols = protocols::Protocols::new();
        for config in showfile.protocols().sacn().sources() {
            protocols.add_sacn_source(config).wrap_err("failed to add sACN source to engine")?;
        }

        let adapters = adapters::Adapters::new(&showfile.adapters().midi())
            .wrap_err("failed to create adapter handler")?;

        let mut this = Self {
            show,
            output_pipeline,
            protocols,
            adapters,
            start_time: Instant::now(),
            frame: 0,
            last_dmx_resolution: Instant::now(),
        };

        this.initialize_show(showfile).wrap_err("failed to initialize show")?;

        Ok(this)
    }

    /// Execute a [Command] to interface with the backend.
    pub fn exec_cmd(&mut self, cmd: Command) -> Result<()> {
        cmd::exec_cmd(self, cmd)
    }

    /// Do a single iteration of DMX resolving and executor state management.
    /// This should be called in a loop externally, with a delay of
    /// [DMX_OUTPUT_UPDATE_INTERVAL].
    pub fn resolve_dmx(&mut self) {
        // FIXME: Cloning the whole show is extremely cursed.
        let show = &self.show.clone();
        for executor in self.show.executors.values_mut() {
            executor.manage_state(&show);
        }

        self.output_pipeline = Pipeline::default();

        // Put each fixture's default values into the output pipeline before resolving
        // other values.
        for fixture in self.show().patch().fixtures().to_vec() {
            for (attribute, value) in fixture.get_default_attribute_values() {
                self.output_pipeline.set_attribute_value(fixture.id(), attribute.clone(), value);
            }
        }

        dmx_resolver::resolve(self.uptime(), &mut self.output_pipeline, &mut self.show);

        self.protocols.update_dmx_output(self.output_pipeline.resolved_multiverse());

        let resolution_delay = self.last_dmx_resolution.elapsed();
        log::debug!("DMX resolution delay: {:?}", resolution_delay);

        self.frame += 1;
        self.last_dmx_resolution = Instant::now();
    }

    /// Gets the resolved output [dmx::Multiverse].
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

    /// The amount of time the engine has been running.
    pub fn uptime(&self) -> EngineUptime {
        EngineUptime { frames: self.frame, duration: self.start_time.elapsed() }
    }

    fn initialize_show(&mut self, showfile: Showfile) -> Result<()> {
        self.show.patch.gdtfs = showfile.patch().gdtf_files().to_vec();

        // Initialize patch.
        for fixture in showfile.patch().fixtures() {
            let fid = FixtureId(fixture.id());

            let address = dmx::Address::new(
                dmx::UniverseId::new(fixture.universe())?,
                dmx::Channel::new(fixture.channel())?,
            );

            let mode = DmxMode::new(fixture.dmx_mode());

            let gdtf = showfile
                .patch()
                .gdtf_files()
                .get(fixture.gdtf_file_index())
                .wrap_err("failed to generate patch: tried to reference GDTF file index that is out of bounds")?
                .to_string();

            self.exec_cmd(Command::Patch(PatchCommand::Add { fid, address, mode, gdtf }))?;
        }

        // Initialize objects.
        for executor in showfile.objects().executors().to_vec() {
            self.show.executors.insert(executor.id(), executor);
        }
        for sequence in showfile.objects().sequences().to_vec() {
            self.show.sequences.insert(sequence.id(), sequence);
        }
        for cue in showfile.objects().cues().to_vec() {
            self.show.cues.insert(cue.id(), cue);
        }
        for fixture_group in showfile.objects().fixture_groups().to_vec() {
            self.show.fixture_groups.insert(fixture_group.id(), fixture_group);
        }
        for dimmer_preset in showfile.objects().dimmer_presets().to_vec() {
            self.show.dimmer_presets.insert(dimmer_preset.id(), dimmer_preset);
        }
        for position_preset in showfile.objects().position_presets().to_vec() {
            self.show.position_presets.insert(position_preset.id(), position_preset);
        }
        for gobo_preset in showfile.objects().gobo_presets().to_vec() {
            self.show.gobo_presets.insert(gobo_preset.id(), gobo_preset);
        }
        for color_preset in showfile.objects().color_presets().to_vec() {
            self.show.color_presets.insert(color_preset.id(), color_preset);
        }
        for beam_preset in showfile.objects().beam_presets().to_vec() {
            self.show.beam_presets.insert(beam_preset.id(), beam_preset);
        }
        for focus_preset in showfile.objects().focus_presets().to_vec() {
            self.show.focus_presets.insert(focus_preset.id(), focus_preset);
        }
        for shapers_preset in showfile.objects().shapers_presets().to_vec() {
            self.show.shapers_presets.insert(shapers_preset.id(), shapers_preset);
        }
        for control_preset in showfile.objects().control_presets().to_vec() {
            self.show.control_presets.insert(control_preset.id(), control_preset);
        }
        for video_preset in showfile.objects().video_presets().to_vec() {
            self.show.video_presets.insert(video_preset.id(), video_preset);
        }

        self.output_pipeline.clear_unresolved();

        Ok(())
    }
}

/// Represents the uptime of the [Engine], including both the number of frames
/// processed and the total elapsed duration since the engine started.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(derive_more::Add, derive_more::Div, derive_more::Mul)]
pub struct EngineUptime {
    frames: usize,
    duration: Duration,
}

impl EngineUptime {
    /// Returns the number of frames that have been processed since the engine
    /// started.
    pub fn frames(&self) -> usize {
        self.frames
    }

    /// Returns the total elapsed duration since the engine started.
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

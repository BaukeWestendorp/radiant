use std::{cell::RefCell, io, rc::Rc, time::Duration};

use artnet::ArtnetNode;
use dmx::DmxOutput;
use gpui::{AppContext, Global, Model, Timer, UpdateGlobal};
use show::{ArtnetNodeSettings, EffectGraphProcessingContext, Show};

pub mod artnet;

const ARTNET_INTERVAL: Duration = Duration::from_millis(40);

pub struct DmxIo {
    artnet_nodes: Vec<ArtnetNode>,
    show: Model<Show>,
}

impl DmxIo {
    pub fn new(show: Model<Show>) -> Self {
        Self {
            artnet_nodes: Vec::new(),
            show,
        }
    }

    pub fn add_artnet_node(&mut self, settings: ArtnetNodeSettings) -> io::Result<()> {
        let node = ArtnetNode::bind(settings)?;
        self.artnet_nodes.push(node);
        Ok(())
    }

    pub fn start_emitting(&self, cx: &AppContext) {
        self.spawn_artnet_task(cx);
    }

    fn spawn_artnet_task(&self, cx: &AppContext) {
        cx.spawn::<_, anyhow::Result<()>>({
            let show = self.show.clone();
            |cx| async move {
                loop {
                    cx.update({
                        let show = show.clone();
                        move |cx| {
                            let dmx_output = compute_dmx_output(show, cx);
                            DmxIo::update_global(cx, |io, _| {
                                io.send_output(dmx_output)
                                    .expect("DMX output should have been sent");
                            });
                        }
                    })
                    .unwrap();

                    Timer::after(ARTNET_INTERVAL).await;
                }
            }
        })
        .detach_and_log_err(cx);
    }

    fn send_output(&mut self, dmx_output: DmxOutput) -> anyhow::Result<()> {
        for node in self.artnet_nodes.iter() {
            let Some(universe) = dmx_output.universe(node.settings.universe) else {
                continue;
            };

            let universe_dmx = universe.bytes();
            node.send_dmx(universe_dmx.to_vec())?;
        }

        Ok(())
    }
}

fn compute_dmx_output(show: Model<Show>, cx: &mut AppContext) -> DmxOutput {
    let dmx_output = Rc::new(RefCell::new(DmxOutput::new()));
    let patch = show.read(cx).patch.read(cx);

    // Set default DMX values
    for fixture in patch.fixtures() {
        for channel in &fixture.dmx_mode(patch).dmx_channels {
            if let Some((_, channel_function)) = channel.initial_function() {
                if let Some(offsets) = &channel.offset {
                    let default_bytes = match &channel_function.default.bytes().get() {
                        1 => channel_function.default.to_u8().to_be_bytes().to_vec(),
                        2 => channel_function.default.to_u16().to_be_bytes().to_vec(),
                        _ => panic!("Unsupported default value size"),
                    };

                    for (i, offset) in offsets.iter().enumerate() {
                        let default = default_bytes[i];
                        let address = fixture
                            .dmx_address()
                            .with_channel_offset(*offset as u16 - 1);

                        dmx_output.borrow_mut().set_channel_value(address, default)
                    }
                }
            }
        }
    }

    // FIXME: This is ad-hoc. We should use an executor system for getting the cues we should process.
    let cue = show
        .read(cx)
        .assets
        .cuelists
        .read(cx)
        .get(&0.into())
        .unwrap()
        .cues[0]
        .clone();

    for line in cue.lines {
        // Initialize context
        let mut context = EffectGraphProcessingContext::new(show.clone(), line, dmx_output.clone());

        // Process frame
        context
            .process_frame(cx)
            .map_err(|err| log::warn!("Failed to process frame: {err}"))
            .ok();
    }

    dmx_output.take()
}

impl Global for DmxIo {}

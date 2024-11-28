use std::{cell::RefCell, io, rc::Rc, time::Duration};

use crate::showfile::{ArtnetNodeSettings, EffectGraphProcessingContext, Showfile};
use artnet::ArtnetNode;
use dmx::DmxOutput;
use gpui::{AppContext, ReadGlobal, Timer};

pub mod artnet;

const ARTNET_INTERVAL: Duration = Duration::from_millis(40);

pub struct DmxIo {
    artnet_nodes: Vec<ArtnetNode>,
}

impl DmxIo {
    pub fn new() -> Self {
        Self {
            artnet_nodes: Vec::new(),
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
            |cx| async move {
                loop {
                    cx.update_global(|this: &mut Self, cx| {
                        let dmx_output = this.compute_dmx_output(cx);
                        this.send_output(dmx_output)
                            .expect("DMX output should have been sent");
                    })
                    .expect("DmxIo global should have been updated");

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

    fn compute_dmx_output(&mut self, cx: &AppContext) -> DmxOutput {
        let dmx_output = Rc::new(RefCell::new(DmxOutput::new()));
        let showfile = Showfile::global(cx);

        // Set default DMX values
        for fixture in showfile.patch().fixtures() {
            for channel in &fixture.dmx_mode(showfile.patch()).dmx_channels {
                if let Some((_, channel_function)) = channel.initial_function() {
                    if let Some(offsets) = &channel.offset {
                        let default_bytes = match &channel_function.default.bytes().get() {
                            1 => channel_function.default.to_u8().to_be_bytes().to_vec(),
                            2 => channel_function.default.to_u16().to_be_bytes().to_vec(),
                            _ => panic!("Unsupported default value size"),
                        };

                        for (i, offset) in offsets.iter().enumerate() {
                            let default = default_bytes[i];
                            let address =
                                fixture.dmx_address.with_channel_offset(*offset as u16 - 1);

                            dmx_output.borrow_mut().set_channel_value(address, default)
                        }
                    }
                }
            }
        }

        for effect in showfile.assets().effects() {
            // Initialize context
            let mut context = EffectGraphProcessingContext::new(
                showfile.clone(),
                effect.id(),
                dmx_output.clone(),
            );

            // Process frame
            context
                .process_frame()
                .map_err(|err| log::warn!("Failed to process frame: {err}"))
                .ok();
        }

        dmx_output.take()
    }
}

impl gpui::Global for DmxIo {}

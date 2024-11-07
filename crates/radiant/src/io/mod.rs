use std::{io, time::Duration};

use artnet::ArtnetNode;
use dmx::DmxOutput;
use gpui::{EventEmitter, ModelContext, Timer};
use show::DmxProtocols;

pub mod artnet;

const ARTNET_INTERVAL: Duration = Duration::from_millis(40);

pub struct IoManager {
    artnet_nodes: Vec<ArtnetNode>,

    dmx_output: DmxOutput,
}

impl IoManager {
    pub fn new(dmx_protocols: &DmxProtocols) -> io::Result<Self> {
        let artnet_nodes = dmx_protocols
            .artnet()
            .iter()
            .map(|settings| ArtnetNode::bind(settings.clone()))
            .collect::<io::Result<Vec<_>>>()?;

        Ok(Self {
            artnet_nodes,
            dmx_output: DmxOutput::default(),
        })
    }

    pub fn start_emitting(&self, cx: &ModelContext<Self>) {
        self.spawn_artnet_task(cx);
    }

    pub fn set_dmx_output(&mut self, dmx_output: DmxOutput) {
        self.dmx_output = dmx_output;
    }

    fn spawn_artnet_task(&self, cx: &ModelContext<Self>) {
        cx.spawn::<_, anyhow::Result<()>>({
            |this, mut cx| async move {
                loop {
                    Timer::after(ARTNET_INTERVAL).await;

                    this.update(&mut cx, |this, cx| -> anyhow::Result<()> {
                        cx.emit(IoManagerEvent::OutputRequested);

                        for node in this.artnet_nodes.iter() {
                            let Some(universe) = this.dmx_output.universe(node.settings.universe)
                            else {
                                log::warn!(
                                    "DMX data not found for universe {}",
                                    node.settings.universe
                                );
                                continue;
                            };

                            let universe_dmx = universe.bytes();
                            node.send_dmx(universe_dmx.to_vec())?;
                        }

                        Ok(())
                    })??;
                }
            }
        })
        .detach_and_log_err(cx);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IoManagerEvent {
    OutputRequested,
}

impl EventEmitter<IoManagerEvent> for IoManager {}

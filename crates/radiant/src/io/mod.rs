use artnet::ArtnetNode;
use dmx::DmxOutput;
use gpui::{AppContext, Global, Model, Timer, UpdateGlobal};
use show::effect_graph::EffectGraphProcessingContext;
use show::Show;
use std::time::Duration;

pub mod artnet;

pub fn init(show: Model<Show>, cx: &mut AppContext) {
    cx.set_global(IoManager::new());

    cx.spawn(|cx| async move {
        loop {
            Timer::after(Duration::from_millis(40)).await;

            // FIXME: This is kind of ad-hoc and should not be in this task. We might need to do something similar to cursor blink thing in Zed.
            cx.update(|cx| {
                let mut context = EffectGraphProcessingContext::default();
                show.update(cx, |show, _cx| {
                    show.effect_graph_mut().process(&mut context).unwrap();
                });

                IoManager::update_global(cx, |io_manager, _cx| {
                    io_manager.set_output(context.dmx_output)
                });
            })
            .unwrap();

            cx.read_global::<IoManager, _>(|io_manager, _cx| {
                if let Some(universe) = io_manager.output.get_universe(0) {
                    io_manager.artnet_node.send_dmx(universe.bytes().to_vec());
                }
            })
            .unwrap();
        }
    })
    .detach();
}

pub struct IoManager {
    output: DmxOutput,
    artnet_node: ArtnetNode,
}

impl IoManager {
    fn new() -> Self {
        Self {
            output: DmxOutput::new(),
            artnet_node: ArtnetNode::bind(),
        }
    }

    pub fn set_output(&mut self, output: DmxOutput) {
        self.output = output;
    }
}

impl Global for IoManager {}

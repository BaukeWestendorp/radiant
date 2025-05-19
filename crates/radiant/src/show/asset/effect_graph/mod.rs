pub mod templates;

mod control;

use control::Control;
use flow::Graph;
use gpui::{App, Entity, ReadGlobal};

use crate::{
    pipeline::Pipeline,
    show::{
        FloatingDmxValue, Show,
        attr::AnyPresetAssetId,
        patch::{Fixture, FixtureId},
    },
};

#[derive(Debug, Clone, flow::Value)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(graph_def = Def, data_type = DataType)]
pub enum Value {
    // Asset
    #[value(color = 0xFE3231)]
    Preset(Option<AnyPresetAssetId>),

    // DMX
    #[value(color = 0xFFF125)]
    #[cast(target = Float, map = |v: &FloatingDmxValue| v.0 as f64)]
    #[cast(target = Bool, map = |v: &FloatingDmxValue| v.0 != 0.0)]
    DmxValue(FloatingDmxValue),
    #[value(color = 0xFF9A31)]
    DmxAddress(dmx::Address),

    // Math
    #[value(color = 0x52B4FF)]
    #[cast(target = Bool, map = |v: &f64| *v != 0.0)]
    #[cast(target = DmxValue, map = |v: &f64| FloatingDmxValue(*v as f32) )]
    Float(f64),
    #[value(color = 0xFF178C)]
    #[cast(target = Float, map = |v: &bool| if *v { 1.0 } else { 0.0 })]
    #[cast(target = DmxValue, map = |v: &bool| if *v { FloatingDmxValue(1.0) } else { FloatingDmxValue(0.0) })]
    Bool(bool),

    #[value(color = 0x9C6EFF)]
    FixtureId(FixtureId),
}

#[derive(Debug, Clone)]
pub struct State {
    fixtures: Vec<FixtureId>,
    pub(crate) group_index: usize,
    pipeline: Entity<Pipeline>,
}

impl State {
    pub fn new(
        mut fixtures: Vec<FixtureId>,
        pipeline: Entity<Pipeline>,
        cx: &App,
    ) -> anyhow::Result<Self> {
        if fixtures.is_empty() {
            anyhow::bail!(
                "A graph can only be processed if it has at least one fixture in the group it's processing!"
            );
        }

        // Filter out all FixtureIds that do not have a corresponding fixture.
        fixtures.retain(|fid| Show::global(cx).patch.read(cx).fixture(*fid).is_some());

        Ok(Self { fixtures, group_index: 0, pipeline })
    }

    pub fn pipeline(&self) -> &Entity<Pipeline> {
        &self.pipeline
    }

    pub fn group_index(&self) -> usize {
        self.group_index
    }

    pub fn group_len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn fixture_id(&self) -> FixtureId {
        *self
            .fixtures
            .get(self.group_index())
            .expect("It should not be possible to have a group_index larger than the FixtureGroup")
    }

    pub fn fixture<'a>(&self, cx: &'a App) -> &'a Fixture {
        let patch = Show::global(cx).patch.read(cx);
        patch
            .fixture(self.fixture_id())
            .expect("All FixtureIds without a corresponding fixture should have been filtered out")
    }
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Def;

impl flow::GraphDef for Def {
    type ProcessingState = State;
    type Value = Value;
    type DataType = DataType;
    type Control = Control;
}

pub type EffectGraph = Graph<Def>;

pub mod templates;

mod control;

use control::Control;
use flow::Graph;
use gpui::Entity;

use crate::show::asset::{Asset, FixtureGroup};

#[derive(Debug, Clone, flow::Value)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(graph_def = Def, data_type = DataType)]
pub enum Value {
    // Math
    #[value(color = 0x52B4FF)]
    Float(f64),
    #[value(color = 0xFF178C)]
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct State {
    pub multiverse: Entity<dmx::Multiverse>,
    pub fixture_group: Entity<Asset<FixtureGroup>>,
    pub fixture_id_index: Option<usize>,
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

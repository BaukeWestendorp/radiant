use std::collections::HashMap;

use crate::show::{FloatingDmxValue, attr::Attribute};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub enum Preset<A: Attribute> {
    Universal(HashMap<A, FloatingDmxValue>),
}

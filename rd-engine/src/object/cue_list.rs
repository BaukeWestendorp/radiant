use std::time::Duration;

use anyhow::Context;
use zeevonk::value::AttributeValues;

use crate::{Object, ObjectId, Slot};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CueList {
    id: ObjectId,
    slot: Slot,
    name: String,
    cues: Vec<Cue>,
}

impl CueList {
    pub fn new(id: ObjectId, slot: Slot, name: String) -> Self {
        Self { id, slot, name, cues: Vec::new() }
    }

    pub fn cues(&self) -> &[Cue] {
        &self.cues
    }

    pub fn cue(&self, index: usize) -> anyhow::Result<&Cue> {
        self.cues.get(index).with_context(|| format!("no cue at index {}", index))
    }
}

impl Object for CueList {
    fn slot(&self) -> Slot {
        self.slot
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cue {
    name: String,
    #[serde(with = "duration_as_seconds")]
    fade_time: Duration,
    values: AttributeValues,
}

impl Cue {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fade_time(&self) -> Duration {
        self.fade_time
    }

    pub fn values(&self) -> &AttributeValues {
        &self.values
    }
}

mod duration_as_seconds {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(duration.as_secs_f64())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = f64::deserialize(deserializer)?;
        Ok(Duration::from_secs_f64(secs))
    }
}

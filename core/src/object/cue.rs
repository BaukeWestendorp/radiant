use crate::engine::EngineUptime;
use crate::object::{AnyPresetId, FixtureGroupId};

super::define_object_id!(CueId);

/// A state of the stage output.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub struct Cue {
    id: CueId,
    pub(crate) name: String,
    pub(crate) recipes: Vec<Recipe>,
}

impl Cue {
    /// Creates a new [Cue] with the specified id.
    pub fn new(id: impl Into<CueId>) -> Self {
        Self { id: id.into(), name: "New Cue".to_string(), recipes: Vec::new() }
    }

    /// Returns this cue's id.
    pub fn id(&self) -> CueId {
        self.id
    }

    /// Returns the name of this cue.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the recipes contained in this cue.
    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

/// A list of fixture group to content combinations.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub struct Recipe {
    /// The id of the [FixtureGroup][crate::object::FixtureGroup] this recipe
    /// applies to.
    pub fixture_group: FixtureGroupId,
    /// The content of this recipe.
    pub content: RecipeContent,
    /// The effect to apply to this recipe's value levels.
    pub level_effect: Option<LevelEffect>,
}

/// Represents the different types of content that can be included in a recipe.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum RecipeContent {
    /// A preset to be applied.
    Preset(AnyPresetId),
}

/// Describes an effect that changes value levels over time using keyframes
/// and interpolation.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub struct LevelEffect {
    /// The type of time measurement used for the keyframes (e.g., frames or
    /// seconds).
    pub time_type: TimeType,
    /// The interpolation curve to use between keyframes.
    pub curve: Curve,
    /// The list of keyframes defining the value changes over time.
    pub keyframes: Vec<Keyframe>,
}

impl LevelEffect {
    /// Computes the level for this effect given the [Engine]'s uptime.
    /// Expects the keyframes to be sorted by time.
    pub fn compute(&self, engine_uptime: EngineUptime) -> f32 {
        if self.keyframes.is_empty() {
            return 0.0;
        }

        let max_time = self.keyframes.last().unwrap().time;
        let time = match self.time_type {
            TimeType::Frames => engine_uptime.frames() as f32,
            TimeType::Seconds => engine_uptime.duration().as_secs_f32(),
        };
        let wrapped_time = time % max_time;

        // Find the two keyframes surrounding the given time
        let mut prev = &self.keyframes[0];
        let mut next = &self.keyframes[0];

        for kf in &self.keyframes {
            if (kf.time) <= wrapped_time {
                prev = kf;
            }
            if (kf.time) >= wrapped_time {
                next = kf;
                break;
            }
        }

        // If time is before the first keyframe
        if wrapped_time <= self.keyframes[0].time {
            return self.keyframes[0].value;
        }

        // If time is after the last keyframe
        if wrapped_time >= self.keyframes[self.keyframes.len() - 1].time {
            return self.keyframes[self.keyframes.len() - 1].value;
        }

        // Interpolate between prev and next
        let t0 = prev.time;
        let t1 = next.time;
        let v0 = prev.value;
        let v1 = next.value;

        if t1 == t0 {
            return v0;
        }

        match self.curve {
            Curve::Linear => {
                let alpha = (wrapped_time - t0) / (t1 - t0);
                v0 + (v1 - v0) * alpha
            }
            Curve::Step => v0,
        }
    }
}

/// The type of curve used for interpolating between keyframes in a
/// [LevelEffect].
#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub enum Curve {
    /// Linear interpolation between keyframes.
    Linear,
    /// Step interpolation, holding the previous keyframe's value until the
    /// next.
    Step,
}

/// A keyframe representing a value at a specific time for use in a
/// [LevelEffect].
#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub struct Keyframe {
    /// The time at which this keyframe occurs.
    pub time: f32,
    /// The value at this keyframe.
    pub value: f32,
}

/// The type of time measurement used for [LevelEffect] keyframes.
#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub enum TimeType {
    /// Time is measured in frames.
    Frames,
    /// Time is measured in seconds.
    Seconds,
}

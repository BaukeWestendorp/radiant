use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::show::ObjectId;

#[derive(object_derive::Object)]
#[object_derive::object]
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    cues: HashMap<CueId, Cue>,

    current_cue: Option<CueId>,

    #[serde(skip)]
    pub(crate) cue_fade_in_starts: HashMap<CueId, Instant>,
    #[serde(skip)]
    pub(crate) cue_fade_out_starts: HashMap<CueId, Instant>,
}

impl Sequence {
    pub fn cue(&self, id: &CueId) -> Option<&Cue> {
        self.cues.get(id)
    }

    pub fn cues(&self) -> impl IntoIterator<Item = &Cue> {
        self.cues.values()
    }

    pub fn active_cues(&self) -> impl IntoIterator<Item = &Cue> {
        self.cues.values().filter(|cue| {
            let id = &cue.id;
            let is_current = self.current_cue.as_ref().map_or(false, |current_id| current_id == id);
            let is_fading_in = self.cue_fade_in_starts.contains_key(id);
            let is_fading_out = self.cue_fade_out_starts.contains_key(id);
            is_current || is_fading_in || is_fading_out
        })
    }

    pub fn first_cue(&self) -> Option<&Cue> {
        self.cues.values().min_by_key(|cue| &cue.id)
    }

    pub fn last_cue(&self) -> Option<&Cue> {
        self.cues.values().max_by_key(|cue| &cue.id)
    }

    pub fn cue_before(&self, id: &CueId) -> Option<&Cue> {
        self.cues.values().filter(|cue| cue.id < *id).max_by_key(|cue| &cue.id)
    }

    pub fn cue_after(&self, id: &CueId) -> Option<&Cue> {
        self.cues.values().filter(|cue| cue.id > *id).min_by_key(|cue| &cue.id)
    }

    pub fn previous_cue(&self) -> Option<&Cue> {
        if self.current_cue().is_none() {
            return self.last_cue();
        }

        self.cue_before(self.current_cue.as_ref()?)
    }

    pub fn current_cue(&self) -> Option<&Cue> {
        self.current_cue.as_ref().and_then(|id| self.cues.get(id))
    }

    pub fn set_current_cue(&mut self, id: Option<CueId>) {
        if let Some(current_cue) = self.current_cue() {
            if current_cue.fade_out_time() > Duration::from_millis(0) {
                self.cue_fade_out_starts.insert(current_cue.id().clone(), Instant::now());
            }
        }

        self.current_cue = id;

        if let Some(current_cue) = self.current_cue() {
            if current_cue.fade_in_time() > Duration::from_millis(0) {
                self.cue_fade_in_starts.insert(current_cue.id().clone(), Instant::now());
            }
        }
    }

    pub fn next_cue(&self) -> Option<&Cue> {
        if self.current_cue().is_none() {
            return self.first_cue();
        }

        self.current_cue.as_ref().and_then(|id| self.cue_after(id))
    }

    pub fn has_fading_cue(&self) -> bool {
        !self.cue_fade_in_starts.is_empty() || !self.cue_fade_out_starts.is_empty()
    }

    pub fn cue_fade_progress(&self, id: &CueId) -> Option<f32> {
        if let Some(start) = self.cue_fade_in_starts.get(id) {
            if let Some(cue) = self.cue(id) {
                let elapsed = start.elapsed();
                let total = cue.fade_in_time();
                if total > Duration::from_millis(0) {
                    let progress = (elapsed.as_secs_f32() / total.as_secs_f32()).min(1.0);
                    return Some(progress);
                }
            }
        }
        if let Some(start) = self.cue_fade_out_starts.get(id) {
            if let Some(cue) = self.cue(id) {
                let elapsed = start.elapsed();
                let total = cue.fade_out_time();
                if total > Duration::from_millis(0) {
                    let progress = 1.0 - (elapsed.as_secs_f32() / total.as_secs_f32()).min(1.0);
                    return Some(progress);
                }
            }
        }
        None
    }

    pub(crate) fn update_fade_times(&mut self) {
        let fade_in_to_remove: Vec<_> = self
            .cue_fade_in_starts
            .iter()
            .filter_map(|(cue_id, start)| {
                if let Some(cue) = self.cue(cue_id) {
                    if start.elapsed() > cue.fade_in_time() { Some(cue_id.clone()) } else { None }
                } else {
                    Some(cue_id.clone())
                }
            })
            .collect();

        for cue_id in fade_in_to_remove {
            self.cue_fade_in_starts.remove(&cue_id);
        }

        let fade_out_to_remove: Vec<_> = self
            .cue_fade_out_starts
            .iter()
            .filter_map(|(cue_id, start)| {
                if let Some(cue) = self.cue(cue_id) {
                    if start.elapsed() > cue.fade_out_time() { Some(cue_id.clone()) } else { None }
                } else {
                    Some(cue_id.clone())
                }
            })
            .collect();

        for cue_id in fade_out_to_remove {
            self.cue_fade_out_starts.remove(&cue_id);
        }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cue {
    id: CueId,
    name: String,
    fade_in_time: Duration,
    fade_out_time: Duration,
    recipes: Vec<Recipe>,
}

impl Cue {
    pub fn id(&self) -> &CueId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fade_in_time(&self) -> Duration {
        self.fade_in_time
    }

    pub fn fade_out_time(&self) -> Duration {
        self.fade_out_time
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CueId(pub(crate) Vec<u32>);

impl std::fmt::Display for CueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."))
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    pub(crate) group_id: Option<ObjectId>,
    pub(crate) preset_id: Option<ObjectId>,
}

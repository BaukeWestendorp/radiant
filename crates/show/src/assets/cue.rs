use crate::showfile;

use super::{Effect, GroupId};

super::asset_id!(pub CueId);

#[derive(Debug, Clone)]
pub struct Cue {
    pub id: CueId,
    pub label: String,
    pub lines: Vec<CueLine>,
}

impl Cue {
    pub fn new(id: CueId) -> Self {
        Self {
            id,
            label: "New Cue".to_string(),
            lines: Vec::new(),
        }
    }
}

impl super::Asset for Cue {
    type Id = CueId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Cue {
    pub(crate) fn from_showfile(cue: showfile::Cue) -> Self {
        Self {
            id: CueId(cue.id),
            label: cue.label,
            lines: cue.lines.into_iter().map(CueLine::from_showfile).collect(),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::Cue {
        showfile::Cue {
            id: self.id.0,
            label: self.label.clone(),
            lines: self.lines.iter().map(CueLine::to_showfile).collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CueLine {
    pub effects: Vec<EffectInstance>,
}

impl CueLine {
    pub(crate) fn from_showfile(line: showfile::CueLine) -> Self {
        Self {
            effects: line
                .effects
                .into_iter()
                .map(EffectInstance::from_showfile)
                .collect(),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::CueLine {
        showfile::CueLine {
            effects: self
                .effects
                .iter()
                .map(EffectInstance::to_showfile)
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EffectInstance {
    pub group: GroupId,
    pub effect: Effect,
}

impl EffectInstance {
    pub(crate) fn from_showfile(instance: showfile::EffectInstance) -> Self {
        Self {
            group: GroupId(instance.group),
            effect: Effect::from_showfile(instance.effect),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::EffectInstance {
        showfile::EffectInstance {
            group: self.group.0,
            effect: self.effect.to_showfile(),
        }
    }
}

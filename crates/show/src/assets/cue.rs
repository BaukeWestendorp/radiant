use crate::showfile;

use super::{Effect, GroupId};

super::asset_id!(pub CueListId);

#[derive(Clone)]
pub struct CueList {
    pub id: CueListId,
    pub label: String,
    pub cues: Vec<Cue>,
}

impl CueList {
    pub fn new(id: CueListId) -> Self {
        Self {
            id,
            label: "New Cue List".to_string(),
            cues: Vec::new(),
        }
    }
}

impl super::Asset for CueList {
    type Id = CueListId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl CueList {
    pub(crate) fn from_showfile(list: showfile::CueList) -> Self {
        Self {
            id: CueListId(list.id),
            label: list.label,
            cues: list.cues.into_iter().map(Cue::from_showfile).collect(),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::CueList {
        showfile::CueList {
            id: self.id.0,
            label: self.label.clone(),
            cues: self.cues.iter().map(Cue::to_showfile).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cue {
    pub label: String,
    pub lines: Vec<CueLine>,
}

impl Cue {
    pub fn new() -> Self {
        Self {
            label: "New Cue".to_string(),
            lines: Vec::new(),
        }
    }
}

impl Cue {
    pub(crate) fn from_showfile(cue: showfile::Cue) -> Self {
        Self {
            label: cue.label,
            lines: cue.lines.into_iter().map(CueLine::from_showfile).collect(),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::Cue {
        showfile::Cue {
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

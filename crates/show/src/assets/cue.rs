use crate::showfile;

use super::{Effect, GroupId};

super::asset_id!(pub SequenceId);

#[derive(Clone)]
pub struct Sequence {
    pub id: SequenceId,
    pub label: String,
    pub cues: Vec<Cue>,
}

impl Sequence {
    pub fn new(id: SequenceId) -> Self {
        Self {
            id,
            label: "New Sequence".to_string(),
            cues: Vec::new(),
        }
    }
}

impl super::Asset for Sequence {
    type Id = SequenceId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn label(&self) -> &str {
        &self.label
    }
}

impl Sequence {
    pub(crate) fn from_showfile(list: showfile::Sequence) -> Self {
        Self {
            id: SequenceId(list.id),
            label: list.label,
            cues: list.cues.into_iter().map(Cue::from_showfile).collect(),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::Sequence {
        showfile::Sequence {
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

    pub fn line_at_index(&self, index: usize) -> Option<&CueLine> {
        self.lines.iter().find(|line| line.index == index)
    }

    pub fn line_at_index_mut(&mut self, index: usize) -> Option<&mut CueLine> {
        self.lines.iter_mut().find(|line| line.index == index)
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

#[derive(Debug, Clone)]
pub struct CueLine {
    pub label: String,
    pub group: GroupId,
    pub effect: Effect,
    pub index: usize,
}

impl CueLine {
    pub(crate) fn from_showfile(line: showfile::CueLine) -> Self {
        Self {
            label: line.label,
            group: line.group.into(),
            effect: Effect::from_showfile(line.effect),
            index: line.index,
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::CueLine {
        showfile::CueLine {
            label: self.label.clone(),
            group: self.group.into(),
            effect: self.effect.to_showfile(),
            index: self.index,
        }
    }
}

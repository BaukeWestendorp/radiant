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
    pub templates: Vec<Template>,
}

impl Cue {
    pub fn new() -> Self {
        Self {
            label: "New Cue".to_string(),
            templates: Vec::new(),
        }
    }
}

impl Cue {
    pub(crate) fn from_showfile(cue: showfile::Cue) -> Self {
        Self {
            label: cue.label,
            templates: cue
                .templates
                .into_iter()
                .map(Template::from_showfile)
                .collect(),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::Cue {
        showfile::Cue {
            label: self.label.clone(),
            templates: self.templates.iter().map(Template::to_showfile).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Template {
    pub label: String,
    pub group: GroupId,
    pub effect: Effect,
}

impl Template {
    pub(crate) fn from_showfile(template: showfile::Template) -> Self {
        Self {
            label: template.label,
            group: template.group.into(),
            effect: Effect::from_showfile(template.effect),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::Template {
        showfile::Template {
            label: self.label.clone(),
            group: self.group.into(),
            effect: self.effect.to_showfile(),
        }
    }
}

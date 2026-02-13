use std::collections::HashMap;

use gpui::{App, AppContext as _, Entity};
use zeevonk::project::FixtureId;

use crate::{
    layout::Layout,
    object::{Effect, EffectId, Group, GroupId},
    showfile::Showfile,
};

pub struct Show {
    groups: Entity<HashMap<GroupId, Group>>,
    effects: Entity<HashMap<EffectId, Effect>>,

    layout: Entity<Layout>,

    selection: Entity<Vec<FixtureId>>,
    modes: Entity<ShowModes>,
}

impl Show {
    pub fn from_showfile(showfile: Showfile, cx: &mut App) -> Self {
        let groups = cx.new(|_| showfile.groups().clone());
        let effects = cx.new(|_| showfile.effects().clone());
        let layout = cx.new(|_| showfile.layout().clone());

        Self {
            groups,
            effects,
            layout,
            selection: cx.new(|_| Vec::new()),
            modes: cx.new(|_| ShowModes::new()),
        }
    }

    pub fn groups(&self) -> Entity<HashMap<GroupId, Group>> {
        self.groups.clone()
    }

    pub fn effects(&self) -> Entity<HashMap<EffectId, Effect>> {
        self.effects.clone()
    }

    pub fn layout(&self) -> Entity<Layout> {
        self.layout.clone()
    }

    pub fn selection(&self) -> Entity<Vec<FixtureId>> {
        self.selection.clone()
    }

    pub fn modes(&self) -> Entity<ShowModes> {
        self.modes.clone()
    }
}

pub struct ShowModes {
    pub highlight: bool,
}

impl ShowModes {
    pub fn new() -> Self {
        Self { highlight: false }
    }
}

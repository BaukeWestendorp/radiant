use std::collections::HashMap;

use gpui::{App, AppContext as _, Entity};
use zeevonk::project::stage::FixtureId;

use crate::{
    layout::Layout,
    object::{Group, GroupId},
    showfile::Showfile,
};

pub struct Show {
    groups: Entity<HashMap<GroupId, Group>>,

    layout: Entity<Layout>,

    selection: Entity<Vec<FixtureId>>,
    modes: Entity<ShowModes>,
}

impl Show {
    pub fn from_showfile(showfile: Showfile, cx: &mut App) -> Self {
        let groups = cx.new(|_| showfile.groups().clone());
        let layout = cx.new(|_| showfile.layout().clone());

        Self {
            groups,
            layout,
            selection: cx.new(|_| Vec::new()),
            modes: cx.new(|_| ShowModes::new()),
        }
    }

    pub fn groups(&self) -> Entity<HashMap<GroupId, Group>> {
        self.groups.clone()
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

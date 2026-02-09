use std::collections::HashMap;

use gpui::{App, AppContext as _, Entity};
use zeevonk::project::stage::FixtureId;

use crate::{
    object::{Group, GroupId},
    showfile::Showfile,
};

pub struct Show {
    groups: Entity<HashMap<GroupId, Group>>,

    selection: Entity<Vec<FixtureId>>,
}

impl Show {
    pub fn from_showfile(showfile: &Showfile, cx: &mut App) -> Self {
        let groups = cx.new(|_| showfile.groups().clone());

        Self { groups, selection: cx.new(|_| Vec::new()) }
    }

    pub fn groups(&self) -> &Entity<HashMap<GroupId, Group>> {
        &self.groups
    }

    pub fn selection(&self) -> &Entity<Vec<FixtureId>> {
        &self.selection
    }
}

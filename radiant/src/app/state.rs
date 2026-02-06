use std::collections::HashMap;

use anyhow::Result;
use gpui::{App, AppContext, Entity, Global};
use zeevonk::{
    Zeevonk,
    attr::Attribute,
    project::{file::ProjectFile, stage::FixtureId},
    value::AttributeValues,
};

use crate::app::object::{Group, GroupId, Preset, PresetId};

pub(crate) fn init(zv_project_file: ProjectFile, cx: &mut App) -> Result<()> {
    let app_state = AppState::new(zv_project_file, cx)?;
    cx.set_global(app_state);
    Ok(())
}

pub struct AppState {
    zeevonk: Zeevonk,

    selection: Entity<Vec<FixtureId>>,
    groups: Entity<HashMap<GroupId, Group>>,
    presets: Entity<HashMap<PresetId, Preset>>,
}

impl AppState {
    pub fn new(zv_project_file: ProjectFile, cx: &mut App) -> Result<Self> {
        let zeevonk = Zeevonk::new(zv_project_file)?;
        zeevonk.start();

        let mut groups = HashMap::new();
        groups.insert(
            2,
            Group {
                name: "Spots".to_string(),
                fixture_ids: vec![
                    "101.1".parse().unwrap(),
                    "102.1".parse().unwrap(),
                    "103.1".parse().unwrap(),
                    "104.1".parse().unwrap(),
                ],
            },
        );
        groups.insert(
            3,
            Group {
                name: "LEDs".to_string(),
                fixture_ids: vec![
                    "501.1.2".parse().unwrap(),
                    "502.1.2".parse().unwrap(),
                    "503.1.2".parse().unwrap(),
                    "504.1.2".parse().unwrap(),
                ],
            },
        );
        groups.insert(
            4,
            Group {
                name: "Blinders".to_string(),
                fixture_ids: vec![
                    "501.1.1".parse().unwrap(),
                    "502.1.1".parse().unwrap(),
                    "503.1.1".parse().unwrap(),
                    "504.1.1".parse().unwrap(),
                ],
            },
        );

        let mut presets = HashMap::new();
        presets.insert(
            1,
            Preset {
                name: "Dimmer 50%".to_string(),
                attribute_values: {
                    let mut values = AttributeValues::new();
                    values.set("101.1".parse().unwrap(), Attribute::Dimmer, 0.5);
                    values
                },
            },
        );

        Ok(Self {
            zeevonk,
            selection: cx.new(|_| Vec::new()),
            groups: cx.new(|_| groups),
            presets: cx.new(|_| presets),
        })
    }

    pub fn zeevonk(&self) -> &Zeevonk {
        &self.zeevonk
    }

    pub fn selection(&self) -> &Entity<Vec<FixtureId>> {
        &self.selection
    }

    pub fn groups(&self) -> &Entity<HashMap<GroupId, Group>> {
        &self.groups
    }

    pub fn presets(&self) -> &Entity<HashMap<PresetId, Preset>> {
        &self.presets
    }
}

impl Global for AppState {}

use crate::showfile::{self, effect_graph};
use gpui::{App, AppContext as _, Entity, SharedString};
use std::{collections::HashMap, marker::PhantomData};

pub use crate::showfile::assets::{cue::*, effect_graph::*, fixture_group::*, presets::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetId<T>(u32, PhantomData<T>);

impl<T> AssetId<T> {
    pub fn new(id: u32) -> Self {
        Self(id, PhantomData)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl<T> Clone for AssetId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AssetId<T> {}

#[derive(Clone, Default)]
pub struct Assets {
    pub effect_graphs: AssetPool<EffectGraph>,
    pub fixture_groups: AssetPool<FixtureGroup>,
    pub dimmer_presets: AssetPool<DimmerPreset>,
    pub cues: AssetPool<Cue>,
}

impl Assets {
    pub(crate) fn from_showfile(assets: &showfile::Assets, cx: &mut gpui::App) -> Self {
        let mut effect_graphs = AssetPool::new();
        for (id, asset) in assets.effect_graphs.clone() {
            effect_graphs.insert(
                AssetId::new(id),
                cx.new(|_cx| {
                    let mut asset = Asset::from_showfile(asset);
                    effect_graph::insert_templates(&mut asset.data);
                    asset
                }),
            );
        }

        let mut fixture_groups = AssetPool::new();
        for (id, asset) in assets.fixture_groups.clone() {
            fixture_groups.insert(AssetId::new(id), cx.new(|_cx| Asset::from_showfile(asset)));
        }

        let mut cues = AssetPool::new();
        for (id, asset) in assets.cues.clone() {
            cues.insert(AssetId::new(id), cx.new(|_cx| Asset::from_showfile(asset)));
        }

        let mut dimmer_presets = AssetPool::new();
        for (id, asset) in assets.dimmer_presets.clone() {
            dimmer_presets.insert(AssetId::new(id), cx.new(|_cx| Asset::from_showfile(asset)));
        }

        Assets { effect_graphs, fixture_groups, dimmer_presets, cues }
    }

    pub(crate) fn to_showfile(&self, cx: &App) -> showfile::Assets {
        let mut effect_graphs = showfile::AssetPool::new();
        for (id, asset) in self.effect_graphs.assets.clone() {
            effect_graphs.insert(id, asset.read(cx).to_showfile());
        }

        let mut fixture_groups = showfile::AssetPool::new();
        for (id, asset) in self.fixture_groups.assets.clone() {
            fixture_groups.insert(id, asset.read(cx).to_showfile());
        }

        let mut cues = showfile::AssetPool::new();
        for (id, asset) in self.cues.assets.clone() {
            cues.insert(id, asset.read(cx).to_showfile());
        }

        let mut dimmer_presets = showfile::AssetPool::new();
        for (id, asset) in self.dimmer_presets.assets.clone() {
            dimmer_presets.insert(id, asset.read(cx).to_showfile());
        }

        showfile::Assets { effect_graphs, fixture_groups, dimmer_presets, cues }
    }
}

#[derive(Debug, Clone)]
pub struct AssetPool<T> {
    assets: HashMap<u32, Entity<Asset<T>>>,
}

impl<T> AssetPool<T> {
    pub fn new() -> Self {
        AssetPool { assets: HashMap::new() }
    }

    pub fn get(&self, id: &AssetId<T>) -> Option<&Entity<Asset<T>>> {
        self.assets.get(&id.as_u32())
    }

    pub fn assets(&self) -> impl Iterator<Item = (AssetId<T>, &Entity<Asset<T>>)> {
        self.assets.iter().map(|(id, asset)| (AssetId::new(*id), asset))
    }

    pub fn insert(&mut self, id: AssetId<T>, asset: Entity<Asset<T>>) {
        self.assets.insert(id.as_u32(), asset);
    }
}

impl<T> Default for AssetPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Asset<T> {
    pub id: AssetId<T>,
    pub label: SharedString,
    pub data: T,
}

impl<T> Asset<T> {
    pub(crate) fn from_showfile(asset: crate::showfile::Asset<T>) -> Self {
        Asset { id: AssetId::new(asset.id), label: asset.label.into(), data: asset.data }
    }

    pub(crate) fn to_showfile(&self) -> crate::showfile::Asset<T>
    where
        T: Clone,
    {
        showfile::Asset {
            id: self.id.as_u32(),
            label: self.label.to_string(),
            data: self.data.clone(),
        }
    }
}

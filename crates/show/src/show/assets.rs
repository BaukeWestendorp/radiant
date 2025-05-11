use crate::showfile::{self, effect_graph};
use gpui::{App, AppContext as _, Entity, SharedString};
use std::{collections::HashMap, marker::PhantomData};

pub use crate::showfile::assets::{effect_graph::*, fixture_group::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetId<T>(u32, PhantomData<T>);

impl<T> AssetId<T> {
    pub fn new(id: u32) -> Self {
        Self(id, PhantomData::default())
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Default)]
pub struct Assets {
    pub effect_graphs: AssetPool<EffectGraph>,
    pub fixture_groups: AssetPool<FixtureGroup>,
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

        Assets { effect_graphs, fixture_groups }
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

        showfile::Assets { effect_graphs, fixture_groups }
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

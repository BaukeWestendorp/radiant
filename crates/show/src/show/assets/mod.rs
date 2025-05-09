use crate::showfile::{self, effect_graph};
use gpui::{App, AppContext as _, Entity};
use std::collections::HashMap;

pub use crate::showfile::assets::{effect_graph::*, fixture_group::*};

#[macro_export]
macro_rules! define_asset {
    ($asset_name:ident, $asset_type:ident, $id:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $id(u32);

        impl $crate::assets::AssetId for $id {
            fn as_u32(&self) -> u32 {
                self.0
            }

            fn from_u32(id: u32) -> Self {
                $id(id)
            }
        }

        impl $id {
            pub const fn new(id: u32) -> Self {
                $id(id)
            }
        }

        impl From<u32> for $id {
            fn from(id: u32) -> Self {
                $id(id)
            }
        }

        impl From<$id> for u32 {
            fn from(id: $id) -> Self {
                id.0
            }
        }

        pub type $asset_type = $crate::assets::Asset<$asset_name, $id>;
    };
}

pub use define_asset;

pub trait AssetId: std::hash::Hash + Eq {
    fn as_u32(&self) -> u32;

    fn from_u32(id: u32) -> Self;
}

#[derive(Clone, Default)]
pub struct Assets {
    pub effect_graphs: AssetPool<EffectGraph, EffectGraphId>,
    pub fixture_groups: AssetPool<FixtureGroup, FixtureGroupId>,
}

impl Assets {
    pub(crate) fn from_showfile(assets: &showfile::Assets, cx: &mut gpui::App) -> Self {
        let mut effect_graphs = AssetPool::new();
        for (id, asset) in assets.effect_graphs.clone() {
            effect_graphs.insert(
                id.into(),
                cx.new(|_cx| {
                    let mut asset = Asset::from_showfile(asset);
                    effect_graph::insert_templates(&mut asset.data);
                    asset
                }),
            );
        }

        let mut fixture_groups = AssetPool::new();
        for (id, asset) in assets.fixture_groups.clone() {
            fixture_groups.insert(id.into(), cx.new(|_cx| Asset::from_showfile(asset)));
        }

        Assets { effect_graphs, fixture_groups }
    }

    pub(crate) fn to_showfile(&self, cx: &App) -> showfile::Assets {
        let mut effect_graphs = showfile::AssetPool::new();
        for (id, asset) in self.effect_graphs.assets.clone() {
            effect_graphs.insert(id.into(), asset.read(cx).to_showfile());
        }

        let mut fixture_groups = showfile::AssetPool::new();
        for (id, asset) in self.fixture_groups.assets.clone() {
            fixture_groups.insert(id.into(), asset.read(cx).to_showfile());
        }

        showfile::Assets { effect_graphs, fixture_groups }
    }
}

#[derive(Debug, Clone)]
pub struct AssetPool<T, Id: AssetId> {
    assets: HashMap<Id, Entity<Asset<T, Id>>>,
}

impl<T, Id: AssetId> AssetPool<T, Id> {
    pub fn new() -> Self {
        AssetPool { assets: HashMap::new() }
    }

    pub fn get(&self, id: &Id) -> Option<&Entity<Asset<T, Id>>> {
        self.assets.get(id)
    }

    pub fn insert(&mut self, id: Id, asset: Entity<Asset<T, Id>>) {
        self.assets.insert(id, asset);
    }
}

impl<T, Id: AssetId> Default for AssetPool<T, Id> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Asset<T, Id: AssetId> {
    pub id: Id,
    pub data: T,
}

impl<T, Id: AssetId> Asset<T, Id> {
    pub(crate) fn from_showfile(asset: crate::showfile::Asset<T>) -> Self {
        Asset { id: Id::from_u32(asset.id), data: asset.data }
    }

    pub(crate) fn to_showfile(&self) -> crate::showfile::Asset<T>
    where
        T: Clone,
    {
        showfile::Asset { id: self.id.as_u32(), data: self.data.clone() }
    }
}

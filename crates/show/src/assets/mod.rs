use crate::showfile::Showfile;
use gpui::{AppContext as _, Entity};
use std::collections::HashMap;

pub use effect_graph::*;

mod effect_graph;

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
            pub fn new(id: u32) -> Self {
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

pub trait AssetId {
    fn as_u32(&self) -> u32;

    fn from_u32(id: u32) -> Self;
}

#[derive(Clone, Default)]
pub struct Assets {
    pub effect_graphs: AssetPool<EffectGraph, EffectGraphId>,
}

impl Assets {
    pub(crate) fn from_showfile(showfile: &Showfile, cx: &mut gpui::App) -> Self {
        let mut effect_graphs = AssetPool::new();
        for (id, asset) in showfile.assets.effect_graphs.clone() {
            effect_graphs.insert(
                id.into(),
                cx.new(|_cx| {
                    let mut asset = Asset::from_showfile(asset);
                    effect_graph::insert_templates(&mut asset.data);
                    asset
                }),
            );
        }
        Assets { effect_graphs }
    }
}

#[derive(Debug, Clone)]
pub struct AssetPool<T, Id: AssetId> {
    assets: HashMap<u32, Entity<Asset<T, Id>>>,
}

impl<T, Id: AssetId> AssetPool<T, Id> {
    pub fn new() -> Self {
        AssetPool { assets: HashMap::new() }
    }

    pub fn get(&self, id: &Id) -> Option<&Entity<Asset<T, Id>>> {
        self.assets.get(&id.as_u32())
    }

    pub fn insert(&mut self, id: Id, asset: Entity<Asset<T, Id>>) {
        self.assets.insert(id.as_u32(), asset);
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
    pub fn from_showfile(asset: crate::showfile::Asset<T>) -> Self {
        Asset { id: Id::from_u32(asset.id), data: asset.data }
    }
}

pub mod cue;
pub mod effect;
pub mod effect_graph;
pub mod group;

use std::{collections::HashMap, hash::Hash};

use gpui::{AppContext, Context, Model};

pub use cue::*;
pub use effect::*;
pub use effect_graph::*;
pub use group::*;

#[derive(Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct AnyAssetId(pub u32);

macro_rules! asset_id {
    ($vis:vis $name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            Default,
            PartialEq,
            Eq,
            Ord,
            PartialOrd,
            Hash,
            serde::Serialize,
            serde::Deserialize,
        )]
        $vis struct $name(pub u32);

        impl From<$name> for crate::assets::AnyAssetId {
            fn from(id: $name) -> Self {
                crate::assets::AnyAssetId(id.0)
            }
        }

        impl From<crate::assets::AnyAssetId> for $name {
            fn from(id: crate::assets::AnyAssetId) -> Self {
                Self(id.0)
            }
        }

        impl From<u32> for $name {
            fn from(id: u32) -> Self {
                Self(id)
            }
        }

        impl From<$name> for u32 {
            fn from(id: $name) -> u32 {
                id.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse()?))
            }
        }
    };
}

pub(crate) use asset_id;

use crate::showfile;

#[derive(Debug, Clone)]
pub struct Assets {
    pub groups: Model<AssetPool<Group>>,
    pub cues: Model<AssetPool<Cue>>,
    pub effect_graphs: Model<AssetPool<EffectGraph>>,
}

impl Assets {
    pub(crate) fn from_showfile(assets: showfile::Assets, cx: &mut AppContext) -> Self {
        Self {
            groups: cx.new_model(|_| {
                assets
                    .groups
                    .into_iter()
                    .map(Group::from_showfile)
                    .collect::<Vec<_>>()
                    .into()
            }),
            cues: cx.new_model(|_| {
                assets
                    .cues
                    .into_iter()
                    .map(Cue::from_showfile)
                    .collect::<Vec<_>>()
                    .into()
            }),
            effect_graphs: cx.new_model(|_| {
                assets
                    .effect_graphs
                    .into_iter()
                    .map(EffectGraph::from_showfile)
                    .collect::<Vec<_>>()
                    .into()
            }),
        }
    }

    pub(crate) fn to_showfile(&self, cx: &AppContext) -> showfile::Assets {
        showfile::Assets {
            groups: self
                .groups
                .read(cx)
                .iter()
                .map(Group::to_showfile)
                .collect(),
            cues: self.cues.read(cx).iter().map(Cue::to_showfile).collect(),
            effect_graphs: self
                .effect_graphs
                .read(cx)
                .iter()
                .map(EffectGraph::to_showfile)
                .collect(),
        }
    }
}

pub trait Asset {
    type Id: PartialEq + Eq + Hash;
    fn id(&self) -> Self::Id;
}

#[derive(Debug, Clone)]
pub struct AssetPool<A: Asset> {
    assets: HashMap<A::Id, A>,
}

impl<A: Asset + 'static> AssetPool<A> {
    pub fn iter(&self) -> impl Iterator<Item = &A> {
        self.assets.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut A> {
        self.assets.values_mut()
    }

    pub fn get(&self, id: &A::Id) -> Option<&A> {
        self.assets
            .iter()
            .find(|(asset_id, _)| *asset_id == id)
            .map(|(_, asset)| asset)
    }

    pub fn get_mut(&mut self, id: &A::Id) -> Option<&mut A> {
        self.assets
            .iter_mut()
            .find(|(asset_id, _)| *asset_id == id)
            .map(|(_, asset)| asset)
    }

    pub fn insert(&mut self, id: A::Id, asset: A) {
        self.assets.insert(id, asset);
    }

    pub fn remove<'b, 'a>(&mut self, id: &A::Id) {
        self.assets.retain(|asset_id, _| asset_id != id);
    }
}

impl<A: Asset> From<Vec<A>> for AssetPool<A> {
    fn from(assets: Vec<A>) -> Self {
        Self {
            assets: assets
                .into_iter()
                .map(|asset| (asset.id(), asset))
                .collect(),
        }
    }
}

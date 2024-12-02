pub mod effect;
pub mod effect_graph;
pub mod group;

use gpui::{AppContext, Context, Model};

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

#[derive(Debug, Clone, PartialEq)]
pub struct Assets {
    pub groups: Model<AssetPool<Group>>,
    pub effects: Model<AssetPool<Effect>>,
    pub effect_graphs: Model<AssetPool<EffectGraph>>,
}

impl Assets {
    pub fn from_showfile(assets: showfile::Assets, cx: &mut AppContext) -> Self {
        Self {
            groups: cx.new_model(|_| {
                assets
                    .groups
                    .into_iter()
                    .map(Group::from)
                    .collect::<Vec<_>>()
                    .into()
            }),
            effects: cx.new_model(|_| {
                assets
                    .effects
                    .into_iter()
                    .map(Effect::from)
                    .collect::<Vec<_>>()
                    .into()
            }),
            effect_graphs: cx.new_model(|_| {
                assets
                    .effect_graphs
                    .into_iter()
                    .map(EffectGraph::from)
                    .collect::<Vec<_>>()
                    .into()
            }),
        }
    }
}

pub trait Asset {
    type Id: PartialEq;
    fn id(&self) -> &Self::Id;
}

pub struct AssetPool<A: Asset> {
    assets: Vec<A>,
}

impl<A: Asset> AssetPool<A> {
    pub fn iter(&self) -> impl Iterator<Item = &A> {
        self.assets.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut A> {
        self.assets.iter_mut()
    }

    pub fn get(&self, id: &A::Id) -> Option<&A> {
        self.assets.iter().find(|asset| asset.id() == id)
    }

    pub fn get_mut(&mut self, id: &A::Id) -> Option<&mut A> {
        self.assets.iter_mut().find(|asset| asset.id() == id)
    }

    pub fn insert(&mut self, asset: A) {
        self.assets.push(asset);
    }

    pub fn remove(&mut self, id: &A::Id) -> Option<A> {
        let index = self.assets.iter().position(|asset| asset.id() == id)?;
        Some(self.assets.remove(index))
    }
}

impl<A: Asset> From<Vec<A>> for AssetPool<A> {
    fn from(assets: Vec<A>) -> Self {
        Self { assets }
    }
}

use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub mod cue;
pub mod effect_graph;
pub mod executor;
pub mod fixture_group;
pub mod preset;
pub mod sequence;

use super::attr::DimmerAttr;

pub use {cue::*, executor::*, fixture_group::*, preset::*, sequence::*};

#[derive(Clone)]
pub struct Assets {
    pub effect_graphs: AssetPool<effect_graph::EffectGraph>,
    pub fixture_groups: AssetPool<FixtureGroup>,

    pub cues: AssetPool<Cue>,
    pub sequences: AssetPool<Sequence>,
    pub executors: AssetPool<Executor>,

    pub dimmer_presets: AssetPool<Preset<DimmerAttr>>,
}

#[derive(Debug, Clone)]
pub struct AssetPool<T>(HashMap<AssetId<T>, gpui::Entity<Asset<T>>>);

impl<T> AssetPool<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<T> Deref for AssetPool<T> {
    type Target = HashMap<AssetId<T>, gpui::Entity<Asset<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for AssetPool<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Asset<T> {
    pub id: AssetId<T>,
    pub label: String,
    pub data: T,
}

#[derive(PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct AssetId<T>(u32, #[serde(skip)] PhantomData<T>);

impl<T> AssetId<T> {
    pub fn new(id: u32) -> Self {
        Self(id, PhantomData)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl<T> std::fmt::Debug for AssetId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> Clone for AssetId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AssetId<T> {}

impl<T> PartialEq for AssetId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for AssetId<T> {}

impl<T> std::hash::Hash for AssetId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub(crate) mod showfile {
    use std::collections::HashMap;

    use gpui::AppContext as _;

    use crate::show::attr::DimmerAttr;

    use super::{
        Asset, AssetId, Cue, Executor, FixtureGroup, Preset, Sequence,
        effect_graph::{self, EffectGraph},
    };

    #[derive(Default)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Assets {
        pub effect_graphs: AssetPool<EffectGraph>,
        pub fixture_groups: AssetPool<FixtureGroup>,

        pub cues: AssetPool<Cue>,
        pub sequences: AssetPool<Sequence>,
        pub executors: AssetPool<Executor>,

        pub dimmer_presets: AssetPool<Preset<DimmerAttr>>,
    }

    impl Assets {
        pub fn into_show(self, cx: &mut gpui::App) -> super::Assets {
            let mut effect_graphs = self.effect_graphs.to_show(cx);
            for (_, asset) in &mut effect_graphs.0 {
                asset.update(cx, |asset, _cx| {
                    effect_graph::templates::insert_templates(&mut asset.data);
                })
            }

            super::Assets {
                effect_graphs,
                fixture_groups: self.fixture_groups.to_show(cx),
                cues: self.cues.to_show(cx),
                sequences: self.sequences.to_show(cx),
                executors: self.executors.to_show(cx),
                dimmer_presets: self.dimmer_presets.to_show(cx),
            }
        }

        pub fn from_show(from: super::Assets, cx: &gpui::App) -> Self {
            Self {
                effect_graphs: AssetPool::from_show(from.effect_graphs, cx),
                fixture_groups: AssetPool::from_show(from.fixture_groups, cx),
                cues: AssetPool::from_show(from.cues, cx),
                sequences: AssetPool::from_show(from.sequences, cx),
                executors: AssetPool::from_show(from.executors, cx),
                dimmer_presets: AssetPool::from_show(from.dimmer_presets, cx),
            }
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct AssetPool<T>(HashMap<AssetId<T>, Asset<T>>);

    impl<T> Default for AssetPool<T> {
        fn default() -> Self {
            Self(HashMap::default())
        }
    }

    impl<T: 'static> AssetPool<T> {
        pub fn to_show(self, cx: &mut gpui::App) -> super::AssetPool<T> {
            let mut pool = super::AssetPool::new();
            for (id, asset) in self.0 {
                pool.insert(id, cx.new(|_| asset));
            }
            pool
        }

        pub fn from_show(from: super::AssetPool<T>, cx: &gpui::App) -> Self
        where
            T: Clone,
        {
            let mut hashmap = HashMap::new();
            for (id, asset) in from.0 {
                hashmap.insert(id, asset.read(cx).clone());
            }
            Self(hashmap)
        }
    }
}

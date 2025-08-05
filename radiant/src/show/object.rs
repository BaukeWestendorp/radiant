use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};

use crate::show::preset::PresetContent;
use crate::show::{FixtureId, Show};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(derive_more::Deref, derive_more::From, derive_more::Into, derive_more::FromStr)]
#[derive(serde::Deserialize)]
pub struct ObjectId(uuid::Uuid);

#[derive(Debug)]
pub struct PoolId<T>(NonZeroU32, PhantomData<T>);

impl<'de, T> serde::Deserialize<'de> for PoolId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = NonZeroU32::deserialize(deserializer)?;
        Ok(PoolId(id, PhantomData))
    }
}

impl<T> PoolId<T> {
    pub fn new(id: NonZeroU32) -> Self {
        PoolId(id, PhantomData)
    }
}

impl<T> Clone for PoolId<T> {
    fn clone(&self) -> Self {
        PoolId(self.0, PhantomData)
    }
}

impl<T> Copy for PoolId<T> {}

impl<T> PartialEq for PoolId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for PoolId<T> {}

impl<T> PartialOrd for PoolId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for PoolId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> std::hash::Hash for PoolId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl ObjectId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

pub trait Object: Debug + Clone + TryFrom<AnyObject>
where
    Self: Sized,
{
    fn pool_id(&self) -> PoolId<Self>;

    fn id(&self) -> ObjectId;

    fn name(&self) -> &str;
}

#[macro_export]
macro_rules! define_objects {
    (
        $(
            $obj_vis:vis struct $obj:ident {
                $( $(#[$field_attr:meta])* $field_vis:vis $field:ident : $field_ty:ty, )*
            }
        )*
    ) => {
        #[derive(Debug, Clone)]
        #[derive(serde::Deserialize)]
        pub enum AnyObject {
            $(
                $obj($obj),
            )*
        }

        impl AnyObject {
            pub fn id(&self) -> ObjectId {
                match self {
                    $(Self::$obj(obj) => obj.id().into(),)*
                }
            }

            pub fn name(&self) -> &str {
                match self {
                    $(Self::$obj(obj) => obj.name.as_str(),)*
                }
            }
        }

        $(
            #[derive(Debug, Clone)]
            #[derive(serde::Deserialize)]
            $obj_vis struct $obj {
                pub(crate) pool_id: PoolId<Self>,
                pub(super) id: ObjectId,
                pub(crate) name: String,

                $( $(#[$field_attr])* $field_vis $field : $field_ty, )*
            }

            impl Object for $obj {
                fn pool_id(&self) -> PoolId<Self> {
                    self.pool_id
                }

                fn id(&self) -> ObjectId {
                    self.id
                }

                fn name(&self) -> &str {
                    &self.name
                }
            }

            impl TryFrom<AnyObject> for $obj {
                type Error = crate::error::Error;

                fn try_from(object: AnyObject) -> Result<Self, Self::Error> {
                    match object {
                        AnyObject::$obj(obj) => Ok(obj),
                        _ => eyre::bail!("failed to convert object"),
                    }
                }
            }

            impl From<$obj> for AnyObject {
                fn from(object: $obj) -> Self {
                    Self::$obj(object)
                }
            }

            impl<'a> TryFrom<&'a AnyObject> for &'a $obj {
                type Error = crate::error::Error;

                fn try_from(object: &'a AnyObject) -> Result<Self, Self::Error> {
                    match object {
                        AnyObject::$obj(obj) => Ok(obj),
                        _ => eyre::bail!("failed to convert object"),
                    }
                }
            }
        )*
    };
}

define_objects! {
    pub struct Group {
        pub(crate) fids: Vec<FixtureId>,
    }

    pub struct Sequence {
        cues: HashMap<CueId, Cue>,

        current_cue: Option<CueId>,

        #[serde(skip)]
        pub(crate) cue_fade_in_starts: HashMap<CueId, Instant>,
        #[serde(skip)]
        pub(crate) cue_fade_out_starts: HashMap<CueId, Instant>,
    }

    pub struct Executor {
        pub(crate) sequence_id: Option<ObjectId>,
        pub(crate) is_on: bool,
    }

    pub struct PresetDimmer { pub(crate) content: PresetContent, }
    pub struct PresetPosition { pub(crate) content: PresetContent, }
    pub struct PresetGobo { pub(crate) content: PresetContent, }
    pub struct PresetColor { pub(crate) content: PresetContent, }
    pub struct PresetBeam { pub(crate) content: PresetContent, }
    pub struct PresetFocus { pub(crate) content: PresetContent, }
    pub struct PresetControl { pub(crate) content: PresetContent, }
    pub struct PresetShapers { pub(crate) content: PresetContent, }
    pub struct PresetVideo { pub(crate) content: PresetContent, }
}

impl Group {
    pub fn new(pool_id: PoolId<Self>, name: String, fids: Vec<FixtureId>) -> Self {
        Self { pool_id, id: ObjectId::new(), name, fids }
    }

    pub fn fids(&self) -> &[FixtureId] {
        &self.fids
    }
}

impl Sequence {
    pub fn new(pool_id: PoolId<Self>, name: String) -> Self {
        Self {
            pool_id,
            id: ObjectId::new(),
            name,
            cues: HashMap::new(),
            current_cue: None,
            cue_fade_in_starts: HashMap::new(),
            cue_fade_out_starts: HashMap::new(),
        }
    }

    pub fn cue(&self, id: &CueId) -> Option<&Cue> {
        self.cues.get(id)
    }

    pub fn cues(&self) -> impl IntoIterator<Item = &Cue> {
        self.cues.values()
    }

    pub fn active_cues(&self) -> impl IntoIterator<Item = &Cue> {
        self.cues.values().filter(|cue| {
            let id = &cue.id;
            let is_current = self.current_cue.as_ref().map_or(false, |current_id| current_id == id);
            let is_fading_in = self.cue_fade_in_starts.contains_key(id);
            let is_fading_out = self.cue_fade_out_starts.contains_key(id);
            is_current || is_fading_in || is_fading_out
        })
    }

    pub fn first_cue(&self) -> Option<&Cue> {
        self.cues.values().min_by_key(|cue| &cue.id)
    }

    pub fn last_cue(&self) -> Option<&Cue> {
        self.cues.values().max_by_key(|cue| &cue.id)
    }

    pub fn cue_before(&self, id: &CueId) -> Option<&Cue> {
        self.cues.values().filter(|cue| cue.id < *id).max_by_key(|cue| &cue.id)
    }

    pub fn cue_after(&self, id: &CueId) -> Option<&Cue> {
        self.cues.values().filter(|cue| cue.id > *id).min_by_key(|cue| &cue.id)
    }

    pub fn previous_cue(&self) -> Option<&Cue> {
        if self.current_cue().is_none() {
            return self.last_cue();
        }

        self.cue_before(self.current_cue.as_ref()?)
    }

    pub fn current_cue(&self) -> Option<&Cue> {
        self.current_cue.as_ref().and_then(|id| self.cues.get(id))
    }

    pub fn set_current_cue(&mut self, id: Option<CueId>) {
        if let Some(current_cue) = self.current_cue() {
            if current_cue.fade_out_time() > Duration::from_millis(0) {
                self.cue_fade_out_starts.insert(current_cue.id().clone(), Instant::now());
            }
        }

        self.current_cue = id;

        if let Some(current_cue) = self.current_cue() {
            if current_cue.fade_in_time() > Duration::from_millis(0) {
                self.cue_fade_in_starts.insert(current_cue.id().clone(), Instant::now());
            }
        }
    }

    pub fn next_cue(&self) -> Option<&Cue> {
        if self.current_cue().is_none() {
            return self.first_cue();
        }

        self.current_cue.as_ref().and_then(|id| self.cue_after(id))
    }

    pub fn has_fading_cue(&self) -> bool {
        !self.cue_fade_in_starts.is_empty() || !self.cue_fade_out_starts.is_empty()
    }

    pub fn cue_fade_progress(&self, id: &CueId) -> Option<f32> {
        if let Some(start) = self.cue_fade_in_starts.get(id) {
            if let Some(cue) = self.cue(id) {
                let elapsed = start.elapsed();
                let total = cue.fade_in_time();
                if total > Duration::from_millis(0) {
                    let progress = (elapsed.as_secs_f32() / total.as_secs_f32()).min(1.0);
                    return Some(progress);
                }
            }
        }
        if let Some(start) = self.cue_fade_out_starts.get(id) {
            if let Some(cue) = self.cue(id) {
                let elapsed = start.elapsed();
                let total = cue.fade_out_time();
                if total > Duration::from_millis(0) {
                    let progress = 1.0 - (elapsed.as_secs_f32() / total.as_secs_f32()).min(1.0);
                    return Some(progress);
                }
            }
        }
        None
    }

    pub(crate) fn update_fade_times(&mut self) {
        let fade_in_to_remove: Vec<_> = self
            .cue_fade_in_starts
            .iter()
            .filter_map(|(cue_id, start)| {
                if let Some(cue) = self.cue(cue_id) {
                    if start.elapsed() > cue.fade_in_time() { Some(cue_id.clone()) } else { None }
                } else {
                    Some(cue_id.clone())
                }
            })
            .collect();

        for cue_id in fade_in_to_remove {
            self.cue_fade_in_starts.remove(&cue_id);
        }

        let fade_out_to_remove: Vec<_> = self
            .cue_fade_out_starts
            .iter()
            .filter_map(|(cue_id, start)| {
                if let Some(cue) = self.cue(cue_id) {
                    if start.elapsed() > cue.fade_out_time() { Some(cue_id.clone()) } else { None }
                } else {
                    Some(cue_id.clone())
                }
            })
            .collect();

        for cue_id in fade_out_to_remove {
            self.cue_fade_out_starts.remove(&cue_id);
        }
    }
}

impl Executor {
    pub fn new(pool_id: PoolId<Self>, name: String) -> Self {
        Self { pool_id, id: ObjectId::new(), name, sequence_id: None, is_on: false }
    }

    pub fn sequence_id(&self) -> Option<ObjectId> {
        self.sequence_id
    }

    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        self.sequence_id.and_then(|id| show.sequence(&id))
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct Cue {
    id: CueId,
    name: String,
    fade_in_time: Duration,
    fade_out_time: Duration,
    recipes: Vec<Recipe>,
}

impl Cue {
    pub fn id(&self) -> &CueId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fade_in_time(&self) -> Duration {
        self.fade_in_time
    }

    pub fn fade_out_time(&self) -> Duration {
        self.fade_out_time
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
pub struct CueId(pub(crate) Vec<u32>);

impl std::fmt::Display for CueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."))
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct Recipe {
    pub(crate) group_id: Option<ObjectId>,
    pub(crate) preset_id: Option<ObjectId>,
}

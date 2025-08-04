use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use std::time::{Duration, Instant};

use crate::show::preset::PresetContent;
use crate::show::{AnyPresetId, FixtureId, Show};

pub trait Object: Debug + Clone
where
    Self: Sized,
{
    fn id(&self) -> ObjectId<Self>;

    fn name(&self) -> &str;
}

#[derive(Debug, PartialOrd, Ord)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct ObjectId<T>(u32, PhantomData<T>);

impl<T> ObjectId<T> {
    pub fn new(id: u32) -> Self {
        Self(id, PhantomData::default())
    }
}

impl<T> Clone for ObjectId<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData::default())
    }
}

impl<T> Copy for ObjectId<T> {}

impl<T> PartialEq for ObjectId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T> Eq for ObjectId<T> {}

impl<T> Hash for ObjectId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl<T> From<ObjectId<T>> for u32 {
    fn from(id: ObjectId<T>) -> Self {
        id.0
    }
}

impl<T> From<u32> for ObjectId<T> {
    fn from(id: u32) -> Self {
        Self::new(id)
    }
}

impl<T> Deref for ObjectId<T> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[derive(serde::Deserialize)]
        pub enum AnyObjectId {
            $(
                $obj(u32),
            )*
        }

        #[derive(Debug, Clone)]
        #[derive(serde::Deserialize)]
        pub enum AnyObject {
            $(
                $obj($obj),
            )*
        }

        impl AnyObject {
            pub fn id(&self) -> AnyObjectId {
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
                pub(crate) id: ObjectId<$obj>,
                pub(crate) name: String,

                $( $(#[$field_attr])* $field_vis $field : $field_ty, )*
            }

            impl Object for $obj {
                fn id(&self) -> ObjectId<Self> {
                    self.id
                }

                fn name(&self) -> &str {
                    &self.name
                }
            }

            impl TryFrom<AnyObjectId> for ObjectId<$obj> {
                type Error = crate::error::Error;

                fn try_from(id: AnyObjectId) -> Result<Self, Self::Error> {
                    match id {
                        AnyObjectId::$obj(id) => Ok(ObjectId::<$obj>::new(id)),
                        _ => eyre::bail!("failed to convert id"),
                    }
                }
            }

            impl From<ObjectId<$obj>> for AnyObjectId {
                fn from(id: ObjectId<$obj>) -> Self {
                    AnyObjectId::$obj(id.into())
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
        pub(crate) sequence_id: Option<ObjectId<Sequence>>,
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
    pub fn fids(&self) -> &[FixtureId] {
        &self.fids
    }
}

impl Sequence {
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
    pub fn sequence_id(&self) -> Option<ObjectId<Sequence>> {
        self.sequence_id
    }

    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        self.sequence_id.and_then(|sequence_id| show.sequences.get(sequence_id))
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
    pub(crate) group: Option<ObjectId<Group>>,
    pub(crate) preset: Option<AnyPresetId>,
}

pub struct ObjectPool<T: Object> {
    objects: HashMap<ObjectId<T>, T>,
}

impl<T: Object> ObjectPool<T> {
    pub fn new() -> Self {
        Self { objects: HashMap::new() }
    }

    pub fn get(&self, id: impl Into<ObjectId<T>>) -> Option<&T> {
        self.objects.get(&id.into())
    }

    pub fn objects(&self) -> impl IntoIterator<Item = &T> {
        self.objects.values()
    }

    pub(crate) fn get_mut(&mut self, id: impl Into<ObjectId<T>>) -> Option<&mut T> {
        self.objects.get_mut(&id.into())
    }

    pub(crate) fn insert(&mut self, object: T) {
        self.objects.insert(object.id(), object);
    }
}

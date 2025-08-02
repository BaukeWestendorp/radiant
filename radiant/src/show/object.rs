use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;

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
                $( $field_vis:vis $field:ident : $field_ty:ty, )*
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

                $( $field_vis $field : $field_ty, )*
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
        pub(crate) cues: Vec<Cue>,
        pub(crate) active_cue: Option<CueId>,
    }

    pub struct Executor {
        pub(crate) sequence_id: Option<ObjectId<Sequence>>,
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
    pub fn previous_cue(&self) -> Option<&Cue> {
        let mut index = self.active_cue_index()?;
        if index == 0 {
            index = self.cues.len();
        } else {
            index -= 1;
        }
        self.cue_at(index)
    }

    pub fn active_cue(&self) -> Option<&Cue> {
        self.active_cue.as_ref().and_then(|id| self.cues.iter().find(|cue| cue.id == *id))
    }

    pub fn set_active_cue(&mut self, id: Option<CueId>) {
        self.active_cue = id;
    }

    pub fn next_cue(&self) -> Option<&Cue> {
        let mut index = self.active_cue_index()?;
        if index == self.cues.len() {
            index = 0;
        } else {
            index += 1;
        }
        self.cue_at(index)
    }

    pub fn active_cue_index(&self) -> Option<usize> {
        self.active_cue
            .as_ref()
            .and_then(|cue_id| self.cues.iter().position(|cue| cue.id == *cue_id))
    }

    pub fn next_cue_index(&self) -> Option<usize> {
        let active_index = self.active_cue_index()?;
        let active_id = self.cue_at(active_index)?.id.clone();

        self.cues.iter().enumerate().filter(|(_, cue)| cue.id > active_id).map(|(idx, _)| idx).min()
    }

    pub fn cue_at(&self, index: usize) -> Option<&Cue> {
        self.cues.get(index)
    }
}

impl Executor {
    pub fn sequence_id(&self) -> Option<ObjectId<Sequence>> {
        self.sequence_id
    }

    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        self.sequence_id.and_then(|sequence_id| show.sequences.get(sequence_id))
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct Cue {
    pub(crate) id: CueId,
    pub(crate) name: String,
    pub(crate) recipes: Vec<Recipe>,
}

impl Cue {
    pub fn id(&self) -> &CueId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize)]
pub struct CueId(pub(crate) Vec<u32>);

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

    pub(crate) fn get_mut(&mut self, id: impl Into<ObjectId<T>>) -> Option<&mut T> {
        self.objects.get_mut(&id.into())
    }

    pub(crate) fn insert(&mut self, object: T) {
        self.objects.insert(object.id(), object);
    }
}

use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::num::NonZeroU32;

use uuid::Uuid;

pub use executor::*;
pub use group::*;
pub use preset::*;
pub use sequence::*;

mod executor;
mod group;
mod preset;
mod sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(derive_more::Display)]
pub enum ObjectKind {
    #[display("group")]
    Group,
    #[display("executor")]
    Executor,
    #[display("sequence")]
    Sequence,

    #[display("preset::dimmer")]
    PresetDimmer,
    #[display("preset::position")]
    PresetPosition,
    #[display("preset::gobo")]
    PresetGobo,
    #[display("preset::color")]
    PresetColor,
    #[display("preset::beam")]
    PresetBeam,
    #[display("preset::focus")]
    PresetFocus,
    #[display("preset::control")]
    PresetControl,
    #[display("preset::shapers")]
    PresetShapers,
    #[display("preset::video")]
    PresetVideo,
}

pub trait Object: Any + Send {
    fn create(id: ObjectId, pool_id: PoolId, name: String) -> Self
    where
        Self: Default;

    fn name(&self) -> &str;

    fn set_name(&mut self, name: String);

    fn id(&self) -> ObjectId;

    fn pool_id(&self) -> PoolId;

    fn set_pool_id(&mut self, pool_id: PoolId);

    fn kind() -> ObjectKind
    where
        Self: Sized;
}

#[derive(Default)]
pub struct ObjectContainer {
    objects: HashMap<ObjectId, Box<dyn Object>>,
}

impl ObjectContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<T: Object>(&self, id: ObjectId) -> Option<&T> {
        self.objects.get(&id)?.as_ref().as_any().downcast_ref::<T>()
    }

    pub fn get_mut<T: Object>(&mut self, id: ObjectId) -> Option<&mut T> {
        self.objects.get_mut(&id)?.as_mut().as_mut_any().downcast_mut::<T>()
    }

    pub fn get_by_pool_id<T: Object>(&self, pool_id: PoolId) -> Option<&T> {
        self.objects.values().find_map(|obj| {
            let obj_ref = obj.as_ref().as_any().downcast_ref::<T>()?;
            if obj_ref.pool_id() == pool_id { Some(obj_ref) } else { None }
        })
    }

    pub fn get_mut_by_pool_id<T: Object>(&mut self, pool_id: PoolId) -> Option<&mut T> {
        self.objects.values_mut().find_map(|obj| {
            let obj_mut = obj.as_mut().as_mut_any().downcast_mut::<T>()?;
            if obj_mut.pool_id() == pool_id { Some(obj_mut) } else { None }
        })
    }

    pub fn all<T: Object>(&self) -> impl Iterator<Item = &T> {
        self.objects.values().filter_map(|obj| obj.as_ref().as_any().downcast_ref::<T>())
    }

    pub fn insert<T: Object>(&mut self, object: T) {
        self.objects.insert(object.id(), Box::new(object));
    }

    pub fn remove(&mut self, id: &ObjectId) -> Option<Box<dyn Object>> {
        self.objects.remove(id)
    }

    pub fn contains(&self, id: &ObjectId) -> bool {
        self.objects.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ObjectId, &Box<dyn Object>)> {
        self.objects.iter()
    }
}

impl dyn Object {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(
    derive_more::Display,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(
    derive_more::Display,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PoolId(NonZeroU32);

impl Default for PoolId {
    fn default() -> Self {
        Self(NonZeroU32::new(1).unwrap())
    }
}

impl PoolId {
    pub fn new(id: NonZeroU32) -> Self {
        PoolId(id)
    }
}

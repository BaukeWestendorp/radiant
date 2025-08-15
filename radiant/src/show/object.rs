use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(derive_more::Deref, derive_more::From, derive_more::Into, derive_more::FromStr)]
#[derive(serde::Deserialize)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub trait Object: Any + Send {
    fn create(id: ObjectId, pool_id: PoolId<Self>, name: String) -> Self
    where
        Self: Sized + Default;

    fn name(&self) -> &str;

    fn set_name(&mut self, name: String);

    fn id(&self) -> ObjectId;

    fn pool_id(&self) -> PoolId<Self>
    where
        Self: Sized;

    fn set_pool_id(&mut self, pool_id: PoolId<Self>)
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

    pub fn get_by_pool_id<T: Object>(&self, pool_id: PoolId<T>) -> Option<&T> {
        self.objects.values().find_map(|obj| {
            let obj_ref = obj.as_ref().as_any().downcast_ref::<T>()?;
            if obj_ref.pool_id() == pool_id { Some(obj_ref) } else { None }
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

impl<T> Default for PoolId<T> {
    fn default() -> Self {
        Self(NonZeroU32::new(1).unwrap(), PhantomData::default())
    }
}

use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    fmt,
    num::NonZeroU32,
    ops,
};
use uuid::Uuid;

mod error;
mod objects;

pub use error::*;
pub use objects::*;

pub trait Object: 'static + Send + Sync {
    fn id(&self) -> ObjectId;
    fn name(&self) -> &str;
}

// A generic container for any object type.
pub struct ObjectContainer<T> {
    items: BTreeMap<ObjectId, T>,
}

impl<T> Default for ObjectContainer<T> {
    fn default() -> Self {
        Self { items: Default::default() }
    }
}

#[derive(Default)]
pub struct ObjectRegistry {
    maps: BTreeMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ObjectRegistry {
    pub fn new() -> Self {
        Self { maps: BTreeMap::new() }
    }

    pub fn insert<T: Object + 'static>(&mut self, item: T) {
        let type_id = TypeId::of::<T>();
        let container = self
            .maps
            .entry(type_id)
            .or_insert_with(|| Box::new(ObjectContainer::<T>::default()))
            .downcast_mut::<ObjectContainer<T>>()
            .unwrap();

        container.items.insert(item.id(), item);
    }

    pub fn get<T: Object + 'static>(&self, id: ObjectId) -> Option<&T> {
        let container = self.maps.get(&TypeId::of::<T>())?.downcast_ref::<ObjectContainer<T>>()?;

        container.items.get(&id)
    }

    pub fn get_all<T: Object + 'static>(&self) -> Vec<&T> {
        match self.maps.get(&TypeId::of::<T>()) {
            Some(boxed) => {
                let container = boxed.downcast_ref::<ObjectContainer<T>>().unwrap();
                container.items.values().collect()
            }
            None => Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Slot(NonZeroU32);

impl Slot {
    pub fn new(slot: u32) -> Result<Self, Error> {
        NonZeroU32::new(slot).map(Self).ok_or(Error::ZeroSlot)
    }

    pub fn new_unchecked(slot: u32) -> Self {
        Self(NonZeroU32::new(slot).expect("slot cannot be zero"))
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Deref for Slot {
    type Target = NonZeroU32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Slot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl ops::Deref for ObjectId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ObjectId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

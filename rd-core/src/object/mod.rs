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
use zeevonk::project::FixtureId;

pub trait Object: 'static + Send + Sync {
    fn kind() -> ObjectKind
    where
        Self: Sized;

    fn id(&self) -> ObjectId;

    fn slot_id(&self) -> SlotId;

    fn name(&self) -> &str;
}

/// A generic container for any object type.
pub struct ObjectContainer<T> {
    items: BTreeMap<ObjectId, T>,
    slot_index: BTreeMap<SlotId, ObjectId>,
}

impl<T: Object> Default for ObjectContainer<T> {
    fn default() -> Self {
        Self { items: Default::default(), slot_index: Default::default() }
    }
}

impl<T: Object> ObjectContainer<T> {
    pub fn insert(&mut self, item: T) {
        let id = item.id();
        let slot_id = item.slot_id();
        self.slot_index.insert(slot_id, id);
        self.items.insert(id, item);
    }

    pub fn get_by_object_id(&self, id: &ObjectId) -> Option<&T> {
        self.items.get(id)
    }

    pub fn get_by_slot_id(&self, slot_id: &SlotId) -> Option<&T> {
        let slot_id = slot_id.try_into().ok()?;
        self.slot_index.get(slot_id).and_then(|id| self.items.get(id))
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.items.values()
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

        container.insert(item);
    }

    pub fn get<T: Object + 'static>(&self, obj: impl Into<ObjectReference>) -> Option<&T> {
        let container = self.maps.get(&TypeId::of::<T>())?.downcast_ref::<ObjectContainer<T>>()?;

        match obj.into() {
            ObjectReference::ObjectId(id) => container.get_by_object_id(&id),
            ObjectReference::Slot(kind, slot_id) => {
                if T::kind() == kind {
                    container.get_by_slot_id(&slot_id)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_all<T: Object + 'static>(&self) -> Vec<&T> {
        match self.maps.get(&TypeId::of::<T>()) {
            Some(boxed) => {
                let container = boxed.downcast_ref::<ObjectContainer<T>>().unwrap();
                container.values().collect()
            }
            None => Vec::new(),
        }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SlotId(NonZeroU32);

impl SlotId {
    pub fn new(slot_id: u32) -> Result<Self, Error> {
        NonZeroU32::new(slot_id).map(Self).ok_or(Error::ZeroSlotId)
    }

    pub fn new_unchecked(slot_id: u32) -> Self {
        Self(NonZeroU32::new(slot_id).expect("slot id cannot be zero"))
    }

    pub fn as_u32(&self) -> u32 {
        self.0.into()
    }
}

impl From<NonZeroU32> for SlotId {
    fn from(nz: NonZeroU32) -> Self {
        SlotId(nz)
    }
}

impl From<SlotId> for NonZeroU32 {
    fn from(slot_id: SlotId) -> Self {
        slot_id.0
    }
}

impl From<SlotId> for u32 {
    fn from(slot_id: SlotId) -> Self {
        slot_id.0.get()
    }
}

impl TryFrom<u32> for SlotId {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        SlotId::new(value)
    }
}

impl TryFrom<i32> for SlotId {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 { SlotId::new(value as u32) } else { Err(Error::ZeroSlotId) }
    }
}

impl From<SlotId> for i32 {
    fn from(slot_id: SlotId) -> Self {
        slot_id.0.get() as i32
    }
}

impl ops::Deref for SlotId {
    type Target = NonZeroU32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for SlotId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for SlotId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ObjectReference {
    ObjectId(ObjectId),
    Slot(ObjectKind, SlotId),
}

impl ObjectReference {
    pub fn object_id(object_id: ObjectId) -> Self {
        Self::ObjectId(object_id)
    }

    pub fn slot(kind: ObjectKind, slot_id: SlotId) -> Self {
        Self::Slot(kind, slot_id)
    }
}

impl From<ObjectId> for ObjectReference {
    fn from(id: ObjectId) -> Self {
        ObjectReference::ObjectId(id)
    }
}

impl From<(ObjectKind, SlotId)> for ObjectReference {
    fn from((kind, slot): (ObjectKind, SlotId)) -> Self {
        ObjectReference::Slot(kind, slot)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FixtureCollection {
    Group(ObjectReference),
    Static(Vec<FixtureId>),
}

impl From<ObjectReference> for FixtureCollection {
    fn from(reference: ObjectReference) -> Self {
        FixtureCollection::Group(reference)
    }
}

impl From<Vec<FixtureId>> for FixtureCollection {
    fn from(ids: Vec<FixtureId>) -> Self {
        FixtureCollection::Static(ids)
    }
}

impl From<FixtureId> for FixtureCollection {
    fn from(id: FixtureId) -> Self {
        FixtureCollection::Static(vec![id])
    }
}

use std::{collections::HashMap, fmt, num::NonZeroU32};

use uuid::Uuid;

mod cue_list;
mod executor_page;
mod group;
mod layout_page;

pub use cue_list::*;
pub use executor_page::*;
pub use group::*;
pub use layout_page::*;

pub trait Object: serde::Serialize + for<'de> serde::Deserialize<'de> {
    fn slot(&self) -> Slot;

    fn id(&self) -> ObjectId;

    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct ObjectCollection<T> {
    objects: Vec<T>,
    object_id_index: HashMap<ObjectId, usize>,
    slot_index: HashMap<Slot, usize>,
}

impl<T> ObjectCollection<T> {
    pub fn all(&self) -> &Vec<T> {
        &self.objects
    }

    pub fn get_by_object_id(&self, object_id: &ObjectId) -> anyhow::Result<&T> {
        self.object_id_index.get(object_id).map(|&ix| &self.objects[ix]).ok_or_else(|| {
            anyhow::anyhow!("{} not found with id: {}", std::any::type_name::<T>(), object_id)
        })
    }

    pub(crate) fn get_by_object_id_mut(&mut self, object_id: &ObjectId) -> anyhow::Result<&mut T> {
        self.object_id_index.get(object_id).map(|&ix| &mut self.objects[ix]).ok_or_else(|| {
            anyhow::anyhow!("{} not found with id: {}", std::any::type_name::<T>(), object_id)
        })
    }

    pub fn get_by_slot(&self, slot: &Slot) -> anyhow::Result<&T> {
        self.slot_index.get(slot).map(|&ix| &self.objects[ix]).ok_or_else(|| {
            anyhow::anyhow!("{} not found with slot: {}", std::any::type_name::<T>(), slot)
        })
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl<T> Default for ObjectCollection<T> {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            object_id_index: Default::default(),
            slot_index: Default::default(),
        }
    }
}

impl<T> serde::Serialize for ObjectCollection<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.objects.serialize(serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for ObjectCollection<T>
where
    T: serde::de::DeserializeOwned + Object,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let objects = Vec::<T>::deserialize(deserializer)?;
        let mut object_id_index = HashMap::new();
        let mut slot_index = HashMap::new();
        for (ix, obj) in objects.iter().enumerate() {
            object_id_index.insert(obj.id(), ix);
            slot_index.insert(obj.slot(), ix);
        }
        Ok(Self { objects, object_id_index, slot_index })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Slot(NonZeroU32);

impl Slot {
    pub fn new(nz: NonZeroU32) -> Self {
        Self(nz)
    }

    pub fn as_u32(&self) -> u32 {
        self.0.into()
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Objects {
    pub(crate) groups: ObjectCollection<Group>,
    pub(crate) executor_pages: ObjectCollection<ExecutorPage>,
    pub(crate) cue_lists: ObjectCollection<CueList>,
    pub(crate) layout_pages: ObjectCollection<LayoutPage>,
}

impl Objects {
    pub fn executors(&self) -> impl Iterator<Item = (ExecutorId, &Executor)> {
        self.executor_pages.all().iter().flat_map(|page| {
            page.executors().iter().enumerate().map(move |(ix, exec)| {
                let slot = Slot::new(NonZeroU32::new(ix as u32 + 1).unwrap());
                let exec_id = ExecutorId::new(page.id(), slot);
                (exec_id, exec)
            })
        })
    }

    pub fn groups(&self) -> &ObjectCollection<Group> {
        &self.groups
    }

    pub fn executor_pages(&self) -> &ObjectCollection<ExecutorPage> {
        &self.executor_pages
    }

    pub fn cue_lists(&self) -> &ObjectCollection<CueList> {
        &self.cue_lists
    }

    pub fn layout_pages(&self) -> &ObjectCollection<LayoutPage> {
        &self.layout_pages
    }
}

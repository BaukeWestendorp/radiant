use std::{collections::HashMap, fmt, num::NonZeroU32};

use uuid::Uuid;

mod executor_page;
mod group;
mod layout_page;
mod preset;
mod sequence;

pub use executor_page::*;
pub use group::*;
pub use layout_page::*;
pub use preset::*;
pub use sequence::*;

pub trait Object: serde::Serialize + for<'de> serde::Deserialize<'de> {
    fn slot(&self) -> Slot;

    fn id(&self) -> ObjectId;

    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ObjectKind {
    Group,
    Sequence,
    ExecutorPage,
    LayoutPage,
    Preset(PresetKind),
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
    pub(crate) sequences: ObjectCollection<Sequence>,
    pub(crate) layout_pages: ObjectCollection<LayoutPage>,

    pub(crate) dimmer_presets: ObjectCollection<Preset>,
    pub(crate) position_presets: ObjectCollection<Preset>,
    pub(crate) gobo_presets: ObjectCollection<Preset>,
    pub(crate) color_presets: ObjectCollection<Preset>,
    pub(crate) beam_presets: ObjectCollection<Preset>,
    pub(crate) focus_presets: ObjectCollection<Preset>,
    pub(crate) control_presets: ObjectCollection<Preset>,
    pub(crate) shapers_presets: ObjectCollection<Preset>,
    pub(crate) video_presets: ObjectCollection<Preset>,
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

    pub fn sequences(&self) -> &ObjectCollection<Sequence> {
        &self.sequences
    }

    pub fn layout_pages(&self) -> &ObjectCollection<LayoutPage> {
        &self.layout_pages
    }

    pub fn presets(&self, kind: PresetKind) -> &ObjectCollection<Preset> {
        match kind {
            PresetKind::Dimmer => &self.dimmer_presets,
            PresetKind::Position => &self.position_presets,
            PresetKind::Gobo => &self.gobo_presets,
            PresetKind::Color => &self.color_presets,
            PresetKind::Beam => &self.beam_presets,
            PresetKind::Focus => &self.focus_presets,
            PresetKind::Control => &self.control_presets,
            PresetKind::Shapers => &self.shapers_presets,
            PresetKind::Video => &self.video_presets,
        }
    }

    pub fn dimmer_presets(&self) -> &ObjectCollection<Preset> {
        &self.dimmer_presets
    }

    pub fn position_presets(&self) -> &ObjectCollection<Preset> {
        &self.position_presets
    }

    pub fn gobo_presets(&self) -> &ObjectCollection<Preset> {
        &self.gobo_presets
    }

    pub fn color_presets(&self) -> &ObjectCollection<Preset> {
        &self.color_presets
    }

    pub fn beam_presets(&self) -> &ObjectCollection<Preset> {
        &self.beam_presets
    }

    pub fn focus_presets(&self) -> &ObjectCollection<Preset> {
        &self.focus_presets
    }

    pub fn control_presets(&self) -> &ObjectCollection<Preset> {
        &self.control_presets
    }

    pub fn shapers_presets(&self) -> &ObjectCollection<Preset> {
        &self.shapers_presets
    }

    pub fn video_presets(&self) -> &ObjectCollection<Preset> {
        &self.video_presets
    }

    pub fn preset_by_object_id(&self, id: &PresetId) -> anyhow::Result<&Preset> {
        match id.kind() {
            PresetKind::Dimmer => self.dimmer_presets.get_by_object_id(&id.object_id()),
            PresetKind::Position => self.position_presets.get_by_object_id(&id.object_id()),
            PresetKind::Gobo => self.gobo_presets.get_by_object_id(&id.object_id()),
            PresetKind::Color => self.color_presets.get_by_object_id(&id.object_id()),
            PresetKind::Beam => self.beam_presets.get_by_object_id(&id.object_id()),
            PresetKind::Focus => self.focus_presets.get_by_object_id(&id.object_id()),
            PresetKind::Control => self.control_presets.get_by_object_id(&id.object_id()),
            PresetKind::Shapers => self.shapers_presets.get_by_object_id(&id.object_id()),
            PresetKind::Video => self.video_presets.get_by_object_id(&id.object_id()),
        }
    }

    pub fn preset_by_slot(&self, slot: &Slot, kind: &PresetKind) -> anyhow::Result<&Preset> {
        match kind {
            PresetKind::Dimmer => self.dimmer_presets.get_by_slot(&slot),
            PresetKind::Position => self.position_presets.get_by_slot(&slot),
            PresetKind::Gobo => self.gobo_presets.get_by_slot(&slot),
            PresetKind::Color => self.color_presets.get_by_slot(&slot),
            PresetKind::Beam => self.beam_presets.get_by_slot(&slot),
            PresetKind::Focus => self.focus_presets.get_by_slot(&slot),
            PresetKind::Control => self.control_presets.get_by_slot(&slot),
            PresetKind::Shapers => self.shapers_presets.get_by_slot(&slot),
            PresetKind::Video => self.video_presets.get_by_slot(&slot),
        }
    }
}

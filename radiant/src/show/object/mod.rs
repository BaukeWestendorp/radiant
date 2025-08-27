use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::num::NonZeroU32;

use uuid::Uuid;

pub use executor::*;
pub use group::*;
pub use preset::*;
pub use sequence::*;

use crate::engine::ObjectReference;

mod executor;
mod group;
mod preset;
mod sequence;

pub trait Object: Send {
    fn create(id: ObjectId, pool_id: PoolId, name: String) -> Self
    where
        Self: Default;

    fn name(&self) -> &str;

    fn set_name(&mut self, name: String);

    fn id(&self) -> ObjectId;

    fn pool_id(&self) -> PoolId;

    fn set_pool_id(&mut self, pool_id: PoolId);

    fn kind(&self) -> ObjectKind;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn into_any_object(self) -> AnyObject;
}

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

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AnyObject {
    Group(Group),
    Executor(Executor),
    Sequence(Sequence),

    PresetDimmer(PresetDimmer),
    PresetPosition(PresetPosition),
    PresetGobo(PresetGobo),
    PresetColor(PresetColor),
    PresetBeam(PresetBeam),
    PresetFocus(PresetFocus),
    PresetControl(PresetControl),
    PresetShapers(PresetShapers),
    PresetVideo(PresetVideo),
}

impl AnyObject {
    pub fn as_object(&self) -> &dyn Object {
        match self {
            AnyObject::Group(o) => o,
            AnyObject::Executor(o) => o,
            AnyObject::Sequence(o) => o,

            AnyObject::PresetDimmer(o) => o,
            AnyObject::PresetPosition(o) => o,
            AnyObject::PresetGobo(o) => o,
            AnyObject::PresetColor(o) => o,
            AnyObject::PresetBeam(o) => o,
            AnyObject::PresetFocus(o) => o,
            AnyObject::PresetControl(o) => o,
            AnyObject::PresetShapers(o) => o,
            AnyObject::PresetVideo(o) => o,
        }
    }

    pub fn as_object_mut(&mut self) -> &mut dyn Object {
        match self {
            AnyObject::Group(o) => o,
            AnyObject::Executor(o) => o,
            AnyObject::Sequence(o) => o,

            AnyObject::PresetDimmer(o) => o,
            AnyObject::PresetPosition(o) => o,
            AnyObject::PresetGobo(o) => o,
            AnyObject::PresetColor(o) => o,
            AnyObject::PresetBeam(o) => o,
            AnyObject::PresetFocus(o) => o,
            AnyObject::PresetControl(o) => o,
            AnyObject::PresetShapers(o) => o,
            AnyObject::PresetVideo(o) => o,
        }
    }

    pub fn as_preset(&self) -> Option<&dyn PresetObject> {
        match self {
            AnyObject::PresetDimmer(o) => Some(o),
            AnyObject::PresetPosition(o) => Some(o),
            AnyObject::PresetGobo(o) => Some(o),
            AnyObject::PresetColor(o) => Some(o),
            AnyObject::PresetBeam(o) => Some(o),
            AnyObject::PresetFocus(o) => Some(o),
            AnyObject::PresetControl(o) => Some(o),
            AnyObject::PresetShapers(o) => Some(o),
            AnyObject::PresetVideo(o) => Some(o),
            _ => None,
        }
    }

    pub fn as_preset_mut(&mut self) -> Option<&mut dyn PresetObject> {
        match self {
            AnyObject::PresetDimmer(o) => Some(o),
            AnyObject::PresetPosition(o) => Some(o),
            AnyObject::PresetGobo(o) => Some(o),
            AnyObject::PresetColor(o) => Some(o),
            AnyObject::PresetBeam(o) => Some(o),
            AnyObject::PresetFocus(o) => Some(o),
            AnyObject::PresetControl(o) => Some(o),
            AnyObject::PresetShapers(o) => Some(o),
            AnyObject::PresetVideo(o) => Some(o),
            _ => None,
        }
    }
}

#[derive(Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ObjectContainer {
    objects: HashMap<ObjectId, AnyObject>,
}

impl ObjectContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<T: Object + 'static>(&self, id: ObjectId) -> Option<&T> {
        self.objects.get(&id).and_then(|obj| obj.as_object().as_any().downcast_ref::<T>())
    }

    pub fn get_mut<T: Object + 'static>(&mut self, id: ObjectId) -> Option<&mut T> {
        self.objects
            .get_mut(&id)
            .and_then(|obj| obj.as_object_mut().as_any_mut().downcast_mut::<T>())
    }

    pub fn get_by_pool_id<T: Object + Default + 'static>(&self, pool_id: PoolId) -> Option<&T> {
        let kind = T::default().kind();
        self.get_any_by_obj_ref(&ObjectReference { kind, pool_id })?
            .as_object()
            .as_any()
            .downcast_ref()
    }

    pub fn get_mut_by_pool_id<T: Object + Default + 'static>(
        &mut self,
        pool_id: PoolId,
    ) -> Option<&mut T> {
        let kind = T::default().kind();
        self.get_any_mut_by_obj_ref(&ObjectReference { kind, pool_id })?
            .as_object_mut()
            .as_any_mut()
            .downcast_mut()
    }

    pub fn get_any(&self, id: ObjectId) -> Option<&AnyObject> {
        self.objects.get(&id)
    }

    pub fn get_any_mut(&mut self, id: ObjectId) -> Option<&mut AnyObject> {
        self.objects.get_mut(&id)
    }

    pub fn get_any_by_obj_ref(&self, obj_ref: &ObjectReference) -> Option<&AnyObject> {
        self.objects.values().find(|obj| {
            let obj = obj.as_object();
            obj.pool_id() == obj_ref.pool_id && obj.kind() == obj_ref.kind
        })
    }

    pub fn get_any_mut_by_obj_ref(&mut self, obj_ref: &ObjectReference) -> Option<&mut AnyObject> {
        self.objects.values_mut().find(|obj| {
            let obj = obj.as_object();
            obj.pool_id() == obj_ref.pool_id && obj.kind() == obj_ref.kind
        })
    }

    pub fn all_of_type<T: Object + 'static>(&self) -> impl IntoIterator<Item = &T> {
        self.objects.values().filter_map(|obj| obj.as_object().as_any().downcast_ref::<T>())
    }

    pub fn insert(&mut self, object: impl Into<AnyObject>) {
        let object = object.into();
        self.objects.insert(object.as_object().id(), object.into());
    }

    pub fn remove(&mut self, id: &ObjectId) {
        self.objects.remove(id);
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

    pub fn iter(&self) -> impl Iterator<Item = (&ObjectId, &AnyObject)> {
        self.objects.iter()
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

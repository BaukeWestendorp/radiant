use std::collections::HashMap;

use crate::comp::ShowfileComponent;
use crate::engine::Engine;
use crate::error::Result;

pub mod group;
pub mod preset;

pub use group::*;
pub use preset::*;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    engine.register_component::<Objects>()?;
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Objects(HashMap<ObjectId, Object>);

impl Objects {
    pub fn get(&self, object_id: impl Into<ObjectId>) -> Option<&Object> {
        self.0.get(&object_id.into())
    }

    pub fn get_mut(&mut self, object_id: impl Into<ObjectId>) -> Option<&mut Object> {
        self.0.get_mut(&object_id.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ObjectId(uuid::Uuid);

impl TryFrom<&str> for ObjectId {
    type Error = uuid::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Object {
    id: ObjectId,
    pub name: String,
    pub kind: ObjectKind,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(strum::EnumDiscriminants)]
#[strum_discriminants(name(ObjectType))]
#[strum_discriminants(derive(Hash, serde::Serialize, serde::Deserialize))]
pub enum ObjectKind {
    Group(Group),
    PresetDimmer(Preset<Dimmer>),
    PresetPosition(Preset<Position>),
    PresetGobo(Preset<Gobo>),
    PresetColor(Preset<Color>),
    PresetBeam(Preset<Beam>),
    PresetFocus(Preset<Focus>),
    PresetControl(Preset<Control>),
    PresetShapers(Preset<Shapers>),
    PresetVideo(Preset<Video>),
}

impl ShowfileComponent for Objects {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "objects.yaml"
    }
}

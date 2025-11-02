use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::comp::Component;
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

#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Objects(#[serde(default)] HashMap<ObjectId, Object>);

impl Objects {
    pub fn get(&self, object_id: impl Into<ObjectId>) -> Option<&Object> {
        self.0.get(&object_id.into())
    }

    pub(crate) fn insert(&mut self, object: Object) {
        self.0.insert(object.id(), object);
    }

    pub(crate) fn remove(&mut self, object_id: impl Into<ObjectId>) {
        self.0.remove(&object_id.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ObjectId(uuid::Uuid);

impl ObjectId {
    pub fn new_unique() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

impl Object {
    pub fn new(kind: ObjectKind) -> Self {
        Self { id: ObjectId::new_unique(), name: format!("New {}", kind.r#type()), kind }
    }

    pub fn id(&self) -> ObjectId {
        self.id
    }
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

impl ObjectKind {
    pub fn r#type(&self) -> ObjectType {
        match self {
            ObjectKind::Group(_) => ObjectType::Group,
            ObjectKind::PresetDimmer(_) => ObjectType::PresetDimmer,
            ObjectKind::PresetPosition(_) => ObjectType::PresetPosition,
            ObjectKind::PresetGobo(_) => ObjectType::PresetGobo,
            ObjectKind::PresetColor(_) => ObjectType::PresetColor,
            ObjectKind::PresetBeam(_) => ObjectType::PresetBeam,
            ObjectKind::PresetFocus(_) => ObjectType::PresetFocus,
            ObjectKind::PresetControl(_) => ObjectType::PresetControl,
            ObjectKind::PresetShapers(_) => ObjectType::PresetShapers,
            ObjectKind::PresetVideo(_) => ObjectType::PresetVideo,
        }
    }

    pub fn default_for_type(r#type: ObjectType) -> ObjectKind {
        match r#type {
            ObjectType::Group => ObjectKind::Group(Group::default()),
            ObjectType::PresetDimmer => ObjectKind::PresetDimmer(Preset::<Dimmer>::default()),
            ObjectType::PresetPosition => ObjectKind::PresetPosition(Preset::<Position>::default()),
            ObjectType::PresetGobo => ObjectKind::PresetGobo(Preset::<Gobo>::default()),
            ObjectType::PresetColor => ObjectKind::PresetColor(Preset::<Color>::default()),
            ObjectType::PresetBeam => ObjectKind::PresetBeam(Preset::<Beam>::default()),
            ObjectType::PresetFocus => ObjectKind::PresetFocus(Preset::<Focus>::default()),
            ObjectType::PresetControl => ObjectKind::PresetControl(Preset::<Control>::default()),
            ObjectType::PresetShapers => ObjectKind::PresetShapers(Preset::<Shapers>::default()),
            ObjectType::PresetVideo => ObjectKind::PresetVideo(Preset::<Video>::default()),
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::Group => write!(f, "Group"),
            ObjectType::PresetDimmer => write!(f, "Dimmer Preset"),
            ObjectType::PresetPosition => write!(f, "Position Preset"),
            ObjectType::PresetGobo => write!(f, "Gobo Preset"),
            ObjectType::PresetColor => write!(f, "Color Preset"),
            ObjectType::PresetBeam => write!(f, "Beam Preset"),
            ObjectType::PresetFocus => write!(f, "Focus Preset"),
            ObjectType::PresetControl => write!(f, "Control Preset"),
            ObjectType::PresetShapers => write!(f, "Shapers Preset"),
            ObjectType::PresetVideo => write!(f, "Video Preset"),
        }
    }
}

impl Component for Objects {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "objects.yaml"
    }

    fn save_to_showfile(&self, showfile_path: &Path) -> Result<()> {
        let file_path = showfile_path.join(Self::relative_file_path());
        let mut file = File::create(&file_path)?;
        let yaml = serde_yaml::to_string(self)?;
        file.write_all(yaml.as_bytes())?;
        Ok(())
    }
}

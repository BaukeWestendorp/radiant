use super::asset::{AssetId, Preset};

pub trait Attribute: Eq + std::hash::Hash {
    fn to_attr(&self) -> Attr;
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnyPresetAssetId {
    Dimmer(AssetId<Preset<DimmerAttr>>),
    Position(AssetId<Preset<PositionAttr>>),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attr {
    Dimmer(DimmerAttr),
    Position(PositionAttr),
}

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dimmer(attr) => write!(f, "{}", attr.to_string()),
            Self::Position(attr) => write!(f, "{}", attr.to_string()),
        }
    }
}

impl Attribute for Attr {
    fn to_attr(&self) -> Attr {
        *self
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DimmerAttr {
    /// Controls the intensity of a fixture.
    Dimmer,
}

impl std::fmt::Display for DimmerAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dimmer => write!(f, "Dimmer"),
        }
    }
}

impl Attribute for DimmerAttr {
    fn to_attr(&self) -> Attr {
        Attr::Dimmer(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionAttr {
    /// Controls the fixture’s sideward movement (horizontal axis).
    Pan,
    /// Controls the fixture’s upward and the downward movement (vertical axis).
    Tilt,
    /// Controls the speed of the fixture’s continuous pan movement (horizontal axis).
    PanRotate,
    /// Controls the speed of the fixture’s continuous tilt movement (vertical axis).
    TiltRotate,
    /// Selects the predefined position effects that are built into the fixture.
    PositionEffect,
    /// Controls the speed of the predefined position effects that are built into the fixture.
    PositionEffectRate,
    /// Snaps or smooth fades with timing in running predefined position effects.
    PositionEffectFade,
    /// Defines a fixture’s x-coordinate within an XYZ coordinate system.
    XyzX,
    /// Defines a fixture’s y-coordinate within an XYZ coordinate system.
    XyzY,
    /// Defines a fixture‘s z-coordinate within an XYZ coordinate system.
    XyzZ,
    /// Defines rotation around X axis.
    RotX,
    /// Defines rotation around Y axis.
    RotY,
    /// Defines rotation around Z axis.
    RotZ,
    /// Scaling on X axis.
    ScaleX,
    /// Scaling on Y axis.
    ScaleY,
    /// Scaling on Y axis.
    ScaleZ,
    /// Unified scaling on all axis.
    ScaleXYZ,
}

impl std::fmt::Display for PositionAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pan => write!(f, "Pan"),
            Self::Tilt => write!(f, "Tilt"),
            Self::PanRotate => write!(f, "PanRotate"),
            Self::TiltRotate => write!(f, "TiltRotate"),
            Self::PositionEffect => write!(f, "PositionEffect"),
            Self::PositionEffectRate => write!(f, "PositionEffectRate"),
            Self::PositionEffectFade => write!(f, "PositionEffectFade"),
            Self::XyzX => write!(f, "XYZ_X"),
            Self::XyzY => write!(f, "XYZ_Y"),
            Self::XyzZ => write!(f, "XYZ_Z"),
            Self::RotX => write!(f, "Rot_X"),
            Self::RotY => write!(f, "Rot_Y"),
            Self::RotZ => write!(f, "Rot_Z"),
            Self::ScaleX => write!(f, "Scale_X"),
            Self::ScaleY => write!(f, "Scale_Y"),
            Self::ScaleZ => write!(f, "Scale_Z"),
            Self::ScaleXYZ => write!(f, "Scale_XYZ"),
        }
    }
}

impl Attribute for PositionAttr {
    fn to_attr(&self) -> Attr {
        Attr::Position(*self)
    }
}

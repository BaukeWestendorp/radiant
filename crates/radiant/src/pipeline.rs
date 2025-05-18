pub mod dmx {
    use std::collections::HashMap;

    use crate::show::patch::{FixtureId, Patch};

    use super::attrs::AttributeValues;

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct FDmxValue(pub f32);

    impl From<FDmxValue> for dmx::Value {
        fn from(value: FDmxValue) -> Self {
            dmx::Value((value.0 * (u8::MAX as f32)).clamp(0.0, 1.0) as u8)
        }
    }

    impl ui::NumberFieldImpl for FDmxValue {
        type Value = Self;

        const MIN: Option<Self> = Some(FDmxValue(0.0));
        const MAX: Option<Self> = Some(FDmxValue(1.0));
        const STEP: Option<f32> = None;

        fn from_str_or_default(s: &str) -> Self::Value {
            Self(s.parse().unwrap_or_default())
        }

        fn to_shared_string(value: &Self::Value) -> gpui::SharedString {
            value.0.to_string().into()
        }

        fn as_f32(value: &Self::Value) -> f32 {
            value.0
        }

        fn from_f32(v: f32) -> Self::Value {
            Self(v)
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct FixtureValues {
        pub fixture_id: FixtureId,
        pub values: HashMap<dmx::Address, dmx::Value>,
    }

    impl FixtureValues {
        pub fn new(fixture_id: FixtureId) -> Self {
            Self { fixture_id, values: HashMap::new() }
        }

        pub fn from_attr_values(
            attr_values: &AttributeValues,
            fixture_id: FixtureId,
            patch: &Patch,
        ) -> Self {
            let mut fixture_values = Self::new(fixture_id);
            for (attribute, value) in attr_values.values.iter() {
                let Some(fixture) = patch.fixture(fixture_id).cloned() else {
                    log::warn!("Could not find fixture with id {:?}", fixture_id);
                    continue;
                };

                let Some(offset) =
                    fixture.channel_offset_for_attr(&attribute.to_string(), patch).cloned()
                else {
                    continue;
                };

                fixture_values.set_dmx_value_at_offset(fixture.address(), &offset, value.0);
            }
            fixture_values
        }

        pub fn set_dmx_value_at_offset(
            &mut self,
            start_address: &dmx::Address,
            offsets: &[i32],
            value: f32,
        ) {
            let value_bytes = match offsets.len() {
                1 => {
                    let byte_value = (value * 0xff as f32) as u8;
                    vec![byte_value]
                }
                2 => {
                    let int_value = (value * 0xffff as f32) as u16;
                    vec![(int_value >> 8) as u8, (int_value & 0xFF) as u8]
                }
                3 => {
                    let int_value = (value * 0xffffff as f32) as u32;
                    vec![
                        (int_value >> 16) as u8,
                        ((int_value >> 8) & 0xFF) as u8,
                        (int_value & 0xFF) as u8,
                    ]
                }
                4 => {
                    let int_value = (value * 0xffffffff_u32 as f32) as u32;
                    vec![
                        (int_value >> 24) as u8,
                        ((int_value >> 16) & 0xFF) as u8,
                        ((int_value >> 8) & 0xFF) as u8,
                        (int_value & 0xFF) as u8,
                    ]
                }
                _ => vec![0],
            };

            for (byte, offset) in value_bytes.iter().zip(offsets) {
                let address = start_address.with_channel_offset(*offset as u16 - 1).unwrap();
                self.values.insert(address, dmx::Value(*byte));
            }
        }
    }
}

pub mod attrs {
    use std::collections::HashMap;

    use crate::show::patch::FixtureId;

    use super::dmx::FDmxValue;

    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, Default)]
    pub struct AttributeValues {
        pub fixture_id: FixtureId,
        pub values: HashMap<Attribute, FDmxValue>,
    }

    impl AttributeValues {
        pub fn new(fixture_id: FixtureId) -> Self {
            Self { fixture_id, values: HashMap::new() }
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Attribute {
        Dimmer(DimmerAttr),
        Position(PositionAttr),
    }

    impl std::fmt::Display for Attribute {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Dimmer(attr) => write!(f, "{}", attr.to_string()),
                Self::Position(attr) => write!(f, "{}", attr.to_string()),
            }
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}

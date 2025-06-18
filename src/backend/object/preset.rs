use std::collections::HashMap;

use crate::backend::patch::{
    attr::{Attribute, AttributeValue},
    fixture::FixtureId,
};

macro_rules! define_preset {
    ($($name:ident, $id:ident, $new_name:literal, $any_name:ident),+ $(,)?) => {
        $(
            crate::define_object_id!($id);

            #[doc = concat!("A ", stringify!($name), " preset")]
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name {
                pub id: $id,
                pub name: String,
                pub content: PresetContent,
            }

            impl $name {
                pub fn new(id: impl Into<$id>, content: PresetContent) -> Self {
                    Self { id: id.into(), name: $new_name.to_string(), content }
                }
            }
        )+

        /// Any preset id.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[derive(derive_more::Display, derive_more::From)]
        pub enum AnyPresetId {
            $(
                $any_name($id),
            )+
        }

        $(
            impl From<&$name> for AnyPresetId {
                fn from(preset: &$name) -> Self {
                    AnyPresetId::$any_name(preset.id)
                }
            }
        )+

        $(
            impl TryFrom<AnyPresetId> for $id {
                type Error = ();

                fn try_from(any_id: AnyPresetId) -> Result<Self, Self::Error> {
                    match any_id {
                        AnyPresetId::$any_name(id) => Ok(id),
                        _ => Err(()),
                    }
                }
            }
        )+

        $(
            impl<'a> From<&'a $name> for &'a AnyPreset {
                fn from(preset: &'a $name) -> Self {
                    // This is safe because AnyPreset contains the same variants
                    // as AnyPresetId, and we're just getting a reference to the
                    // variant that was created with From<$name> for AnyPreset
                    unsafe {
                        let any_preset = (preset as *const $name).cast::<AnyPreset>();
                        &*any_preset
                    }
                }
            }
        )+

        $(
            impl<'a> TryFrom<&'a AnyPreset> for &'a $name {
                type Error = ();

                fn try_from(any_preset: &'a AnyPreset) -> Result<Self, Self::Error> {
                    match any_preset {
                        AnyPreset::$any_name(preset) => Ok(preset),
                        _ => Err(()),
                    }
                }
            }
        )+

        /// Any preset.
        #[derive(Debug, Clone, PartialEq)]
        #[derive(derive_more::From)]
        pub enum AnyPreset {
            $(
                $any_name($name),
            )+
        }
    };
}

/// A collection of attribute values, either connected to specific fixtures, fixture types, or generic attributes.
#[derive(Debug, Clone, PartialEq)]
pub enum PresetContent {
    Selective(SelectivePreset),
}

/// A preset that has attribute values for specific fixures.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectivePreset {
    attribute_values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl SelectivePreset {
    pub fn get_attribute_values(
        &self,
    ) -> impl IntoIterator<Item = (&(FixtureId, Attribute), &AttributeValue)> {
        self.attribute_values.iter()
    }

    pub fn set_attribute_value(
        &mut self,
        fixture_id: FixtureId,
        attribute: Attribute,
        value: AttributeValue,
    ) {
        self.attribute_values.insert((fixture_id, attribute), value);
    }
}

define_preset!(DimmerPreset, DimmerPresetId, "New Dimmer Preset", Dimmer);

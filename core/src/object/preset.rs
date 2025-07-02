use std::collections::BTreeMap;

use crate::{Attribute, AttributeValue, FeatureGroup, FixtureId};

macro_rules! define_preset {
    ($(($name:ident, $id:ident, $any_name:ident, $new_name:literal)),+ $(,)?) => {
        $(
            crate::define_object_id!($id);

            #[doc = concat!("A ", stringify!($name), " preset")]
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name {
                id: $id,
                pub name: String,
                pub content: PresetContent,
            }

            impl $name {
                pub fn new(id: impl Into<$id>, content: PresetContent) -> Self {
                    Self { id: id.into(), name: $new_name.to_string(), content }
                }

                pub fn id(&self) -> $id {
                    self.id
                }

                pub fn into_any(self) -> AnyPreset {
                    AnyPreset::$any_name(self)
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
            impl<'a> TryFrom<&'a mut AnyPreset> for &'a mut $name {
                type Error = ();

                fn try_from(any_preset: &'a mut AnyPreset) -> Result<Self, Self::Error> {
                    match any_preset {
                        AnyPreset::$any_name(preset) => Ok(preset),
                        _ => Err(()),
                    }
                }
            }
        )+

        $(
            impl From<$name> for AnyPresetId {
                fn from(preset: $name) -> Self {
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
            impl<'a> TryFrom<&'a AnyPreset> for &'a $name {
                type Error = ();

                fn try_from(any_preset: &'a AnyPreset) -> Result<Self, Self::Error> {
                    #[allow(unreachable_patterns)]
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

        /// Any preset.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[derive(derive_more::From)]
        pub enum PresetKind {
            $(
                $any_name,
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
#[derive(Debug, Clone, PartialEq)]
pub struct SelectivePreset {
    attribute_values: BTreeMap<(FixtureId, Attribute), AttributeValue>,
    filter: FeatureGroup,
}

impl SelectivePreset {
    pub fn new(feature_group_filter: FeatureGroup) -> Self {
        Self { attribute_values: BTreeMap::new(), filter: feature_group_filter }
    }

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
        if attribute.feature_group().is_some_and(|fg| fg == self.filter) {
            self.attribute_values.insert((fixture_id, attribute), value);
        }
    }

    pub fn clear(&mut self) {
        self.attribute_values.clear();
    }
}

define_preset!(
    (DimmerPreset, DimmerPresetId, Dimmer, "New Dimmer Preset"),
    (ColorPreset, ColorPresetId, Color, "New Color Preset"),
);

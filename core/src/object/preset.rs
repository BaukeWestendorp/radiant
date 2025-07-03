use std::collections::BTreeMap;

use crate::patch::{Attribute, AttributeValue, FeatureGroup, FixtureId};

macro_rules! define_preset {
    ($(($name:ident, $id:ident, $any_name:ident, $new_name:literal)),+ $(,)?) => {
        $(
            super::define_object_id!($id);

            #[doc = concat!("A ", stringify!($name), " preset")]
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name {
                id: $id,
                pub(crate) name: String,
                pub(crate) content: PresetContent,
            }

            impl $name {
                /// Creates a new preset with the specified id and content.
                pub fn new(id: impl Into<$id>, content: PresetContent) -> Self {
                    Self { id: id.into(), name: $new_name.to_string(), content }
                }

                /// Returns this preset's unique id.
                pub fn id(&self) -> $id {
                    self.id
                }

                /// Returns this preset's name.
                pub fn name(&self) -> &str {
                    &self.name
                }

                /// Converts this preset into an [AnyPreset] enum variant.
                pub fn into_any(self) -> AnyPreset {
                    AnyPreset::$any_name(self)
                }

                /// Gets this preset's content.
                pub fn content(&self) -> &PresetContent {
                    &self.content
                }
            }
        )+

        /// Any preset id.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[derive(derive_more::Display, derive_more::From)]
        pub enum AnyPresetId {
            $(
                #[doc = concat!("A ", stringify!($name), " preset id")]
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
                #[doc = concat!("A ", stringify!($name), " preset")]
                $any_name($name),
            )+
        }

        /// Any preset.
        ///
        /// Represents the different kinds of presets available in the system
        /// without carrying the actual preset data.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[derive(derive_more::From)]
        pub enum PresetKind {
            $(
                #[doc = concat!("A ", stringify!($name), " preset")]
                $any_name,
            )+
        }
    };
}

/// A collection of attribute values, either connected to specific fixtures,
/// fixture types, or generic attributes.
#[derive(Debug, Clone, PartialEq)]
pub enum PresetContent {
    /// A preset that applies to specific fixtures with targeted attribute
    /// values.
    Selective(SelectivePreset),
}

/// A preset that has attribute values for specific fixtures.
///
/// Selective presets store attribute values mapped to specific fixture and
/// attribute combinations, with filtering based on feature groups to ensure
/// only compatible attributes are stored.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectivePreset {
    attribute_values: BTreeMap<(FixtureId, Attribute), AttributeValue>,
    filter: FeatureGroup,
}

impl SelectivePreset {
    /// Creates a new selective preset with the specified feature group filter.
    ///
    /// The filter determines which attributes can be stored in this preset
    /// based on their feature group compatibility.
    pub fn new(feature_group_filter: FeatureGroup) -> Self {
        Self { attribute_values: BTreeMap::new(), filter: feature_group_filter }
    }

    /// Returns an iterator over all attribute values stored in this preset.
    ///
    /// Each item in the iterator is a tuple containing the fixture-attribute
    /// key and the corresponding attribute value.
    pub fn get_attribute_values(
        &self,
    ) -> impl IntoIterator<Item = (&(FixtureId, Attribute), &AttributeValue)> {
        self.attribute_values.iter()
    }

    /// Sets an attribute value for a specific fixture.
    ///
    /// The attribute value is only stored if the attribute's feature group
    /// matches this preset's filter. Attributes that don't match the filter
    /// are silently ignored.
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

    /// Clears all attribute values from this preset.
    ///
    /// After calling this method, the preset will contain no attribute values
    /// but will retain its feature group filter.
    pub fn clear(&mut self) {
        self.attribute_values.clear();
    }
}

define_preset!(
    (DimmerPreset, DimmerPresetId, Dimmer, "New Dimmer Preset"),
    (ColorPreset, ColorPresetId, Color, "New Color Preset"),
);

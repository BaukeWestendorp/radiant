use std::collections::HashMap;

use crate::show::{Attribute, AttributeValue, FixtureId, FixtureTypeId, Patch};

macro_rules! preset_kind_and_content {
    ($kind:ident, $preset:ident) => {
        #[derive(object_derive::Object)]
        #[object_derive::object]
        #[derive(Clone, Default)]
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct $preset {
            content: PresetContent,
        }

        impl $preset {
            pub fn content(&self) -> &PresetContent {
                &self.content
            }

            pub fn fixture_ids(&self, patch: &Patch) -> Vec<FixtureId> {
                match &self.content {
                    PresetContent::Universal(preset) => {
                        let mut attributes = preset.values().keys();
                        patch
                            .fixtures()
                            .iter()
                            .filter(|fixture| {
                                attributes.any(|attr| fixture.has_attribute(attr, patch))
                            })
                            .map(|f| f.fid())
                            .collect()
                    }
                    PresetContent::Global(preset) => {
                        let mut fixture_types = preset.values().keys().map(|(f_ty, _)| f_ty);
                        patch
                            .fixtures()
                            .iter()
                            .filter(|fixture| {
                                fixture_types.any(|f_ty| f_ty == fixture.fixture_type_id())
                            })
                            .map(|f| f.fid())
                            .collect()
                    }
                    PresetContent::Selective(preset) => {
                        preset.values().keys().map(|(fid, _)| *fid).collect()
                    }
                }
            }
        }
    };
}

preset_kind_and_content!(Dimmer, PresetDimmer);
preset_kind_and_content!(Position, PresetPosition);
preset_kind_and_content!(Gobo, PresetGobo);
preset_kind_and_content!(Color, PresetColor);
preset_kind_and_content!(Beam, PresetBeam);
preset_kind_and_content!(Focus, PresetFocus);
preset_kind_and_content!(Control, PresetControl);
preset_kind_and_content!(Shapers, PresetShapers);
preset_kind_and_content!(Video, PresetVideo);

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PresetContent {
    Universal(UniversalPreset),
    Global(GlobalPreset),
    Selective(SelectivePreset),
}

impl Default for PresetContent {
    fn default() -> Self {
        Self::Universal(UniversalPreset::default())
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniversalPreset {
    values: HashMap<Attribute, AttributeValue>,
}

impl UniversalPreset {
    pub fn values(&self) -> &HashMap<Attribute, AttributeValue> {
        &self.values
    }
}

impl Default for UniversalPreset {
    fn default() -> Self {
        Self { values: HashMap::default() }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GlobalPreset {
    values: HashMap<(FixtureTypeId, Attribute), AttributeValue>,
}

impl GlobalPreset {
    pub fn values(&self) -> &HashMap<(FixtureTypeId, Attribute), AttributeValue> {
        &self.values
    }
}

impl Default for GlobalPreset {
    fn default() -> Self {
        Self { values: HashMap::default() }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SelectivePreset {
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl SelectivePreset {
    pub fn values(&self) -> &HashMap<(FixtureId, Attribute), AttributeValue> {
        &self.values
    }
}

impl Default for SelectivePreset {
    fn default() -> Self {
        Self { values: HashMap::default() }
    }
}

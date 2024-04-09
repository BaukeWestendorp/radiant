use gdtf::FeatureGroupType;

use crate::{AttributeValues, Show};

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Presets {
    pub beam: Vec<BeamPreset>,
    pub color: Vec<ColorPreset>,
    pub dimmer: Vec<DimmerPreset>,
    pub focus: Vec<FocusPreset>,
    pub gobo: Vec<GoboPreset>,
    pub position: Vec<PositionPreset>,
    pub all: Vec<AllPreset>,
}

impl Presets {
    pub fn new() -> Self {
        Self {
            beam: Vec::new(),
            color: Vec::new(),
            dimmer: Vec::new(),
            focus: Vec::new(),
            gobo: Vec::new(),
            position: Vec::new(),
            all: Vec::new(),
        }
    }
}

pub trait Preset {
    fn id(&self) -> usize;

    fn label(&self) -> &str;

    fn set_label(&mut self, label: &str);

    fn feature_groups(&self) -> &[FeatureGroupType];

    fn attribute_values(&self) -> &AttributeValues;
}

pub enum AffectedAttributes {
    All,
    Specific(Vec<&'static str>),
}

macro_rules! preset {
    (
        $name:ident,
        $field:ident,
        $getter:ident,
        $getter_mut:ident,
        $getter_all:ident,
        $activation_groups:expr) => {
        #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            pub(crate) id: usize,
            pub(crate) label: String,
            pub attribute_values: AttributeValues,
        }

        impl $name {
            pub fn new(id: usize, label: &str) -> Self {
                Self {
                    id,
                    label: label.to_string(),
                    attribute_values: AttributeValues::new(),
                }
            }
        }

        impl Preset for $name {
            fn id(&self) -> usize {
                self.id
            }

            fn label(&self) -> &str {
                &self.label
            }

            fn set_label(&mut self, label: &str) {
                self.label = label.to_string();
            }

            fn feature_groups(&self) -> &[FeatureGroupType] {
                $activation_groups
            }

            fn attribute_values(&self) -> &AttributeValues {
                &self.attribute_values
            }
        }

        impl Show {
            pub fn $getter(&self, preset_id: usize) -> Option<&$name> {
                self.presets.$field.iter().find(|c| c.id == preset_id)
            }

            pub fn $getter_mut(&mut self, preset_id: usize) -> Option<&mut $name> {
                self.presets.$field.iter_mut().find(|c| c.id == preset_id)
            }

            pub fn $getter_all(&self) -> &Vec<$name> {
                &self.presets.$field
            }
        }

        impl From<crate::showfile::Preset> for $name {
            fn from(val: crate::showfile::Preset) -> Self {
                $name {
                    id: val.id,
                    label: val.label,
                    attribute_values: val.attribute_values,
                }
            }
        }
    };
}

preset!(
    BeamPreset,
    beam,
    beam_preset,
    beam_preset_mut,
    beam_presets,
    &[FeatureGroupType::Beam]
);

preset!(
    ColorPreset,
    color,
    color_preset,
    color_preset_mut,
    color_presets,
    &[FeatureGroupType::Color]
);

preset!(
    DimmerPreset,
    dimmer,
    dimmer_preset,
    dimmer_preset_mut,
    dimmer_presets,
    &[FeatureGroupType::Dimmer]
);

preset!(
    FocusPreset,
    focus,
    focus_preset,
    focus_preset_mut,
    focus_presets,
    &[FeatureGroupType::Focus]
);

preset!(
    GoboPreset,
    gobo,
    gobo_preset,
    gobo_preset_mut,
    gobo_presets,
    &[FeatureGroupType::Gobo]
);

preset!(
    PositionPreset,
    position,
    position_preset,
    position_preset_mut,
    position_presets,
    &[FeatureGroupType::Position]
);

preset!(
    AllPreset,
    all,
    all_preset,
    all_preset_mut,
    all_presets,
    &[
        FeatureGroupType::Beam,
        FeatureGroupType::Color,
        FeatureGroupType::Dimmer,
        FeatureGroupType::Focus,
        FeatureGroupType::Gobo,
        FeatureGroupType::Position
    ]
);

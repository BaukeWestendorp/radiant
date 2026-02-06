use zeevonk::value::AttributeValues;

pub type PresetId = u32;

pub struct Preset {
    pub name: String,
    pub attribute_values: AttributeValues,
}

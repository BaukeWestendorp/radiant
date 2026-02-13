#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct EffectDefinition {
    name: String,

    script_path: String,
}

impl EffectDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn script_path(&self) -> &str {
        &self.script_path
    }
}

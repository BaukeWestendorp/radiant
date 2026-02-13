pub type EffectId = u32;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    name: String,

    script: String,
}

impl Effect {
    pub fn new(name: String, script: String) -> Self {
        Self { name, script }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn script(&self) -> &str {
        &self.script
    }
}

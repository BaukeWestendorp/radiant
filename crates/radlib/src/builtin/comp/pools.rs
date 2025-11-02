use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU32;
use std::path::Path;

use crate::builtin::{ObjectId, ObjectType};
use crate::comp::Component;
use crate::engine::Engine;
use crate::error::Result;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    engine.register_component::<Pools>()?;
    Ok(())
}

#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pools(#[serde(default)] HashMap<ObjectType, Pool>);

impl Pools {
    pub fn pool(&self, object_type: ObjectType) -> &Pool {
        self.0.get(&object_type).expect("a pool should always exist for every object type")
    }

    pub(crate) fn pool_mut(&mut self, object_type: ObjectType) -> &mut Pool {
        self.0.get_mut(&object_type).expect("a pool should always exist for every object type")
    }

    pub fn get(&self, object_type: ObjectType, pool_id: PoolId) -> Option<ObjectId> {
        self.pool(object_type).get(pool_id)
    }
}

impl Component for Pools {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "pools.yaml"
    }

    fn save_to_showfile(&self, showfile_path: &Path) -> Result<()> {
        let file_path = showfile_path.join(Self::relative_file_path());
        let mut file = File::create(&file_path)?;
        let yaml = serde_yaml::to_string(self)?;
        file.write_all(yaml.as_bytes())?;
        Ok(())
    }
}

#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pool(HashMap<PoolId, ObjectId>);

impl Pool {
    pub fn get(&self, pool_id: PoolId) -> Option<ObjectId> {
        self.0.get(&pool_id).copied()
    }

    pub(crate) fn insert(&mut self, pool_id: PoolId, object_id: ObjectId) {
        self.0.insert(pool_id, object_id);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PoolId(pub NonZeroU32);

impl fmt::Display for PoolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

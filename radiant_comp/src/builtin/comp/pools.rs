use std::collections::HashMap;
use std::num::NonZeroU32;

use crate::builtin::{ObjectId, ObjectType};
use crate::comp::ShowfileComponent;
use crate::engine::Engine;
use crate::error::Result;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    engine.register_component::<Pools>()?;
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pools(HashMap<ObjectType, Pool>);

impl Pools {
    pub fn pool(&self, object_type: ObjectType) -> Option<&Pool> {
        self.0.get(&object_type)
    }

    pub fn get(&self, object_type: ObjectType, pool_id: PoolId) -> Option<ObjectId> {
        self.pool(object_type)?.get(pool_id)
    }
}

impl ShowfileComponent for Pools {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "pools.yaml"
    }
}

#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pool(HashMap<PoolId, ObjectId>);

impl Pool {
    pub fn get(&self, pool_id: PoolId) -> Option<ObjectId> {
        self.0.get(&pool_id).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PoolId(pub NonZeroU32);

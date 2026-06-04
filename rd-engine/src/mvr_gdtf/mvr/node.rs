use std::{fmt, hash, str};

use uuid::Uuid;

pub struct NodeId<T> {
    uuid: Uuid,
    _marker: std::marker::PhantomData<T>,
}

impl<T> NodeId<T> {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid, _marker: std::marker::PhantomData }
    }

    pub fn as_uuid(&self) -> Uuid {
        self.uuid
    }
}

impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        Self { uuid: self.uuid, _marker: std::marker::PhantomData }
    }
}

impl<T> Copy for NodeId<T> {}

impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<T> Eq for NodeId<T> {}

impl<T> hash::Hash for NodeId<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<T> From<Uuid> for NodeId<T> {
    fn from(uuid: Uuid) -> Self {
        Self { uuid, _marker: std::marker::PhantomData }
    }
}

impl<T> str::FromStr for NodeId<T> {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(Uuid::from_str(s)?))
    }
}

impl<T> fmt::Debug for NodeId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId<{}>({:?})", std::any::type_name::<T>(), self.uuid)
    }
}

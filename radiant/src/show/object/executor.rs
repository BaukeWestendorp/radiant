use crate::show::{ObjectId, Sequence, Show};

#[derive(object_derive::Object)]
#[object_derive::object]
#[derive(Clone, Default)]
#[derive(serde::Deserialize)]
pub struct Executor {
    pub(crate) sequence_id: Option<ObjectId>,
    pub(crate) is_on: bool,
}

impl Executor {
    pub fn sequence_id(&self) -> Option<ObjectId> {
        self.sequence_id
    }

    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        self.sequence_id.and_then(|id| show.objects.get(id))
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

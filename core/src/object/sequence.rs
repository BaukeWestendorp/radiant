use crate::object::CueId;

super::define_object_id!(SequenceId);

/// A sequence of [Cue][crate::object::Cue]s that can be activated using an
/// [Executor][crate::object::Executor].
///
/// Sequences provide an ordered collection of cues that can be played back
/// sequentially through executor controls.
#[derive(Debug, Clone, PartialEq)]
pub struct Sequence {
    id: SequenceId,
    pub(crate) name: String,
    pub(crate) cues: Vec<CueId>,
}

impl Sequence {
    /// Creates a new [Sequence][crate::object::Sequence] with the specified id.
    ///
    /// The sequence is initialized with a default name and an empty cue list.
    pub fn new(id: impl Into<SequenceId>) -> Self {
        Self { id: id.into(), name: "New Sequence".to_string(), cues: Vec::new() }
    }

    /// Returns this sequence's unique id.
    pub fn id(&self) -> SequenceId {
        self.id
    }

    /// Returns the name of this sequence.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the number of cues in this sequence.
    pub fn len(&self) -> usize {
        self.cues.len()
    }

    /// Returns `true` if the sequence contains no cues.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a slice of all [CueId]s in this sequence.
    ///
    /// The cues are returned in the order they will be executed,
    /// from first to last in the sequence.
    pub fn cues(&self) -> &[CueId] {
        &self.cues
    }
}

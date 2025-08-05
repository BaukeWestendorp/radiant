use crate::show::{
    Attribute, AttributeValue, Executor, FixtureId, FixtureTypeId, Group, ObjectId, PoolId,
    PresetBeam, PresetColor, PresetControl, PresetDimmer, PresetFocus, PresetGobo, PresetPosition,
    PresetShapers, PresetVideo, Sequence,
};

pub enum Command {
    PatchAdd { fid: FixtureId, address: dmx::Address, type_id: FixtureTypeId, dmx_mode: String },
    CreateGroup { pool_id: PoolId<Group>, name: Option<String>, fids: Vec<FixtureId> },
    CreateSequence { pool_id: PoolId<Sequence>, name: Option<String> },
    CreateExecutor { pool_id: PoolId<Executor>, name: Option<String> },
    CreatePresetDimmer { pool_id: PoolId<PresetDimmer>, name: Option<String> },
    CreatePresetPosition { pool_id: PoolId<PresetPosition>, name: Option<String> },
    CreatePresetGobo { pool_id: PoolId<PresetGobo>, name: Option<String> },
    CreatePresetColor { pool_id: PoolId<PresetColor>, name: Option<String> },
    CreatePresetBeam { pool_id: PoolId<PresetBeam>, name: Option<String> },
    CreatePresetFocus { pool_id: PoolId<PresetFocus>, name: Option<String> },
    CreatePresetControl { pool_id: PoolId<PresetControl>, name: Option<String> },
    CreatePresetShapers { pool_id: PoolId<PresetShapers>, name: Option<String> },
    CreatePresetVideo { pool_id: PoolId<PresetVideo>, name: Option<String> },
    ProgrammerSetAttribute { fid: FixtureId, attribute: Attribute, value: AttributeValue },
    Go { executor_id: ObjectId },
    SelectReferencedFixtures { id: ObjectId },
    SelectFixture { fid: FixtureId },
    ClearFixtureSelection,
}

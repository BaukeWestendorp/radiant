use crate::show::{
    Attribute, AttributeValue, FixtureId, FixtureTypeId, Group, ObjectId, PresetBeam, PresetColor,
    PresetControl, PresetDimmer, PresetFocus, PresetGobo, PresetPosition, PresetShapers,
    PresetVideo,
};

pub enum Command {
    PatchAdd { fid: FixtureId, address: dmx::Address, type_id: FixtureTypeId, dmx_mode: String },
    CreateGroup { id: ObjectId<Group>, name: Option<String>, fids: Vec<u32> },
    CreatePresetDimmer { id: ObjectId<PresetDimmer>, name: Option<String> },
    CreatePresetPosition { id: ObjectId<PresetPosition>, name: Option<String> },
    CreatePresetGobo { id: ObjectId<PresetGobo>, name: Option<String> },
    CreatePresetColor { id: ObjectId<PresetColor>, name: Option<String> },
    CreatePresetBeam { id: ObjectId<PresetBeam>, name: Option<String> },
    CreatePresetFocus { id: ObjectId<PresetFocus>, name: Option<String> },
    CreatePresetControl { id: ObjectId<PresetControl>, name: Option<String> },
    CreatePresetShapers { id: ObjectId<PresetShapers>, name: Option<String> },
    CreatePresetVideo { id: ObjectId<PresetVideo>, name: Option<String> },
    ProgrammerSetAttribute { fid: FixtureId, attribute: Attribute, value: AttributeValue },
}

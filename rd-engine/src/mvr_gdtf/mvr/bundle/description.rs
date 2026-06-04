#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuxData {
    #[serde(default, rename = "Class")]
    pub class: Vec<BasicChildListAttribute>,
    #[serde(default, rename = "Symdef")]
    pub symdef: Vec<Symdef>,
    #[serde(default, rename = "Position")]
    pub position: Vec<BasicChildListAttribute>,
    #[serde(default, rename = "MappingDefinition")]
    pub mapping_definition: Vec<MappingDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Address {
    #[serde(default = "Address::default_break_", rename = "@break")]
    pub r#break: i32,
    #[serde(rename = "$text")]
    pub content: String,
}

impl Address {
    #[must_use]
    pub fn default_break_() -> i32 {
        0i32
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Addresses {
    #[serde(default, rename = "Address")]
    pub address: Vec<Address>,
    #[serde(default, rename = "Network")]
    pub network: Vec<Network>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Alignment {
    #[serde(default, rename = "@geometry")]
    pub geometry: Option<String>,
    #[serde(default = "Alignment::default_up", rename = "@up")]
    pub up: String,
    #[serde(default = "Alignment::default_direction", rename = "@direction")]
    pub direction: String,
}

impl Alignment {
    #[must_use]
    pub fn default_up() -> String {
        String::from("0,0,1")
    }

    #[must_use]
    pub fn default_direction() -> String {
        String::from("0,0,-1")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Alignments {
    #[serde(default, rename = "Alignment")]
    pub alignment: Vec<Alignment>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BasicChildListAttribute {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "BasicChildListAttribute::default_name", rename = "@name")]
    pub name: String,
}

impl BasicChildListAttribute {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChildList {
    #[serde(default, rename = "$value")]
    pub content: Vec<ChildListContent>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ChildListContent {
    #[serde(rename = "SceneObject")]
    SceneObject(SceneObject),
    #[serde(rename = "GroupObject")]
    GroupObject(GroupObject),
    #[serde(rename = "FocusPoint")]
    FocusPoint(FocusPoint),
    #[serde(rename = "Fixture")]
    Fixture(Fixture),
    #[serde(rename = "Support")]
    Support(Support),
    #[serde(rename = "Truss")]
    Truss(Truss),
    #[serde(rename = "VideoScreen")]
    VideoScreen(VideoScreen),
    #[serde(rename = "Projector")]
    Projector(Projector),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Connection {
    #[serde(rename = "@own")]
    pub own: String,
    #[serde(rename = "@other")]
    pub other: String,
    #[serde(rename = "@toObject")]
    pub to_object: String,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Connections {
    #[serde(default, rename = "Connection")]
    pub connection: Vec<Connection>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CustomCommands {
    #[serde(default, rename = "CustomCommand")]
    pub custom_command: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Data {
    #[serde(rename = "@provider")]
    pub provider: String,
    #[serde(default = "Data::default_ver", rename = "@ver")]
    pub ver: String,
}

impl Data {
    #[must_use]
    pub fn default_ver() -> String {
        String::from("1")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Fixture::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Fixture::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "Focus")]
    pub focus: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "DMXInvertPan")]
    pub dmx_invert_pan: Option<bool>,
    #[serde(default, rename = "DMXInvertTilt")]
    pub dmx_invert_tilt: Option<bool>,
    #[serde(default, rename = "Position")]
    pub position: Option<String>,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: Option<String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(rename = "UnitNumber")]
    pub unit_number: i32,
    #[serde(default, rename = "ChildPosition")]
    pub child_position: Option<String>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Protocols")]
    pub protocols: Option<Protocols>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "Color")]
    pub color: Option<String>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "Mappings")]
    pub mappings: Option<Mappings>,
    #[serde(default, rename = "Gobo")]
    pub gobo: Option<Gobo>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
}

impl Fixture {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }

    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FocusPoint {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "FocusPoint::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
}

impl FocusPoint {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneralSceneDescription {
    #[serde(rename = "@verMajor")]
    pub ver_major: i32,
    #[serde(rename = "@verMinor")]
    pub ver_minor: i32,
    #[serde(default, rename = "@provider")]
    pub provider: Option<String>,
    #[serde(default, rename = "@providerVersion")]
    pub provider_version: Option<String>,
    #[serde(default, rename = "UserData")]
    pub user_data: Option<UserData>,
    #[serde(rename = "Scene")]
    pub scene: Scene,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Geometries {
    #[serde(default, rename = "Geometry3D")]
    pub geometry_3d: Vec<Geometry3D>,
    #[serde(default, rename = "Symbol")]
    pub symbol: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Geometry3D {
    #[serde(rename = "@fileName")]
    pub file_name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gobo {
    #[serde(default = "Gobo::default_rotation", rename = "@rotation")]
    pub rotation: f32,
    #[serde(rename = "$value")]
    pub file_name: String,
}

impl Gobo {
    #[must_use]
    pub fn default_rotation() -> f32 {
        0f32
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GroupObject {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "GroupObject::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(default, rename = "ChildList")]
    /// FIXME: Spec bug? Spec says 0-1 children, but XSD does not say minOccurs="0".
    pub child_list: Box<ChildList>,
}

impl GroupObject {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Layer {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Layer::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<ChildList>,
}

impl Layer {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Layers {
    #[serde(default, rename = "Layer")]
    pub layer: Vec<Layer>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mapping {
    #[serde(rename = "@linkedDef")]
    pub linked_def: String,
    #[serde(default, rename = "ux")]
    pub ux: Option<i32>,
    #[serde(default, rename = "uy")]
    pub uy: Option<i32>,
    #[serde(default, rename = "ox")]
    pub ox: Option<i32>,
    #[serde(default, rename = "oy")]
    pub oy: Option<i32>,
    #[serde(default, rename = "rz")]
    pub rz: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MappingDefinition {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "MappingDefinition::default_name", rename = "@name")]
    pub name: String,
    #[serde(rename = "SizeX")]
    pub size_x: i32,
    #[serde(rename = "SizeY")]
    pub size_y: i32,
    #[serde(rename = "Source")]
    pub source: Source,
    #[serde(default, rename = "ScaleHandeling")]
    pub scale_handeling: Option<ScaleHandeling>,
}

impl MappingDefinition {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mappings {
    #[serde(default, rename = "Mapping")]
    pub mapping: Vec<Mapping>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Network {
    #[serde(rename = "@geometry")]
    pub geometry: String,
    #[serde(default, rename = "@ipv4")]
    pub ipv_4: Option<String>,
    #[serde(default, rename = "@subnetmask")]
    pub subnetmask: Option<String>,
    #[serde(default, rename = "@ipv6")]
    pub ipv_6: Option<String>,
    #[serde(default, rename = "@dhcp")]
    pub dhcp: Option<String>,
    #[serde(default, rename = "@hostname")]
    pub hostname: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Overwrite {
    #[serde(rename = "@universal")]
    pub universal: String,
    #[serde(default = "Overwrite::default_target", rename = "@target")]
    pub target: String,
}

impl Overwrite {
    #[must_use]
    pub fn default_target() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Overwrites {
    #[serde(default, rename = "Overwrite")]
    pub overwrite: Vec<Overwrite>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Projection {
    #[serde(default, rename = "Source")]
    pub source: Option<Source>,
    #[serde(default, rename = "ScaleHandeling")]
    pub scale_handeling: Option<ScaleHandeling>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Projections {
    #[serde(default, rename = "Projection")]
    pub projection: Vec<Projection>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Projector {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Projector::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Projector::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(rename = "Projections")]
    pub projections: Projections,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: Option<String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}

impl Projector {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }

    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocol {
    #[serde(default = "Protocol::default_geometry", rename = "@geometry")]
    pub geometry: String,
    #[serde(default = "Protocol::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Protocol::default_type_", rename = "@type")]
    pub r#type: String,
    #[serde(default = "Protocol::default_version", rename = "@version")]
    pub version: String,
    #[serde(default, rename = "@transmission")]
    pub transmission: Option<Transmission>,
}

impl Protocol {
    #[must_use]
    pub fn default_geometry() -> String {
        String::from("NetworkInOut_1")
    }

    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }

    #[must_use]
    pub fn default_type_() -> String {
        String::from("")
    }

    #[must_use]
    pub fn default_version() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocols {
    #[serde(default, rename = "Protocol")]
    pub protocol: Vec<Protocol>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScaleHandeling {
    #[serde(default = "ScaleHandeling::default_enum_", rename = "@Enum")]
    pub r#enum: Scale,
}

impl ScaleHandeling {
    #[must_use]
    pub fn default_enum_() -> Scale {
        Scale::ScaleKeepRatio
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Scene {
    #[serde(default, rename = "AUXData")]
    pub aux_data: Option<AuxData>,
    #[serde(rename = "Layers")]
    pub layers: Layers,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SceneObject {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "SceneObject::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "SceneObject::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: Option<String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
}

impl SceneObject {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }

    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Source {
    #[serde(rename = "@linkedGeometry")]
    pub linked_geometry: String,
    #[serde(rename = "@type")]
    pub r#type: SourceType,
    #[serde(default, rename = "$text")]
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SourceType {
    #[serde(rename = "NDI")]
    Ndi,
    #[serde(rename = "File")]
    File,
    #[serde(rename = "CITP")]
    Citp,
    #[serde(rename = "CaptureDevice")]
    CaptureDevice,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sources {
    #[serde(default, rename = "Source")]
    pub source: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Support {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Support::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Support::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(default, rename = "Position")]
    pub position: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(rename = "ChainLength")]
    pub chain_length: f32,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: Option<String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
}

impl Support {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }

    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Symbol {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(rename = "@symdef")]
    pub symdef: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Symdef {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Symdef::default_name", rename = "@name")]
    pub name: String,
    #[serde(rename = "ChildList")]
    pub child_list: SymdefChildList,
}

impl Symdef {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}

pub type SymdefChildList = Geometries;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Transmission {
    #[serde(rename = "Unicast")]
    Unicast,
    #[serde(rename = "Multicast")]
    Multicast,
    #[serde(rename = "Broadcast")]
    Broadcast,
    #[serde(rename = "Anycast")]
    Anycast,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Truss {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Truss::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Truss::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(default, rename = "Position")]
    pub position: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "ChildPosition")]
    pub child_position: Option<String>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: Option<String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}

impl Truss {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }

    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserData {
    #[serde(default, rename = "Data")]
    pub data: Vec<Data>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoScreen {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "VideoScreen::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "VideoScreen::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Sources")]
    pub sources: Option<Sources>,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: Option<String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}

impl VideoScreen {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }

    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Scale {
    #[serde(rename = "ScaleKeepRatio")]
    ScaleKeepRatio,
    #[serde(rename = "ScaleIgnoreRatio")]
    ScaleIgnoreRatio,
    #[serde(rename = "KeepSizeCenter")]
    KeepSizeCenter,
}

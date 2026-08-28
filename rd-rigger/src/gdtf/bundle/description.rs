#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ActivationGroup {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ActivationGroups {
    #[serde(default, rename = "ActivationGroup")]
    pub activation_groups: Vec<ActivationGroup>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AdditionalColorSpaces {
    #[serde(default, rename = "ColorSpace")]
    pub color_spaces: Vec<ColorSpace>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnimationSystem {
    #[serde(rename = "@P1")]
    pub p1: String,
    #[serde(rename = "@P2")]
    pub p2: String,
    #[serde(rename = "@P3")]
    pub p3: String,
    #[serde(rename = "@Radius")]
    pub radius: f32,
}

// FIXME: The GDTF XSD has a mistake: It should be a list, not an optional.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ArtNet {
    #[serde(default, rename = "Map")]
    pub map: Vec<Map>,
}

// FIXME: The GDTF XSD has a mistake: sACN is empty.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SAcn {
    #[serde(default, rename = "Map")]
    pub map: Vec<Map>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Pretty")]
    pub pretty: String,
    #[serde(default, rename = "@ActivationGroup")]
    pub activation_group: Option<String>,
    #[serde(default, rename = "@Feature")]
    pub feature: Option<String>,
    #[serde(default, rename = "@MainAttribute")]
    pub main_attribute: Option<String>,
    #[serde(default = "Attribute::default_physical_unit", rename = "@PhysicalUnit")]
    pub physical_unit: PhysicalUnit,
    #[serde(default, rename = "@Color")]
    pub color: Option<String>,
    #[serde(default, rename = "SubPhysicalUnit")]
    pub sub_physical_units: Vec<SubPhysicalUnit>,
}

impl Attribute {
    #[must_use]
    pub fn default_physical_unit() -> PhysicalUnit {
        PhysicalUnit::None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AttributeDefinitions {
    #[serde(default, rename = "ActivationGroups")]
    pub activation_groups: Option<ActivationGroups>,
    #[serde(rename = "FeatureGroups")]
    pub feature_groups: FeatureGroups,
    #[serde(rename = "Attributes")]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    #[serde(default, rename = "Attribute")]
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BasicGeometryType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "BasicGeometryType::default_position", rename = "@Position")]
    pub position: String,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
}

impl BasicGeometryType {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Beam {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "Beam::default_position", rename = "@Position")]
    pub position: String,
    #[serde(default, rename = "@LampType")]
    pub lamp_type: Option<LampType>,
    #[serde(default, rename = "@PowerConsumption")]
    pub power_consumption: Option<f32>,
    #[serde(default, rename = "@LuminousFlux")]
    pub luminous_flux: Option<f32>,
    #[serde(default, rename = "@ColorTemperature")]
    pub color_temperature: Option<f32>,
    #[serde(default, rename = "@BeamAngle")]
    pub beam_angle: Option<f32>,
    #[serde(default, rename = "@FieldAngle")]
    pub field_angle: Option<f32>,
    #[serde(default, rename = "@ThrowRatio")]
    pub throw_ratio: Option<f32>,
    #[serde(default, rename = "@RectangleRatio")]
    pub rectangle_ratio: Option<f32>,
    #[serde(default, rename = "@BeamRadius")]
    pub beam_radius: Option<f32>,
    #[serde(default, rename = "@BeamType")]
    pub beam_type: Option<BeamType>,
    #[serde(default, rename = "@ColorRenderingIndex")]
    pub color_rendering_index: Option<u8>,
    #[serde(default, rename = "@EmitterSpectrum")]
    pub emitter_spectrum: Option<String>,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
}

impl Beam {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BeamType {
    #[serde(rename = "Wash")]
    Wash,
    #[serde(rename = "Spot")]
    Spot,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Rectangle")]
    Rectangle,
    #[serde(rename = "PC")]
    Pc,
    #[serde(rename = "Fresnel")]
    Fresnel,
    #[serde(rename = "Glow")]
    Glow,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Break {
    #[serde(default = "Break::default_dmx_offset", rename = "@DMXOffset")]
    pub dmx_offset: u32,
    #[serde(default = "Break::default_dmx_break", rename = "@DMXBreak")]
    pub dmx_break: u8,
}

impl Break {
    #[must_use]
    pub fn default_dmx_offset() -> u32 {
        1u32
    }
    #[must_use]
    pub fn default_dmx_break() -> u8 {
        1u8
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Ces {
    #[serde(rename = "CES01")]
    Ces01,
    #[serde(rename = "CES02")]
    Ces02,
    #[serde(rename = "CES03")]
    Ces03,
    #[serde(rename = "CES04")]
    Ces04,
    #[serde(rename = "CES05")]
    Ces05,
    #[serde(rename = "CES06")]
    Ces06,
    #[serde(rename = "CES07")]
    Ces07,
    #[serde(rename = "CES08")]
    Ces08,
    #[serde(rename = "CES09")]
    Ces09,
    #[serde(rename = "CES10")]
    Ces10,
    #[serde(rename = "CES11")]
    Ces11,
    #[serde(rename = "CES12")]
    Ces12,
    #[serde(rename = "CES13")]
    Ces13,
    #[serde(rename = "CES14")]
    Ces14,
    #[serde(rename = "CES15")]
    Ces15,
    #[serde(rename = "CES16")]
    Ces16,
    #[serde(rename = "CES17")]
    Ces17,
    #[serde(rename = "CES18")]
    Ces18,
    #[serde(rename = "CES19")]
    Ces19,
    #[serde(rename = "CES20")]
    Ces20,
    #[serde(rename = "CES21")]
    Ces21,
    #[serde(rename = "CES22")]
    Ces22,
    #[serde(rename = "CES23")]
    Ces23,
    #[serde(rename = "CES24")]
    Ces24,
    #[serde(rename = "CES25")]
    Ces25,
    #[serde(rename = "CES26")]
    Ces26,
    #[serde(rename = "CES27")]
    Ces27,
    #[serde(rename = "CES28")]
    Ces28,
    #[serde(rename = "CES29")]
    Ces29,
    #[serde(rename = "CES30")]
    Ces30,
    #[serde(rename = "CES31")]
    Ces31,
    #[serde(rename = "CES32")]
    Ces32,
    #[serde(rename = "CES33")]
    Ces33,
    #[serde(rename = "CES34")]
    Ces34,
    #[serde(rename = "CES35")]
    Ces35,
    #[serde(rename = "CES36")]
    Ces36,
    #[serde(rename = "CES37")]
    Ces37,
    #[serde(rename = "CES38")]
    Ces38,
    #[serde(rename = "CES39")]
    Ces39,
    #[serde(rename = "CES40")]
    Ces40,
    #[serde(rename = "CES41")]
    Ces41,
    #[serde(rename = "CES42")]
    Ces42,
    #[serde(rename = "CES43")]
    Ces43,
    #[serde(rename = "CES44")]
    Ces44,
    #[serde(rename = "CES45")]
    Ces45,
    #[serde(rename = "CES46")]
    Ces46,
    #[serde(rename = "CES47")]
    Ces47,
    #[serde(rename = "CES48")]
    Ces48,
    #[serde(rename = "CES49")]
    Ces49,
    #[serde(rename = "CES50")]
    Ces50,
    #[serde(rename = "CES51")]
    Ces51,
    #[serde(rename = "CES52")]
    Ces52,
    #[serde(rename = "CES53")]
    Ces53,
    #[serde(rename = "CES54")]
    Ces54,
    #[serde(rename = "CES55")]
    Ces55,
    #[serde(rename = "CES56")]
    Ces56,
    #[serde(rename = "CES57")]
    Ces57,
    #[serde(rename = "CES58")]
    Ces58,
    #[serde(rename = "CES59")]
    Ces59,
    #[serde(rename = "CES60")]
    Ces60,
    #[serde(rename = "CES61")]
    Ces61,
    #[serde(rename = "CES62")]
    Ces62,
    #[serde(rename = "CES63")]
    Ces63,
    #[serde(rename = "CES64")]
    Ces64,
    #[serde(rename = "CES65")]
    Ces65,
    #[serde(rename = "CES66")]
    Ces66,
    #[serde(rename = "CES67")]
    Ces67,
    #[serde(rename = "CES68")]
    Ces68,
    #[serde(rename = "CES69")]
    Ces69,
    #[serde(rename = "CES70")]
    Ces70,
    #[serde(rename = "CES71")]
    Ces71,
    #[serde(rename = "CES72")]
    Ces72,
    #[serde(rename = "CES73")]
    Ces73,
    #[serde(rename = "CES74")]
    Ces74,
    #[serde(rename = "CES75")]
    Ces75,
    #[serde(rename = "CES76")]
    Ces76,
    #[serde(rename = "CES77")]
    Ces77,
    #[serde(rename = "CES78")]
    Ces78,
    #[serde(rename = "CES79")]
    Ces79,
    #[serde(rename = "CES80")]
    Ces80,
    #[serde(rename = "CES81")]
    Ces81,
    #[serde(rename = "CES82")]
    Ces82,
    #[serde(rename = "CES83")]
    Ces83,
    #[serde(rename = "CES84")]
    Ces84,
    #[serde(rename = "CES85")]
    Ces85,
    #[serde(rename = "CES86")]
    Ces86,
    #[serde(rename = "CES87")]
    Ces87,
    #[serde(rename = "CES88")]
    Ces88,
    #[serde(rename = "CES89")]
    Ces89,
    #[serde(rename = "CES90")]
    Ces90,
    #[serde(rename = "CES91")]
    Ces91,
    #[serde(rename = "CES92")]
    Ces92,
    #[serde(rename = "CES93")]
    Ces93,
    #[serde(rename = "CES94")]
    Ces94,
    #[serde(rename = "CES95")]
    Ces95,
    #[serde(rename = "CES96")]
    Ces96,
    #[serde(rename = "CES97")]
    Ces97,
    #[serde(rename = "CES98")]
    Ces98,
    #[serde(rename = "CES99")]
    Ces99,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Citp;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PosiStageNet;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenSoundControl;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cri {
    #[serde(default = "Cri::default_ces", rename = "@CES")]
    pub ces: Ces,
    #[serde(default, rename = "@ColorRenderingIndex")]
    pub color_rendering_index: Option<u8>,
}

impl Cri {
    #[must_use]
    pub fn default_ces() -> Ces {
        Ces::Ces01
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CriGroup {
    #[serde(default, rename = "@ColorTemperature")]
    pub color_temperature: Option<f32>,
    #[serde(rename = "CRI")]
    pub cris: Vec<Cri>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CrIs {
    #[serde(default, rename = "CRIGroup")]
    pub cri_groups: Vec<CriGroup>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChannelFunction {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
    #[serde(default = "ChannelFunction::default_attribute", rename = "@Attribute")]
    pub attribute: String,
    #[serde(
        default = "ChannelFunction::default_original_attribute",
        rename = "@OriginalAttribute"
    )]
    pub original_attribute: String,
    #[serde(default = "ChannelFunction::default_dmx_from", rename = "@DMXFrom")]
    pub dmx_from: String,
    #[serde(default = "ChannelFunction::default_default", rename = "@Default")]
    pub default: String,
    #[serde(default, rename = "@PhysicalFrom")]
    pub physical_from: Option<f32>,
    #[serde(default, rename = "@PhysicalTo")]
    pub physical_to: Option<f32>,
    #[serde(default, rename = "@RealFade")]
    pub real_fade: Option<f32>,
    #[serde(default, rename = "@RealAcceleration")]
    pub real_acceleration: Option<f32>,
    #[serde(default, rename = "@Wheel")]
    pub wheel: Option<String>,
    #[serde(default, rename = "@Emitter")]
    pub emitter: Option<String>,
    #[serde(default, rename = "@Filter")]
    pub filter: Option<String>,
    #[serde(default, rename = "@ColorSpace")]
    pub color_space: Option<String>,
    // FIXME: The XSD makes this generate as @Gammut. GDTF XSD bug?
    #[serde(default, rename = "@Gamut")]
    pub gamut: Option<String>,
    #[serde(default, rename = "@ModeMaster")]
    pub mode_master: Option<String>,
    #[serde(default = "ChannelFunction::default_mode_from", rename = "@ModeFrom")]
    pub mode_from: String,
    #[serde(default = "ChannelFunction::default_mode_to", rename = "@ModeTo")]
    pub mode_to: String,
    #[serde(default, rename = "@DMXProfile")]
    pub dmx_profile: Option<String>,
    #[serde(default, rename = "@Min")]
    pub min: Option<f32>,
    #[serde(default, rename = "@Max")]
    pub max: Option<f32>,
    #[serde(default, rename = "@CustomName")]
    pub custom_name: Option<String>,
    #[serde(default, rename = "ChannelSet")]
    pub channel_sets: Vec<ChannelSet>,
    #[serde(default, rename = "SubChannelSet")]
    pub sub_channel_sets: Vec<SubChannelSet>,
}

impl ChannelFunction {
    #[must_use]
    pub fn default_attribute() -> String {
        String::from("NoFeature")
    }
    #[must_use]
    pub fn default_original_attribute() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_dmx_from() -> String {
        String::from("0/1")
    }
    #[must_use]
    pub fn default_default() -> String {
        String::from("0/1")
    }
    #[must_use]
    pub fn default_mode_from() -> String {
        String::from("0/1")
    }
    #[must_use]
    pub fn default_mode_to() -> String {
        String::from("0/1")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChannelSet {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
    #[serde(default = "ChannelSet::default_dmx_from", rename = "@DMXFrom")]
    pub dmx_from: String,
    #[serde(default, rename = "@PhysicalFrom")]
    pub physical_from: Option<f32>,
    #[serde(default, rename = "@PhysicalTo")]
    pub physical_to: Option<f32>,
    #[serde(default, rename = "@WheelSlotIndex")]
    pub wheel_slot_index: Option<i32>,
}

impl ChannelSet {
    #[must_use]
    pub fn default_dmx_from() -> String {
        String::from("0/1")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ColorSpace {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
    #[serde(default = "ColorSpace::default_mode", rename = "@Mode")]
    pub mode: ColorSpaceMode,
    #[serde(default, rename = "@Red")]
    pub red: Option<String>,
    #[serde(default, rename = "@Green")]
    pub green: Option<String>,
    #[serde(default, rename = "@Blue")]
    pub blue: Option<String>,
    #[serde(default, rename = "@WhitePoint")]
    pub white_point: Option<String>,
}

impl ColorSpace {
    #[must_use]
    pub fn default_mode() -> ColorSpaceMode {
        ColorSpaceMode::SRgb
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ColorSpaceMode {
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "sRGB")]
    SRgb,
    #[serde(rename = "ProPhoto")]
    ProPhoto,
    #[serde(rename = "ANSI")]
    Ansi,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Connector {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub r#type: String,
    #[serde(default, rename = "@DMXBreak")]
    pub dmx_break: Option<u8>,
    #[serde(default, rename = "@Gender")]
    pub gender: Option<i32>,
    #[serde(default, rename = "@Length")]
    pub length: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Connectors {
    #[serde(default, rename = "Connector")]
    pub connectors: Vec<Connector>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CrossSectionType {
    #[serde(rename = "TrussFramework")]
    TrussFramework,
    #[serde(rename = "Tube")]
    Tube,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxChannel {
    #[serde(default = "DmxChannel::default_dmx_break", rename = "@DMXBreak")]
    pub dmx_break: String,
    #[serde(default = "DmxChannel::default_offset", rename = "@Offset")]
    pub offset: String,
    #[serde(default, rename = "@InitialFunction")]
    pub initial_function: Option<String>,
    #[serde(default = "DmxChannel::default_highlight", rename = "@Highlight")]
    pub highlight: String,
    #[serde(rename = "@Geometry")]
    pub geometry: String,
    #[serde(default, rename = "LogicalChannel")]
    pub logical_channels: Vec<LogicalChannel>,
}

impl DmxChannel {
    #[must_use]
    pub fn default_dmx_break() -> String {
        String::from("1")
    }
    #[must_use]
    pub fn default_offset() -> String {
        String::from("None")
    }
    #[must_use]
    pub fn default_highlight() -> String {
        String::from("None")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxChannels {
    #[serde(default, rename = "DMXChannel")]
    pub dmx_channels: Vec<DmxChannel>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxMode {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Geometry")]
    pub geometry: Option<String>,
    #[serde(default, rename = "@Description")]
    pub description: Option<String>,
    #[serde(rename = "DMXChannels")]
    pub dmx_channels: DmxChannels,
    #[serde(default, rename = "Relations")]
    pub relations: Option<Relations>,
    #[serde(default, rename = "FTMacros")]
    pub ft_macros: Option<FtMacros>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxModes {
    #[serde(default, rename = "DMXMode")]
    pub dmx_modes: Vec<DmxMode>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxPersonality {
    #[serde(default, rename = "@Value")]
    pub value: Option<String>,
    #[serde(default, rename = "@DMXMode")]
    pub dmx_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxProfile {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
    #[serde(default, rename = "Point")]
    pub points: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxProfiles {
    #[serde(default, rename = "DMXProfile")]
    pub dmx_profiles: Vec<DmxProfile>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Display {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "Display::default_position", rename = "@Position")]
    pub position: String,
    #[serde(default, rename = "@Texture")]
    pub texture: Option<String>,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
}

impl Display {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Emitter {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Color")]
    pub color: Option<String>,
    #[serde(default, rename = "@DominantWaveLength")]
    pub dominant_wave_length: Option<f32>,
    #[serde(default, rename = "@DiodePart")]
    pub diode_part: Option<String>,
    #[serde(default, rename = "Measurement")]
    pub measurements: Vec<EmitterMeasurement>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmitterMeasurement {
    #[serde(rename = "@Physical")]
    pub physical: f32,
    #[serde(rename = "@LuminousIntensity")]
    pub luminous_intensity: f32,
    #[serde(default = "EmitterMeasurement::default_interpolation_to", rename = "@InterpolationTo")]
    pub interpolation_to: InterpolationTo,
    #[serde(default, rename = "@Transmission")]
    pub transmission: Option<f32>,
    #[serde(default, rename = "MeasurementPoint")]
    pub measurement_points: Vec<MeasurementPoint>,
}

impl EmitterMeasurement {
    #[must_use]
    pub fn default_interpolation_to() -> InterpolationTo {
        InterpolationTo::Linear
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Emitters {
    #[serde(default, rename = "Emitter")]
    pub emitters: Vec<Emitter>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FtMacro {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@ChannelFunction")]
    pub channel_function: Option<String>,
    #[serde(default, rename = "MacroDMX")]
    pub macro_dmx: Option<MacroDmx>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FtMacros {
    #[serde(default, rename = "FTMacro")]
    pub ft_macros: Vec<FtMacro>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FtPreset;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FtPresets {
    #[serde(default, rename = "FTPreset")]
    pub content: Vec<FtPreset>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ftrdm {
    #[serde(default, rename = "@ManufacturerID")]
    pub manufacturer_id: Option<String>,
    #[serde(default, rename = "@DeviceModelID")]
    pub device_model_id: Option<String>,
    #[serde(default, rename = "SoftwareVersionID")]
    pub software_version_ids: Vec<SoftwareVersionId>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Facet {
    #[serde(default, rename = "@Color")]
    pub color: Option<String>,
    #[serde(rename = "@Rotation")]
    pub rotation: String,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Feature {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FeatureGroup {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Pretty")]
    pub pretty: Option<String>,
    #[serde(default, rename = "Feature")]
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FeatureGroups {
    #[serde(default, rename = "FeatureGroup")]
    pub feature_groups: Vec<FeatureGroup>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Filter {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Color")]
    pub color: Option<String>,
    #[serde(default, rename = "Measurement")]
    pub measurements: Vec<FilterMeasurement>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FilterMeasurement {
    #[serde(rename = "@Physical")]
    pub physical: f32,
    #[serde(rename = "@Transmission")]
    pub transmission: f32,
    #[serde(default = "FilterMeasurement::default_interpolation_to", rename = "@InterpolationTo")]
    pub interpolation_to: InterpolationTo,
    #[serde(default, rename = "MeasurementPoint")]
    pub measurement_points: Vec<MeasurementPoint>,
}

impl FilterMeasurement {
    #[must_use]
    pub fn default_interpolation_to() -> InterpolationTo {
        InterpolationTo::Linear
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Filters {
    #[serde(default, rename = "Filter")]
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@ShortName")]
    pub short_name: Option<String>,
    #[serde(default, rename = "@LongName")]
    pub long_name: Option<String>,
    #[serde(rename = "@Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "@Description")]
    pub description: String,
    #[serde(rename = "@FixtureTypeID")]
    pub fixture_type_id: String,
    #[serde(default, rename = "@Thumbnail")]
    pub thumbnail: Option<String>,
    #[serde(default, rename = "@ThumbnailOffsetX")]
    pub thumbnail_offset_x: Option<i64>,
    #[serde(default, rename = "@ThumbnailOffsetY")]
    pub thumbnail_offset_y: Option<i64>,
    #[serde(default, rename = "@RefFT")]
    pub ref_ft: Option<String>,
    #[serde(default = "FixtureType::default_can_have_children", rename = "@CanHaveChildren")]
    pub can_have_children: YesNo,
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: AttributeDefinitions,
    #[serde(default, rename = "Wheels")]
    pub wheels: Option<Wheels>,
    #[serde(default, rename = "PhysicalDescriptions")]
    pub physical_descriptions: Option<PhysicalDescriptions>,
    #[serde(default, rename = "Models")]
    pub models: Option<Models>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(rename = "DMXModes")]
    pub dmx_modes: DmxModes,
    #[serde(default, rename = "Revisions")]
    pub revisions: Option<Revisions>,
    #[serde(default, rename = "FTPresets")]
    pub ft_presets: Option<FtPresets>,
    #[serde(default, rename = "Protocols")]
    pub protocols: Option<Protocols>,
}

impl FixtureType {
    #[must_use]
    pub fn default_can_have_children() -> YesNo {
        YesNo::Yes
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gamut {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
    #[serde(default, rename = "@Points")]
    pub points: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gamuts {
    #[serde(default, rename = "Gamut")]
    pub gamuts: Vec<Gamut>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GdtfDescription {
    #[serde(rename = "@DataVersion")]
    pub data_version: String,
    #[serde(rename = "FixtureType")]
    pub fixture_type: FixtureType,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Geometries {
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Geometry {
    #[serde(rename = "Geometry")]
    Geometry(BasicGeometryType),
    #[serde(rename = "Axis")]
    Axis(BasicGeometryType),
    #[serde(rename = "FilterBeam")]
    FilterBeam(BasicGeometryType),
    #[serde(rename = "FilterColor")]
    FilterColor(BasicGeometryType),
    #[serde(rename = "FilterGobo")]
    FilterGobo(BasicGeometryType),
    #[serde(rename = "FilterShaper")]
    FilterShaper(BasicGeometryType),
    #[serde(rename = "Beam")]
    Beam(Beam),
    #[serde(rename = "MediaServerLayer")]
    MediaServerLayer(BasicGeometryType),
    #[serde(rename = "MediaServerCamera")]
    MediaServerCamera(BasicGeometryType),
    #[serde(rename = "MediaServerMaster")]
    MediaServerMaster(BasicGeometryType),
    #[serde(rename = "Display")]
    Display(Display),
    #[serde(rename = "Laser")]
    Laser(Laser),
    #[serde(rename = "GeometryReference")]
    GeometryReference(GeometryReference),
    #[serde(rename = "WiringObject")]
    WiringObject(WiringObject),
    #[serde(rename = "Inventory")]
    Inventory(Inventory),
    #[serde(rename = "Structure")]
    Structure(Structure),
    #[serde(rename = "Support")]
    Support(Support),
    #[serde(rename = "Magnet")]
    Magnet(BasicGeometryType),
}

impl Geometry {
    pub fn name(&self) -> &str {
        match self {
            Geometry::Geometry(v) => &v.name,
            Geometry::Axis(v) => &v.name,
            Geometry::FilterBeam(v) => &v.name,
            Geometry::FilterColor(v) => &v.name,
            Geometry::FilterGobo(v) => &v.name,
            Geometry::FilterShaper(v) => &v.name,
            Geometry::Beam(v) => &v.name,
            Geometry::MediaServerLayer(v) => &v.name,
            Geometry::MediaServerCamera(v) => &v.name,
            Geometry::MediaServerMaster(v) => &v.name,
            Geometry::Display(v) => &v.name,
            Geometry::Laser(v) => &v.name,
            Geometry::GeometryReference(v) => &v.name,
            Geometry::WiringObject(v) => &v.name,
            Geometry::Inventory(v) => &v.name,
            Geometry::Structure(v) => &v.name,
            Geometry::Support(v) => &v.name,
            Geometry::Magnet(v) => &v.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeometryReference {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "GeometryReference::default_position", rename = "@Position")]
    pub position: String,
    #[serde(default, rename = "@Geometry")]
    pub geometry: Option<String>,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
    #[serde(default, rename = "Break")]
    pub breaks: Vec<Break>,
}

impl GeometryReference {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum InterpolationTo {
    #[serde(rename = "Linear")]
    Linear,
    #[serde(rename = "Step")]
    Step,
    #[serde(rename = "Log")]
    Log,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Inventory {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "Inventory::default_position", rename = "@Position")]
    pub position: String,

    #[serde(default, rename = "@Count")]
    pub count: Option<u32>,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
}

impl Inventory {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LampType {
    #[serde(rename = "Discharge")]
    Discharge,
    #[serde(rename = "Tungsten")]
    Tungsten,
    #[serde(rename = "Halogen")]
    Halogen,
    #[serde(rename = "LED")]
    Led,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Laser {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "Laser::default_position", rename = "@Position")]
    pub position: String,
    #[serde(default = "Laser::default_color_type", rename = "@ColorType")]
    pub color_type: LaserColorType,
    #[serde(default, rename = "@Color")]
    pub color: Option<f32>,
    #[serde(default, rename = "@OutputStrength")]
    pub output_strength: Option<f32>,
    #[serde(default, rename = "@Emitter")]
    pub emitter: Option<String>,
    #[serde(default, rename = "@BeamDiameter")]
    pub beam_diameter: Option<f32>,
    #[serde(default, rename = "@BeamDivergenceMin")]
    pub beam_divergence_min: Option<f32>,
    #[serde(default, rename = "@BeamDivergenceMax")]
    pub beam_divergence_max: Option<f32>,
    #[serde(default, rename = "@ScanAnglePan")]
    pub scan_angle_pan: Option<f32>,
    #[serde(default, rename = "@ScanAngleTilt")]
    pub scan_angle_tilt: Option<f32>,
    #[serde(default, rename = "@ScanSpeed")]
    pub scan_speed: Option<f32>,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
    #[serde(default, rename = "Protocol")]
    pub protocols: Vec<Protocol>,
}

impl Laser {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
    #[must_use]
    pub fn default_color_type() -> LaserColorType {
        LaserColorType::Rgb
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LaserColorType {
    #[serde(rename = "RGB")]
    Rgb,
    #[serde(rename = "SingleWaveLength")]
    SingleWaveLength,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LegHeight {
    #[serde(default, rename = "@Value")]
    pub value: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicalChannel {
    #[serde(rename = "@Attribute")]
    pub attribute: String,
    #[serde(default = "LogicalChannel::default_snap", rename = "@Snap")]
    pub snap: Snap,
    #[serde(default = "LogicalChannel::default_master", rename = "@Master")]
    pub master: Master,
    #[serde(default, rename = "@MibFade")]
    pub mib_fade: Option<f32>,
    #[serde(default, rename = "@DMXChangeTimeLimit")]
    pub dmx_change_time_limit: Option<f32>,
    #[serde(default, rename = "ChannelFunction")]
    pub channel_functions: Vec<ChannelFunction>,
}

impl LogicalChannel {
    #[must_use]
    pub fn default_snap() -> Snap {
        Snap::No
    }
    #[must_use]
    pub fn default_master() -> Master {
        Master::None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MacroDmx {
    #[serde(default, rename = "MacroDMXStep")]
    pub macro_dmx_steps: Vec<MacroDmxStep>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MacroDmxStep {
    #[serde(default, rename = "@Duration")]
    pub duration: Option<f32>,
    #[serde(default, rename = "MacroDMXValue")]
    pub macro_dmx_values: Vec<MacroDmxValue>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MacroDmxValue {
    #[serde(rename = "@Value")]
    pub value: String,
    #[serde(rename = "@DMXChannel")]
    pub dmx_channel: String,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Map {
    #[serde(default, rename = "@Key")]
    pub key: u32,
    #[serde(default, rename = "@Value")]
    pub value: u32,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Master {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Grand")]
    Grand,
    #[serde(rename = "Group")]
    Group,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MeasurementPoint {
    #[serde(rename = "@WaveLength")]
    pub wave_length: f32,
    #[serde(rename = "@Energy")]
    pub energy: f32,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Model {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default = "Model::default_length", rename = "@Length")]
    pub length: f32,
    #[serde(default = "Model::default_width", rename = "@Width")]
    pub width: f32,
    #[serde(default = "Model::default_height", rename = "@Height")]
    pub height: f32,
    #[serde(default = "Model::default_primitive_type", rename = "@PrimitiveType")]
    pub primitive_type: PrimitiveType,
    #[serde(default = "Model::default_file", rename = "@File")]
    pub file: String,
    #[serde(default, rename = "@SVGOffsetX")]
    pub svg_offset_x: Option<f32>,
    #[serde(default, rename = "@SVGOffsetY")]
    pub svg_offset_y: Option<f32>,
    #[serde(default, rename = "@SVGSideOffsetX")]
    pub svg_side_offset_x: Option<f32>,
    #[serde(default, rename = "@SVGSideOffsetY")]
    pub svg_side_offset_y: Option<f32>,
    #[serde(default, rename = "@SVGFrontOffsetX")]
    pub svg_front_offset_x: Option<f32>,
    #[serde(default, rename = "@SVGFrontOffsetY")]
    pub svg_front_offset_y: Option<f32>,
}

impl Model {
    #[must_use]
    pub fn default_length() -> f32 {
        0f32
    }
    #[must_use]
    pub fn default_width() -> f32 {
        0f32
    }
    #[must_use]
    pub fn default_height() -> f32 {
        0f32
    }
    #[must_use]
    pub fn default_primitive_type() -> PrimitiveType {
        PrimitiveType::Undefined
    }
    #[must_use]
    pub fn default_file() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Models {
    #[serde(default, rename = "Model")]
    pub models: Vec<Model>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OperatingTemperature {
    #[serde(default, rename = "@Low")]
    pub low: Option<f32>,
    #[serde(default, rename = "@High")]
    pub high: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhysicalDescriptions {
    #[serde(default, rename = "Emitters")]
    pub emitters: Option<Emitters>,
    #[serde(default, rename = "Filters")]
    pub filters: Option<Filters>,
    #[serde(default, rename = "ColorSpace")]
    pub color_space: Option<ColorSpace>,
    #[serde(default, rename = "AdditionalColorSpaces")]
    pub additional_color_spaces: Option<AdditionalColorSpaces>,
    #[serde(default, rename = "Gamuts")]
    pub gamuts: Option<Gamuts>,
    #[serde(default, rename = "DMXProfiles")]
    pub dmx_profiles: Option<DmxProfiles>,
    #[serde(default, rename = "CRIs")]
    pub cr_is: Option<CrIs>,
    #[serde(default, rename = "Connectors")]
    pub connectors: Option<Connectors>,
    #[serde(default, rename = "Properties")]
    pub properties: Option<Properties>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PhysicalUnit {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Percent")]
    Percent,
    #[serde(rename = "Length")]
    Length,
    #[serde(rename = "Mass")]
    Mass,
    #[serde(rename = "Time")]
    Time,
    #[serde(rename = "Temperature")]
    Temperature,
    #[serde(rename = "LuminousIntensity")]
    LuminousIntensity,
    #[serde(rename = "Angle")]
    Angle,
    #[serde(rename = "Force")]
    Force,
    #[serde(rename = "Frequency")]
    Frequency,
    #[serde(rename = "Current")]
    Current,
    #[serde(rename = "Voltage")]
    Voltage,
    #[serde(rename = "Power")]
    Power,
    #[serde(rename = "Energy")]
    Energy,
    #[serde(rename = "Area")]
    Area,
    #[serde(rename = "Volume")]
    Volume,
    #[serde(rename = "Speed")]
    Speed,
    #[serde(rename = "Acceleration")]
    Acceleration,
    #[serde(rename = "AngularSpeed")]
    AngularSpeed,
    #[serde(rename = "AngularAccc")]
    AngularAccc,
    #[serde(rename = "WaveLength")]
    WaveLength,
    #[serde(rename = "ColorComponent")]
    ColorComponent,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PinPatch {
    #[serde(default, rename = "@ToWiringObject")]
    pub to_wiring_object: Option<String>,
    #[serde(default, rename = "@FromPin")]
    pub from_pin: Option<i32>,
    #[serde(default, rename = "@ToPin")]
    pub to_pin: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Point {
    #[serde(default, rename = "@DMXPercentage")]
    pub dmx_percentage: Option<f32>,
    #[serde(default, rename = "@CFC0")]
    pub cfc_0: Option<f32>,
    #[serde(default, rename = "@CFC1")]
    pub cfc_1: Option<f32>,
    #[serde(default, rename = "@CFC2")]
    pub cfc_2: Option<f32>,
    #[serde(default, rename = "@CFC3")]
    pub cfc_3: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PowerConsumption {
    #[serde(default, rename = "@Value")]
    pub value: Option<f32>,
    #[serde(default, rename = "@PowerFactor")]
    pub power_factor: Option<f32>,
    #[serde(rename = "@Connector")]
    pub connector: String,
    #[serde(default, rename = "@VoltageLow")]
    pub voltage_low: Option<f32>,
    #[serde(default, rename = "@VoltageHigh")]
    pub voltage_high: Option<f32>,
    #[serde(default, rename = "@FrequencyLow")]
    pub frequency_low: Option<f32>,
    #[serde(default, rename = "@FrequencyHigh")]
    pub frequency_high: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PrimitiveType {
    #[serde(rename = "Undefined")]
    Undefined,
    #[serde(rename = "Cube")]
    Cube,
    #[serde(rename = "Cylinder")]
    Cylinder,
    #[serde(rename = "Sphere")]
    Sphere,
    #[serde(rename = "Base")]
    Base,
    #[serde(rename = "Yoke")]
    Yoke,
    #[serde(rename = "Head")]
    Head,
    #[serde(rename = "Scanner")]
    Scanner,
    #[serde(rename = "Conventional")]
    Conventional,
    #[serde(rename = "Pigtail")]
    Pigtail,
    #[serde(rename = "Base1_1")]
    Base11,
    #[serde(rename = "Scanner1_1")]
    Scanner11,
    #[serde(rename = "Conventional1_1")]
    Conventional11,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Properties {
    #[serde(default, rename = "$value")]
    pub content: Vec<PropertiesContent>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PropertiesContent {
    #[serde(rename = "OperatingTemperature")]
    OperatingTemperature(OperatingTemperature),
    #[serde(rename = "Weight")]
    Weight(Weight),
    #[serde(rename = "PowerConsumption")]
    PowerConsumption(PowerConsumption),
    #[serde(rename = "LegHeight")]
    LegHeight(LegHeight),
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocol {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocols {
    #[serde(default, rename = "FTRDM")]
    pub ftrdm: Option<Ftrdm>,
    #[serde(default, rename = "Art-Net")]
    pub art_net: Option<ArtNet>,
    #[serde(default, rename = "sACN")]
    pub s_acn: Option<SAcn>,
    #[serde(default, rename = "PosiStageNet")]
    pub posi_stage_net: Option<PosiStageNet>,
    #[serde(default, rename = "OpenSoundControl")]
    pub open_sound_control: Option<OpenSoundControl>,
    #[serde(default, rename = "CITP")]
    pub citp: Option<Citp>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Relation {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Master")]
    pub master: String,
    #[serde(rename = "@Follower")]
    pub follower: String,
    #[serde(rename = "@Type")]
    pub r#type: RelationType,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RelationType {
    #[serde(rename = "Multiply")]
    Multiply,
    #[serde(rename = "Override")]
    Override,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Relations {
    #[serde(default, rename = "Relation")]
    pub relations: Vec<Relation>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Revision {
    #[serde(default = "Revision::default_text", rename = "@Text")]
    pub text: String,
    #[serde(default, rename = "@Date")]
    pub date: Option<String>,
    #[serde(default = "Revision::default_user_id", rename = "@UserID")]
    pub user_id: i32,
    #[serde(default = "Revision::default_modified_by", rename = "@ModifiedBy")]
    pub modified_by: String,
}

impl Revision {
    #[must_use]
    pub fn default_text() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_user_id() -> i32 {
        0i32
    }
    #[must_use]
    pub fn default_modified_by() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Revisions {
    #[serde(default, rename = "Revision")]
    pub revisions: Vec<Revision>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Slot {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Color")]
    pub color: Option<String>,
    #[serde(default, rename = "@Filter")]
    pub filter: Option<String>,
    #[serde(default = "Slot::default_media_file_name", rename = "@MediaFileName")]
    pub media_file_name: String,
    #[serde(default, rename = "@BeamInfluence")]
    pub beam_influence: Option<String>,
    #[serde(default, rename = "$value")]
    pub content: Vec<SlotContent>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SlotContent {
    #[serde(rename = "Facet")]
    Facet(Facet),
    #[serde(rename = "AnimationSystem")]
    AnimationSystem(AnimationSystem),
}

impl Slot {
    #[must_use]
    pub fn default_media_file_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Snap {
    #[serde(rename = "Yes")]
    Yes,
    #[serde(rename = "No")]
    No,
    #[serde(rename = "On")]
    On,
    #[serde(rename = "Off")]
    Off,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SoftwareVersionId {
    #[serde(default, rename = "@Value")]
    pub value: Option<String>,
    #[serde(default, rename = "DMXPersonality")]
    pub dmx_personalities: Vec<DmxPersonality>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Structure {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "Structure::default_position", rename = "@Position")]
    pub position: String,
    #[serde(default, rename = "@LinkedGeometry")]
    pub linked_geometry: Option<String>,
    #[serde(default, rename = "@StructureType")]
    pub structure_type: Option<StructureType>,
    #[serde(default, rename = "@CrossSectionType")]
    pub cross_section_type: Option<CrossSectionType>,
    #[serde(default, rename = "@CrossSectionHeight")]
    pub cross_section_height: Option<f32>,
    #[serde(default, rename = "@CrossSectionWallThickness")]
    pub cross_section_wall_thickness: Option<f32>,
    #[serde(default, rename = "@TrussCrossSection")]
    pub truss_cross_section: Option<String>,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
}

impl Structure {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum StructureType {
    #[serde(rename = "CenterLineBased")]
    CenterLineBased,
    #[serde(rename = "Detail")]
    Detail,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubChannelSet {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
    #[serde(rename = "@PhysicalFrom")]
    pub physical_from: f32,
    #[serde(rename = "@PhysicalTo")]
    pub physical_to: f32,
    #[serde(rename = "@SubPhysicalUnit")]
    pub sub_physical_unit: String,
    #[serde(default, rename = "@DMXProfile")]
    pub dmx_profile: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SubPhysicalType {
    #[serde(rename = "PlacementOffset")]
    PlacementOffset,
    #[serde(rename = "Amplitude")]
    Amplitude,
    #[serde(rename = "AmplitudeMin")]
    AmplitudeMin,
    #[serde(rename = "Duration")]
    Duration,
    #[serde(rename = "DutyCycle")]
    DutyCycle,
    #[serde(rename = "TimeOffset")]
    TimeOffset,
    #[serde(rename = "MinimumOpening")]
    MinimumOpening,
    #[serde(rename = "Value")]
    Value,
    #[serde(rename = "RatioHorizontal")]
    RatioHorizontal,
    #[serde(rename = "RatioVertical")]
    RatioVertical,
    #[serde(rename = "AmplitudeMax")]
    AmplitudeMax,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubPhysicalUnit {
    #[serde(rename = "@Type")]
    pub r#type: SubPhysicalType,
    #[serde(default, rename = "@PhysicalUnit")]
    pub physical_unit: Option<PhysicalUnit>,
    #[serde(default, rename = "@PhysicalFrom")]
    pub physical_from: Option<f32>,
    #[serde(default, rename = "@PhysicalTo")]
    pub physical_to: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Support {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "Support::default_position", rename = "@Position")]
    pub position: String,
    #[serde(rename = "@SupportType")]
    pub support_type: SupportType,
    #[serde(default, rename = "@RopeCrossSection")]
    pub rope_cross_section: Option<String>,
    #[serde(default, rename = "@RopeOffset")]
    pub rope_offset: Option<String>,
    #[serde(default, rename = "@CapacityX")]
    pub capacity_x: Option<f32>,
    #[serde(default, rename = "@CapacityY")]
    pub capacity_y: Option<f32>,
    #[serde(default, rename = "@CapacityZ")]
    pub capacity_z: Option<f32>,
    #[serde(default, rename = "@CapacityXX")]
    pub capacity_xx: Option<f32>,
    #[serde(default, rename = "@CapacityYY")]
    pub capacity_yy: Option<f32>,
    #[serde(default, rename = "@CapacityZZ")]
    pub capacity_zz: Option<f32>,
    #[serde(default, rename = "@ResistanceX")]
    pub resistance_x: Option<f32>,
    #[serde(default, rename = "@ResistanceY")]
    pub resistance_y: Option<f32>,
    #[serde(default, rename = "@ResistanceZ")]
    pub resistance_z: Option<f32>,
    #[serde(default, rename = "@ResistanceXX")]
    pub resistance_xx: Option<f32>,
    #[serde(default, rename = "@ResistanceYY")]
    pub resistance_yy: Option<f32>,
    #[serde(default, rename = "@ResistanceZZ")]
    pub resistance_zz: Option<f32>,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
}

impl Support {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SupportType {
    #[serde(rename = "Rope")]
    Rope,
    #[serde(rename = "GroundSupport")]
    GroundSupport,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Weight {
    #[serde(default, rename = "@Value")]
    pub value: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wheel {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
    #[serde(default, rename = "Slot")]
    pub slots: Vec<Slot>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wheels {
    #[serde(default, rename = "Wheel")]
    pub wheels: Vec<Wheel>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WiringComponentType {
    #[serde(rename = "Input")]
    Input,
    #[serde(rename = "Output")]
    Output,
    #[serde(rename = "PowerSource")]
    PowerSource,
    #[serde(rename = "Consumer")]
    Consumer,
    #[serde(rename = "Fuse")]
    Fuse,
    #[serde(rename = "NetworkProvider")]
    NetworkProvider,
    #[serde(rename = "NetworkInput")]
    NetworkInput,
    #[serde(rename = "NetworkOutput")]
    NetworkOutput,
    #[serde(rename = "NetworkInOut")]
    NetworkInOut,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WiringFuseRating {
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "K")]
    K,
    #[serde(rename = "Z")]
    Z,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WiringObject {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default = "WiringObject::default_position", rename = "@Position")]
    pub position: String,
    #[serde(default, rename = "@ConnectorType")]
    pub connector_type: Option<String>,
    #[serde(default, rename = "@ComponentType")]
    pub component_type: Option<WiringComponentType>,
    #[serde(default, rename = "@SignalType")]
    pub signal_type: Option<String>,
    #[serde(default, rename = "@PinCount")]
    pub pin_count: Option<i32>,
    #[serde(default, rename = "@ElectricalPayLoad")]
    pub electrical_pay_load: Option<f32>,
    #[serde(default, rename = "@VoltageRangeMax")]
    pub voltage_range_max: Option<f32>,
    #[serde(default, rename = "@VoltageRangeMin")]
    pub voltage_range_min: Option<f32>,
    #[serde(default, rename = "@FrequencyRangeMax")]
    pub frequency_range_max: Option<f32>,
    #[serde(default, rename = "@FrequencyRangeMin")]
    pub frequency_range_min: Option<f32>,
    #[serde(default, rename = "@MaxPayLoad")]
    pub max_pay_load: Option<f32>,
    #[serde(default, rename = "@Voltage")]
    pub voltage: Option<f32>,
    #[serde(default, rename = "@SignalLayer")]
    pub signal_layer: Option<i32>,
    #[serde(default, rename = "@CosPhi")]
    pub cos_phi: Option<f32>,
    #[serde(default, rename = "@FuseCurrent")]
    pub fuse_current: Option<f32>,
    #[serde(default, rename = "@FuseRating")]
    pub fuse_rating: Option<WiringFuseRating>,
    #[serde(default, rename = "@Orientation")]
    pub orientation: Option<WiringOrientation>,
    #[serde(default, rename = "@WireGroup")]
    pub wire_group: Option<String>,
    #[serde(default, rename = "$value")]
    pub children: Vec<Geometry>,
    #[serde(default, rename = "PinPatch")]
    pub pin_patches: Vec<PinPatch>,
}

impl WiringObject {
    #[must_use]
    pub fn default_position() -> String {
        String::from("{1,0,0,0}{0,1,0,0}{0,0,1,0}{0,0,0,1}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WiringOrientation {
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "Right")]
    Right,
    #[serde(rename = "Top")]
    Top,
    #[serde(rename = "Bottom")]
    Bottom,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum YesNo {
    #[serde(rename = "Yes")]
    Yes,
    #[serde(rename = "No")]
    No,
}

impl From<YesNo> for bool {
    fn from(value: YesNo) -> Self {
        match value {
            YesNo::Yes => true,
            YesNo::No => false,
        }
    }
}

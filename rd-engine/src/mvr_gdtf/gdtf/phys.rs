use std::str::FromStr as _;

use crate::mvr_gdtf::{
    CieColor,
    gdtf::{Name, bundle, parse_optional_name},
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Emitter {
    name: Name,
    color: EmitterColor,
    diode_part: Option<String>,
    measurements: Vec<EmitterMeasurement>,
}

impl Emitter {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn color(&self) -> EmitterColor {
        self.color
    }

    pub fn diode_part(&self) -> Option<&str> {
        self.diode_part.as_deref()
    }

    pub fn measurements(&self) -> &[EmitterMeasurement] {
        &self.measurements
    }
}

impl bundle::FromBundle for Emitter {
    type Source = bundle::Emitter;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: Name::new(source.name.clone()),
            color: if let Some(c) = source.color.as_deref() {
                EmitterColor::Color(CieColor::from_str(c).unwrap())
            } else {
                EmitterColor::DominantWaveLength(source.dominant_wave_length.unwrap())
            },
            diode_part: match source.diode_part.as_deref() {
                Some("") | None => None,
                Some(part) => Some(part.to_string()),
            },
            measurements: source
                .measurements
                .iter()
                .map(|m| EmitterMeasurement::from_bundle(m, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmitterColor {
    Color(CieColor),
    DominantWaveLength(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmitterMeasurement {
    physical: f32,
    luminous_intensity: f32,
    interpolation_to: InterpolationTo,
    points: Vec<MeasurementPoint>,
}

impl EmitterMeasurement {
    pub fn physical(&self) -> f32 {
        self.physical
    }

    pub fn luminous_intensity(&self) -> f32 {
        self.luminous_intensity
    }

    pub fn interpolation_to(&self) -> InterpolationTo {
        self.interpolation_to
    }

    pub fn points(&self) -> &[MeasurementPoint] {
        &self.points
    }
}

impl bundle::FromBundle for EmitterMeasurement {
    type Source = bundle::EmitterMeasurement;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            physical: source.physical,
            luminous_intensity: source.luminous_intensity,
            interpolation_to: InterpolationTo::from_bundle(&source.interpolation_to, bundle),
            points: source
                .measurement_points
                .iter()
                .map(|mp| MeasurementPoint::from_bundle(mp, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
    name: Name,
    color: CieColor,
    measurements: Vec<FilterMeasurement>,
}

impl Filter {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn color(&self) -> CieColor {
        self.color
    }

    pub fn measurements(&self) -> &[FilterMeasurement] {
        &self.measurements
    }
}

impl bundle::FromBundle for Filter {
    type Source = bundle::Filter;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: Name::new(source.name.clone()),
            color: source
                .color
                .as_deref()
                .map(|c| CieColor::from_str(c).unwrap())
                .unwrap_or_default(),
            measurements: source
                .measurements
                .iter()
                .map(|m| FilterMeasurement::from_bundle(m, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterMeasurement {
    physical: f32,
    transmission: f32,
    interpolation_to: InterpolationTo,
    points: Vec<MeasurementPoint>,
}

impl FilterMeasurement {
    pub fn physical(&self) -> f32 {
        self.physical
    }

    pub fn transmission(&self) -> f32 {
        self.transmission
    }

    pub fn interpolation_to(&self) -> InterpolationTo {
        self.interpolation_to
    }

    pub fn points(&self) -> &[MeasurementPoint] {
        &self.points
    }
}

impl bundle::FromBundle for FilterMeasurement {
    type Source = bundle::FilterMeasurement;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            physical: source.physical,
            transmission: source.transmission,
            interpolation_to: InterpolationTo::from_bundle(&source.interpolation_to, bundle),
            points: source
                .measurement_points
                .iter()
                .map(|p| MeasurementPoint::from_bundle(p, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeasurementPoint {
    wave_length: f32,
    energy: f32,
}

impl MeasurementPoint {
    pub fn wave_length(&self) -> f32 {
        self.wave_length
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }
}

impl bundle::FromBundle for MeasurementPoint {
    type Source = bundle::MeasurementPoint;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self { wave_length: source.wave_length, energy: source.energy }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationTo {
    Linear,
    Step,
    Log,
}

impl bundle::FromBundle for InterpolationTo {
    type Source = bundle::InterpolationTo;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::InterpolationTo::Linear => Self::Linear,
            bundle::InterpolationTo::Step => Self::Step,
            bundle::InterpolationTo::Log => Self::Log,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorSpace {
    name: Option<Name>,
    mode: ColorSpaceMode,
}

impl ColorSpace {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn mode(&self) -> &ColorSpaceMode {
        &self.mode
    }
}

impl bundle::FromBundle for ColorSpace {
    type Source = bundle::ColorSpace;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self { name: parse_optional_name(source.name.as_deref()), mode: source.into() }
    }
}

// FIXME: Make red, green, blue and white_point values available with
// getters for SRgb, ProPhoto and Ansi too as seen in Table 21 of GDTF.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorSpaceMode {
    SRgb,
    ProPhoto,
    Ansi,
    Custom { red: CieColor, green: CieColor, blue: CieColor, white_point: CieColor },
}

impl From<&bundle::ColorSpace> for ColorSpaceMode {
    fn from(value: &bundle::ColorSpace) -> Self {
        match value.mode {
            bundle::ColorSpaceMode::Custom => Self::Custom {
                red: CieColor::from_str(value.red.as_deref().unwrap()).unwrap(),
                green: CieColor::from_str(value.green.as_deref().unwrap()).unwrap(),
                blue: CieColor::from_str(value.blue.as_deref().unwrap()).unwrap(),
                white_point: CieColor::from_str(value.white_point.as_deref().unwrap()).unwrap(),
            },
            bundle::ColorSpaceMode::SRgb => Self::SRgb,
            bundle::ColorSpaceMode::ProPhoto => Self::ProPhoto,
            bundle::ColorSpaceMode::Ansi => Self::Ansi,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gamut {
    name: Option<Name>,
    points: Vec<CieColor>,
}

impl Gamut {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn points(&self) -> &[CieColor] {
        &self.points
    }
}

impl bundle::FromBundle for Gamut {
    type Source = bundle::Gamut;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        let points = source.points.as_deref().map(util::parse_cie_color_array).unwrap_or_default();
        Self { name: parse_optional_name(source.name.as_deref()), points }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxProfile {
    name: Option<Name>,
    points: Vec<DmxPoint>,
}

impl DmxProfile {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn points(&self) -> &[DmxPoint] {
        &self.points
    }
}

impl bundle::FromBundle for DmxProfile {
    type Source = bundle::DmxProfile;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: parse_optional_name(source.name.as_deref()),
            points: source.points.iter().map(|p| DmxPoint::from_bundle(p, bundle)).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DmxPoint {
    dmx_percentage: f32,
    cfc: [f32; 4],
}

impl DmxPoint {
    pub fn dmx_percentage(&self) -> f32 {
        self.dmx_percentage
    }

    pub fn cfc(&self) -> [f32; 4] {
        self.cfc
    }
}

impl bundle::FromBundle for DmxPoint {
    type Source = bundle::Point;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            dmx_percentage: source.dmx_percentage.unwrap_or(0.0),
            cfc: [
                source.cfc_0.unwrap_or(0.0),
                source.cfc_1.unwrap_or(0.0),
                source.cfc_2.unwrap_or(0.0),
                source.cfc_3.unwrap_or(0.0),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CriGroup {
    color_temperature: f32,
    cris: Vec<Cri>,
}

impl CriGroup {
    pub fn color_temperature(&self) -> f32 {
        self.color_temperature
    }

    pub fn cris(&self) -> &[Cri] {
        &self.cris
    }
}

impl bundle::FromBundle for CriGroup {
    type Source = bundle::CriGroup;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            color_temperature: source.color_temperature.unwrap_or(6000.0),
            cris: source.cris.iter().map(|c| Cri::from_bundle(c, bundle)).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cri {
    ces: Ces,
    color_rendering_index: u8,
}

impl Cri {
    pub fn ces(&self) -> Ces {
        self.ces.clone()
    }

    pub fn color_rendering_index(&self) -> u8 {
        self.color_rendering_index
    }
}

impl bundle::FromBundle for Cri {
    type Source = bundle::Cri;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            ces: Ces::from_bundle(&source.ces, bundle),
            color_rendering_index: source.color_rendering_index.unwrap_or(100),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Properties {
    pub(in crate::mvr_gdtf::gdtf) operating_temperature: Option<OperatingTemperature>,
    pub(in crate::mvr_gdtf::gdtf) weight: Option<f32>,
    pub(in crate::mvr_gdtf::gdtf) leg_height: Option<f32>,
}

impl Properties {
    pub fn operating_temperature(&self) -> Option<&OperatingTemperature> {
        self.operating_temperature.as_ref()
    }

    pub fn weight(&self) -> Option<f32> {
        self.weight
    }

    pub fn leg_height(&self) -> Option<f32> {
        self.leg_height
    }
}

impl bundle::FromBundle for Properties {
    type Source = bundle::Properties;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let mut operating_temperature = None;
        let mut weight = None;
        let mut leg_height = None;

        for item in &source.content {
            match item {
                bundle::PropertiesContent::OperatingTemperature(ot) => {
                    operating_temperature = Some(OperatingTemperature::from_bundle(ot, bundle));
                }
                bundle::PropertiesContent::Weight(w) => {
                    weight = w.value;
                }
                bundle::PropertiesContent::LegHeight(lh) => {
                    leg_height = lh.value;
                }
                bundle::PropertiesContent::PowerConsumption(_) => {}
            }
        }

        Self { operating_temperature, weight, leg_height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperatingTemperature {
    low: f32,
    high: f32,
}

impl OperatingTemperature {
    pub fn low(&self) -> f32 {
        self.low
    }

    pub fn high(&self) -> f32 {
        self.high
    }
}

impl bundle::FromBundle for OperatingTemperature {
    type Source = bundle::OperatingTemperature;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self { low: source.low.unwrap_or(0.0), high: source.high.unwrap_or(40.0) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ces(pub u8);

impl Ces {
    pub fn new(value: u8) -> Option<Self> {
        if value >= 1 && value <= 99 { Some(Ces(value)) } else { None }
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

impl bundle::FromBundle for Ces {
    type Source = bundle::Ces;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        let n = match source {
            bundle::Ces::Ces01 => 1,
            bundle::Ces::Ces02 => 2,
            bundle::Ces::Ces03 => 3,
            bundle::Ces::Ces04 => 4,
            bundle::Ces::Ces05 => 5,
            bundle::Ces::Ces06 => 6,
            bundle::Ces::Ces07 => 7,
            bundle::Ces::Ces08 => 8,
            bundle::Ces::Ces09 => 9,
            bundle::Ces::Ces10 => 10,
            bundle::Ces::Ces11 => 11,
            bundle::Ces::Ces12 => 12,
            bundle::Ces::Ces13 => 13,
            bundle::Ces::Ces14 => 14,
            bundle::Ces::Ces15 => 15,
            bundle::Ces::Ces16 => 16,
            bundle::Ces::Ces17 => 17,
            bundle::Ces::Ces18 => 18,
            bundle::Ces::Ces19 => 19,
            bundle::Ces::Ces20 => 20,
            bundle::Ces::Ces21 => 21,
            bundle::Ces::Ces22 => 22,
            bundle::Ces::Ces23 => 23,
            bundle::Ces::Ces24 => 24,
            bundle::Ces::Ces25 => 25,
            bundle::Ces::Ces26 => 26,
            bundle::Ces::Ces27 => 27,
            bundle::Ces::Ces28 => 28,
            bundle::Ces::Ces29 => 29,
            bundle::Ces::Ces30 => 30,
            bundle::Ces::Ces31 => 31,
            bundle::Ces::Ces32 => 32,
            bundle::Ces::Ces33 => 33,
            bundle::Ces::Ces34 => 34,
            bundle::Ces::Ces35 => 35,
            bundle::Ces::Ces36 => 36,
            bundle::Ces::Ces37 => 37,
            bundle::Ces::Ces38 => 38,
            bundle::Ces::Ces39 => 39,
            bundle::Ces::Ces40 => 40,
            bundle::Ces::Ces41 => 41,
            bundle::Ces::Ces42 => 42,
            bundle::Ces::Ces43 => 43,
            bundle::Ces::Ces44 => 44,
            bundle::Ces::Ces45 => 45,
            bundle::Ces::Ces46 => 46,
            bundle::Ces::Ces47 => 47,
            bundle::Ces::Ces48 => 48,
            bundle::Ces::Ces49 => 49,
            bundle::Ces::Ces50 => 50,
            bundle::Ces::Ces51 => 51,
            bundle::Ces::Ces52 => 52,
            bundle::Ces::Ces53 => 53,
            bundle::Ces::Ces54 => 54,
            bundle::Ces::Ces55 => 55,
            bundle::Ces::Ces56 => 56,
            bundle::Ces::Ces57 => 57,
            bundle::Ces::Ces58 => 58,
            bundle::Ces::Ces59 => 59,
            bundle::Ces::Ces60 => 60,
            bundle::Ces::Ces61 => 61,
            bundle::Ces::Ces62 => 62,
            bundle::Ces::Ces63 => 63,
            bundle::Ces::Ces64 => 64,
            bundle::Ces::Ces65 => 65,
            bundle::Ces::Ces66 => 66,
            bundle::Ces::Ces67 => 67,
            bundle::Ces::Ces68 => 68,
            bundle::Ces::Ces69 => 69,
            bundle::Ces::Ces70 => 70,
            bundle::Ces::Ces71 => 71,
            bundle::Ces::Ces72 => 72,
            bundle::Ces::Ces73 => 73,
            bundle::Ces::Ces74 => 74,
            bundle::Ces::Ces75 => 75,
            bundle::Ces::Ces76 => 76,
            bundle::Ces::Ces77 => 77,
            bundle::Ces::Ces78 => 78,
            bundle::Ces::Ces79 => 79,
            bundle::Ces::Ces80 => 80,
            bundle::Ces::Ces81 => 81,
            bundle::Ces::Ces82 => 82,
            bundle::Ces::Ces83 => 83,
            bundle::Ces::Ces84 => 84,
            bundle::Ces::Ces85 => 85,
            bundle::Ces::Ces86 => 86,
            bundle::Ces::Ces87 => 87,
            bundle::Ces::Ces88 => 88,
            bundle::Ces::Ces89 => 89,
            bundle::Ces::Ces90 => 90,
            bundle::Ces::Ces91 => 91,
            bundle::Ces::Ces92 => 92,
            bundle::Ces::Ces93 => 93,
            bundle::Ces::Ces94 => 94,
            bundle::Ces::Ces95 => 95,
            bundle::Ces::Ces96 => 96,
            bundle::Ces::Ces97 => 97,
            bundle::Ces::Ces98 => 98,
            bundle::Ces::Ces99 => 99,
        };
        Ces(n)
    }
}

use zeevonk::{attr::Attribute, value::ClampedValue};

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Parameter {
    Dimmer(DimmerParameter),
    Position(PositionParameter),
    Gobo(GoboParameter),
    Color(ColorParameter),
    Beam(BeamParameter),
    Focus(FocusParameter),
    Control(ControlParameter),
    Shapers(ShapersParameter),
    Video(VideoParameter),
    Raw((Attribute, ClampedValue)),
}

impl Parameter {
    pub fn dimmer(value: impl Into<ClampedValue>) -> Self {
        Self::Dimmer(DimmerParameter::Dimmer(value.into()))
    }

    pub fn pan(value: impl Into<ClampedValue>) -> Self {
        Self::Position(PositionParameter::Pan(value.into()))
    }

    pub fn tilt(value: impl Into<ClampedValue>) -> Self {
        Self::Position(PositionParameter::Tilt(value.into()))
    }

    pub fn rgb(
        r: impl Into<ClampedValue>,
        g: impl Into<ClampedValue>,
        b: impl Into<ClampedValue>,
    ) -> Self {
        Self::Color(ColorParameter::Rgb { r: r.into(), g: g.into(), b: b.into() })
    }

    pub fn rgbw(
        r: impl Into<ClampedValue>,
        g: impl Into<ClampedValue>,
        b: impl Into<ClampedValue>,
        w: impl Into<ClampedValue>,
    ) -> Self {
        Self::Color(ColorParameter::Rgbw { r: r.into(), g: g.into(), b: b.into(), w: w.into() })
    }

    pub fn cmy(
        c: impl Into<ClampedValue>,
        m: impl Into<ClampedValue>,
        y: impl Into<ClampedValue>,
    ) -> Self {
        Self::Color(ColorParameter::Cmy { c: c.into(), m: m.into(), y: y.into() })
    }

    pub fn hsb(
        hue: impl Into<ClampedValue>,
        sat: impl Into<ClampedValue>,
        bright: impl Into<ClampedValue>,
    ) -> Self {
        Self::Color(ColorParameter::Hsb {
            hue: hue.into(),
            saturation: sat.into(),
            brightness: bright.into(),
        })
    }

    pub fn wheel(wheel_number: u8) -> ColorWheelBuilder {
        ColorWheelBuilder::new(wheel_number)
    }

    pub fn cto(value: impl Into<ClampedValue>) -> Self {
        Self::Color(ColorParameter::Cto(value.into()))
    }

    pub fn raw(attr: Attribute, value: impl Into<ClampedValue>) -> Self {
        Self::Raw((attr, value.into()))
    }

    pub fn to_attribute_values(&self) -> Vec<(Attribute, ClampedValue)> {
        match self {
            Parameter::Dimmer(p) => p.to_attributes(),
            Parameter::Position(p) => p.to_attributes(),
            Parameter::Color(p) => p.to_attributes(),
            Parameter::Gobo(_) => Vec::new(),
            Parameter::Beam(_) => Vec::new(),
            Parameter::Focus(_) => Vec::new(),
            Parameter::Control(_) => Vec::new(),
            Parameter::Shapers(_) => Vec::new(),
            Parameter::Video(_) => Vec::new(),
            Parameter::Raw(p) => vec![*p],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DimmerParameter {
    Dimmer(ClampedValue),
}

impl DimmerParameter {
    fn to_attributes(&self) -> Vec<(Attribute, ClampedValue)> {
        match self {
            DimmerParameter::Dimmer(v) => vec![(Attribute::Dimmer, *v)],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PositionParameter {
    Pan(ClampedValue),
    Tilt(ClampedValue),
}

impl PositionParameter {
    fn to_attributes(&self) -> Vec<(Attribute, ClampedValue)> {
        match self {
            PositionParameter::Pan(v) => vec![(Attribute::Pan, *v)],
            PositionParameter::Tilt(v) => vec![(Attribute::Tilt, *v)],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ColorParameter {
    Rgb {
        r: ClampedValue,
        g: ClampedValue,
        b: ClampedValue,
    },
    Rgbw {
        r: ClampedValue,
        g: ClampedValue,
        b: ClampedValue,
        w: ClampedValue,
    },
    Cmy {
        c: ClampedValue,
        m: ClampedValue,
        y: ClampedValue,
    },
    Hsb {
        hue: ClampedValue,
        saturation: ClampedValue,
        brightness: ClampedValue,
    },
    Wheel {
        wheel: u8,
        index: Option<ClampedValue>,
        spin: Option<ClampedValue>,
        random: Option<ClampedValue>,
        audio: Option<ClampedValue>,
    },
    ColorEffects {
        n: u8,
        value: ClampedValue,
    },
    ColorMacro {
        n: u8,
        value: ClampedValue,
        rate: Option<ClampedValue>,
    },
    Cto(ClampedValue),
    Ctc(ClampedValue),
    Ctb(ClampedValue),
    Tint(ClampedValue),
}

impl ColorParameter {
    fn to_attributes(&self) -> Vec<(Attribute, ClampedValue)> {
        match self {
            ColorParameter::Rgb { r, g, b } => {
                use palette::{FromColor, Hsv, Srgb};
                let rgb = Srgb::new(r.as_f32(), g.as_f32(), b.as_f32());

                let c = ClampedValue::from(1.0 - r.as_f32());
                let m = ClampedValue::from(1.0 - g.as_f32());
                let y = ClampedValue::from(1.0 - b.as_f32());

                let hsv: Hsv = Hsv::from_color(rgb);
                let hue = ClampedValue::from(hsv.hue.into_positive_degrees() / 360.0);
                let saturation = ClampedValue::from(hsv.saturation);
                let brightness = ClampedValue::from(hsv.value);

                vec![
                    (Attribute::ColorAddR, *r),
                    (Attribute::ColorAddG, *g),
                    (Attribute::ColorAddB, *b),
                    (Attribute::ColorSubC, c),
                    (Attribute::ColorSubM, m),
                    (Attribute::ColorSubY, y),
                    (Attribute::HsbHue, hue),
                    (Attribute::HsbSaturation, saturation),
                    (Attribute::HsbBrightness, brightness),
                ]
            }
            ColorParameter::Rgbw { r, g, b, w } => {
                use palette::{FromColor, Hsv, Srgb};
                let rgb = Srgb::new(r.as_f32(), g.as_f32(), b.as_f32());

                let c = ClampedValue::from(1.0 - r.as_f32());
                let m = ClampedValue::from(1.0 - g.as_f32());
                let y = ClampedValue::from(1.0 - b.as_f32());

                let hsv: Hsv = Hsv::from_color(rgb);
                let hue = ClampedValue::from(hsv.hue.into_positive_degrees() / 360.0);
                let saturation = ClampedValue::from(hsv.saturation);
                let brightness = ClampedValue::from(hsv.value);

                vec![
                    (Attribute::ColorAddR, *r),
                    (Attribute::ColorAddG, *g),
                    (Attribute::ColorAddB, *b),
                    (Attribute::ColorAddW, *w),
                    (Attribute::ColorSubC, c),
                    (Attribute::ColorSubM, m),
                    (Attribute::ColorSubY, y),
                    (Attribute::HsbHue, hue),
                    (Attribute::HsbSaturation, saturation),
                    (Attribute::HsbBrightness, brightness),
                ]
            }
            ColorParameter::Cmy { c, m, y } => {
                use palette::{FromColor, Hsv, Srgb};
                let r = ClampedValue::from(1.0 - c.as_f32());
                let g = ClampedValue::from(1.0 - m.as_f32());
                let b = ClampedValue::from(1.0 - y.as_f32());

                let rgb = Srgb::new(r.as_f32(), g.as_f32(), b.as_f32());
                let hsv: Hsv = Hsv::from_color(rgb);
                let hue = ClampedValue::from(hsv.hue.into_positive_degrees() / 360.0);
                let saturation = ClampedValue::from(hsv.saturation);
                let brightness = ClampedValue::from(hsv.value);

                vec![
                    (Attribute::ColorSubC, *c),
                    (Attribute::ColorSubM, *m),
                    (Attribute::ColorSubY, *y),
                    (Attribute::ColorAddR, r),
                    (Attribute::ColorAddG, g),
                    (Attribute::ColorAddB, b),
                    (Attribute::HsbHue, hue),
                    (Attribute::HsbSaturation, saturation),
                    (Attribute::HsbBrightness, brightness),
                ]
            }
            ColorParameter::Hsb { hue, saturation, brightness } => {
                use palette::{FromColor, Hsv, Srgb};
                let hsv = Hsv::new(
                    palette::RgbHue::from_degrees(hue.as_f32() * 360.0),
                    saturation.as_f32(),
                    brightness.as_f32(),
                );
                let rgb: Srgb = Srgb::from_color(hsv);

                let r = ClampedValue::from(rgb.red);
                let g = ClampedValue::from(rgb.green);
                let b = ClampedValue::from(rgb.blue);
                let c = ClampedValue::from(1.0 - rgb.red);
                let m = ClampedValue::from(1.0 - rgb.green);
                let y = ClampedValue::from(1.0 - rgb.blue);

                vec![
                    (Attribute::HsbHue, *hue),
                    (Attribute::HsbSaturation, *saturation),
                    (Attribute::HsbBrightness, *brightness),
                    (Attribute::ColorAddR, r),
                    (Attribute::ColorAddG, g),
                    (Attribute::ColorAddB, b),
                    (Attribute::ColorSubC, c),
                    (Attribute::ColorSubM, m),
                    (Attribute::ColorSubY, y),
                ]
            }
            ColorParameter::Wheel { wheel, index, spin, random, audio } => {
                let mut out = Vec::with_capacity(4);
                if let Some(idx) = index {
                    out.push((Attribute::Color(*wheel), *idx));
                }
                if let Some(spin) = spin {
                    out.push((Attribute::ColorWheelSpin(*wheel), *spin));
                }
                if let Some(random) = random {
                    out.push((Attribute::ColorWheelRandom(*wheel), *random));
                }
                if let Some(audio) = audio {
                    out.push((Attribute::ColorWheelAudio(*wheel), *audio));
                }
                out
            }
            ColorParameter::ColorEffects { n, value } => {
                vec![(Attribute::ColorEffects(*n), *value)]
            }
            ColorParameter::ColorMacro { n, value, rate } => {
                let mut out = vec![(Attribute::ColorMacro(*n), *value)];
                if let Some(r) = rate {
                    out.push((Attribute::ColorMacroRate(*n), *r));
                }
                out
            }
            ColorParameter::Cto(v) => vec![(Attribute::Cto, *v)],
            ColorParameter::Ctc(v) => vec![(Attribute::Ctc, *v)],
            ColorParameter::Ctb(v) => vec![(Attribute::Ctb, *v)],
            ColorParameter::Tint(v) => vec![(Attribute::Tint, *v)],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GoboParameter;
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BeamParameter;
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FocusParameter;
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlParameter;
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShapersParameter;
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoParameter;

pub struct ColorWheelBuilder {
    wheel: u8,
    index: Option<ClampedValue>,
    spin: Option<ClampedValue>,
    random: Option<ClampedValue>,
    audio: Option<ClampedValue>,
}

impl ColorWheelBuilder {
    pub fn new(wheel: u8) -> Self {
        Self { wheel, index: None, spin: None, random: None, audio: None }
    }

    pub fn index(mut self, value: impl Into<ClampedValue>) -> Self {
        self.index = Some(value.into());
        self
    }

    pub fn spin(mut self, value: impl Into<ClampedValue>) -> Self {
        self.spin = Some(value.into());
        self
    }

    pub fn random(mut self, value: impl Into<ClampedValue>) -> Self {
        self.random = Some(value.into());
        self
    }

    pub fn build(self) -> Parameter {
        Parameter::Color(ColorParameter::Wheel {
            wheel: self.wheel,
            index: self.index,
            spin: self.spin,
            random: self.random,
            audio: self.audio,
        })
    }
}

impl From<ColorWheelBuilder> for Parameter {
    fn from(builder: ColorWheelBuilder) -> Self {
        builder.build()
    }
}

impl From<DimmerParameter> for Parameter {
    fn from(p: DimmerParameter) -> Self {
        Parameter::Dimmer(p)
    }
}
impl From<PositionParameter> for Parameter {
    fn from(p: PositionParameter) -> Self {
        Parameter::Position(p)
    }
}
impl From<ColorParameter> for Parameter {
    fn from(p: ColorParameter) -> Self {
        Parameter::Color(p)
    }
}
impl From<(Attribute, ClampedValue)> for Parameter {
    fn from(v: (Attribute, ClampedValue)) -> Self {
        Parameter::Raw(v)
    }
}

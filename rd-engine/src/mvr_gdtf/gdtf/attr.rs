use std::{
    collections::HashMap,
    fmt,
    str::{self, FromStr as _},
};

use crate::mvr_gdtf::{
    CieColor,
    gdtf::{Gdtf, Name, NodePath, bundle},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ActivationGroup {
    PanTilt,
    Xyz,
    RotXyz,
    ScaleXyz,
    ColorRgb,
    ColorHsb,
    ColorCie,
    ColorIndirect,
    Gobo(u32),
    GoboPos(u32),
    AnimationWheel(u32),
    AnimationWheelPos(u32),
    AnimationSystem(u32),
    AnimationSystemPos(u32),
    Prism,
    BeamShaper,
    Shaper,
    Custom(String),
}

impl ActivationGroup {
    pub fn name(&self) -> Name {
        Name::new(self.to_string())
    }
}

impl<'src> bundle::FromBundle<'src> for ActivationGroup {
    type Source = bundle::ActivationGroup;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        let name = Name::new(&source.name);
        ActivationGroup::from_str(name.as_str()).unwrap()
    }
}

impl fmt::Display for ActivationGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActivationGroup::PanTilt => write!(f, "PanTilt"),
            ActivationGroup::Xyz => write!(f, "XYZ"),
            ActivationGroup::RotXyz => write!(f, "Rot_XYZ"),
            ActivationGroup::ScaleXyz => write!(f, "Scale_XYZ"),
            ActivationGroup::ColorRgb => write!(f, "ColorRGB"),
            ActivationGroup::ColorHsb => write!(f, "ColorHSB"),
            ActivationGroup::ColorCie => write!(f, "ColorCIE"),
            ActivationGroup::ColorIndirect => write!(f, "ColorIndirect"),
            ActivationGroup::Gobo(n) => write!(f, "Gobo{n}"),
            ActivationGroup::GoboPos(n) => write!(f, "Gobo{n}Pos"),
            ActivationGroup::AnimationWheel(n) => write!(f, "AnimationWheel{n}"),
            ActivationGroup::AnimationWheelPos(n) => write!(f, "AnimationWheel{n}Pos"),
            ActivationGroup::AnimationSystem(n) => write!(f, "AnimationSystem{n}"),
            ActivationGroup::AnimationSystemPos(n) => write!(f, "AnimationSystem{n}Pos"),
            ActivationGroup::Prism => write!(f, "Prism"),
            ActivationGroup::BeamShaper => write!(f, "BeamShaper"),
            ActivationGroup::Shaper => write!(f, "Shaper"),
            ActivationGroup::Custom(name) => write!(f, "{name}"),
        }
    }
}

impl str::FromStr for ActivationGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let activation_group = match s {
            "PanTilt" => Self::PanTilt,
            "XYZ" => Self::Xyz,
            "Rot_XYZ" => Self::RotXyz,
            "Scale_XYZ" => Self::ScaleXyz,
            "ColorRGB" => Self::ColorRgb,
            "ColorHSB" => Self::ColorHsb,
            "ColorCIE" => Self::ColorCie,
            "ColorIndirect" => Self::ColorIndirect,
            "Prism" => Self::Prism,
            "BeamShaper" => Self::BeamShaper,
            "Shaper" => Self::Shaper,
            _ => {
                if let Some(n) = extract_attr_n::<u32>(s, "Gobo", Some("Pos")) {
                    Self::GoboPos(n)
                } else if let Some(n) = extract_attr_n::<u32>(s, "Gobo", None) {
                    Self::Gobo(n)
                } else if let Some(n) = extract_attr_n::<u32>(s, "AnimationWheel", Some("Pos")) {
                    Self::AnimationWheelPos(n)
                } else if let Some(n) = extract_attr_n::<u32>(s, "AnimationWheel", None) {
                    Self::AnimationWheel(n)
                } else if let Some(n) = extract_attr_n::<u32>(s, "AnimationSystem", Some("Pos")) {
                    Self::AnimationSystemPos(n)
                } else if let Some(n) = extract_attr_n::<u32>(s, "AnimationSystem", None) {
                    Self::AnimationSystem(n)
                } else {
                    Self::Custom(s.to_string())
                }
            }
        };

        Ok(activation_group)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    name: AttributeName,
    pretty: String,
    activation_group: Option<NodePath>,
    feature: Option<NodePath>,
    main_attribute: Option<NodePath>,
    physical_unit: PhysicalUnit,
    color: Option<CieColor>,

    sub_physical_units: Vec<SubPhysicalUnit>,
    sub_physical_units_by_type: HashMap<SubPhysicalUnitType, usize>,
}

impl Attribute {
    pub fn name(&self) -> &AttributeName {
        &self.name
    }

    pub fn pretty_name(&self) -> &str {
        &self.pretty
    }

    pub fn activation_group_node(&self) -> Option<&NodePath> {
        self.activation_group.as_ref()
    }

    pub fn activation_group<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a ActivationGroup> {
        let activation_group = self.activation_group_node()?.parts().get(0)?;
        gdtf.activation_group(activation_group)
    }

    pub fn feature_node(&self) -> Option<&NodePath> {
        self.feature.as_ref()
    }

    pub fn feature_group<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a FeatureGroup> {
        let feature_group = self.feature_node()?.parts().get(0)?;
        gdtf.feature_group(feature_group)
    }

    pub fn feature<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Feature> {
        let feature_group = self.feature_node()?.parts().get(0)?;
        let feature = self.feature_node()?.parts().get(1)?;
        gdtf.feature_group(feature_group)?.feature(feature)
    }

    pub fn main_attribute_node(&self) -> Option<&NodePath> {
        self.main_attribute.as_ref()
    }

    pub fn main_attribute<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Attribute> {
        let main_attribute = self.main_attribute_node()?.parts().get(0)?;
        gdtf.attribute(&AttributeName::from_str(main_attribute.as_str()).unwrap())
    }

    pub fn physical_unit(&self) -> PhysicalUnit {
        self.physical_unit
    }

    pub fn color(&self) -> Option<CieColor> {
        self.color
    }

    pub fn sub_physical_units(&self) -> &[SubPhysicalUnit] {
        &self.sub_physical_units
    }

    pub fn sub_physical_unit(&self, r#type: &SubPhysicalUnitType) -> Option<&SubPhysicalUnit> {
        let ix = self.sub_physical_units_by_type.get(r#type)?;
        Some(&self.sub_physical_units[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for Attribute {
    type Source = bundle::Attribute;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let sub_physical_units: Vec<SubPhysicalUnit> = source
            .sub_physical_units
            .iter()
            .map(|spu| SubPhysicalUnit::from_bundle(spu, bundle))
            .collect();
        let sub_physical_units_by_type = sub_physical_units
            .iter()
            .enumerate()
            .map(|(ix, spu)| (spu.r#type().clone(), ix))
            .collect();

        Self {
            name: AttributeName::from_str(&source.name).unwrap(),
            pretty: source.pretty.to_owned(),
            activation_group: source
                .activation_group
                .as_deref()
                .map(|ag| NodePath::from_str(ag).unwrap()),
            feature: source.feature.as_ref().map(|s| NodePath::from_str(s).unwrap()),
            main_attribute: source
                .main_attribute
                .as_deref()
                .map(|ma| NodePath::from_str(ma).unwrap()),
            physical_unit: PhysicalUnit::from_bundle(&source.physical_unit),
            color: source.color.as_deref().map(|c| {
                CieColor::from_str(c)
                    .inspect_err(|_| eprintln!("Failed to parse color: {c}. Defaulting to white."))
                    .unwrap_or_default()
            }),
            sub_physical_units,
            sub_physical_units_by_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeName {
    /// Controls the intensity of a fixture.
    Dimmer,

    /// Controls the fixture’s sideward movement (horizontal axis).
    Pan,
    /// Controls the fixture’s upward and the downward movement (vertical axis).
    Tilt,
    /// Controls the speed of the fixture’s continuous pan movement (horizontal
    /// axis).
    PanRotate,
    /// Controls the speed of the fixture’s continuous tilt movement (vertical
    /// axis).
    TiltRotate,
    /// Selects the predefined position effects that are built into the fixture.
    PositionEffect,
    /// Controls the speed of the predefined position effects that are built
    /// into the fixture.
    PositionEffectRate,
    /// Snaps or smooth fades with timing in running predefined position
    /// effects.
    PositionEffectFade,
    /// Defines a fixture’s x-coordinate within an XYZ coordinate system.
    XyzX,
    /// Defines a fixture’s y-coordinate within an XYZ coordinate system.
    XyzY,
    /// Defines a fixture‘s z-coordinate within an XYZ coordinate system.
    XyzZ,
    /// Defines rotation around X axis.
    RotX,
    /// Defines rotation around Y axis.
    RotY,
    /// Defines rotation around Z axis.
    RotZ,
    /// Scaling on X axis.
    ScaleX,
    /// Scaling on Y axis.
    ScaleY,
    /// Scaling on Y axis.
    ScaleZ,
    /// Unified scaling on all axis.
    ScaleXYZ,

    /// The fixture’s gobo wheel (n). This is the main attribute of gobo wheel’s
    /// (n) wheel control. Selects gobos in gobo wheel (n). A different channel
    /// function sets the angle of the indexed position in the selected gobo or
    /// the angular speed of its continuous rotation.
    Gobo(u8),
    /// Selects gobos whose rotation is continuous in gobo wheel (n) and
    /// controls the angular speed of the gobo’s spin within the same channel
    /// function.
    GoboSelectSpin(u8),
    /// Selects gobos which shake in gobo wheel (n) and controls the frequency
    /// of the gobo’s shake within the same channel function.
    GoboSelectShake(u8),
    /// Selects gobos which run effects in gobo wheel (n).
    GoboSelectEffects(u8),
    /// Controls angle of indexed rotation of gobo wheel (n).
    GoboWheelIndex(u8),
    /// Controls the speed and direction of continuous rotation of gobo wheel
    /// (n).
    GoboWheelSpin(u8),
    /// Controls frequency of the shake of gobo wheel (n).
    GoboWheelShake(u8),
    /// Controls speed of gobo wheel´s (n) random gobo slot selection.
    GoboWheelRandom(u8),
    /// Controls audio-controlled functionality of gobo wheel (n).
    GoboWheelAudio(u8),
    /// Controls angle of indexed rotation of gobos in gobo wheel (n). This is
    /// the main attribute of gobo wheel’s (n) wheel slot control.
    GoboPos(u8),
    /// Controls the speed and direction of continuous rotation of gobos in gobo
    /// wheel (n).
    GoboPosRotate(u8),
    /// Controls frequency of the shake of gobos in gobo wheel (n).
    GoboPosShake(u8),
    /// This is the main attribute of the animation wheel’s (n) wheel control.
    /// Selects slots in the animation wheel. A different channel function sets
    /// the angle of the indexed position in the selected slot or the angular
    /// speed of its continuous rotation. Is used for animation effects with
    /// multiple slots.
    AnimationWheel(u8),
    /// Controls audio-controlled functionality of animation wheel (n).
    AnimationWheelAudio(u8),
    /// Selects predefined effects in animation wheel (n).
    AnimationWheelMacro(u8),
    /// Controls frequency of animation wheel (n) random slot selection.
    AnimationWheelRandom(u8),
    /// Selects slots which run effects in animation wheel (n).
    AnimationWheelSelectEffects(u8),
    /// Selects slots which shake in animation wheel and controls the frequency
    /// of the slots shake within the same channel function.
    AnimationWheelSelectShake(u8),
    /// Selects slots whose rotation is continuous in animation wheel and
    /// controls the angular speed of the slot spin within the same channel
    /// function
    AnimationWheelSelectSpin(u8),
    /// Controls angle of indexed rotation of slots in animation wheel. This is
    /// the main attribute of animation wheel (n) wheel slot control.
    AnimationWheelPos(u8),
    /// Controls the speed and direction of continuous rotation of slots in
    /// animation wheel (n).
    AnimationWheelPosRotate(u8),
    /// Controls frequency of the shake of slots in animation wheel (n).
    AnimationWheelPosShake(u8),
    /// This is the main attribute of the animation system insertion control.
    /// Controls the insertion of the fixture’s animation system in the light
    /// output. Is used for animation effects where a disk is inserted into the
    /// light output.
    AnimationSystem(u8),
    /// Sets frequency of animation system (n) insertion ramp.
    AnimationSystemRamp(u8),
    /// Sets frequency of animation system (n) insertion shake.
    AnimationSystemShake(u8),
    /// Controls audio-controlled functionality of animation system (n)
    /// insertion.
    AnimationSystemAudio(u8),
    /// Controls frequency of animation system (n) random insertion.
    AnimationSystemRandom(u8),
    /// This is the main attribute of the animation system spinning control.
    /// Controls angle of indexed rotation of animation system (n) disk.
    AnimationSystemPos(u8),
    /// Controls the speed and direction of continuous rotation of animation
    /// system (n) disk.
    AnimationSystemPosRotate(u8),
    /// Controls frequency of the shake of animation system (n) disk.
    AnimationSystemPosShake(u8),
    /// Controls random speed of animation system (n) disk.
    AnimationSystemPosRandom(u8),
    /// Controls audio-controlled functionality of animation system (n) disk.
    AnimationSystemPosAudio(u8),
    /// Selects predefined effects in animation system (n).
    AnimationSystemMacro(u8),
    /// Selects folder that contains media content.
    MediaFolder(u8),
    /// Selects file with media content.
    MediaContent(u8),
    /// Selects folder that contains 3D model content. For example 3D meshes for
    /// mapping.
    ModelFolder(u8),
    /// Selects file with 3D model content.
    ModelContent(u8),
    /// Defines media playback mode.
    PlayMode,
    /// Defines starting point of media content playback.
    PlayBegin,
    /// Defines end point of media content playback.
    PlayEnd,
    /// Adjusts playback speed of media content.
    PlaySpeed,

    /// Selects predefined color effects built into the fixture.
    ColorEffects(u8),
    /// The fixture’s color wheel (n). Selects colors in color wheel (n). This
    /// is the main attribute of color wheel’s (n) wheel control.
    Color(u8),
    /// Controls angle of indexed rotation of color wheel (n)
    ColorWheelIndex(u8),
    /// Controls the speed and direction of continuous rotation of color wheel
    /// (n).
    ColorWheelSpin(u8),
    /// Controls frequency of color wheel´s (n) random color slot selection.
    ColorWheelRandom(u8),
    /// Controls audio-controlled functionality of color wheel (n).
    ColorWheelAudio(u8),
    /// Controls the intensity of the fixture’s red emitters for direct additive
    /// color mixing.
    ColorAddR,
    /// Controls the intensity of the fixture’s green emitters for direct
    /// additive color mixing
    ColorAddG,
    /// Controls the intensity of the fixture’s blue emitters for direct
    /// additive color mixing.
    ColorAddB,
    /// Controls the intensity of the fixture’s cyan emitters for direct
    /// additive color mixing.
    ColorAddC,
    /// Controls the intensity of the fixture’s magenta emitters for direct
    /// additive color mixing.
    ColorAddM,
    /// Controls the intensity of the fixture’s yellow emitters for direct
    /// additive color mixing.
    ColorAddY,
    /// Controls the intensity of the fixture’s amber emitters for direct
    /// additive color mixing.
    ColorAddRY,
    /// Controls the intensity of the fixture’s lime emitters for direct
    /// additive color mixing.
    ColorAddGY,
    /// Controls the intensity of the fixture’s blue-green emitters for direct
    /// additive color mixing.
    ColorAddGC,
    /// Controls the intensity of the fixture’s light-blue emitters for direct
    /// additive color mixing.
    ColorAddBC,
    /// Controls the intensity of the fixture’s purple emitters for direct
    /// additive color mixing.
    ColorAddBM,
    /// Controls the intensity of the fixture’s pink emitters for direct
    /// additive color mixing.
    ColorAddRM,
    /// Controls the intensity of the fixture’s white emitters for direct
    /// additive color mixing.
    ColorAddW,
    /// Controls the intensity of the fixture’s warm white emitters for direct
    /// additive color mixing.
    ColorAddWW,
    /// Controls the intensity of the fixture’s cool white emitters for direct
    /// additive color mixing.
    ColorAddCW,
    /// Controls the intensity of the fixture’s UV emitters for direct additive
    /// color mixing.
    ColorAddUV,
    /// Controls the insertion of the fixture’s red filter flag for direct
    /// subtractive color mixing.
    ColorSubR,
    /// Controls the insertion of the fixture’s green filter flag for direct
    /// subtractive color mixing.
    ColorSubG,
    /// Controls the insertion of the fixture’s blue filter flag for direct
    /// subtractive color mixing.
    ColorSubB,
    /// Controls the insertion of the fixture’s cyan filter flag for direct
    /// subtractive color mixing.
    ColorSubC,
    /// Controls the insertion of the fixture’s magenta filter flag for direct
    /// subtractive color mixing.
    ColorSubM,
    /// Controls the insertion of the fixture’s yellow filter flag for direct
    /// subtractive color mixing.
    ColorSubY,
    /// Selects predefined colors that are programed in the fixture’s firmware.
    ColorMacro(u8),
    /// Controls the time between Color Macro steps.
    ColorMacroRate(u8),
    /// Controls the fixture’s “Correct to orange” wheel or mixing system.
    Cto,
    /// Controls the fixture’s “Correct to color” wheel or mixing system.
    Ctc,
    /// Controls the fixture’s “Correct to blue” wheel or mixing system.
    Ctb,
    /// Controls the fixture’s “Correct green to magenta” wheel or mixing
    /// system.
    Tint,
    /// Controls the fixture’s color attribute regarding the hue.
    HsbHue,
    /// Controls the fixture’s color attribute regarding the saturation.
    HsbSaturation,
    /// Controls the fixture’s color attribute regarding the brightness.
    HsbBrightness,
    /// Controls the fixture’s color attribute regarding the quality.
    HsbQuality,
    /// Controls the fixture’s CIE 1931 color attribute regarding the
    /// chromaticity x.
    CieX,
    /// Controls the fixture’s CIE 1931 color attribute regarding the
    /// chromaticity y.
    CieY,
    /// Controls the fixture’s CIE 1931 color attribute regarding the brightness
    /// (Y).
    CieBrightness,
    /// Controls the fixture’s red attribute for indirect RGB color mixing.
    ColorRgbRed,
    /// Controls the fixture’s green attribute for indirect RGB color mixing.
    ColorRgbGreen,
    /// Controls the fixture’s blue attribute for indirect RGB color mixing.
    ColorRgbBlue,
    /// Controls the fixture’s cyan attribute for indirect CMY color mixing.
    ColorRgbCyan,
    /// Controls the fixture’s magenta attribute for indirect CMY color mixing.
    ColorRgbMagenta,
    /// Controls the fixture’s yellow attribute for indirect CMY color mixing.
    ColorRgbYellow,
    /// Controls the fixture’s quality attribute for indirect color mixing.
    ColorRgbQuality,
    /// Adjusts color boost red of content.
    VideoBoostR,
    /// Adjusts color boost green of content.
    VideoBoostG,
    /// Adjusts color boost blue of content.
    VideoBoostB,
    /// Adjusts color hue shift of content.
    VideoHueShift,
    /// Adjusts saturation of content.
    VideoSaturation,
    /// Adjusts brightness of content.
    VideoBrightness,
    /// Adjusts contrast of content.
    VideoContrast,
    /// Adjusts red color for color keying.
    VideoKeyColorR,
    /// Adjusts green color for color keying.
    VideoKeyColorG,
    /// Adjusts blue color for color keying.
    VideoKeyColorB,
    /// Adjusts intensity of color keying.
    VideoKeyIntensity,
    /// Adjusts tolerance of color keying.
    VideoKeyTolerance,

    /// Controls the length of a strobe flash.
    StrobeDuration,
    /// Controls the time between strobe flashes.
    StrobeRate,
    /// Controls the frequency of strobe flashes.
    StrobeFrequency,
    /// Strobe mode shutter. Use this attribute together with StrobeFrequency to
    /// define the type of the shutter / strobe.
    StrobeModeShutter,
    /// Strobe mode strobe. Use this attribute together with StrobeFrequency to
    /// define the type of the shutter / strobe.
    StrobeModeStrobe,
    /// Strobe mode pulse. Use this attribute together with StrobeFrequency to
    /// define the type of the shutter / strobe.
    StrobeModePulse,
    /// Strobe mode opening pulse. Use this attribute together with
    /// StrobeFrequency to define the type of the shutter / strobe.
    StrobeModePulseOpen,
    /// Strobe mode closing pulse. Use this attribute together with
    /// StrobeFrequency to define the type of the shutter / strobe.
    StrobeModePulseClose,
    /// Strobe mode random strobe. Use this attribute together with
    /// StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeRandom,
    /// Strobe mode random pulse. Use this attribute together with
    /// StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeRandomPulse,
    /// Strobe mode random opening pulse. Use this attribute together with
    /// StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeRandomPulseOpen,
    /// Strobe mode random closing pulse. Use this attribute together with
    /// StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeRandomPulseClose,
    /// Strobe mode random shutter effect feature. Use this attribute together
    /// with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeEffect,
    /// Controls the fixture´s mechanical or electronical shutter feature.
    Shutter(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical
    /// strobe shutter feature.
    ShutterStrobe(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical pulse
    /// shutter feature.
    ShutterStrobePulse(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical
    /// closing pulse shutter feature. The pulse is described by a ramp
    /// function.
    ShutterStrobePulseClose(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical
    /// opening pulse shutter feature. The pulse is described by a ramp
    /// function.
    ShutterStrobePulseOpen(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical
    /// random strobe shutter feature.
    ShutterStrobeRandom(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical
    /// random pulse shutter feature.
    ShutterStrobeRandomPulse(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical
    /// random closing pulse shutter feature. The pulse is described by a ramp
    /// function.
    ShutterStrobeRandomPulseClose(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical
    /// random opening pulse shutter feature. The pulse is described by a ramp
    /// function.
    ShutterStrobeRandomPulseOpen(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical
    /// shutter effect feature.
    ShutterStrobeEffect(u8),
    /// Controls the diameter of the fixture’s beam.
    Iris,
    /// Sets frequency of the iris’s strobe feature.
    IrisStrobe,
    /// Sets frequency of the iris’s random movement.
    IrisStrobeRandom,
    /// Sets frequency of iris’s closing pulse.
    IrisPulseClose,
    /// Sets frequency of iris’s opening pulse.
    IrisPulseOpen,
    /// Sets frequency of iris’s random closing pulse.
    IrisRandomPulseClose,
    /// Sets frequency of iris’s random opening pulse.
    IrisRandomPulseOpen,
    /// The ability to soften the fixture’s spot light with a frosted lens.
    Frost(u8),
    /// Sets frequency of frost’s opening pulse
    FrostPulseOpen(u8),
    /// Sets frequency of frost’s closing pulse.
    FrostPulseClose(u8),
    /// Sets frequency of frost’s ramp.
    FrostRamp(u8),
    /// The fixture's prism wheel (n). Selects prisms in prism wheel (n). A
    /// different channel function sets the angle of the indexed position in the
    /// selected prism or the angular speed of its continuous rotation. This is
    /// the main attribute of prism wheel's (n) wheel control.
    Prism(u8),
    /// Selects prisms whose rotation is continuous in prism wheel (n) and
    /// controls the angular speed of the prism's spin within the same channel
    /// function.
    PrismSelectSpin(u8),
    /// Macro functions of prism wheel (n).
    PrismMacro(u8),
    /// Controls angle of indexed rotation of prisms in prism wheel (n). This is
    /// the main attribute of prism wheel's 1 wheel slot control.
    PrismPos(u8),
    /// Controls the speed and direction of continuous rotation of prisms in
    /// prism wheel (n).
    PrismPosRotate(u8),

    /// Generically predefined macros and effects of a fixture.
    Effects(u8),
    /// Frequency of running effects.
    EffectsRate(u8),
    /// Snapping or smooth look of running effects.
    EffectsFade(u8),
    /// Controls parameter (m) of effect (n).
    EffectsAdjust(u8, u8),
    /// Controls angle of indexed rotation of slot/effect in effect wheel/macro
    /// (n). This is the main attribute of effect wheel/macro (n) slot/effect
    /// control.
    EffectsPos(u8),
    /// Controls speed and direction of slot/effect in effect wheel (n).
    EffectsPosRotate(u8),
    /// Sets offset between running effects and effects 2.
    EffectsSync,
    /// Activates fixture's beam shaper.
    BeamShaper,
    /// Predefined presets for fixture's beam shaper positions.
    BeamShaperMacro,
    /// Indexing of fixture's beam shaper.
    BeamShaperPos,
    /// Continuous rotation of fixture's beam shaper.
    BeamShaperPosRotate,
    /// Controls the spread of the fixture's beam/spot.
    Zoom,
    /// Selects spot mode of zoom.
    ZoomModeSpot,
    /// Selects beam mode of zoom.
    ZoomModeBeam,
    /// Controls the image size within the defined projection. Used on digital
    /// projection based devices.
    DigitalZoom,
    /// Controls the sharpness of the fixture's spot light. Can blur or sharpen
    /// the edge of the spot.
    Focus(u8),
    /// Autofocuses functionality using presets.
    FocusAdjust(u8),
    /// Autofocuses functionality using distance.
    FocusDistance(u8),

    /// Controls the channel of a fixture.
    Control(u8),
    /// Selects different modes of intensity.
    DimmerMode,
    /// Selects different dimmer curves of the fixture.
    DimmerCurve,
    /// Close the light output under certain conditions (movement correction,
    /// gobo movement, etc.).
    BlackoutMode,
    /// Controls LED frequency.
    LedFrequency,
    /// Changes zones of LEDs.
    LedZoneMode,
    /// Controls behavior of LED pixels.
    PixelMode,
    /// Selects fixture's pan mode. Selects between a limited pan range (e.g.
    /// -270 to 270) or a continuous pan range.
    PanMode,
    /// Selects fixture's pan mode. Selects between a limited tilt range (e.g.
    /// -130 to 130) or a continuous tilt range.
    TiltMode,
    /// Selects fixture's pan/tilt mode. Selects between a limited pan/tilt
    /// range or a continuous pan/tilt range.
    PanTiltMode,
    /// Selects the fixture's position mode.
    PositionModes,
    /// Changes control between selecting, indexing, and rotating the gobos of
    /// gobo wheel (n).
    GoboWheelMode(u8),
    /// Defines whether the gobo wheel takes the shortest distance between two
    /// positions.
    GoboWheelShortcutMode,
    /// Changes control between selecting, indexing, and rotating the slots of
    /// animation wheel (n).
    AnimationWheelMode(u8),
    /// Defines whether the animation wheel takes the shortest distance between
    /// two positions.
    AnimationWheelShortcutMode,
    /// Changes control between selecting, continuous selection, half selection,
    /// random selection, color spinning, etc. in colors of color wheel (n).
    ColorMode(u8),
    /// Defines whether the color wheel takes the shortest distance between two
    /// colors.
    ColorWheelShortcutMode,
    /// Controls how Cyan is used within the fixture's cyan CMY-mixing feature.
    CyanMode,
    /// Controls how Cyan is used within the fixture's magenta CMY-mixing.
    MagentaMode,
    /// Controls how Cyan is used within the fixture's yellow CMY-mixing
    /// feature.
    YellowMode,
    /// Changes control between selecting continuous selection, half selection,
    /// random selection, color spinning, etc. in color mixing.
    ColorMixMode,
    /// Selects chromatic behavior of the device.
    ChromaticMode,
    /// Sets calibration mode (for example on/off).
    ColorCalibrationMode,
    /// Controls consistent behavior of color.
    ColorConsistency,
    /// Controls special color related functions.
    ColorControl,
    /// Controls color model (CMY/RGB/HSV…).
    ColorModelMode,
    /// Resets settings of color control channel.
    ColorSettingsReset,
    /// Controls behavior of color uniformity.
    ColorUniformity,
    /// Controls CRI settings of output.
    CriMode,
    /// Custom color related functions (save, recall..).
    CustomColor,
    /// Settings for UV stability color behavior.
    UvStability,
    /// Settings for Wavelength correction of colors.
    WavelengthCorrection,
    /// Controls if White LED is proportionally added to RGB.
    WhiteCount,
    /// Changes strobe style - strobe, pulse, random strobe, etc. - of the
    /// shutter attribute.
    StrobeMode,
    /// Changes modes of the fixture´s zoom.
    ZoomMode,
    /// Changes modes of the fixture's focus - manual or auto- focus.
    FocusMode,
    /// Changes modes of the fixture's iris - linear, strobe, pulse.
    IrisMode,
    /// Controls fan (n) mode.
    FanMode(u8),
    /// Selects follow spot control mode.
    FollowSpotMode,
    /// Changes mode to control either index or rotation of the beam effects.
    BeamEffectIndexRotateMode,
    /// Movement speed of the fixture's intensity.
    IntensityMSpeed,
    /// Movement speed of the fixture's pan/tilt.
    PositionMSpeed,
    /// Movement speed of the fixture's ColorMix presets.
    ColorMixMSpeed,
    /// Movement speed of the fixture's color wheel.
    ColorWheelSelectMSpeed,
    /// Movement speed of the fixture's gobo wheel (n).
    GoboWheelMSpeed(u8),
    /// Movement speed of the fixture's iris.
    IrisMSpeed,
    /// Movement speed of the fixture's prism wheel (n).
    PrismMSpeed(u8),
    /// Movement speed of the fixture's focus.
    FocusMSpeed,
    /// Movement speed of the fixture's frost (n).
    FrostMSpeed(u8),
    /// Movement speed of the fixture's zoom.
    ZoomMSpeed,
    /// Movement speed of the fixture's shapers.
    FrameMSpeed,
    /// General speed of fixture's features.
    GlobalMSpeed,
    /// Movement speed of the fixture's frost.
    ReflectorAdjust,
    /// Generally resets the entire fixture.
    FixtureGlobalReset,
    /// Resets the fixture's dimmer.
    DimmerReset,
    /// Resets the fixture's shutter.
    ShutterReset,
    /// Resets the fixture's beam features.
    BeamReset,
    /// Resets the fixture's color mixing system.
    ColorMixReset,
    /// Resets the fixture's color wheel.
    ColorWheelReset,
    /// Resets the fixture's focus.
    FocusReset,
    /// Resets the fixture's shapers.
    FrameReset,
    /// Resets the fixture's gobo wheel.
    GoboWheelReset,
    /// Resets the fixture's intensity.
    IntensityReset,
    /// Resets the fixture's iris.
    IrisReset,
    /// Resets the fixture's pan/tilt.
    PositionReset,
    /// Resets the fixture's pan.
    PanReset,
    /// Resets the fixture's tilt.
    TiltReset,
    /// Resets the fixture's zoom.
    ZoomReset,
    /// Resets the fixture's CTB.
    CtbReset,
    /// Resets the fixture's CTO.
    CtoReset,
    /// Resets the fixture's CTC.
    CtcReset,
    /// Resets the fixture's animation system features.
    AnimationSystemReset,
    /// Resets the fixture's calibration.
    FixtureCalibrationReset,
    /// Generally controls features of the fixture.
    Function,
    /// Controls the fixture's lamp on/lamp off feature.
    LampControl,
    /// Adjusts intensity of display.
    DisplayIntensity,
    /// Selects DMX Input.
    DmxInput,
    /// Ranges without a functionality.
    NoFeature,
    /// Fog or hazer's blower feature.
    Blower(u8),
    /// Fog or hazer's Fan feature.
    Fan(u8),
    /// Fog or hazer's Fog feature.
    Fog(u8),
    /// Fog or hazer's Haze feature.
    Haze(u8),
    /// Controls the energy consumption of the lamp.
    LampPowerMode,
    /// Controls a fixture or device fan.
    Fans,

    /// 1 of 2 shutters that shape the top/right/bottom/left of the beam.
    BladeA(u8),
    /// 2 of 2 shutters that shape the top/right/bottom/left of the beam.
    BladeB(u8),
    /// Rotates position of blade(n).
    BladeRot(u8),
    /// Rotates position of blade assembly.
    ShaperRot,
    /// Predefined presets for shaper positions.
    ShaperMacros,
    /// Speed of predefined effects on shapers.
    ShaperMacrosSpeed,
    /// 1 of 2 soft edge blades that shape the top/right/bottom/left of the
    /// beam.
    BladeSoftA(u8),
    /// 2 of 2 soft edge blades that shape the top/right/bottom/left of the
    /// beam.
    BladeSoftB(u8),
    /// 1 of 2 corners that shape the top/right/bottom/left of the beam.
    KeyStoneA(u8),
    /// 2 of 2 corners that shape the top/right/bottom/left of the beam.
    KeyStoneB(u8),

    /// Controls video features.
    Video,
    /// Selects dedicated effects which are used for media.
    VideoEffectType(u8),
    /// Controls parameter (m) of VideoEffect(n)Type.
    VideoEffectParameter(u8, u8),
    /// Selects the video camera(n).
    VideoCamera(u8),
    /// Adjusts sound volume.
    VideoSoundVolume(u8),
    /// Defines mode of video blending.
    VideoBlendMode,
    /// Defines media input source e.g. a camera input.
    InputSource,
    /// Defines field of view.
    FieldOfView,

    /// Any other non-standard attribute.
    Custom(String),
}

impl fmt::Display for AttributeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dimmer => write!(f, "Dimmer"),

            Self::Pan => write!(f, "Pan"),
            Self::Tilt => write!(f, "Tilt"),
            Self::PanRotate => write!(f, "PanRotate"),
            Self::TiltRotate => write!(f, "TiltRotate"),
            Self::PositionEffect => write!(f, "PositionEffect"),
            Self::PositionEffectRate => write!(f, "PositionEffectRate"),
            Self::PositionEffectFade => write!(f, "PositionEffectFade"),
            Self::XyzX => write!(f, "XYZ_X"),
            Self::XyzY => write!(f, "XYZ_Y"),
            Self::XyzZ => write!(f, "XYZ_Z"),
            Self::RotX => write!(f, "Rot_X"),
            Self::RotY => write!(f, "Rot_Y"),
            Self::RotZ => write!(f, "Rot_Z"),
            Self::ScaleX => write!(f, "Scale_X"),
            Self::ScaleY => write!(f, "Scale_Y"),
            Self::ScaleZ => write!(f, "Scale_Z"),
            Self::ScaleXYZ => write!(f, "Scale_XYZ"),

            Self::Gobo(n) => write!(f, "Gobo{n}"),
            Self::GoboSelectSpin(n) => write!(f, "Gobo{n}SelectSpin"),
            Self::GoboSelectShake(n) => write!(f, "Gobo{n}SelectShake"),
            Self::GoboSelectEffects(n) => write!(f, "Gobo{n}SelectEffects"),
            Self::GoboWheelIndex(n) => write!(f, "Gobo{n}WheelIndex"),
            Self::GoboWheelSpin(n) => write!(f, "Gobo{n}WheelSpin"),
            Self::GoboWheelShake(n) => write!(f, "Gobo{n}WheelShake"),
            Self::GoboWheelRandom(n) => write!(f, "Gobo{n}WheelRandom"),
            Self::GoboWheelAudio(n) => write!(f, "Gobo{n}WheelAudio"),
            Self::GoboPos(n) => write!(f, "Gobo{n}Pos"),
            Self::GoboPosRotate(n) => write!(f, "Gobo{n}PosRotate"),
            Self::GoboPosShake(n) => write!(f, "Gobo{n}PosShake"),
            Self::AnimationWheel(n) => write!(f, "AnimationWheel{n}"),
            Self::AnimationWheelAudio(n) => write!(f, "AnimationWheel{n}Audio"),
            Self::AnimationWheelMacro(n) => write!(f, "AnimationWheel{n}Macro"),
            Self::AnimationWheelRandom(n) => write!(f, "AnimationWheel{n}Random"),
            Self::AnimationWheelSelectEffects(n) => write!(f, "AnimationWheel{n}SelectEffects"),
            Self::AnimationWheelSelectShake(n) => write!(f, "AnimationWheel{n}SelectShake"),
            Self::AnimationWheelSelectSpin(n) => write!(f, "AnimationWheel{n}SelectSpin"),
            Self::AnimationWheelPos(n) => write!(f, "AnimationWheel{n}Pos"),
            Self::AnimationWheelPosRotate(n) => write!(f, "AnimationWheel{n}PosRotate"),
            Self::AnimationWheelPosShake(n) => write!(f, "AnimationWheel{n}PosShake"),
            Self::AnimationSystem(n) => write!(f, "AnimationSystem{n}"),
            Self::AnimationSystemRamp(n) => write!(f, "AnimationSystem{n}Ramp"),
            Self::AnimationSystemShake(n) => write!(f, "AnimationSystem{n}Shake"),
            Self::AnimationSystemAudio(n) => write!(f, "AnimationSystem{n}Audio"),
            Self::AnimationSystemRandom(n) => write!(f, "AnimationSystem{n}Random"),
            Self::AnimationSystemPos(n) => write!(f, "AnimationSystem{n}Pos"),
            Self::AnimationSystemPosRotate(n) => write!(f, "AnimationSystem{n}PosRotate"),
            Self::AnimationSystemPosShake(n) => write!(f, "AnimationSystem{n}PosShake"),
            Self::AnimationSystemPosRandom(n) => write!(f, "AnimationSystem{n}PosRandom"),
            Self::AnimationSystemPosAudio(n) => write!(f, "AnimationSystem{n}PosAudio"),
            Self::AnimationSystemMacro(n) => write!(f, "AnimationSystem{n}Macro"),
            Self::MediaFolder(n) => write!(f, "MediaFolder{n}"),
            Self::MediaContent(n) => write!(f, "MediaContent{n}"),
            Self::ModelFolder(n) => write!(f, "ModelFolder{n}"),
            Self::ModelContent(n) => write!(f, "ModelContent{n}"),
            Self::PlayMode => write!(f, "PlayMode"),
            Self::PlayBegin => write!(f, "PlayBegin"),
            Self::PlayEnd => write!(f, "PlayEnd"),
            Self::PlaySpeed => write!(f, "PlaySpeed"),

            Self::ColorEffects(n) => write!(f, "ColorEffects{n}"),
            Self::Color(n) => write!(f, "Color{n}"),
            Self::ColorWheelIndex(n) => write!(f, "ColorWheel{n}Index"),
            Self::ColorWheelSpin(n) => write!(f, "ColorWheel{n}Spin"),
            Self::ColorWheelRandom(n) => write!(f, "ColorWheel{n}Random"),
            Self::ColorWheelAudio(n) => write!(f, "ColorWheel{n}Audio"),
            Self::ColorAddR => write!(f, "ColorAdd_R"),
            Self::ColorAddG => write!(f, "ColorAdd_G"),
            Self::ColorAddB => write!(f, "ColorAdd_B"),
            Self::ColorAddC => write!(f, "ColorAdd_C"),
            Self::ColorAddM => write!(f, "ColorAdd_M"),
            Self::ColorAddY => write!(f, "ColorAdd_Y"),
            Self::ColorAddRY => write!(f, "ColorAdd_RY"),
            Self::ColorAddGY => write!(f, "ColorAdd_GY"),
            Self::ColorAddGC => write!(f, "ColorAdd_GC"),
            Self::ColorAddBC => write!(f, "ColorAdd_BC"),
            Self::ColorAddBM => write!(f, "ColorAdd_BM"),
            Self::ColorAddRM => write!(f, "ColorAdd_RM"),
            Self::ColorAddW => write!(f, "ColorAdd_W"),
            Self::ColorAddWW => write!(f, "ColorAdd_WW"),
            Self::ColorAddCW => write!(f, "ColorAdd_CW"),
            Self::ColorAddUV => write!(f, "ColorAdd_UV"),
            Self::ColorSubR => write!(f, "ColorSub_R"),
            Self::ColorSubG => write!(f, "ColorSub_G"),
            Self::ColorSubB => write!(f, "ColorSub_B"),
            Self::ColorSubC => write!(f, "ColorSub_C"),
            Self::ColorSubM => write!(f, "ColorSub_M"),
            Self::ColorSubY => write!(f, "ColorSub_Y"),
            Self::ColorMacro(n) => write!(f, "ColorMacro{n}"),
            Self::ColorMacroRate(n) => write!(f, "ColorMacroRate{n}"),
            Self::Cto => write!(f, "CTO"),
            Self::Ctc => write!(f, "CTC"),
            Self::Ctb => write!(f, "CTB"),
            Self::Tint => write!(f, "Tint"),
            Self::HsbHue => write!(f, "HSB_Hue"),
            Self::HsbSaturation => write!(f, "HSB_Saturation"),
            Self::HsbBrightness => write!(f, "HSB_Brightness"),
            Self::HsbQuality => write!(f, "HSB_Quality"),
            Self::CieX => write!(f, "CIE_X"),
            Self::CieY => write!(f, "CIE_Y"),
            Self::CieBrightness => write!(f, "CIE_Brightness"),
            Self::ColorRgbRed => write!(f, "ColorRGB_Red"),
            Self::ColorRgbGreen => write!(f, "ColorRGB_Green"),
            Self::ColorRgbBlue => write!(f, "ColorRGB_Blue"),
            Self::ColorRgbCyan => write!(f, "ColorRGB_Cyan"),
            Self::ColorRgbMagenta => write!(f, "ColorRGB_Magenta"),
            Self::ColorRgbYellow => write!(f, "ColorRGB_Yellow"),
            Self::ColorRgbQuality => write!(f, "ColorRGB_Quality"),
            Self::VideoBoostR => write!(f, "VideoBoost_R"),
            Self::VideoBoostG => write!(f, "VideoBoost_G"),
            Self::VideoBoostB => write!(f, "VideoBoost_B"),
            Self::VideoHueShift => write!(f, "VideoHueShift"),
            Self::VideoSaturation => write!(f, "VideoSaturation"),
            Self::VideoBrightness => write!(f, "VideoBrightness"),
            Self::VideoContrast => write!(f, "VideoContrast"),
            Self::VideoKeyColorR => write!(f, "VideoKeyColor_R"),
            Self::VideoKeyColorG => write!(f, "VideoKeyColor_G"),
            Self::VideoKeyColorB => write!(f, "VideoKeyColor_B"),
            Self::VideoKeyIntensity => write!(f, "VideoKeyIntensity"),
            Self::VideoKeyTolerance => write!(f, "VideoKeyTolerance"),

            Self::StrobeDuration => write!(f, "StrobeDuration"),
            Self::StrobeRate => write!(f, "StrobeRate"),
            Self::StrobeFrequency => write!(f, "StrobeFrequency"),
            Self::StrobeModeShutter => write!(f, "StrobeModeShutter"),
            Self::StrobeModeStrobe => write!(f, "StrobeModeStrobe"),
            Self::StrobeModePulse => write!(f, "StrobeModePulse"),
            Self::StrobeModePulseOpen => write!(f, "StrobeModePulseOpen"),
            Self::StrobeModePulseClose => write!(f, "StrobeModePulseClose"),
            Self::StrobeModeRandom => write!(f, "StrobeModeRandom"),
            Self::StrobeModeRandomPulse => write!(f, "StrobeModeRandomPulse"),
            Self::StrobeModeRandomPulseOpen => write!(f, "StrobeModeRandomPulseOpen"),
            Self::StrobeModeRandomPulseClose => write!(f, "StrobeModeRandomPulseClose"),
            Self::StrobeModeEffect => write!(f, "StrobeModeEffect"),
            Self::Shutter(n) => write!(f, "Shutter{n}"),
            Self::ShutterStrobe(n) => write!(f, "Shutter{n}Strobe"),
            Self::ShutterStrobePulse(n) => write!(f, "Shutter{n}StrobePulse"),
            Self::ShutterStrobePulseClose(n) => write!(f, "Shutter{n}StrobePulseClose"),
            Self::ShutterStrobePulseOpen(n) => write!(f, "Shutter{n}StrobePulseOpen"),
            Self::ShutterStrobeRandom(n) => write!(f, "Shutter{n}StrobeRandom"),
            Self::ShutterStrobeRandomPulse(n) => write!(f, "Shutter{n}StrobeRandomPulse"),
            Self::ShutterStrobeRandomPulseClose(n) => write!(f, "Shutter{n}StrobeRandomPulseClose"),
            Self::ShutterStrobeRandomPulseOpen(n) => write!(f, "Shutter{n}StrobeRandomPulseOpen"),
            Self::ShutterStrobeEffect(n) => write!(f, "Shutter{n}StrobeEffect"),
            Self::Iris => write!(f, "Iris"),
            Self::IrisStrobe => write!(f, "IrisStrobe"),
            Self::IrisStrobeRandom => write!(f, "IrisStrobeRandom"),
            Self::IrisPulseClose => write!(f, "IrisPulseClose"),
            Self::IrisPulseOpen => write!(f, "IrisPulseOpen"),
            Self::IrisRandomPulseClose => write!(f, "IrisRandomPulseClose"),
            Self::IrisRandomPulseOpen => write!(f, "IrisRandomPulseOpen"),
            Self::Frost(n) => write!(f, "Frost{n}"),
            Self::FrostPulseOpen(n) => write!(f, "Frost{n}PulseOpen"),
            Self::FrostPulseClose(n) => write!(f, "Frost{n}PulseClose"),
            Self::FrostRamp(n) => write!(f, "Frost{n}Ramp"),
            Self::Prism(n) => write!(f, "Prism{n}"),
            Self::PrismSelectSpin(n) => write!(f, "Prism{n}SelectSpin"),
            Self::PrismMacro(n) => write!(f, "Prism{n}Macro"),
            Self::PrismPos(n) => write!(f, "Prism{n}Pos"),
            Self::PrismPosRotate(n) => write!(f, "Prism{n}PosRotate"),

            Self::Effects(n) => write!(f, "Effects{n}"),
            Self::EffectsRate(n) => write!(f, "Effects{n}Rate"),
            Self::EffectsFade(n) => write!(f, "Effects{n}Fade"),
            Self::EffectsAdjust(n, m) => write!(f, "Effects{n}Adjust{m}"),
            Self::EffectsPos(n) => write!(f, "Effects{n}Pos"),
            Self::EffectsPosRotate(n) => write!(f, "Effects{n}PosRotate"),
            Self::EffectsSync => write!(f, "EffectsSync"),
            Self::BeamShaper => write!(f, "BeamShaper"),
            Self::BeamShaperMacro => write!(f, "BeamShaperMacro"),
            Self::BeamShaperPos => write!(f, "BeamShaperPos"),
            Self::BeamShaperPosRotate => write!(f, "BeamShaperPosRotate"),
            Self::Zoom => write!(f, "Zoom"),
            Self::ZoomModeSpot => write!(f, "ZoomModeSpot"),
            Self::ZoomModeBeam => write!(f, "ZoomModeBeam"),
            Self::DigitalZoom => write!(f, "DigitalZoom"),
            Self::Focus(n) => write!(f, "Focus{n}"),
            Self::FocusAdjust(n) => write!(f, "Focus{n}Adjust"),
            Self::FocusDistance(n) => write!(f, "Focus{n}Distance"),

            Self::Control(n) => write!(f, "Control{n}"),
            Self::DimmerMode => write!(f, "DimmerMode"),
            Self::DimmerCurve => write!(f, "DimmerCurve"),
            Self::BlackoutMode => write!(f, "BlackoutMode"),
            Self::LedFrequency => write!(f, "LedFrequency"),
            Self::LedZoneMode => write!(f, "LedZoneMode"),
            Self::PixelMode => write!(f, "PixelMode"),
            Self::PanMode => write!(f, "PanMode"),
            Self::TiltMode => write!(f, "TiltMode"),
            Self::PanTiltMode => write!(f, "PanTiltMode"),
            Self::PositionModes => write!(f, "PositionModes"),
            Self::GoboWheelMode(n) => write!(f, "Gobo{n}WheelMode"),
            Self::GoboWheelShortcutMode => write!(f, "GoboWheelShortcutMode"),
            Self::AnimationWheelMode(n) => write!(f, "Animation{n}WheelMode"),
            Self::AnimationWheelShortcutMode => write!(f, "AnimationWheelShortcutMode"),
            Self::ColorMode(n) => write!(f, "Color{n}Mode"),
            Self::ColorWheelShortcutMode => write!(f, "ColorWheelShortcutMode"),
            Self::CyanMode => write!(f, "CyanMode"),
            Self::MagentaMode => write!(f, "MagentaMode"),
            Self::YellowMode => write!(f, "YellowMode"),
            Self::ColorMixMode => write!(f, "ColorMixMode"),
            Self::ChromaticMode => write!(f, "ChromaticMode"),
            Self::ColorCalibrationMode => write!(f, "ColorCalibrationMode"),
            Self::ColorConsistency => write!(f, "ColorConsistency"),
            Self::ColorControl => write!(f, "ColorControl"),
            Self::ColorModelMode => write!(f, "ColorModelMode"),
            Self::ColorSettingsReset => write!(f, "ColorSettingsReset"),
            Self::ColorUniformity => write!(f, "ColorUniformity"),
            Self::CriMode => write!(f, "CriMode"),
            Self::CustomColor => write!(f, "CustomColor"),
            Self::UvStability => write!(f, "UvStability"),
            Self::WavelengthCorrection => write!(f, "WavelengthCorrection"),
            Self::WhiteCount => write!(f, "WhiteCount"),
            Self::StrobeMode => write!(f, "StrobeMode"),
            Self::ZoomMode => write!(f, "ZoomMode"),
            Self::FocusMode => write!(f, "FocusMode"),
            Self::IrisMode => write!(f, "IrisMode"),
            Self::FanMode(n) => write!(f, "Fan{n}Mode"),
            Self::FollowSpotMode => write!(f, "FollowSpotMode"),
            Self::BeamEffectIndexRotateMode => write!(f, "BeamEffectIndexRotateMode"),
            Self::IntensityMSpeed => write!(f, "IntensityMSpeed"),
            Self::PositionMSpeed => write!(f, "PositionMSpeed"),
            Self::ColorMixMSpeed => write!(f, "ColorMixMSpeed"),
            Self::ColorWheelSelectMSpeed => write!(f, "ColorWheelSelectMSpeed"),
            Self::GoboWheelMSpeed(n) => write!(f, "Gobo{n}WheelMSpeed"),
            Self::IrisMSpeed => write!(f, "IrisMSpeed"),
            Self::PrismMSpeed(n) => write!(f, "Prism{n}MSpeed"),
            Self::FocusMSpeed => write!(f, "FocusMSpeed"),
            Self::FrostMSpeed(n) => write!(f, "Frost{n}MSpeed"),
            Self::ZoomMSpeed => write!(f, "ZoomMSpeed"),
            Self::FrameMSpeed => write!(f, "FrameMSpeed"),
            Self::GlobalMSpeed => write!(f, "GlobalMSpeed"),
            Self::ReflectorAdjust => write!(f, "ReflectorAdjust"),
            Self::FixtureGlobalReset => write!(f, "FixtureGlobalReset"),
            Self::DimmerReset => write!(f, "DimmerReset"),
            Self::ShutterReset => write!(f, "ShutterReset"),
            Self::BeamReset => write!(f, "BeamReset"),
            Self::ColorMixReset => write!(f, "ColorMixReset"),
            Self::ColorWheelReset => write!(f, "ColorWheelReset"),
            Self::FocusReset => write!(f, "FocusReset"),
            Self::FrameReset => write!(f, "FrameReset"),
            Self::GoboWheelReset => write!(f, "GoboWheelReset"),
            Self::IntensityReset => write!(f, "IntensityReset"),
            Self::IrisReset => write!(f, "IrisReset"),
            Self::PositionReset => write!(f, "PositionReset"),
            Self::PanReset => write!(f, "PanReset"),
            Self::TiltReset => write!(f, "TiltReset"),
            Self::ZoomReset => write!(f, "ZoomReset"),
            Self::CtbReset => write!(f, "CTBReset"),
            Self::CtoReset => write!(f, "CTOReset"),
            Self::CtcReset => write!(f, "CTCReset"),
            Self::AnimationSystemReset => write!(f, "AnimationSystemReset"),
            Self::FixtureCalibrationReset => write!(f, "FixtureCalibrationReset"),
            Self::Function => write!(f, "Function"),
            Self::LampControl => write!(f, "LampControl"),
            Self::DisplayIntensity => write!(f, "DisplayIntensity"),
            Self::DmxInput => write!(f, "DMXInput"),
            Self::NoFeature => write!(f, "NoFeature"),
            Self::Blower(n) => write!(f, "Blower{n}"),
            Self::Fan(n) => write!(f, "Fan{n}"),
            Self::Fog(n) => write!(f, "Fog{n}"),
            Self::Haze(n) => write!(f, "Haze{n}"),
            Self::LampPowerMode => write!(f, "LampPowerMode"),
            Self::Fans => write!(f, "Fans"),

            Self::BladeA(n) => write!(f, "Blade{n}A"),
            Self::BladeB(n) => write!(f, "Blade{n}B"),
            Self::BladeRot(n) => write!(f, "Blade{n}Rot"),
            Self::ShaperRot => write!(f, "ShaperRot"),
            Self::ShaperMacros => write!(f, "ShaperMacros"),
            Self::ShaperMacrosSpeed => write!(f, "ShaperMacrosSpeed"),
            Self::BladeSoftA(n) => write!(f, "BladeSoft{n}A"),
            Self::BladeSoftB(n) => write!(f, "BladeSoft{n}B"),
            Self::KeyStoneA(n) => write!(f, "KeyStone{n}A"),
            Self::KeyStoneB(n) => write!(f, "KeyStone{n}B"),

            Self::Video => write!(f, "Video"),
            Self::VideoEffectType(n) => write!(f, "VideoEffect{n}Type"),
            Self::VideoEffectParameter(n, m) => write!(f, "VideoEffect{n}Parameter{m}"),
            Self::VideoCamera(n) => write!(f, "VideoCamera{n}"),
            Self::VideoSoundVolume(n) => write!(f, "VideoSoundVolume{n}"),
            Self::VideoBlendMode => write!(f, "VideoBlendMode"),
            Self::InputSource => write!(f, "InputSource"),
            Self::FieldOfView => write!(f, "FieldOfView"),

            Self::Custom(name) => write!(f, "{name}"),
        }
    }
}

impl str::FromStr for AttributeName {
    type Err = ();

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let attribute = match s {
            "Dimmer" => Self::Dimmer,

            "Pan" => Self::Pan,
            "Tilt" => Self::Tilt,
            "PanRotate" => Self::PanRotate,
            "TiltRotate" => Self::TiltRotate,
            "PositionEffect" => Self::PositionEffect,
            "PositionEffectRate" => Self::PositionEffectRate,
            "PositionEffectFade" => Self::PositionEffectFade,
            "XYZ_X" => Self::XyzX,
            "XYZ_Y" => Self::XyzY,
            "XYZ_Z" => Self::XyzZ,
            "Rot_X" => Self::RotX,
            "Rot_Y" => Self::RotY,
            "Rot_Z" => Self::RotZ,
            "Scale_X" => Self::ScaleX,
            "Scale_Y" => Self::ScaleY,
            "Scale_Z" => Self::ScaleZ,
            "Scale_XYZ" => Self::ScaleXYZ,

            "PlayMode" => Self::PlayMode,
            "PlayBegin" => Self::PlayBegin,
            "PlayEnd" => Self::PlayEnd,
            "PlaySpeed" => Self::PlaySpeed,

            "ColorAdd_R" => Self::ColorAddR,
            "ColorAdd_G" => Self::ColorAddG,
            "ColorAdd_B" => Self::ColorAddB,
            "ColorAdd_C" => Self::ColorAddC,
            "ColorAdd_M" => Self::ColorAddM,
            "ColorAdd_Y" => Self::ColorAddY,
            "ColorAdd_RY" => Self::ColorAddRY,
            "ColorAdd_GY" => Self::ColorAddGY,
            "ColorAdd_GC" => Self::ColorAddGC,
            "ColorAdd_BC" => Self::ColorAddBC,
            "ColorAdd_BM" => Self::ColorAddBM,
            "ColorAdd_RM" => Self::ColorAddRM,
            "ColorAdd_W" => Self::ColorAddW,
            "ColorAdd_WW" => Self::ColorAddWW,
            "ColorAdd_CW" => Self::ColorAddCW,
            "ColorAdd_UV" => Self::ColorAddUV,
            "ColorSub_R" => Self::ColorSubR,
            "ColorSub_G" => Self::ColorSubG,
            "ColorSub_B" => Self::ColorSubB,
            "ColorSub_C" => Self::ColorSubC,
            "ColorSub_M" => Self::ColorSubM,
            "ColorSub_Y" => Self::ColorSubY,
            "CTO" => Self::Cto,
            "CTC" => Self::Ctc,
            "CTB" => Self::Ctb,
            "Tint" => Self::Tint,
            "HSB_Hue" => Self::HsbHue,
            "HSB_Saturation" => Self::HsbSaturation,
            "HSB_Brightness" => Self::HsbBrightness,
            "HSB_Quality" => Self::HsbQuality,
            "CIE_X" => Self::CieX,
            "CIE_Y" => Self::CieY,
            "CIE_Brightness" => Self::CieBrightness,
            "ColorRGB_Red" => Self::ColorRgbRed,
            "ColorRGB_Green" => Self::ColorRgbGreen,
            "ColorRGB_Blue" => Self::ColorRgbBlue,
            "ColorRGB_Cyan" => Self::ColorRgbCyan,
            "ColorRGB_Magenta" => Self::ColorRgbMagenta,
            "ColorRGB_Yellow" => Self::ColorRgbYellow,
            "ColorRGB_Quality" => Self::ColorRgbQuality,
            "VideoBoost_R" => Self::VideoBoostR,
            "VideoBoost_G" => Self::VideoBoostG,
            "VideoBoost_B" => Self::VideoBoostB,
            "VideoHueShift" => Self::VideoHueShift,
            "VideoSaturation" => Self::VideoSaturation,
            "VideoBrightness" => Self::VideoBrightness,
            "VideoContrast" => Self::VideoContrast,
            "VideoKeyColor_R" => Self::VideoKeyColorR,
            "VideoKeyColor_G" => Self::VideoKeyColorG,
            "VideoKeyColor_B" => Self::VideoKeyColorB,
            "VideoKeyIntensity" => Self::VideoKeyIntensity,
            "VideoKeyTolerance" => Self::VideoKeyTolerance,

            "StrobeDuration" => Self::StrobeDuration,
            "StrobeRate" => Self::StrobeRate,
            "StrobeFrequency" => Self::StrobeFrequency,
            "StrobeModeShutter" => Self::StrobeModeShutter,
            "StrobeModeStrobe" => Self::StrobeModeStrobe,
            "StrobeModePulse" => Self::StrobeModePulse,
            "StrobeModePulseOpen" => Self::StrobeModePulseOpen,
            "StrobeModePulseClose" => Self::StrobeModePulseClose,
            "StrobeModeRandom" => Self::StrobeModeRandom,
            "StrobeModeRandomPulse" => Self::StrobeModeRandomPulse,
            "StrobeModeRandomPulseOpen" => Self::StrobeModeRandomPulseOpen,
            "StrobeModeRandomPulseClose" => Self::StrobeModeRandomPulseClose,
            "StrobeModeEffect" => Self::StrobeModeEffect,
            "Iris" => Self::Iris,
            "IrisStrobe" => Self::IrisStrobe,
            "IrisStrobeRandom" => Self::IrisStrobeRandom,
            "IrisPulseClose" => Self::IrisPulseClose,
            "IrisPulseOpen" => Self::IrisPulseOpen,
            "IrisRandomPulseClose" => Self::IrisRandomPulseClose,
            "IrisRandomPulseOpen" => Self::IrisRandomPulseOpen,

            "EffectsSync" => Self::EffectsSync,
            "BeamShaper" => Self::BeamShaper,
            "BeamShaperMacro" => Self::BeamShaperMacro,
            "BeamShaperPos" => Self::BeamShaperPos,
            "BeamShaperPosRotate" => Self::BeamShaperPosRotate,
            "Zoom" => Self::Zoom,
            "ZoomModeSpot" => Self::ZoomModeSpot,
            "ZoomModeBeam" => Self::ZoomModeBeam,
            "DigitalZoom" => Self::DigitalZoom,

            "DimmerMode" => Self::DimmerMode,
            "DimmerCurve" => Self::DimmerCurve,
            "BlackoutMode" => Self::BlackoutMode,
            "LEDFrequency" => Self::LedFrequency,
            "LEDZoneMode" => Self::LedZoneMode,
            "PixelMode" => Self::PixelMode,
            "PanMode" => Self::PanMode,
            "TiltMode" => Self::TiltMode,
            "PanTiltMode" => Self::PanTiltMode,
            "PositionModes" => Self::PositionModes,
            "GoboWheelShortcutMode" => Self::GoboWheelShortcutMode,
            "AnimationWheelShortcutMode" => Self::AnimationWheelShortcutMode,
            "ColorWheelShortcutMode" => Self::ColorWheelShortcutMode,
            "CyanMode" => Self::CyanMode,
            "MagentaMode" => Self::MagentaMode,
            "YellowMode" => Self::YellowMode,
            "ColorMixMode" => Self::ColorMixMode,
            "ChromaticMode" => Self::ChromaticMode,
            "ColorCalibrationMode" => Self::ColorCalibrationMode,
            "ColorConsistency" => Self::ColorConsistency,
            "ColorControl" => Self::ColorControl,
            "ColorModelMode" => Self::ColorModelMode,
            "ColorSettingsReset" => Self::ColorSettingsReset,
            "ColorUniformity" => Self::ColorUniformity,
            "CRIMode" => Self::CriMode,
            "CustomColor" => Self::CustomColor,
            "UVStability" => Self::UvStability,
            "WavelengthCorrection" => Self::WavelengthCorrection,
            "WhiteCount" => Self::WhiteCount,
            "StrobeMode" => Self::StrobeMode,
            "ZoomMode" => Self::ZoomMode,
            "FocusMode" => Self::FocusMode,
            "IrisMode" => Self::IrisMode,
            "FollowSpotMode" => Self::FollowSpotMode,
            "BeamEffectIndexRotateMode" => Self::BeamEffectIndexRotateMode,
            "IntensityMSpeed" => Self::IntensityMSpeed,
            "PositionMSpeed" => Self::PositionMSpeed,
            "ColorMixMSpeed" => Self::ColorMixMSpeed,
            "ColorWheelSelectMSpeed" => Self::ColorWheelSelectMSpeed,
            "IrisMSpeed" => Self::IrisMSpeed,
            "FocusMSpeed" => Self::FocusMSpeed,
            "ZoomMSpeed" => Self::ZoomMSpeed,
            "FrameMSpeed" => Self::FrameMSpeed,
            "GlobalMSpeed" => Self::GlobalMSpeed,
            "ReflectorAdjust" => Self::ReflectorAdjust,
            "FixtureGlobalReset" => Self::FixtureGlobalReset,
            "DimmerReset" => Self::DimmerReset,
            "ShutterReset" => Self::ShutterReset,
            "BeamReset" => Self::BeamReset,
            "ColorMixReset" => Self::ColorMixReset,
            "ColorWheelReset" => Self::ColorWheelReset,
            "FocusReset" => Self::FocusReset,
            "FrameReset" => Self::FrameReset,
            "GoboWheelReset" => Self::GoboWheelReset,
            "IntensityReset" => Self::IntensityReset,
            "IrisReset" => Self::IrisReset,
            "PositionReset" => Self::PositionReset,
            "PanReset" => Self::PanReset,
            "TiltReset" => Self::TiltReset,
            "ZoomReset" => Self::ZoomReset,
            "CTBReset" => Self::CtbReset,
            "CTOReset" => Self::CtoReset,
            "CTCReset" => Self::CtcReset,
            "AnimationSystemReset" => Self::AnimationSystemReset,
            "FixtureCalibrationReset" => Self::FixtureCalibrationReset,
            "Function" => Self::Function,
            "LampControl" => Self::LampControl,
            "DisplayIntensity" => Self::DisplayIntensity,
            "DMXInput" => Self::DmxInput,
            "NoFeature" => Self::NoFeature,

            "LampPowerMode" => Self::LampPowerMode,
            "Fans" => Self::Fans,

            "ShaperRot" => Self::ShaperRot,
            "ShaperMacros" => Self::ShaperMacros,
            "ShaperMacrosSpeed" => Self::ShaperMacrosSpeed,

            "Video" => Self::Video,
            "VideoBlendMode" => Self::VideoBlendMode,
            "InputSource" => Self::InputSource,
            "FieldOfView" => Self::FieldOfView,

            s => {
                     if let Some(n) = extract_attr_n(s, "Gobo", None) { Self::Gobo(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("SelectSpin")) { Self::GoboSelectSpin(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("SelectShake")) { Self::GoboSelectShake(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("SelectEffects")) { Self::GoboSelectEffects(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("WheelIndex")) { Self::GoboWheelIndex(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("WheelSpin")) { Self::GoboWheelSpin(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("WheelShake")) { Self::GoboWheelShake(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("WheelRandom")) { Self::GoboWheelRandom(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("WheelAudio")) { Self::GoboWheelAudio(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("Pos")) { Self::GoboPos(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("PosRotate")) { Self::GoboPosRotate(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("PosShake")) { Self::GoboPosShake(n) }

                else if let Some(n) = extract_attr_n(s, "AnimationWheel", None) { Self::AnimationWheel(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("Audio")) { Self::AnimationWheelAudio(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("Macro")) { Self::AnimationWheelMacro(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("Random")) { Self::AnimationWheelRandom(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("SelectEffects")) { Self::AnimationWheelSelectEffects(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("SelectShake")) { Self::AnimationWheelSelectShake(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("SelectSpin")) { Self::AnimationWheelSelectSpin(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("Pos")) { Self::AnimationWheelPos(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("PosRotate")) { Self::AnimationWheelPosRotate(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("PosShake")) { Self::AnimationWheelPosShake(n) }

                else if let Some(n) = extract_attr_n(s, "AnimationSystem", None) { Self::AnimationSystem(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("Ramp")) { Self::AnimationSystemRamp(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("Shake")) { Self::AnimationSystemShake(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("Audio")) { Self::AnimationSystemAudio(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("Random")) { Self::AnimationSystemRandom(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("Pos")) { Self::AnimationSystemPos(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("PosRotate")) { Self::AnimationSystemPosRotate(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("PosShake")) { Self::AnimationSystemPosShake(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("PosRandom")) { Self::AnimationSystemPosRandom(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("PosAudio")) { Self::AnimationSystemPosAudio(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationSystem", Some("Macro")) { Self::AnimationSystemMacro(n) }

                else if let Some(n) = extract_attr_n(s, "MediaFolder",  None) { Self::MediaFolder(n) }
                else if let Some(n) = extract_attr_n(s, "MediaContent", None) { Self::MediaContent(n) }
                else if let Some(n) = extract_attr_n(s, "ModelFolder",  None) { Self::ModelFolder(n) }
                else if let Some(n) = extract_attr_n(s, "ModelContent", None) { Self::ModelContent(n) }

                else if let Some(n) = extract_attr_n(s, "ColorEffects", None) { Self::ColorEffects(n) }
                else if let Some(n) = extract_attr_n(s, "Color", None) { Self::Color(n) }
                else if let Some(n) = extract_attr_n(s, "Color", Some("WheelIndex")) { Self::ColorWheelIndex(n) }
                else if let Some(n) = extract_attr_n(s, "Color", Some("WheelSpin")) { Self::ColorWheelSpin(n) }
                else if let Some(n) = extract_attr_n(s, "Color", Some("WheelRandom")) { Self::ColorWheelRandom(n) }
                else if let Some(n) = extract_attr_n(s, "Color", Some("WheelAudio")) { Self::ColorWheelAudio(n) }

                else if let Some(n) = extract_attr_n(s, "ColorMacro", None) { Self::ColorMacro(n) }
                else if let Some(n) = extract_attr_n(s, "ColorMacro", Some("Rate")) { Self::ColorMacroRate(n) }

                else if let Some(n) = extract_attr_n(s, "Shutter", None) { Self::Shutter(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("Strobe")) { Self::ShutterStrobe(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("StrobePulse")) { Self::ShutterStrobePulse(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("StrobePulseClose")) { Self::ShutterStrobePulseClose(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("StrobePulseOpen")) { Self::ShutterStrobePulseOpen(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("StrobeRandom")) { Self::ShutterStrobeRandom(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("StrobeRandomPulse")) { Self::ShutterStrobeRandomPulse(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("StrobeRandomPulseClose")) { Self::ShutterStrobeRandomPulseClose(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("StrobeRandomPulseOpen")) { Self::ShutterStrobeRandomPulseOpen(n) }
                else if let Some(n) = extract_attr_n(s, "Shutter", Some("StrobeEffect")) { Self::ShutterStrobeEffect(n) }

                else if let Some(n) = extract_attr_n(s, "Frost", None) { Self::Frost(n) }
                else if let Some(n) = extract_attr_n(s, "Frost", Some("PulseOpen")) { Self::FrostPulseOpen(n) }
                else if let Some(n) = extract_attr_n(s, "Frost", Some("PulseClose")) { Self::FrostPulseClose(n) }
                else if let Some(n) = extract_attr_n(s, "Frost", Some("Ramp")) { Self::FrostRamp(n) }

                else if let Some(n) = extract_attr_n(s, "Prism", None) { Self::Prism(n) }
                else if let Some(n) = extract_attr_n(s, "Prism", Some("SelectSpin")) { Self::PrismSelectSpin(n) }
                else if let Some(n) = extract_attr_n(s, "Prism", Some("Macro")) { Self::PrismMacro(n) }
                else if let Some(n) = extract_attr_n(s, "Prism", Some("Pos")) { Self::PrismPos(n) }
                else if let Some(n) = extract_attr_n(s, "Prism", Some("PosRotate")) { Self::PrismPosRotate(n) }

                else if let Some((n, m)) = extract_attr_n_m(s, "Effects", "Adjust", None) { Self::EffectsAdjust(n, m) }
                else if let Some(n) = extract_attr_n(s, "Effects", None) { Self::Effects(n) }
                else if let Some(n) = extract_attr_n(s, "Effects", Some("Rate")) { Self::EffectsRate(n) }
                else if let Some(n) = extract_attr_n(s, "Effects", Some("Fade")) { Self::EffectsFade(n) }
                else if let Some(n) = extract_attr_n(s, "Effects", Some("Pos")) { Self::EffectsPos(n) }
                else if let Some(n) = extract_attr_n(s, "Effects", Some("PosRotate")) { Self::EffectsPosRotate(n) }
                else if let Some(n) = extract_attr_n(s, "Focus", None) { Self::Focus(n) }
                else if let Some(n) = extract_attr_n(s, "Focus", Some("Adjust")) { Self::FocusAdjust(n) }
                else if let Some(n) = extract_attr_n(s, "Focus", Some("Distance")) { Self::FocusDistance(n) }
                else if let Some(n) = extract_attr_n(s, "Control", None) { Self::Control(n) }
                else if let Some(n) = extract_attr_n(s, "Gobo", Some("WheelMode")) { Self::GoboWheelMode(n) }
                else if let Some(n) = extract_attr_n(s, "AnimationWheel", Some("Mode")) { Self::AnimationWheelMode(n) }
                else if let Some(n) = extract_attr_n(s, "Color", Some("Mode")) { Self::ColorMode(n) }

                else if let Some(n) = extract_attr_n(s, "Fan", Some("Mode")) { Self::FanMode(n) }
                else if let Some(n) = extract_attr_n(s, "GoboWheel", Some("MSpeed")) { Self::GoboWheelMSpeed(n) }
                else if let Some(n) = extract_attr_n(s, "Prism", Some("MSpeed")) { Self::PrismMSpeed(n) }
                else if let Some(n) = extract_attr_n(s, "Frost", Some("MSpeed")) { Self::FrostMSpeed(n) }
                else if let Some(n) = extract_attr_n(s, "Blower", None) { Self::Blower(n) }
                else if let Some(n) = extract_attr_n(s, "Fan", None) { Self::Fan(n) }
                else if let Some(n) = extract_attr_n(s, "Fog", None) { Self::Fog(n) }
                else if let Some(n) = extract_attr_n(s, "Haze", None) { Self::Haze(n) }

                else if let Some(n) = extract_attr_n(s, "Blade", Some("A")) { Self::BladeA(n) }
                else if let Some(n) = extract_attr_n(s, "Blade", Some("B")) { Self::BladeB(n) }
                else if let Some(n) = extract_attr_n(s, "Blade", Some("Rot")) { Self::BladeRot(n) }
                else if let Some(n) = extract_attr_n(s, "BladeSoft", Some("A")) { Self::BladeSoftA(n) }
                else if let Some(n) = extract_attr_n(s, "BladeSoft", Some("B")) { Self::BladeSoftB(n) }
                else if let Some(n) = extract_attr_n(s, "KeyStone", Some("A")) { Self::KeyStoneA(n) }
                else if let Some(n) = extract_attr_n(s, "KeyStone", Some("B")) { Self::KeyStoneB(n) }

                else if let Some(n) = extract_attr_n(s, "VideoEffect", Some("Type")) { Self::VideoEffectType(n) }
                else if let Some((n, m)) = extract_attr_n_m(s, "VideoEffect", "Parameter", None) { Self::VideoEffectParameter(n, m) }
                else if let Some(n) = extract_attr_n(s, "VideoCamera", None) { Self::VideoCamera(n) }
                else if let Some(n) = extract_attr_n(s, "VideoSoundVolume", None) { Self::VideoSoundVolume(n) }

                else { Self::Custom(s.to_string()) }
            }
        };

        Ok(attribute)
    }
}

impl serde::Serialize for AttributeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for AttributeName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AttributeNameVisitor;

        impl<'de> serde::de::Visitor<'de> for AttributeNameVisitor {
            type Value = AttributeName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing an AttributeName")
            }

            fn visit_str<E>(self, value: &str) -> Result<AttributeName, E>
            where
                E: serde::de::Error,
            {
                value.parse().map_err(|_| E::custom(format!("invalid AttributeName: {}", value)))
            }
        }

        deserializer.deserialize_str(AttributeNameVisitor)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Feature {
    name: Name,
}

impl Feature {
    pub fn name(&self) -> &Name {
        &self.name
    }
}

impl<'src> bundle::FromBundle<'src> for Feature {
    type Source = bundle::Feature;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Feature { name: Name::new(&source.name) }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FeatureGroup {
    name: Name,
    pretty: String,
    features: Vec<Feature>,
    features_by_name: HashMap<Name, usize>,
}

impl FeatureGroup {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn pretty_name(&self) -> &str {
        &self.pretty
    }

    pub fn features(&self) -> &[Feature] {
        &self.features
    }

    pub fn feature(&self, name: &Name) -> Option<&Feature> {
        let ix = self.features_by_name.get(name)?;
        Some(&self.features[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for FeatureGroup {
    type Source = bundle::FeatureGroup;

    fn from_bundle(feature_group: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let mut features = Vec::new();
        let mut features_by_name = HashMap::default();
        for (i, feature) in feature_group.features.iter().enumerate() {
            let f = Feature::from_bundle(feature, bundle);
            features_by_name.insert(f.name().clone(), i);
            features.push(f);
        }

        let name = Name::new(&feature_group.name);

        Self {
            pretty: feature_group.pretty.to_owned().unwrap_or(name.to_string()),
            name,
            features,
            features_by_name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PhysicalUnit {
    #[default]
    None,
    Percent,
    Length,
    Mass,
    Time,
    Temperature,
    LuminousIntensity,
    Angle,
    Force,
    Frequency,
    Current,
    Voltage,
    Power,
    Energy,
    Area,
    Volume,
    Speed,
    Acceleration,
    AngularSpeed,
    AngularAcceleration,
    WaveLength,
    ColorComponent,
}

impl PhysicalUnit {
    pub fn suffix(&self) -> &str {
        match self {
            PhysicalUnit::None => "",
            PhysicalUnit::Percent => "%",
            PhysicalUnit::Length => "m",
            PhysicalUnit::Mass => "kg",
            PhysicalUnit::Time => "s",
            PhysicalUnit::Temperature => "K",
            PhysicalUnit::LuminousIntensity => "cd",
            PhysicalUnit::Angle => "°",
            PhysicalUnit::Force => "N",
            PhysicalUnit::Frequency => "Hz",
            PhysicalUnit::Current => "A",
            PhysicalUnit::Voltage => "V",
            PhysicalUnit::Power => "W",
            PhysicalUnit::Energy => "J",
            PhysicalUnit::Area => "m²",
            PhysicalUnit::Volume => "m³",
            PhysicalUnit::Speed => "m/s",
            PhysicalUnit::Acceleration => "m/s²",
            PhysicalUnit::AngularSpeed => "°/s",
            PhysicalUnit::AngularAcceleration => "°/s²",
            PhysicalUnit::WaveLength => "nm",
            PhysicalUnit::ColorComponent => "",
        }
    }

    pub fn format_value(&self, value: f32) -> String {
        let formatted = match self {
            PhysicalUnit::Percent => {
                format!("{:.0}", value)
            }

            PhysicalUnit::Length | PhysicalUnit::Area | PhysicalUnit::Volume => {
                format!("{:.2}", value)
            }

            PhysicalUnit::WaveLength => {
                format!("{:.1}", value)
            }

            PhysicalUnit::Angle
            | PhysicalUnit::AngularSpeed
            | PhysicalUnit::AngularAcceleration => {
                format!("{:.1}", value)
            }

            PhysicalUnit::Mass
            | PhysicalUnit::Time
            | PhysicalUnit::Temperature
            | PhysicalUnit::Force
            | PhysicalUnit::Frequency
            | PhysicalUnit::Current
            | PhysicalUnit::Voltage
            | PhysicalUnit::Power
            | PhysicalUnit::Energy
            | PhysicalUnit::Speed
            | PhysicalUnit::Acceleration
            | PhysicalUnit::LuminousIntensity
            | PhysicalUnit::ColorComponent => {
                format!("{:.2}", value)
            }

            PhysicalUnit::None => {
                format!("{:.2}", value)
            }
        };

        format!("{}{}", formatted, self.suffix())
    }
}

impl fmt::Display for PhysicalUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhysicalUnit::None => write!(f, "No Unit"),
            PhysicalUnit::Percent => write!(f, "Percent"),
            PhysicalUnit::Length => write!(f, "Length"),
            PhysicalUnit::Mass => write!(f, "Mass"),
            PhysicalUnit::Time => write!(f, "Time"),
            PhysicalUnit::Temperature => write!(f, "Temperature"),
            PhysicalUnit::LuminousIntensity => write!(f, "Luminous Intensity"),
            PhysicalUnit::Angle => write!(f, "Angle"),
            PhysicalUnit::Force => write!(f, "Force"),
            PhysicalUnit::Frequency => write!(f, "Frequency"),
            PhysicalUnit::Current => write!(f, "Current"),
            PhysicalUnit::Voltage => write!(f, "Voltage"),
            PhysicalUnit::Power => write!(f, "Power"),
            PhysicalUnit::Energy => write!(f, "Energy"),
            PhysicalUnit::Area => write!(f, "Area"),
            PhysicalUnit::Volume => write!(f, "Volume"),
            PhysicalUnit::Speed => write!(f, "Speed"),
            PhysicalUnit::Acceleration => write!(f, "Acceleration"),
            PhysicalUnit::AngularSpeed => write!(f, "Angular Speed"),
            PhysicalUnit::AngularAcceleration => write!(f, "Angular Acceleration"),
            PhysicalUnit::WaveLength => write!(f, "Wave Length"),
            PhysicalUnit::ColorComponent => write!(f, "Color Component"),
        }
    }
}

impl PhysicalUnit {
    pub fn from_bundle(physical_unit: &bundle::PhysicalUnit) -> Self {
        match physical_unit {
            bundle::PhysicalUnit::None => PhysicalUnit::None,
            bundle::PhysicalUnit::Percent => PhysicalUnit::Percent,
            bundle::PhysicalUnit::Length => PhysicalUnit::Length,
            bundle::PhysicalUnit::Mass => PhysicalUnit::Mass,
            bundle::PhysicalUnit::Time => PhysicalUnit::Time,
            bundle::PhysicalUnit::Temperature => PhysicalUnit::Temperature,
            bundle::PhysicalUnit::LuminousIntensity => PhysicalUnit::LuminousIntensity,
            bundle::PhysicalUnit::Angle => PhysicalUnit::Angle,
            bundle::PhysicalUnit::Force => PhysicalUnit::Force,
            bundle::PhysicalUnit::Frequency => PhysicalUnit::Frequency,
            bundle::PhysicalUnit::Current => PhysicalUnit::Current,
            bundle::PhysicalUnit::Voltage => PhysicalUnit::Voltage,
            bundle::PhysicalUnit::Power => PhysicalUnit::Power,
            bundle::PhysicalUnit::Energy => PhysicalUnit::Energy,
            bundle::PhysicalUnit::Area => PhysicalUnit::Area,
            bundle::PhysicalUnit::Volume => PhysicalUnit::Volume,
            bundle::PhysicalUnit::Speed => PhysicalUnit::Speed,
            bundle::PhysicalUnit::Acceleration => PhysicalUnit::Acceleration,
            bundle::PhysicalUnit::AngularSpeed => PhysicalUnit::AngularSpeed,
            bundle::PhysicalUnit::AngularAccc => PhysicalUnit::AngularAcceleration,
            bundle::PhysicalUnit::WaveLength => PhysicalUnit::WaveLength,
            bundle::PhysicalUnit::ColorComponent => PhysicalUnit::ColorComponent,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubPhysicalUnit {
    r#type: SubPhysicalUnitType,
    physical_unit: PhysicalUnit,
    physical_from: f32,
    physical_to: f32,
}

impl SubPhysicalUnit {
    pub fn r#type(&self) -> SubPhysicalUnitType {
        self.r#type
    }

    pub fn physical_unit(&self) -> PhysicalUnit {
        self.physical_unit
    }

    pub fn physical_from(&self) -> f32 {
        self.physical_from
    }

    pub fn physical_to(&self) -> f32 {
        self.physical_to
    }
}

impl<'src> bundle::FromBundle<'src> for SubPhysicalUnit {
    type Source = bundle::SubPhysicalUnit;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            r#type: SubPhysicalUnitType::from_bundle(&source.r#type, bundle),
            physical_unit: source
                .physical_unit
                .as_ref()
                .map(PhysicalUnit::from_bundle)
                .unwrap_or_default(),
            physical_from: source.physical_from.unwrap_or(0.0),
            physical_to: source.physical_to.unwrap_or(1.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SubPhysicalUnitType {
    PlacementOffset,
    Amplitude,
    AmplitudeMin,
    AmplitudeMax,
    Duration,
    DutyCycle,
    TimeOffset,
    MinimumOpening,
    Value,
    RatioHorizontal,
    RatioVertical,
}

impl fmt::Display for SubPhysicalUnitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubPhysicalUnitType::PlacementOffset => write!(f, "PlacementOffset"),
            SubPhysicalUnitType::Amplitude => write!(f, "Amplitude"),
            SubPhysicalUnitType::AmplitudeMin => write!(f, "AmplitudeMin"),
            SubPhysicalUnitType::AmplitudeMax => write!(f, "AmplitudeMax"),
            SubPhysicalUnitType::Duration => write!(f, "Duration"),
            SubPhysicalUnitType::DutyCycle => write!(f, "DutyCycle"),
            SubPhysicalUnitType::TimeOffset => write!(f, "TimeOffset"),
            SubPhysicalUnitType::MinimumOpening => write!(f, "MinimumOpening"),
            SubPhysicalUnitType::Value => write!(f, "Value"),
            SubPhysicalUnitType::RatioHorizontal => write!(f, "RatioHorizontal"),
            SubPhysicalUnitType::RatioVertical => write!(f, "RatioVertical"),
        }
    }
}

impl<'src> bundle::FromBundle<'src> for SubPhysicalUnitType {
    type Source = bundle::SubPhysicalType;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::SubPhysicalType::PlacementOffset => SubPhysicalUnitType::PlacementOffset,
            bundle::SubPhysicalType::Amplitude => SubPhysicalUnitType::Amplitude,
            bundle::SubPhysicalType::AmplitudeMin => SubPhysicalUnitType::AmplitudeMin,
            bundle::SubPhysicalType::AmplitudeMax => SubPhysicalUnitType::AmplitudeMax,
            bundle::SubPhysicalType::Duration => SubPhysicalUnitType::Duration,
            bundle::SubPhysicalType::DutyCycle => SubPhysicalUnitType::DutyCycle,
            bundle::SubPhysicalType::TimeOffset => SubPhysicalUnitType::TimeOffset,
            bundle::SubPhysicalType::MinimumOpening => SubPhysicalUnitType::MinimumOpening,
            bundle::SubPhysicalType::Value => SubPhysicalUnitType::Value,
            bundle::SubPhysicalType::RatioHorizontal => SubPhysicalUnitType::RatioHorizontal,
            bundle::SubPhysicalType::RatioVertical => SubPhysicalUnitType::RatioVertical,
        }
    }
}

fn extract_attr_n<T: str::FromStr>(s: &str, prefix: &str, suffix: Option<&str>) -> Option<T> {
    s.strip_prefix(prefix).and_then(|rest| {
        if let Some(suffix) = suffix {
            rest.strip_suffix(suffix).and_then(|number_part| number_part.parse::<T>().ok())
        } else {
            rest.parse::<T>().ok()
        }
    })
}

fn extract_attr_n_m<T: str::FromStr>(
    s: &str,
    prefix: &str,
    middle: &str,
    suffix: Option<&str>,
) -> Option<(T, T)> {
    if !s.starts_with(prefix) {
        return None;
    }

    let rest = &s[prefix.len()..];

    if let Some(middle_pos) = rest.find(middle) {
        let n_part = &rest[..middle_pos];
        let n = n_part.parse::<T>().ok()?;

        let after_middle = &rest[middle_pos + middle.len()..];

        let m_part = if let Some(suffix) = suffix {
            after_middle.strip_suffix(suffix)?
        } else {
            after_middle
        };

        let m = m_part.parse::<T>().ok()?;

        Some((n, m))
    } else {
        None
    }
}

use super::asset::{AssetId, Preset};

pub trait Attribute: Eq + std::hash::Hash {
    fn to_attr(&self) -> Attr;
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnyPresetAssetId {
    Dimmer(AssetId<Preset<DimmerAttr>>),
    Position(AssetId<Preset<PositionAttr>>),
    Gobo(AssetId<Preset<GoboAttr>>),
    Color(AssetId<Preset<ColorAttr>>),
    Beam(AssetId<Preset<BeamAttr>>),
    Focus(AssetId<Preset<FocusAttr>>),
    Control(AssetId<Preset<ControlAttr>>),
    Shapers(AssetId<Preset<ShapersAttr>>),
    Video(AssetId<Preset<VideoAttr>>),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attr {
    Dimmer(DimmerAttr),
    Position(PositionAttr),
    Gobo(GoboAttr),
    Color(ColorAttr),
    Beam(BeamAttr),
    Focus(FocusAttr),
    Control(ControlAttr),
    Shapers(ShapersAttr),
    Video(VideoAttr),
}

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dimmer(attr) => write!(f, "{}", attr),
            Self::Position(attr) => write!(f, "{}", attr),
            Self::Gobo(attr) => write!(f, "{}", attr),
            Self::Color(attr) => write!(f, "{}", attr),
            Self::Beam(attr) => write!(f, "{}", attr),
            Self::Focus(attr) => write!(f, "{}", attr),
            Self::Control(attr) => write!(f, "{}", attr),
            Self::Shapers(attr) => write!(f, "{}", attr),
            Self::Video(attr) => write!(f, "{}", attr),
        }
    }
}

impl Attribute for Attr {
    fn to_attr(&self) -> Attr {
        *self
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DimmerAttr {
    /// Controls the intensity of a fixture.
    Dimmer,
}

impl std::fmt::Display for DimmerAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dimmer => write!(f, "Dimmer"),
        }
    }
}

impl Attribute for DimmerAttr {
    fn to_attr(&self) -> Attr {
        Attr::Dimmer(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionAttr {
    /// Controls the fixture’s sideward movement (horizontal axis).
    Pan,
    /// Controls the fixture’s upward and the downward movement (vertical axis).
    Tilt,
    /// Controls the speed of the fixture’s continuous pan movement (horizontal axis).
    PanRotate,
    /// Controls the speed of the fixture’s continuous tilt movement (vertical axis).
    TiltRotate,
    /// Selects the predefined position effects that are built into the fixture.
    PositionEffect,
    /// Controls the speed of the predefined position effects that are built into the fixture.
    PositionEffectRate,
    /// Snaps or smooth fades with timing in running predefined position effects.
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
}

impl std::fmt::Display for PositionAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
        }
    }
}

impl Attribute for PositionAttr {
    fn to_attr(&self) -> Attr {
        Attr::Position(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoboAttr {
    /// The fixture’s gobo wheel (n). This is the main attribute of gobo wheel’s (n) wheel control. Selects gobos in gobo wheel (n). A different channel function sets the angle of the indexed position in the selected gobo or the angular speed of its continuous rotation.
    Gobo(u8),
    /// Selects gobos whose rotation is continuous in gobo wheel (n) and controls the angular speed of the gobo’s spin within the same channel function.
    GoboSelectSpin(u8),
    /// Selects gobos which shake in gobo wheel (n) and controls the frequency of the gobo’s shake within the same channel function.
    GoboSelectShake(u8),
    /// Selects gobos which run effects in gobo wheel (n).
    GoboSelectEffects(u8),
    /// Controls angle of indexed rotation of gobo wheel (n).
    GoboWheelIndex(u8),
    /// Controls the speed and direction of continuous rotation of gobo wheel (n).
    GoboWheelSpin(u8),
    /// Controls frequency of the shake of gobo wheel (n).
    GoboWheelShake(u8),
    /// Controls speed of gobo wheel´s (n) random gobo slot selection.
    GoboWheelRandom(u8),
    /// Controls audio-controlled functionality of gobo wheel (n).
    GoboWheelAudio(u8),
    /// Controls angle of indexed rotation of gobos in gobo wheel (n). This is the main attribute of gobo wheel’s (n) wheel slot control.
    GoboPos(u8),
    /// Controls the speed and direction of continuous rotation of gobos in gobo wheel (n).
    GoboPosRotate(u8),
    /// Controls frequency of the shake of gobos in gobo wheel (n).
    GoboPosShake(u8),
    /// This is the main attribute of the animation wheel’s (n) wheel control. Selects slots in the animation wheel. A different channel function sets the angle of the indexed position in the selected slot or the angular speed of its continuous rotation. Is used for animation effects with multiple slots.
    AnimationWheel(u8),
    /// Controls audio-controlled functionality of animation wheel (n).
    AnimationWheelAudio(u8),
    /// Selects predefined effects in animation wheel (n).
    AnimationWheelMacro(u8),
    /// Controls frequency of animation wheel (n) random slot selection.
    AnimationWheelRandom(u8),
    /// Selects slots which run effects in animation wheel (n).
    AnimationWheelSelectEffects(u8),
    /// Selects slots which shake in animation wheel and controls the frequency of the slots shake within the same channel function.
    AnimationWheelSelectShake(u8),
    /// Selects slots whose rotation is continuous in animation wheel and controls the angular speed of the slot spin within the same channel function
    AnimationWheelSelectSpin(u8),
    /// Controls angle of indexed rotation of slots in animation wheel. This is the main attribute of animation wheel (n) wheel slot control.
    AnimationWheelPos(u8),
    /// Controls the speed and direction of continuous rotation of slots in animation wheel (n).
    AnimationWheelPosRotate(u8),
    /// Controls frequency of the shake of slots in animation wheel (n).
    AnimationWheelPosShake(u8),
    /// This is the main attribute of the animation system insertion control. Controls the insertion of the fixture’s animation system in the light output. Is used for animation effects where a disk is inserted into the light output.
    AnimationSystem(u8),
    /// Sets frequency of animation system (n) insertion ramp.
    AnimationSystemRamp(u8),
    /// Sets frequency of animation system (n) insertion shake.
    AnimationSystemShake(u8),
    /// Controls audio-controlled functionality of animation system (n) insertion.
    AnimationSystemAudio(u8),
    /// Controls frequency of animation system (n) random insertion.
    AnimationSystemRandom(u8),
    /// This is the main attribute of the animation system spinning control. Controls angle of indexed rotation of animation system (n) disk.
    AnimationSystemPos(u8),
    /// Controls the speed and direction of continuous rotation of animation system (n) disk.
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
    /// Selects folder that contains 3D model content. For example 3D meshes for mapping.
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
}

impl std::fmt::Display for GoboAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
        }
    }
}

impl Attribute for GoboAttr {
    fn to_attr(&self) -> Attr {
        Attr::Gobo(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorAttr {
    /// Selects predefined color effects built into the fixture.
    ColorEffects(u8),
    /// The fixture’s color wheel (n). Selects colors in color wheel (n). This is the main attribute of color wheel’s (n) wheel control.
    Color(u8),
    /// Controls angle of indexed rotation of color wheel (n)
    ColorWheelIndex(u8),
    /// Controls the speed and direction of continuous rotation of color wheel (n).
    ColorWheelSpin(u8),
    /// Controls frequency of color wheel´s (n) random color slot selection.
    ColorWheelRandom(u8),
    /// Controls audio-controlled functionality of color wheel (n).
    ColorWheelAudio(u8),
    /// Controls the intensity of the fixture’s red emitters for direct additive color mixing.
    ColorAddR,
    /// Controls the intensity of the fixture’s green emitters for direct additive color mixing
    ColorAddG,
    /// Controls the intensity of the fixture’s blue emitters for direct additive color mixing.
    ColorAddB,
    /// Controls the intensity of the fixture’s cyan emitters for direct additive color mixing.
    ColorAddC,
    /// Controls the intensity of the fixture’s magenta emitters for direct additive color mixing.
    ColorAddM,
    /// Controls the intensity of the fixture’s yellow emitters for direct additive color mixing.
    ColorAddY,
    /// Controls the intensity of the fixture’s amber emitters for direct additive color mixing.
    ColorAddRY,
    /// Controls the intensity of the fixture’s lime emitters for direct additive color mixing.
    ColorAddGY,
    /// Controls the intensity of the fixture’s blue-green emitters for direct additive color mixing.
    ColorAddGC,
    /// Controls the intensity of the fixture’s light-blue emitters for direct additive color mixing.
    ColorAddBC,
    /// Controls the intensity of the fixture’s purple emitters for direct additive color mixing.
    ColorAddBM,
    /// Controls the intensity of the fixture’s pink emitters for direct additive color mixing.
    ColorAddRM,
    /// Controls the intensity of the fixture’s white emitters for direct additive color mixing.
    ColorAddW,
    /// Controls the intensity of the fixture’s warm white emitters for direct additive color mixing.
    ColorAddWW,
    /// Controls the intensity of the fixture’s cool white emitters for direct additive color mixing.
    ColorAddCW,
    /// Controls the intensity of the fixture’s UV emitters for direct additive color mixing.
    ColorAddUV,
    /// Controls the insertion of the fixture’s red filter flag for direct subtractive color mixing.
    ColorSubR,
    /// Controls the insertion of the fixture’s green filter flag for direct subtractive color mixing.
    ColorSubG,
    /// Controls the insertion of the fixture’s blue filter flag for direct subtractive color mixing.
    ColorSubB,
    /// Controls the insertion of the fixture’s cyan filter flag for direct subtractive color mixing.
    ColorSubC,
    /// Controls the insertion of the fixture’s magenta filter flag for direct subtractive color mixing.
    ColorSubM,
    /// Controls the insertion of the fixture’s yellow filter flag for direct subtractive color mixing.
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
    /// Controls the fixture’s “Correct green to magenta” wheel or mixing system.
    Tint,
    /// Controls the fixture’s color attribute regarding the hue.
    HsbHue,
    /// Controls the fixture’s color attribute regarding the saturation.
    HsbSaturation,
    /// Controls the fixture’s color attribute regarding the brightness.
    HsbBrightness,
    /// Controls the fixture’s color attribute regarding the quality.
    HsbQuality,
    /// Controls the fixture’s CIE 1931 color attribute regarding the chromaticity x.
    CieX,
    /// Controls the fixture’s CIE 1931 color attribute regarding the chromaticity y.
    CieY,
    /// Controls the fixture’s CIE 1931 color attribute regarding the brightness (Y).
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
}

impl std::fmt::Display for ColorAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
        }
    }
}

impl Attribute for ColorAttr {
    fn to_attr(&self) -> Attr {
        Attr::Color(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeamAttr {
    /// Controls the length of a strobe flash.
    StrobeDuration,
    /// Controls the time between strobe flashes.
    StrobeRate,
    /// Controls the frequency of strobe flashes.
    StrobeFrequency,
    /// Strobe mode shutter. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeShutter,
    /// Strobe mode strobe. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeStrobe,
    /// Strobe mode pulse. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModePulse,
    /// Strobe mode opening pulse. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModePulseOpen,
    /// Strobe mode closing pulse. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModePulseClose,
    /// Strobe mode random strobe. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeRandom,
    /// Strobe mode random pulse. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeRandomPulse,
    /// Strobe mode random opening pulse. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeRandomPulseOpen,
    /// Strobe mode random closing pulse. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeRandomPulseClose,
    /// Strobe mode random shutter effect feature. Use this attribute together with StrobeFrequency to define the type of the shutter / strobe.
    StrobeModeEffect,
    /// Controls the fixture´s mechanical or electronical shutter feature.
    Shutter(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical strobe shutter feature.
    ShutterStrobe(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical pulse shutter feature.
    ShutterStrobePulse(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical closing pulse shutter feature. The pulse is described by a ramp function.
    ShutterStrobePulseClose(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical opening pulse shutter feature. The pulse is described by a ramp function.
    ShutterStrobePulseOpen(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical random strobe shutter feature.
    ShutterStrobeRandom(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical random pulse shutter feature.
    ShutterStrobeRandomPulse(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical random closing pulse shutter feature. The pulse is described by a ramp function.
    ShutterStrobeRandomPulseClose(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical random opening pulse shutter feature. The pulse is described by a ramp function.
    ShutterStrobeRandomPulseOpen(u8),
    /// Controls the frequency of the fixture´s mechanical or electronical shutter effect feature.
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
}

impl std::fmt::Display for BeamAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::FrostPulseOpen(n) => write!(f, "FrostPulseOpen{n}"),
            Self::FrostPulseClose(n) => write!(f, "FrostPulseClose{n}"),
            Self::FrostRamp(n) => write!(f, "FrostRamp{n}"),
        }
    }
}

impl Attribute for BeamAttr {
    fn to_attr(&self) -> Attr {
        Attr::Beam(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusAttr {
    /// The fixture’s prism wheel (n). Selects prisms in prism wheel (n). A different channel function sets the angle of the indexed position in the selected prism or the angular speed of its continuous rotation. This is the main attribute of prism wheel’s (n) wheel control.
    Prism(u8),
    /// Selects prisms whose rotation is continuous in prism wheel (n) and controls the angular speed of the prism’s spin within the same channel function.
    PrismSelectSpin(u8),
    /// Macro functions of prism wheel (n).
    PrismMacro(u8),
    /// Controls angle of indexed rotation of prisms in prism wheel (n). This is the main attribute of prism wheel’s 1 wheel slot control.
    PrismPos(u8),
    /// Controls the speed and direction of continuous rotation of prisms in prism wheel (n).
    PrismPosRotate(u8),
}

impl std::fmt::Display for FocusAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prism(n) => write!(f, "Prism{n}"),
            Self::PrismSelectSpin(n) => write!(f, "PrismSelectSpin{n}"),
            Self::PrismMacro(n) => write!(f, "PrismMacro{n}"),
            Self::PrismPos(n) => write!(f, "PrismPos{n}"),
            Self::PrismPosRotate(n) => write!(f, "PrismPosRotate{n}"),
        }
    }
}

impl Attribute for FocusAttr {
    fn to_attr(&self) -> Attr {
        Attr::Focus(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlAttr {}

impl std::fmt::Display for ControlAttr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl Attribute for ControlAttr {
    fn to_attr(&self) -> Attr {
        Attr::Control(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapersAttr {}

impl std::fmt::Display for ShapersAttr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl Attribute for ShapersAttr {
    fn to_attr(&self) -> Attr {
        Attr::Shapers(*self)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoAttr {}

impl std::fmt::Display for VideoAttr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl Attribute for VideoAttr {
    fn to_attr(&self) -> Attr {
        Attr::Video(*self)
    }
}

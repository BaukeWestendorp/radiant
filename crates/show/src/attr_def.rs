use anyhow::bail;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
/// To describe the fixture types attributes are used. Attributes define the function. (n) and (m) are wildcards for the enumeration of attributes like Gobo(n) - Gobo1 and Gobo2 or VideoEffect(n)Parameter(m) - VideoEffect1Parameter1 and VideoEffect1Parameter2. Fixture Type Attributes without wildcards (n) or (m) are not enumerated. The enumeration starts with 1.
pub enum SharedString {
    #[default]
    /// Controls the intensity of a fixture.
    Dimmer,
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
    ScaleXyz,
    /// The fixture’s gobo wheel (`n`). This is the main attribute of gobo wheel’s (`n`) wheel control. Selects gobos in gobo wheel (`n`). A different channel function sets the angle of the indexed position in the selected gobo or the angular speed of its continuous rotation.
    Gobo { n: u32 },
    /// Selects gobos whose rotation is continuous in gobo wheel (`n`) and controls the angular speed of the gobo’s spin within the same channel function.
    GoboSelectSpin { n: u32 },
    /// Selects gobos which shake in gobo wheel (`n`) and controls the frequency of the gobo’s shake within the same channel function.
    GoboSelectShake { n: u32 },
    /// Selects gobos which run effects in gobo wheel (`n`).
    GoboSelectEffects { n: u32 },
    /// Controls angle of indexed rotation of gobo wheel (`n`).
    GoboWheelIndex { n: u32 },
    /// Controls the speed and direction of continuous rotation of gobo wheel (`n`).
    GoboWheelSpin { n: u32 },
    /// Controls frequency of the shake of gobo wheel (`n`).
    GoboWheelShake { n: u32 },
    /// Controls speed of gobo wheel´s (`n`) random gobo slot selection.
    GoboWheelRandom { n: u32 },
    /// Controls audio-controlled functionality of gobo wheel (`n`).
    GoboWheelAudio { n: u32 },
    /// Controls angle of indexed rotation of gobos in gobo wheel (`n`). This is the main attribute of gobo wheel’s (`n`) wheel slot control.
    GoboPos { n: u32 },
    /// Controls the speed and direction of continuous rotation of gobos in gobo wheel (`n`).
    GoboPosRotate { n: u32 },
    /// Controls frequency of the shake of gobos in gobo wheel (`n`).
    GoboPosShake { n: u32 },
    /// This is the main attribute of the animation wheel’s (`n`) wheel control. Selects slots in the animation wheel. A different channel function sets the angle of the indexed position in the selected slot or the angular speed of its continuous rotation. Is used for animation effects with multiple slots.
    AnimationWheel { n: u32 },
    /// Controls audio-controlled functionality of animation wheel (`n`).
    AnimationWheelAudio { n: u32 },
    /// Selects predefined effects in animation wheel (`n`).
    AnimationWheelMacro { n: u32 },
    /// Controls frequency of animation wheel (`n`) random slot selection.
    AnimationWheelRandom { n: u32 },
    /// Selects slots which run effects in animation wheel (`n`).
    AnimationWheelSelectEffects { n: u32 },
    /// Selects slots which shake in animation wheel and controls the frequency of the slots shake within the same channel function.
    AnimationWheelSelectShake { n: u32 },
    /// Selects slots whose rotation is continuous in animation wheel and controls the angular speed of the slot spin within the same channel function
    AnimationWheelSelectSpin { n: u32 },
    /// Controls angle of indexed rotation of slots in animation wheel. This is the main attribute of animation wheel (`n`) wheel slot control.
    AnimationWheelPos { n: u32 },
    /// Controls the speed and direction of continuous rotation of slots in animation wheel (`n`).
    AnimationWheelPosRotate { n: u32 },
    /// Controls frequency of the shake of slots in animation wheel (`n`).
    AnimationWheelPosShake { n: u32 },
    /// This is the main attribute of the animation system insertion control. Controls the insertion of the fixture’s animation system in the light output. Is used for animation effects where a disk is inserted into the light output.
    AnimationSystem { n: u32 },
    /// Sets frequency of animation system (`n`) insertion ramp.
    AnimationSystemRamp { n: u32 },
    /// Sets frequency of animation system (`n`) insertion shake.
    AnimationSystemShake { n: u32 },
    /// Controls audio-controlled functionality of animation system (`n`) insertion.
    AnimationSystemAudio { n: u32 },
    /// Controls frequency of animation system (`n`) random insertion.
    AnimationSystemRandom { n: u32 },
    /// This is the main attribute of the animation system spinning control. Controls angle of indexed rotation of animation system (`n`) disk.
    AnimationSystemPos { n: u32 },
    /// Controls the speed and direction of continuous rotation of animation system (`n`) disk.
    AnimationSystemPosRotate { n: u32 },
    /// Controls frequency of the shake of animation system (`n`) disk.
    AnimationSystemPosShake { n: u32 },
    /// Controls random speed of animation system (`n`) disk.
    AnimationSystemPosRandom { n: u32 },
    /// Controls audio-controlled functionality of animation system (`n`) disk.
    AnimationSystemPosAudio { n: u32 },
    /// Selects predefined effects in animation system (`n`).
    AnimationSystemMacro { n: u32 },
    /// Selects folder that contains media content.
    MediaFolder { n: u32 },
    /// Selects file with media content.
    MediaContent { n: u32 },
    /// Selects folder that contains 3D model content. For example 3D meshes for mapping.
    ModelFolder { n: u32 },
    /// Selects file with 3D model content.
    ModelContent { n: u32 },
    /// Defines media playback mode.
    PlayMode,
    /// Defines starting point of media content playback.
    PlayBegin,
    /// Defines end point of media content playback.
    PlayEnd,
    /// Adjusts playback speed of media content.
    PlaySpeed,
    /// Selects predefined color effects built into the fixture.
    ColorEffects { n: u32 },
    /// The fixture’s color wheel (`n`). Selects colors in color wheel (`n`). This is the main attribute of color wheel’s (`n`) wheel control.
    Color { n: u32 },
    /// Controls angle of indexed rotation of color wheel (`n`)
    ColorWheelIndex { n: u32 },
    /// Controls the speed and direction of continuous rotation of color wheel (`n`).
    ColorWheelSpin { n: u32 },
    /// Controls frequency of color wheel´s (`n`) random color slot selection.
    ColorWheelRandom { n: u32 },
    /// Controls audio-controlled functionality of color wheel (`n`).
    ColorWheelAudio { n: u32 },
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
    ColorMacro { n: u32 },
    /// Controls the time between Color Macro steps.
    ColorMacroRate { n: u32 },
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
    Shutter { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical strobe shutter feature.
    ShutterStrobe { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical pulse shutter feature.
    ShutterStrobePulse { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical closing pulse shutter feature. The pulse is described by a ramp function.
    ShutterStrobePulseClose { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical opening pulse shutter feature. The pulse is described by a ramp function.
    ShutterStrobePulseOpen { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical random strobe shutter feature.
    ShutterStrobeRandom { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical random pulse shutter feature.
    ShutterStrobeRandomPulse { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical random closing pulse shutter feature. The pulse is described by a ramp function.
    ShutterStrobeRandomPulseClose { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical random opening pulse shutter feature. The pulse is described by a ramp function.
    ShutterStrobeRandomPulseOpen { n: u32 },
    /// Controls the frequency of the fixture´s mechanical or electronical shutter effect feature.
    ShutterStrobeEffect { n: u32 },
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
    Frost { n: u32 },
    /// Sets frequency of frost’s opening pulse
    FrostPulseOpen { n: u32 },
    /// Sets frequency of frost’s closing pulse.
    FrostPulseClose { n: u32 },
    /// Sets frequency of frost’s ramp.
    FrostRamp { n: u32 },
    /// The fixture’s prism wheel (`n`). Selects prisms in prism wheel (`n`). A different channel function sets the angle of the indexed position in the selected prism or the angular speed of its continuous rotation. This is the main attribute of prism wheel’s (`n`) wheel control.
    Prism { n: u32 },
    /// Selects prisms whose rotation is continuous in prism wheel (`n`) and controls the angular speed of the prism’s spin within the same channel function.
    PrismSelectSpin { n: u32 },
    /// Macro functions of prism wheel (`n`).
    PrismMacro { n: u32 },
    /// Controls angle of indexed rotation of prisms in prism wheel (`n`). This is the main attribute of prism wheel’s 1 wheel slot control.
    PrismPos { n: u32 },
    /// Controls the speed and direction of continuous rotation of prisms in prism wheel (`n`).
    PrismPosRotate { n: u32 },
    /// Generically predefined macros and effects of a fixture.
    Effects { n: u32 },
    /// Frequency of running effects.
    EffectsRate { n: u32 },
    /// Snapping or smooth look of running effects.
    EffectsFade { n: u32 },
    /// Controls parameter (`m`) of effect (`n`)
    EffectsAdjust { n: u32, m: u32 },
    /// Controls angle of indexed rotation of slot/effect in effect wheel/macro (`n`). This is the main attribute of effect wheel/macro (`n`) slot/effect control.
    EffectsPos { n: u32 },
    /// Controls speed and direction of slot/effect in effect wheel (`n`).
    EffectsPosRotate { n: u32 },
    /// Sets offset between running effects and effects 2.
    EffectsSync,
    /// Activates fixture’s beam shaper.
    BeamShaper,
    /// Predefined presets for fixture’s beam shaper positions.
    BeamShaperMacro,
    /// Indexing of fixture’s beam shaper.
    BeamShaperPos,
    /// Continuous rotation of fixture’s beam shaper.
    BeamShaperPosRotate,
    /// Controls the spread of the fixture’s beam/spot.
    Zoom,
    /// Selects spot mode of zoom.
    ZoomModeSpot,
    /// Selects beam mode of zoom.
    ZoomModeBeam,
    /// Controls the image size within the defined projection. Used on digital projection based devices
    DigitalZoom,
    /// Controls the sharpness of the fixture’s spot light. Can blur or sharpen the edge of the spot.
    Focus { n: u32 },
    /// Autofocuses functionality using presets.
    FocusAdjust { n: u32 },
    /// Autofocuses functionality using distance.
    FocusDistance { n: u32 },
    /// Controls the channel of a fixture.
    Control { n: u32 },
    /// Selects different modes of intensity.
    DimmerMode,
    /// Selects different dimmer curves of the fixture.
    DimmerCurve,
    /// Close the light output under certain conditions (movement correction, gobo movement, etc.).
    BlackoutMode,
    /// Controls LED frequency.
    LedFrequency,
    /// Changes zones of LEDs.
    LedZoneMode,
    /// Controls behavior of LED pixels.
    PixelMode,
    /// Selects fixture’s pan mode. Selects between a limited pan range (e.g. -270 to 270) or a continuous pan range.
    PanMode,
    /// Selects fixture’s pan mode. Selects between a limited tilt range (e.g. -130 to 130) or a continuous tilt range.
    TiltMode,
    /// Selects fixture’s pan/tilt mode. Selects between a limited pan/tilt range or a continuous pan/tilt range.
    PanTiltMode,
    /// Selects the fixture’s position mode.
    PositionModes,
    /// Changes control between selecting, indexing, and rotating the gobos of gobo wheel (`n`).
    GoboWheelMode { n: u32 },
    /// Defines whether the gobo wheel takes the shortest distance between two positions.
    GoboWheelShortcutMode,
    /// Changes control between selecting, indexing, and rotating the slots of animation wheel (`n`).
    AnimationWheelMode { n: u32 },
    /// Defines whether the animation wheel takes the shortest distance between two positions.
    AnimationWheelShortcutMode,
    /// Changes control between selecting, continuous selection, half selection, random selection, color spinning, etc. in colors of color wheel (`n`).
    ColorMode { n: u32 },
    /// Defines whether the color wheel takes the shortest distance between two colors.
    ColorWheelShortcutMode,
    /// Controls how Cyan is used within the fixture’s cyan CMY-mixing feature.
    CyanMode,
    /// Controls how Cyan is used within the fixture’s magenta CMY-mixing.
    MagentaMode,
    /// Controls how Cyan is used within the fixture’s yellow CMY-mixing feature.
    YellowMode,
    /// Changes control between selecting continuous selection, half selection, random selection, color spinning, etc. in color mixing.
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
    /// Changes strobe style - strobe, pulse, random strobe, etc. - of the shutter attribute.
    StrobeMode,
    /// Changes modes of the fixture´s zoom.
    ZoomMode,
    /// Changes modes of the fixture’s focus - manual or auto- focus.
    FocusMode,
    /// Changes modes of the fixture’s iris - linear, strobe, pulse.
    IrisMode,
    /// Controls fan (`n`) mode.
    FanMode { n: u32 },
    /// Selects follow spot control mode.
    FollowSpotMode,
    /// Changes mode to control either index or rotation of the beam effects.
    BeamEffectIndexRotateMode,
    /// Movement speed of the fixture’s intensity.
    IntensityMSpeed,
    /// Movement speed of the fixture’s pan/tilt.
    PositionMSpeed,
    /// Movement speed of the fixture’s ColorMix presets.
    ColorMixMSpeed,
    /// Movement speed of the fixture’s color wheel.
    ColorWheelSelectMSpeed,
    /// Movement speed of the fixture’s gobo wheel (`n`).
    GoboWheelMSpeed { n: u32 },
    /// Movement speed of the fixture’s iris.
    IrisMSpeed,
    /// Movement speed of the fixture’s prism wheel (`n`).
    PrismMSpeed { n: u32 },
    /// Movement speed of the fixture’s focus.
    FocusMSpeed,
    /// Movement speed of the fixture’s frost (`n`).
    FrostMSpeed { n: u32 },
    /// Movement speed of the fixture’s zoom.
    ZoomMSpeed,
    /// Movement speed of the fixture’s shapers.
    FrameMSpeed,
    /// General speed of fixture’s features.
    GlobalMSpeed,
    /// Movement speed of the fixture’s frost.
    ReflectorAdjust,
    /// Generally resets the entire fixture.
    FixtureGlobalReset,
    /// Resets the fixture’s dimmer.
    DimmerReset,
    /// Resets the fixture’s shutter.
    ShutterReset,
    /// Resets the fixture’s beam features.
    BeamReset,
    /// Resets the fixture’s color mixing system.
    ColorMixReset,
    /// Resets the fixture’s color wheel.
    ColorWheelReset,
    /// Resets the fixture’s focus.
    FocusReset,
    /// Resets the fixture’s shapers.
    FrameReset,
    /// Resets the fixture’s gobo wheel.
    GoboWheelReset,
    /// Resets the fixture’s intensity.
    IntensityReset,
    /// Resets the fixture’s iris.
    IrisReset,
    /// Resets the fixture’s pan/tilt.
    PositionReset,
    /// Resets the fixture’s pan.
    PanReset,
    /// Resets the fixture’s tilt.
    TiltReset,
    /// Resets the fixture’s zoom.
    ZoomReset,
    /// Resets the fixture’s CTB.
    CtbReset,
    /// Resets the fixture’s CTO.
    CtoReset,
    /// Resets the fixture’s CTC.
    CtcReset,
    /// Resets the fixture’s animation system features.
    AnimationSystemReset,
    /// Resets the fixture’s calibration.
    FixtureCalibrationReset,
    /// Generally controls features of the fixture.
    Function,
    /// Controls the fixture’s lamp on/lamp off feature.
    LampControl,
    /// Adjusts intensity of display
    DisplayIntensity,
    /// Selects DMX Input
    DmxInput,
    /// Ranges without a functionality.
    NoFeature,
    /// Fog or hazer‘s blower feature.
    Blower { n: u32 },
    /// Fog or hazer’s Fan feature.
    Fan { n: u32 },
    /// Fog or hazer’s Fog feature.
    Fog { n: u32 },
    /// Fog or hazer’s Haze feature.
    Haze { n: u32 },
    /// Controls the energy consumption of the lamp.
    LampPowerMode,
    /// Controls a fixture or device fan.
    Fans,
    /// 1 of 2 shutters that shape the top/right/bottom/left of the beam.
    BladeA { n: u32 },
    /// 2 of 2 shutters that shape the top/right/bottom/left of the beam.
    BladeB { n: u32 },
    /// Rotates position of blade(`n`).
    BladeRot { n: u32 },
    /// Rotates position of blade assembly.
    ShaperRot,
    /// Predefined presets for shaper positions.
    ShaperMacros,
    /// Speed of predefined effects on shapers.
    ShaperMacrosSpeed,
    /// 1 of 2 soft edge blades that shape the top/right/bottom/left of the beam.
    BladeSoftA { n: u32 },
    /// 2 of 2 soft edge blades that shape the top/right/bottom/left of the beam.
    BladeSoftB { n: u32 },
    /// 1 of 2 corners that shape the top/right/bottom/left of the beam.
    KeyStoneA { n: u32 },
    /// 2 of 2 corners that shape the top/right/bottom/left of the beam.
    KeyStoneB { n: u32 },
    /// Controls video features.
    Video,
    /// Selects dedicated effects which are used for media.
    VideoEffectType { n: u32 },
    /// Controls parameter (`m`) of VideoEffect(`n`)Type.
    VideoEffectParameter { n: u32, m: u32 },
    /// Selects the video camera(`n`).
    VideoCamera { n: u32 },
    /// Adjusts sound volume.
    VideoSoundVolume { n: u32 },
    /// Defines mode of video blending.
    VideoBlendMode,
    /// Defines media input source e.g. a camera input.
    InputSource,
    /// Defines field of view.
    FieldOfView,
}

impl std::fmt::Display for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            Self::ScaleXyz => write!(f, "Scale_XYZ"),
            Self::Gobo { n } => write!(f, "Gobo{n}"),
            Self::GoboSelectSpin { n } => write!(f, "Gobo{n}SelectSpin"),
            Self::GoboSelectShake { n } => write!(f, "Gobo{n}SelectShake"),
            Self::GoboSelectEffects { n } => write!(f, "Gobo{n}SelectEffects"),
            Self::GoboWheelIndex { n } => write!(f, "Gobo{n}WheelIndex"),
            Self::GoboWheelSpin { n } => write!(f, "Gobo{n}WheelSpin"),
            Self::GoboWheelShake { n } => write!(f, "Gobo{n}WheelShake"),
            Self::GoboWheelRandom { n } => write!(f, "Gobo{n}WheelRandom"),
            Self::GoboWheelAudio { n } => write!(f, "Gobo{n}WheelAudio"),
            Self::GoboPos { n } => write!(f, "Gobo{n}Pos"),
            Self::GoboPosRotate { n } => write!(f, "Gobo{n}PosRotate"),
            Self::GoboPosShake { n } => write!(f, "Gobo{n}PosShake"),
            Self::AnimationWheel { n } => write!(f, "AnimationWheel{n}"),
            Self::AnimationWheelAudio { n } => write!(f, "AnimationWheel{n}Audio"),
            Self::AnimationWheelMacro { n } => write!(f, "AnimationWheel{n}Macro"),
            Self::AnimationWheelRandom { n } => write!(f, "AnimationWheel{n}Random"),
            Self::AnimationWheelSelectEffects { n } => write!(f, "AnimationWheel{n}SelectEffects"),
            Self::AnimationWheelSelectShake { n } => write!(f, "AnimationWheel{n}SelectShake"),
            Self::AnimationWheelSelectSpin { n } => write!(f, "AnimationWheel{n}SelectSpin"),
            Self::AnimationWheelPos { n } => write!(f, "AnimationWheel{n}Pos"),
            Self::AnimationWheelPosRotate { n } => write!(f, "AnimationWheel{n}PosRotate"),
            Self::AnimationWheelPosShake { n } => write!(f, "AnimationWheel{n}PosShake"),
            Self::AnimationSystem { n } => write!(f, "AnimationSystem{n}"),
            Self::AnimationSystemRamp { n } => write!(f, "AnimationSystem{n}Ramp"),
            Self::AnimationSystemShake { n } => write!(f, "AnimationSystem{n}Shake"),
            Self::AnimationSystemAudio { n } => write!(f, "AnimationSystem{n}Audio"),
            Self::AnimationSystemRandom { n } => write!(f, "AnimationSystem{n}Random"),
            Self::AnimationSystemPos { n } => write!(f, "AnimationSystem{n}Pos"),
            Self::AnimationSystemPosRotate { n } => write!(f, "AnimationSystem{n}PosRotate"),
            Self::AnimationSystemPosShake { n } => write!(f, "AnimationSystem{n}PosShake"),
            Self::AnimationSystemPosRandom { n } => write!(f, "AnimationSystem{n}PosRandom"),
            Self::AnimationSystemPosAudio { n } => write!(f, "AnimationSystem{n}PosAudio"),
            Self::AnimationSystemMacro { n } => write!(f, "AnimationSystem{n}Macro"),
            Self::MediaFolder { n } => write!(f, "MediaFolder{n}"),
            Self::MediaContent { n } => write!(f, "MediaContent{n}"),
            Self::ModelFolder { n } => write!(f, "ModelFolder{n}"),
            Self::ModelContent { n } => write!(f, "ModelContent{n}"),
            Self::PlayMode => write!(f, "PlayMode"),
            Self::PlayBegin => write!(f, "PlayBegin"),
            Self::PlayEnd => write!(f, "PlayEnd"),
            Self::PlaySpeed => write!(f, "PlaySpeed"),
            Self::ColorEffects { n } => write!(f, "ColorEffects{n}"),
            Self::Color { n } => write!(f, "Color{n}"),
            Self::ColorWheelIndex { n } => write!(f, "Color{n}WheelIndex"),
            Self::ColorWheelSpin { n } => write!(f, "Color{n}WheelSpin"),
            Self::ColorWheelRandom { n } => write!(f, "Color{n}WheelRandom"),
            Self::ColorWheelAudio { n } => write!(f, "Color{n}WheelAudio"),
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
            Self::ColorMacro { n } => write!(f, "ColorMacro{n}"),
            Self::ColorMacroRate { n } => write!(f, "ColorMacro{n}Rate"),
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
            Self::Shutter { n } => write!(f, "Shutter{n}"),
            Self::ShutterStrobe { n } => write!(f, "Shutter{n}Strobe"),
            Self::ShutterStrobePulse { n } => write!(f, "Shutter{n}StrobePulse"),
            Self::ShutterStrobePulseClose { n } => write!(f, "Shutter{n}StrobePulseClose"),
            Self::ShutterStrobePulseOpen { n } => write!(f, "Shutter{n}StrobePulseOpen"),
            Self::ShutterStrobeRandom { n } => write!(f, "Shutter{n}StrobeRandom"),
            Self::ShutterStrobeRandomPulse { n } => write!(f, "Shutter{n}StrobeRandomPulse"),
            Self::ShutterStrobeRandomPulseClose { n } => {
                write!(f, "Shutter{n}StrobeRandomPulseClose")
            }
            Self::ShutterStrobeRandomPulseOpen { n } => {
                write!(f, "Shutter{n}StrobeRandomPulseOpen")
            }
            Self::ShutterStrobeEffect { n } => write!(f, "Shutter{n}StrobeEffect"),
            Self::Iris => write!(f, "Iris"),
            Self::IrisStrobe => write!(f, "IrisStrobe"),
            Self::IrisStrobeRandom => write!(f, "IrisStrobeRandom"),
            Self::IrisPulseClose => write!(f, "IrisPulseClose"),
            Self::IrisPulseOpen => write!(f, "IrisPulseOpen"),
            Self::IrisRandomPulseClose => write!(f, "IrisRandomPulseClose"),
            Self::IrisRandomPulseOpen => write!(f, "IrisRandomPulseOpen"),
            Self::Frost { n } => write!(f, "Frost{n}"),
            Self::FrostPulseOpen { n } => write!(f, "Frost{n}PulseOpen"),
            Self::FrostPulseClose { n } => write!(f, "Frost{n}PulseClose"),
            Self::FrostRamp { n } => write!(f, "Frost{n}Ramp"),
            Self::Prism { n } => write!(f, "Prism{n}"),
            Self::PrismSelectSpin { n } => write!(f, "Prism{n}SelectSpin"),
            Self::PrismMacro { n } => write!(f, "Prism{n}Macro"),
            Self::PrismPos { n } => write!(f, "Prism{n}Pos"),
            Self::PrismPosRotate { n } => write!(f, "Prism{n}PosRotate"),
            Self::Effects { n } => write!(f, "Effects{n}"),
            Self::EffectsRate { n } => write!(f, "Effects{n}Rate"),
            Self::EffectsFade { n } => write!(f, "Effects{n}Fade"),
            Self::EffectsAdjust { n, m } => write!(f, "Effects{n}Adjust{m}"),
            Self::EffectsPos { n } => write!(f, "Effects{n}Pos"),
            Self::EffectsPosRotate { n } => write!(f, "Effects{n}PosRotate"),
            Self::EffectsSync => write!(f, "EffectsSync"),
            Self::BeamShaper => write!(f, "BeamShaper"),
            Self::BeamShaperMacro => write!(f, "BeamShaperMacro"),
            Self::BeamShaperPos => write!(f, "BeamShaperPos"),
            Self::BeamShaperPosRotate => write!(f, "BeamShaperPosRotate"),
            Self::Zoom => write!(f, "Zoom"),
            Self::ZoomModeSpot => write!(f, "ZoomModeSpot"),
            Self::ZoomModeBeam => write!(f, "ZoomModeBeam"),
            Self::DigitalZoom => write!(f, "DigitalZoom"),
            Self::Focus { n } => write!(f, "Focus{n}"),
            Self::FocusAdjust { n } => write!(f, "Focus{n}Adjust"),
            Self::FocusDistance { n } => write!(f, "Focus{n}Distance"),
            Self::Control { n } => write!(f, "Control{n}"),
            Self::DimmerMode => write!(f, "DimmerMode"),
            Self::DimmerCurve => write!(f, "DimmerCurve"),
            Self::BlackoutMode => write!(f, "BlackoutMode"),
            Self::LedFrequency => write!(f, "LEDFrequency"),
            Self::LedZoneMode => write!(f, "LEDZoneMode"),
            Self::PixelMode => write!(f, "PixelMode"),
            Self::PanMode => write!(f, "PanMode"),
            Self::TiltMode => write!(f, "TiltMode"),
            Self::PanTiltMode => write!(f, "PanTiltMode"),
            Self::PositionModes => write!(f, "PositionModes"),
            Self::GoboWheelMode { n } => write!(f, "Gobo{n}WheelMode"),
            Self::GoboWheelShortcutMode => write!(f, "GoboWheelShortcutMode"),
            Self::AnimationWheelMode { n } => write!(f, "AnimationWheel{n}Mode"),
            Self::AnimationWheelShortcutMode => write!(f, "AnimationWheelShortcutMode"),
            Self::ColorMode { n } => write!(f, "Color{n}Mode"),
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
            Self::CriMode => write!(f, "CRIMode"),
            Self::CustomColor => write!(f, "CustomColor"),
            Self::UvStability => write!(f, "UVStability"),
            Self::WavelengthCorrection => write!(f, "WavelengthCorrection"),
            Self::WhiteCount => write!(f, "WhiteCount"),
            Self::StrobeMode => write!(f, "StrobeMode"),
            Self::ZoomMode => write!(f, "ZoomMode"),
            Self::FocusMode => write!(f, "FocusMode"),
            Self::IrisMode => write!(f, "IrisMode"),
            Self::FanMode { n } => write!(f, "Fan{n}Mode"),
            Self::FollowSpotMode => write!(f, "FollowSpotMode"),
            Self::BeamEffectIndexRotateMode => write!(f, "BeamEffectIndexRotateMode"),
            Self::IntensityMSpeed => write!(f, "IntensityMSpeed"),
            Self::PositionMSpeed => write!(f, "PositionMSpeed"),
            Self::ColorMixMSpeed => write!(f, "ColorMixMSpeed"),
            Self::ColorWheelSelectMSpeed => write!(f, "ColorWheelSelectMSpeed"),
            Self::GoboWheelMSpeed { n } => write!(f, "GoboWheel{n}MSpeed"),
            Self::IrisMSpeed => write!(f, "IrisMSpeed"),
            Self::PrismMSpeed { n } => write!(f, "Prism{n}MSpeed"),
            Self::FocusMSpeed => write!(f, "FocusMSpeed"),
            Self::FrostMSpeed { n } => write!(f, "Frost{n}MSpeed"),
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
            Self::Blower { n } => write!(f, "Blower{n}"),
            Self::Fan { n } => write!(f, "Fan{n}"),
            Self::Fog { n } => write!(f, "Fog{n}"),
            Self::Haze { n } => write!(f, "Haze{n}"),
            Self::LampPowerMode => write!(f, "LampPowerMode"),
            Self::Fans => write!(f, "Fans"),
            Self::BladeA { n } => write!(f, "Blade{n}A"),
            Self::BladeB { n } => write!(f, "Blade{n}B"),
            Self::BladeRot { n } => write!(f, "Blade{n}Rot"),
            Self::ShaperRot => write!(f, "ShaperRot"),
            Self::ShaperMacros => write!(f, "ShaperMacros"),
            Self::ShaperMacrosSpeed => write!(f, "ShaperMacrosSpeed"),
            Self::BladeSoftA { n } => write!(f, "BladeSoft{n}A"),
            Self::BladeSoftB { n } => write!(f, "BladeSoft{n}B"),
            Self::KeyStoneA { n } => write!(f, "KeyStone{n}A"),
            Self::KeyStoneB { n } => write!(f, "KeyStone{n}B"),
            Self::Video => write!(f, "Video"),
            Self::VideoEffectType { n } => write!(f, "VideoEffect{n}Type"),
            Self::VideoEffectParameter { n, m } => write!(f, "VideoEffect{n}Parameter{m}"),
            Self::VideoCamera { n } => write!(f, "VideoCamera{n}"),
            Self::VideoSoundVolume { n } => write!(f, "VideoSoundVolume{n}"),
            Self::VideoBlendMode => write!(f, "VideoBlendMode"),
            Self::InputSource => write!(f, "InputSource"),
            Self::FieldOfView => write!(f, "FieldOfView"),
        }
    }
}

impl std::str::FromStr for SharedString {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"([a-zA-Z_]+|\d+)").unwrap();
        let split: Vec<&str> = re.find_iter(s).map(|m| m.as_str()).collect();
        let first = split.get(0).ok_or(anyhow::anyhow!("No first element"))?;
        match *first {
            "Dimmer" => Ok(Self::Dimmer),
            "Pan" => Ok(Self::Pan),
            "Tilt" => Ok(Self::Tilt),
            "PanRotate" => Ok(Self::PanRotate),
            "TiltRotate" => Ok(Self::TiltRotate),
            "PositionEffect" => Ok(Self::PositionEffect),
            "PositionEffectRate" => Ok(Self::PositionEffectRate),
            "PositionEffectFade" => Ok(Self::PositionEffectFade),
            "XYZ_X" => Ok(Self::XyzX),
            "XYZ_Y" => Ok(Self::XyzY),
            "XYZ_Z" => Ok(Self::XyzZ),
            "Rot_X" => Ok(Self::RotX),
            "Rot_Y" => Ok(Self::RotY),
            "Rot_Z" => Ok(Self::RotZ),
            "Scale_X" => Ok(Self::ScaleX),
            "Scale_Y" => Ok(Self::ScaleY),
            "Scale_Z" => Ok(Self::ScaleZ),
            "Scale_XYZ" => Ok(Self::ScaleXyz),
            "Gobo" => match split.len() {
                2 => Ok(Self::Gobo {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "SelectSpin" => Ok(Self::GoboSelectSpin { n }),
                        "SelectShake" => Ok(Self::GoboSelectShake { n }),
                        "SelectEffects" => Ok(Self::GoboSelectEffects { n }),
                        "WheelIndex" => Ok(Self::GoboWheelIndex { n }),
                        "WheelSpin" => Ok(Self::GoboWheelSpin { n }),
                        "WheelShake" => Ok(Self::GoboWheelShake { n }),
                        "WheelRandom" => Ok(Self::GoboWheelRandom { n }),
                        "WheelAudio" => Ok(Self::GoboWheelAudio { n }),
                        "Pos" => Ok(Self::GoboPos { n }),
                        "PosRotate" => Ok(Self::GoboPosRotate { n }),
                        "PosShake" => Ok(Self::GoboPosShake { n }),
                        "WheelMode" => Ok(Self::GoboWheelMode { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "AnimationWheel" => match split.len() {
                2 => Ok(Self::AnimationWheel {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "Audio" => Ok(Self::AnimationWheelAudio { n }),
                        "Macro" => Ok(Self::AnimationWheelMacro { n }),
                        "Random" => Ok(Self::AnimationWheelRandom { n }),
                        "SelectEffects" => Ok(Self::AnimationWheelSelectEffects { n }),
                        "SelectShake" => Ok(Self::AnimationWheelSelectShake { n }),
                        "SelectSpin" => Ok(Self::AnimationWheelSelectSpin { n }),
                        "Pos" => Ok(Self::AnimationWheelPos { n }),
                        "PosRotate" => Ok(Self::AnimationWheelPosRotate { n }),
                        "PosShake" => Ok(Self::AnimationWheelPosShake { n }),
                        "Mode" => Ok(Self::AnimationWheelMode { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "AnimationSystem" => match split.len() {
                2 => Ok(Self::AnimationSystem {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "Ramp" => Ok(Self::AnimationSystemRamp { n }),
                        "Shake" => Ok(Self::AnimationSystemShake { n }),
                        "Audio" => Ok(Self::AnimationSystemAudio { n }),
                        "Random" => Ok(Self::AnimationSystemRandom { n }),
                        "Pos" => Ok(Self::AnimationSystemPos { n }),
                        "PosRotate" => Ok(Self::AnimationSystemPosRotate { n }),
                        "PosShake" => Ok(Self::AnimationSystemPosShake { n }),
                        "PosRandom" => Ok(Self::AnimationSystemPosRandom { n }),
                        "PosAudio" => Ok(Self::AnimationSystemPosAudio { n }),
                        "Macro" => Ok(Self::AnimationSystemMacro { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },

            "MediaFolder" => match split.len() {
                2 => Ok(Self::MediaFolder {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "MediaContent" => match split.len() {
                2 => Ok(Self::MediaContent {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "ModelFolder" => match split.len() {
                2 => Ok(Self::ModelFolder {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "ModelContent" => match split.len() {
                2 => Ok(Self::ModelContent {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "PlayMode" => Ok(Self::PlayMode),
            "PlayBegin" => Ok(Self::PlayBegin),
            "PlayEnd" => Ok(Self::PlayEnd),
            "PlaySpeed" => Ok(Self::PlaySpeed),
            "ColorEffects" => match split.len() {
                2 => Ok(Self::ColorEffects {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "Color" => match split.len() {
                2 => Ok(Self::Color {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "WheelIndex" => Ok(Self::ColorWheelIndex { n }),
                        "WheelSpin" => Ok(Self::ColorWheelSpin { n }),
                        "WheelRandom" => Ok(Self::ColorWheelRandom { n }),
                        "WheelAudio" => Ok(Self::ColorWheelAudio { n }),
                        "Mode" => Ok(Self::ColorMode { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "ColorAdd_R" => Ok(Self::ColorAddR),
            "ColorAdd_G" => Ok(Self::ColorAddG),
            "ColorAdd_B" => Ok(Self::ColorAddB),
            "ColorAdd_C" => Ok(Self::ColorAddC),
            "ColorAdd_M" => Ok(Self::ColorAddM),
            "ColorAdd_Y" => Ok(Self::ColorAddY),
            "ColorAdd_RY" => Ok(Self::ColorAddRY),
            "ColorAdd_GY" => Ok(Self::ColorAddGY),
            "ColorAdd_GC" => Ok(Self::ColorAddGC),
            "ColorAdd_BC" => Ok(Self::ColorAddBC),
            "ColorAdd_BM" => Ok(Self::ColorAddBM),
            "ColorAdd_RM" => Ok(Self::ColorAddRM),
            "ColorAdd_W" => Ok(Self::ColorAddW),
            "ColorAdd_WW" => Ok(Self::ColorAddWW),
            "ColorAdd_CW" => Ok(Self::ColorAddCW),
            "ColorAdd_UV" => Ok(Self::ColorAddUV),
            "ColorSub_R" => Ok(Self::ColorSubR),
            "ColorSub_G" => Ok(Self::ColorSubG),
            "ColorSub_B" => Ok(Self::ColorSubB),
            "ColorSub_C" => Ok(Self::ColorSubC),
            "ColorSub_M" => Ok(Self::ColorSubM),
            "ColorSub_Y" => Ok(Self::ColorSubY),
            "ColorMacro" => match split.len() {
                2 => Ok(Self::ColorMacro {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "Rate" => Ok(Self::ColorMacroRate { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "CTO" => Ok(Self::Cto),
            "CTC" => Ok(Self::Ctc),
            "CTB" => Ok(Self::Ctb),
            "Tint" => Ok(Self::Tint),
            "HSB_Hue" => Ok(Self::HsbHue),
            "HSB_Saturation" => Ok(Self::HsbSaturation),
            "HSB_Brightness" => Ok(Self::HsbBrightness),
            "HSB_Quality" => Ok(Self::HsbQuality),
            "CIE_X" => Ok(Self::CieX),
            "CIE_Y" => Ok(Self::CieY),
            "CIE_Brightness" => Ok(Self::CieBrightness),
            "ColorRGB_Red" => Ok(Self::ColorRgbRed),
            "ColorRGB_Green" => Ok(Self::ColorRgbGreen),
            "ColorRGB_Blue" => Ok(Self::ColorRgbBlue),
            "ColorRGB_Cyan" => Ok(Self::ColorRgbCyan),
            "ColorRGB_Magenta" => Ok(Self::ColorRgbMagenta),
            "ColorRGB_Yellow" => Ok(Self::ColorRgbYellow),
            "ColorRGB_Quality" => Ok(Self::ColorRgbQuality),
            "VideoBoost_R" => Ok(Self::VideoBoostR),
            "VideoBoost_G" => Ok(Self::VideoBoostG),
            "VideoBoost_B" => Ok(Self::VideoBoostB),
            "VideoHueShift" => Ok(Self::VideoHueShift),
            "VideoSaturation" => Ok(Self::VideoSaturation),
            "VideoBrightness" => Ok(Self::VideoBrightness),
            "VideoContrast" => Ok(Self::VideoContrast),
            "VideoKeyColor_R" => Ok(Self::VideoKeyColorR),
            "VideoKeyColor_G" => Ok(Self::VideoKeyColorG),
            "VideoKeyColor_B" => Ok(Self::VideoKeyColorB),
            "VideoKeyIntensity" => Ok(Self::VideoKeyIntensity),
            "VideoKeyTolerance" => Ok(Self::VideoKeyTolerance),
            "StrobeDuration" => Ok(Self::StrobeDuration),
            "StrobeRate" => Ok(Self::StrobeRate),
            "StrobeFrequency" => Ok(Self::StrobeFrequency),
            "StrobeModeShutter" => Ok(Self::StrobeModeShutter),
            "StrobeModeStrobe" => Ok(Self::StrobeModeStrobe),
            "StrobeModePulse" => Ok(Self::StrobeModePulse),
            "StrobeModePulseOpen" => Ok(Self::StrobeModePulseOpen),
            "StrobeModePulseClose" => Ok(Self::StrobeModePulseClose),
            "StrobeModeRandom" => Ok(Self::StrobeModeRandom),
            "StrobeModeRandomPulse" => Ok(Self::StrobeModeRandomPulse),
            "StrobeModeRandomPulseOpen" => Ok(Self::StrobeModeRandomPulseOpen),
            "StrobeModeRandomPulseClose" => Ok(Self::StrobeModeRandomPulseClose),
            "StrobeModeEffect" => Ok(Self::StrobeModeEffect),
            "Shutter" => match split.len() {
                2 => Ok(Self::Shutter {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "Strobe" => Ok(Self::ShutterStrobe { n }),
                        "StrobePulse" => Ok(Self::ShutterStrobePulse { n }),
                        "StrobePulseClose" => Ok(Self::ShutterStrobePulseClose { n }),
                        "StrobePulseOpen" => Ok(Self::ShutterStrobePulseOpen { n }),
                        "StrobeRandom" => Ok(Self::ShutterStrobeRandom { n }),
                        "StrobeRandomPulse" => Ok(Self::ShutterStrobeRandomPulse { n }),
                        "StrobeRandomPulseClose" => Ok(Self::ShutterStrobeRandomPulseClose { n }),
                        "StrobeRandomPulseOpen" => Ok(Self::ShutterStrobeRandomPulseOpen { n }),
                        "StrobeEffect" => Ok(Self::ShutterStrobeEffect { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "Iris" => Ok(Self::Iris),
            "IrisStrobe" => Ok(Self::IrisStrobe),
            "IrisStrobeRandom" => Ok(Self::IrisStrobeRandom),
            "IrisPulseClose" => Ok(Self::IrisPulseClose),
            "IrisPulseOpen" => Ok(Self::IrisPulseOpen),
            "IrisRandomPulseClose" => Ok(Self::IrisRandomPulseClose),
            "IrisRandomPulseOpen" => Ok(Self::IrisRandomPulseOpen),
            "Frost" => match split.len() {
                2 => Ok(Self::Frost {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "PulseOpen" => Ok(Self::FrostPulseOpen { n }),
                        "PulseClose" => Ok(Self::FrostPulseClose { n }),
                        "Ramp" => Ok(Self::FrostRamp { n }),
                        "MSpeed" => Ok(Self::FrostMSpeed { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "Prism" => match split.len() {
                2 => Ok(Self::Prism {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "SelectSpin" => Ok(Self::PrismSelectSpin { n }),
                        "Macro" => Ok(Self::PrismMacro { n }),
                        "Pos" => Ok(Self::PrismPos { n }),
                        "PosRotate" => Ok(Self::PrismPosRotate { n }),
                        "MSpeed" => Ok(Self::PrismMSpeed { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "Effects" => match split.len() {
                2 => Ok(Self::Effects {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "Rate" => Ok(Self::EffectsRate { n }),
                        "Fade" => Ok(Self::EffectsFade { n }),
                        "Pos" => Ok(Self::EffectsPos { n }),
                        "PosRotate" => Ok(Self::EffectsPosRotate { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                4 => {
                    let n = split[1].parse()?;
                    let m = split[3].parse()?;
                    match split[2] {
                        "Adjust" => Ok(Self::EffectsAdjust { n, m }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "EffectsSync" => Ok(Self::EffectsSync),
            "BeamShaper" => Ok(Self::BeamShaper),
            "BeamShaperMacro" => Ok(Self::BeamShaperMacro),
            "BeamShaperPos" => Ok(Self::BeamShaperPos),
            "BeamShaperPosRotate" => Ok(Self::BeamShaperPosRotate),
            "Zoom" => Ok(Self::Zoom),
            "ZoomModeSpot" => Ok(Self::ZoomModeSpot),
            "ZoomModeBeam" => Ok(Self::ZoomModeBeam),
            "DigitalZoom" => Ok(Self::DigitalZoom),
            "Focus" => match split.len() {
                2 => Ok(Self::Focus {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "Adjust" => Ok(Self::FocusAdjust { n }),
                        "Distance" => Ok(Self::FocusDistance { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "Control" => match split.len() {
                2 => Ok(Self::Control {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "DimmerMode" => Ok(Self::DimmerMode),
            "DimmerCurve" => Ok(Self::DimmerCurve),
            "BlackoutMode" => Ok(Self::BlackoutMode),
            "LEDFrequency" => Ok(Self::LedFrequency),
            "LEDZoneMode" => Ok(Self::LedZoneMode),
            "PixelMode" => Ok(Self::PixelMode),
            "PanMode" => Ok(Self::PanMode),
            "TiltMode" => Ok(Self::TiltMode),
            "PanTiltMode" => Ok(Self::PanTiltMode),
            "PositionModes" => Ok(Self::PositionModes),
            "GoboWheelShortcutMode" => Ok(Self::GoboWheelShortcutMode),
            "AnimationWheelShortcutMode" => Ok(Self::AnimationWheelShortcutMode),
            "ColorWheelShortcutMode" => Ok(Self::ColorWheelShortcutMode),
            "CyanMode" => Ok(Self::CyanMode),
            "MagentaMode" => Ok(Self::MagentaMode),
            "YellowMode" => Ok(Self::YellowMode),
            "ColorMixMode" => Ok(Self::ColorMixMode),
            "ChromaticMode" => Ok(Self::ChromaticMode),
            "ColorCalibrationMode" => Ok(Self::ColorCalibrationMode),
            "ColorConsistency" => Ok(Self::ColorConsistency),
            "ColorControl" => Ok(Self::ColorControl),
            "ColorModelMode" => Ok(Self::ColorModelMode),
            "ColorSettingsReset" => Ok(Self::ColorSettingsReset),
            "ColorUniformity" => Ok(Self::ColorUniformity),
            "CRIMode" => Ok(Self::CriMode),
            "CustomColor" => Ok(Self::CustomColor),
            "UVStability" => Ok(Self::UvStability),
            "WavelengthCorrection" => Ok(Self::WavelengthCorrection),
            "WhiteCount" => Ok(Self::WhiteCount),
            "StrobeMode" => Ok(Self::StrobeMode),
            "ZoomMode" => Ok(Self::ZoomMode),
            "FocusMode" => Ok(Self::FocusMode),
            "IrisMode" => Ok(Self::IrisMode),
            "Fan" => match split.len() {
                2 => Ok(Self::Fan {
                    n: split[1].parse()?,
                }),
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "Mode" => Ok(Self::FanMode { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "FollowSpotMode" => Ok(Self::FollowSpotMode),
            "BeamEffectIndexRotateMode" => Ok(Self::BeamEffectIndexRotateMode),
            "IntensityMSpeed" => Ok(Self::IntensityMSpeed),
            "PositionMSpeed" => Ok(Self::PositionMSpeed),
            "ColorMixMSpeed" => Ok(Self::ColorMixMSpeed),
            "ColorWheelSelectMSpeed" => Ok(Self::ColorWheelSelectMSpeed),
            "GoboWheel" => match split.len() {
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "MSpeed" => Ok(Self::GoboWheelMSpeed { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "IrisMSpeed" => Ok(Self::IrisMSpeed),
            "FocusMSpeed" => Ok(Self::FocusMSpeed),
            "ZoomMSpeed" => Ok(Self::ZoomMSpeed),
            "FrameMSpeed" => Ok(Self::FrameMSpeed),
            "GlobalMSpeed" => Ok(Self::GlobalMSpeed),
            "ReflectorAdjust" => Ok(Self::ReflectorAdjust),
            "FixtureGlobalReset" => Ok(Self::FixtureGlobalReset),
            "DimmerReset" => Ok(Self::DimmerReset),
            "ShutterReset" => Ok(Self::ShutterReset),
            "BeamReset" => Ok(Self::BeamReset),
            "ColorMixReset" => Ok(Self::ColorMixReset),
            "ColorWheelReset" => Ok(Self::ColorWheelReset),
            "FocusReset" => Ok(Self::FocusReset),
            "FrameReset" => Ok(Self::FrameReset),
            "GoboWheelReset" => Ok(Self::GoboWheelReset),
            "IntensityReset" => Ok(Self::IntensityReset),
            "IrisReset" => Ok(Self::IrisReset),
            "PositionReset" => Ok(Self::PositionReset),
            "PanReset" => Ok(Self::PanReset),
            "TiltReset" => Ok(Self::TiltReset),
            "ZoomReset" => Ok(Self::ZoomReset),
            "CTBReset" => Ok(Self::CtbReset),
            "CTOReset" => Ok(Self::CtoReset),
            "CTCReset" => Ok(Self::CtcReset),
            "AnimationSystemReset" => Ok(Self::AnimationSystemReset),
            "FixtureCalibrationReset" => Ok(Self::FixtureCalibrationReset),
            "Function" => Ok(Self::Function),
            "LampControl" => Ok(Self::LampControl),
            "DisplayIntensity" => Ok(Self::DisplayIntensity),
            "DMXInput" => Ok(Self::DmxInput),
            "NoFeature" => Ok(Self::NoFeature),
            "Blower" => match split.len() {
                2 => Ok(Self::Blower {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "Fog" => match split.len() {
                2 => Ok(Self::Fog {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "Haze" => match split.len() {
                2 => Ok(Self::Haze {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "LampPowerMode" => Ok(Self::LampPowerMode),
            "Fans" => Ok(Self::Fans),
            "Blade" => match split.len() {
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "A" => Ok(Self::BladeA { n }),
                        "B" => Ok(Self::BladeB { n }),
                        "Rot" => Ok(Self::BladeRot { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "ShaperRot" => Ok(Self::ShaperRot),
            "ShaperMacros" => Ok(Self::ShaperMacros),
            "ShaperMacrosSpeed" => Ok(Self::ShaperMacrosSpeed),
            "BladeSoft" => match split.len() {
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "A" => Ok(Self::BladeSoftA { n }),
                        "B" => Ok(Self::BladeSoftB { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "KeyStone" => match split.len() {
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "A" => Ok(Self::KeyStoneA { n }),
                        "B" => Ok(Self::KeyStoneB { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "Video" => Ok(Self::Video),
            "VideoEffect" => match split.len() {
                3 => {
                    let n = split[1].parse()?;
                    match split[2] {
                        "Type" => Ok(Self::VideoEffectType { n }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                4 => {
                    let n = split[1].parse()?;
                    let m = split[3].parse()?;
                    match split[2] {
                        "Parameter" => Ok(Self::VideoEffectParameter { n, m }),
                        _ => bail!("Invalid Attribute"),
                    }
                }
                _ => bail!("Invalid Attribute"),
            },
            "VideoCamera" => match split.len() {
                2 => Ok(Self::VideoCamera {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "VideoSoundVolume" => match split.len() {
                2 => Ok(Self::VideoSoundVolume {
                    n: split[1].parse()?,
                }),
                _ => bail!("Invalid Attribute"),
            },
            "VideoBlendMode" => Ok(Self::VideoBlendMode),
            "InputSource" => Ok(Self::InputSource),
            "FieldOfView" => Ok(Self::FieldOfView),
            _ => bail!("Invalid Attribute"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attr_def::SharedString;

    #[test]
    fn test_attribute_definition_from_str() {
        assert_eq!(
            "Dimmer".parse::<SharedString>().unwrap(),
            SharedString::Dimmer
        );
        assert_eq!("Pan".parse::<SharedString>().unwrap(), SharedString::Pan);
        assert_eq!("Tilt".parse::<SharedString>().unwrap(), SharedString::Tilt);
        assert_eq!(
            "PanRotate".parse::<SharedString>().unwrap(),
            SharedString::PanRotate
        );
        assert_eq!(
            "TiltRotate".parse::<SharedString>().unwrap(),
            SharedString::TiltRotate
        );
        assert_eq!(
            "PositionEffect".parse::<SharedString>().unwrap(),
            SharedString::PositionEffect
        );
        assert_eq!(
            "PositionEffectRate".parse::<SharedString>().unwrap(),
            SharedString::PositionEffectRate
        );
        assert_eq!(
            "PositionEffectFade".parse::<SharedString>().unwrap(),
            SharedString::PositionEffectFade
        );
        assert_eq!("XYZ_X".parse::<SharedString>().unwrap(), SharedString::XyzX);
        assert_eq!("XYZ_Y".parse::<SharedString>().unwrap(), SharedString::XyzY);
        assert_eq!("XYZ_Z".parse::<SharedString>().unwrap(), SharedString::XyzZ);
        assert_eq!("Rot_X".parse::<SharedString>().unwrap(), SharedString::RotX);
        assert_eq!("Rot_Y".parse::<SharedString>().unwrap(), SharedString::RotY);
        assert_eq!("Rot_Z".parse::<SharedString>().unwrap(), SharedString::RotZ);
        assert_eq!(
            "Scale_X".parse::<SharedString>().unwrap(),
            SharedString::ScaleX
        );
        assert_eq!(
            "Scale_Y".parse::<SharedString>().unwrap(),
            SharedString::ScaleY
        );
        assert_eq!(
            "Scale_Z".parse::<SharedString>().unwrap(),
            SharedString::ScaleZ
        );
        assert_eq!(
            "Scale_XYZ".parse::<SharedString>().unwrap(),
            SharedString::ScaleXyz
        );
        assert_eq!(
            "Gobo4".parse::<SharedString>().unwrap(),
            SharedString::Gobo { n: 4 }
        );
        assert_eq!(
            "Gobo4SelectSpin".parse::<SharedString>().unwrap(),
            SharedString::GoboSelectSpin { n: 4 }
        );
        assert_eq!(
            "Gobo4SelectShake".parse::<SharedString>().unwrap(),
            SharedString::GoboSelectShake { n: 4 }
        );
        assert_eq!(
            "Gobo4SelectEffects".parse::<SharedString>().unwrap(),
            SharedString::GoboSelectEffects { n: 4 }
        );
        assert_eq!(
            "Gobo4WheelIndex".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelIndex { n: 4 }
        );
        assert_eq!(
            "Gobo4WheelSpin".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelSpin { n: 4 }
        );
        assert_eq!(
            "Gobo4WheelShake".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelShake { n: 4 }
        );
        assert_eq!(
            "Gobo4WheelRandom".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelRandom { n: 4 }
        );
        assert_eq!(
            "Gobo4WheelAudio".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelAudio { n: 4 }
        );
        assert_eq!(
            "Gobo4Pos".parse::<SharedString>().unwrap(),
            SharedString::GoboPos { n: 4 }
        );
        assert_eq!(
            "Gobo4PosRotate".parse::<SharedString>().unwrap(),
            SharedString::GoboPosRotate { n: 4 }
        );
        assert_eq!(
            "Gobo4PosShake".parse::<SharedString>().unwrap(),
            SharedString::GoboPosShake { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheel { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4Audio".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheelAudio { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4Macro".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheelMacro { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4Random".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheelRandom { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4SelectEffects"
                .parse::<SharedString>()
                .unwrap(),
            SharedString::AnimationWheelSelectEffects { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4SelectShake"
                .parse::<SharedString>()
                .unwrap(),
            SharedString::AnimationWheelSelectShake { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4SelectSpin".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheelSelectSpin { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4Pos".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheelPos { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4PosRotate".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheelPosRotate { n: 4 }
        );
        assert_eq!(
            "AnimationWheel4PosShake".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheelPosShake { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystem { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4Ramp".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemRamp { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4Shake".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemShake { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4Audio".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemAudio { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4Random".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemRandom { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4Pos".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemPos { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4PosRotate".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemPosRotate { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4PosShake".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemPosShake { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4PosRandom".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemPosRandom { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4PosAudio".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemPosAudio { n: 4 }
        );
        assert_eq!(
            "AnimationSystem4Macro".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemMacro { n: 4 }
        );
        assert_eq!(
            "MediaFolder4".parse::<SharedString>().unwrap(),
            SharedString::MediaFolder { n: 4 }
        );
        assert_eq!(
            "MediaContent4".parse::<SharedString>().unwrap(),
            SharedString::MediaContent { n: 4 }
        );
        assert_eq!(
            "ModelFolder4".parse::<SharedString>().unwrap(),
            SharedString::ModelFolder { n: 4 }
        );
        assert_eq!(
            "ModelContent4".parse::<SharedString>().unwrap(),
            SharedString::ModelContent { n: 4 }
        );
        assert_eq!(
            "PlayMode".parse::<SharedString>().unwrap(),
            SharedString::PlayMode
        );
        assert_eq!(
            "PlayBegin".parse::<SharedString>().unwrap(),
            SharedString::PlayBegin
        );
        assert_eq!(
            "PlayEnd".parse::<SharedString>().unwrap(),
            SharedString::PlayEnd
        );
        assert_eq!(
            "PlaySpeed".parse::<SharedString>().unwrap(),
            SharedString::PlaySpeed
        );
        assert_eq!(
            "ColorEffects4".parse::<SharedString>().unwrap(),
            SharedString::ColorEffects { n: 4 }
        );
        assert_eq!(
            "Color4".parse::<SharedString>().unwrap(),
            SharedString::Color { n: 4 }
        );
        assert_eq!(
            "Color4WheelIndex".parse::<SharedString>().unwrap(),
            SharedString::ColorWheelIndex { n: 4 }
        );
        assert_eq!(
            "Color4WheelSpin".parse::<SharedString>().unwrap(),
            SharedString::ColorWheelSpin { n: 4 }
        );
        assert_eq!(
            "Color4WheelRandom".parse::<SharedString>().unwrap(),
            SharedString::ColorWheelRandom { n: 4 }
        );
        assert_eq!(
            "Color4WheelAudio".parse::<SharedString>().unwrap(),
            SharedString::ColorWheelAudio { n: 4 }
        );
        assert_eq!(
            "ColorAdd_R".parse::<SharedString>().unwrap(),
            SharedString::ColorAddR
        );
        assert_eq!(
            "ColorAdd_G".parse::<SharedString>().unwrap(),
            SharedString::ColorAddG
        );
        assert_eq!(
            "ColorAdd_B".parse::<SharedString>().unwrap(),
            SharedString::ColorAddB
        );
        assert_eq!(
            "ColorAdd_C".parse::<SharedString>().unwrap(),
            SharedString::ColorAddC
        );
        assert_eq!(
            "ColorAdd_M".parse::<SharedString>().unwrap(),
            SharedString::ColorAddM
        );
        assert_eq!(
            "ColorAdd_Y".parse::<SharedString>().unwrap(),
            SharedString::ColorAddY
        );
        assert_eq!(
            "ColorAdd_RY".parse::<SharedString>().unwrap(),
            SharedString::ColorAddRY
        );
        assert_eq!(
            "ColorAdd_GY".parse::<SharedString>().unwrap(),
            SharedString::ColorAddGY
        );
        assert_eq!(
            "ColorAdd_GC".parse::<SharedString>().unwrap(),
            SharedString::ColorAddGC
        );
        assert_eq!(
            "ColorAdd_BC".parse::<SharedString>().unwrap(),
            SharedString::ColorAddBC
        );
        assert_eq!(
            "ColorAdd_BM".parse::<SharedString>().unwrap(),
            SharedString::ColorAddBM
        );
        assert_eq!(
            "ColorAdd_RM".parse::<SharedString>().unwrap(),
            SharedString::ColorAddRM
        );
        assert_eq!(
            "ColorAdd_W".parse::<SharedString>().unwrap(),
            SharedString::ColorAddW
        );
        assert_eq!(
            "ColorAdd_WW".parse::<SharedString>().unwrap(),
            SharedString::ColorAddWW
        );
        assert_eq!(
            "ColorAdd_CW".parse::<SharedString>().unwrap(),
            SharedString::ColorAddCW
        );
        assert_eq!(
            "ColorAdd_UV".parse::<SharedString>().unwrap(),
            SharedString::ColorAddUV
        );
        assert_eq!(
            "ColorSub_R".parse::<SharedString>().unwrap(),
            SharedString::ColorSubR
        );
        assert_eq!(
            "ColorSub_G".parse::<SharedString>().unwrap(),
            SharedString::ColorSubG
        );
        assert_eq!(
            "ColorSub_B".parse::<SharedString>().unwrap(),
            SharedString::ColorSubB
        );
        assert_eq!(
            "ColorSub_C".parse::<SharedString>().unwrap(),
            SharedString::ColorSubC
        );
        assert_eq!(
            "ColorSub_M".parse::<SharedString>().unwrap(),
            SharedString::ColorSubM
        );
        assert_eq!(
            "ColorSub_Y".parse::<SharedString>().unwrap(),
            SharedString::ColorSubY
        );
        assert_eq!(
            "ColorMacro4".parse::<SharedString>().unwrap(),
            SharedString::ColorMacro { n: 4 }
        );
        assert_eq!(
            "ColorMacro4Rate".parse::<SharedString>().unwrap(),
            SharedString::ColorMacroRate { n: 4 }
        );
        assert_eq!("CTO".parse::<SharedString>().unwrap(), SharedString::Cto);
        assert_eq!("CTC".parse::<SharedString>().unwrap(), SharedString::Ctc);
        assert_eq!("CTB".parse::<SharedString>().unwrap(), SharedString::Ctb);
        assert_eq!("Tint".parse::<SharedString>().unwrap(), SharedString::Tint);
        assert_eq!(
            "HSB_Hue".parse::<SharedString>().unwrap(),
            SharedString::HsbHue
        );
        assert_eq!(
            "HSB_Saturation".parse::<SharedString>().unwrap(),
            SharedString::HsbSaturation
        );
        assert_eq!(
            "HSB_Brightness".parse::<SharedString>().unwrap(),
            SharedString::HsbBrightness
        );
        assert_eq!(
            "HSB_Quality".parse::<SharedString>().unwrap(),
            SharedString::HsbQuality
        );
        assert_eq!("CIE_X".parse::<SharedString>().unwrap(), SharedString::CieX);
        assert_eq!("CIE_Y".parse::<SharedString>().unwrap(), SharedString::CieY);
        assert_eq!(
            "CIE_Brightness".parse::<SharedString>().unwrap(),
            SharedString::CieBrightness
        );
        assert_eq!(
            "ColorRGB_Red".parse::<SharedString>().unwrap(),
            SharedString::ColorRgbRed
        );
        assert_eq!(
            "ColorRGB_Green".parse::<SharedString>().unwrap(),
            SharedString::ColorRgbGreen
        );
        assert_eq!(
            "ColorRGB_Blue".parse::<SharedString>().unwrap(),
            SharedString::ColorRgbBlue
        );
        assert_eq!(
            "ColorRGB_Cyan".parse::<SharedString>().unwrap(),
            SharedString::ColorRgbCyan
        );
        assert_eq!(
            "ColorRGB_Magenta".parse::<SharedString>().unwrap(),
            SharedString::ColorRgbMagenta
        );
        assert_eq!(
            "ColorRGB_Yellow".parse::<SharedString>().unwrap(),
            SharedString::ColorRgbYellow
        );
        assert_eq!(
            "ColorRGB_Quality".parse::<SharedString>().unwrap(),
            SharedString::ColorRgbQuality
        );
        assert_eq!(
            "VideoBoost_R".parse::<SharedString>().unwrap(),
            SharedString::VideoBoostR
        );
        assert_eq!(
            "VideoBoost_G".parse::<SharedString>().unwrap(),
            SharedString::VideoBoostG
        );
        assert_eq!(
            "VideoBoost_B".parse::<SharedString>().unwrap(),
            SharedString::VideoBoostB
        );
        assert_eq!(
            "VideoHueShift".parse::<SharedString>().unwrap(),
            SharedString::VideoHueShift
        );
        assert_eq!(
            "VideoSaturation".parse::<SharedString>().unwrap(),
            SharedString::VideoSaturation
        );
        assert_eq!(
            "VideoBrightness".parse::<SharedString>().unwrap(),
            SharedString::VideoBrightness
        );
        assert_eq!(
            "VideoContrast".parse::<SharedString>().unwrap(),
            SharedString::VideoContrast
        );
        assert_eq!(
            "VideoKeyColor_R".parse::<SharedString>().unwrap(),
            SharedString::VideoKeyColorR
        );
        assert_eq!(
            "VideoKeyColor_G".parse::<SharedString>().unwrap(),
            SharedString::VideoKeyColorG
        );
        assert_eq!(
            "VideoKeyColor_B".parse::<SharedString>().unwrap(),
            SharedString::VideoKeyColorB
        );
        assert_eq!(
            "VideoKeyIntensity".parse::<SharedString>().unwrap(),
            SharedString::VideoKeyIntensity
        );
        assert_eq!(
            "VideoKeyTolerance".parse::<SharedString>().unwrap(),
            SharedString::VideoKeyTolerance
        );
        assert_eq!(
            "StrobeDuration".parse::<SharedString>().unwrap(),
            SharedString::StrobeDuration
        );
        assert_eq!(
            "StrobeRate".parse::<SharedString>().unwrap(),
            SharedString::StrobeRate
        );
        assert_eq!(
            "StrobeFrequency".parse::<SharedString>().unwrap(),
            SharedString::StrobeFrequency
        );
        assert_eq!(
            "StrobeModeShutter".parse::<SharedString>().unwrap(),
            SharedString::StrobeModeShutter
        );
        assert_eq!(
            "StrobeModeStrobe".parse::<SharedString>().unwrap(),
            SharedString::StrobeModeStrobe
        );
        assert_eq!(
            "StrobeModePulse".parse::<SharedString>().unwrap(),
            SharedString::StrobeModePulse
        );
        assert_eq!(
            "StrobeModePulseOpen".parse::<SharedString>().unwrap(),
            SharedString::StrobeModePulseOpen
        );
        assert_eq!(
            "StrobeModePulseClose".parse::<SharedString>().unwrap(),
            SharedString::StrobeModePulseClose
        );
        assert_eq!(
            "StrobeModeRandom".parse::<SharedString>().unwrap(),
            SharedString::StrobeModeRandom
        );
        assert_eq!(
            "StrobeModeRandomPulse".parse::<SharedString>().unwrap(),
            SharedString::StrobeModeRandomPulse
        );
        assert_eq!(
            "StrobeModeRandomPulseOpen".parse::<SharedString>().unwrap(),
            SharedString::StrobeModeRandomPulseOpen
        );
        assert_eq!(
            "StrobeModeRandomPulseClose"
                .parse::<SharedString>()
                .unwrap(),
            SharedString::StrobeModeRandomPulseClose
        );
        assert_eq!(
            "StrobeModeEffect".parse::<SharedString>().unwrap(),
            SharedString::StrobeModeEffect
        );
        assert_eq!(
            "Shutter4".parse::<SharedString>().unwrap(),
            SharedString::Shutter { n: 4 }
        );
        assert_eq!(
            "Shutter4Strobe".parse::<SharedString>().unwrap(),
            SharedString::ShutterStrobe { n: 4 }
        );
        assert_eq!(
            "Shutter4StrobePulse".parse::<SharedString>().unwrap(),
            SharedString::ShutterStrobePulse { n: 4 }
        );
        assert_eq!(
            "Shutter4StrobePulseClose".parse::<SharedString>().unwrap(),
            SharedString::ShutterStrobePulseClose { n: 4 }
        );
        assert_eq!(
            "Shutter4StrobePulseOpen".parse::<SharedString>().unwrap(),
            SharedString::ShutterStrobePulseOpen { n: 4 }
        );
        assert_eq!(
            "Shutter4StrobeRandom".parse::<SharedString>().unwrap(),
            SharedString::ShutterStrobeRandom { n: 4 }
        );
        assert_eq!(
            "Shutter4StrobeRandomPulse".parse::<SharedString>().unwrap(),
            SharedString::ShutterStrobeRandomPulse { n: 4 }
        );
        assert_eq!(
            "Shutter4StrobeRandomPulseClose"
                .parse::<SharedString>()
                .unwrap(),
            SharedString::ShutterStrobeRandomPulseClose { n: 4 }
        );
        assert_eq!(
            "Shutter4StrobeRandomPulseOpen"
                .parse::<SharedString>()
                .unwrap(),
            SharedString::ShutterStrobeRandomPulseOpen { n: 4 }
        );
        assert_eq!(
            "Shutter4StrobeEffect".parse::<SharedString>().unwrap(),
            SharedString::ShutterStrobeEffect { n: 4 }
        );
        assert_eq!("Iris".parse::<SharedString>().unwrap(), SharedString::Iris);
        assert_eq!(
            "IrisStrobe".parse::<SharedString>().unwrap(),
            SharedString::IrisStrobe
        );
        assert_eq!(
            "IrisStrobeRandom".parse::<SharedString>().unwrap(),
            SharedString::IrisStrobeRandom
        );
        assert_eq!(
            "IrisPulseClose".parse::<SharedString>().unwrap(),
            SharedString::IrisPulseClose
        );
        assert_eq!(
            "IrisPulseOpen".parse::<SharedString>().unwrap(),
            SharedString::IrisPulseOpen
        );
        assert_eq!(
            "IrisRandomPulseClose".parse::<SharedString>().unwrap(),
            SharedString::IrisRandomPulseClose
        );
        assert_eq!(
            "IrisRandomPulseOpen".parse::<SharedString>().unwrap(),
            SharedString::IrisRandomPulseOpen
        );
        assert_eq!(
            "Frost4".parse::<SharedString>().unwrap(),
            SharedString::Frost { n: 4 }
        );
        assert_eq!(
            "Frost4PulseOpen".parse::<SharedString>().unwrap(),
            SharedString::FrostPulseOpen { n: 4 }
        );
        assert_eq!(
            "Frost4PulseClose".parse::<SharedString>().unwrap(),
            SharedString::FrostPulseClose { n: 4 }
        );
        assert_eq!(
            "Frost4Ramp".parse::<SharedString>().unwrap(),
            SharedString::FrostRamp { n: 4 }
        );
        assert_eq!(
            "Prism4".parse::<SharedString>().unwrap(),
            SharedString::Prism { n: 4 }
        );
        assert_eq!(
            "Prism4SelectSpin".parse::<SharedString>().unwrap(),
            SharedString::PrismSelectSpin { n: 4 }
        );
        assert_eq!(
            "Prism4Macro".parse::<SharedString>().unwrap(),
            SharedString::PrismMacro { n: 4 }
        );
        assert_eq!(
            "Prism4Pos".parse::<SharedString>().unwrap(),
            SharedString::PrismPos { n: 4 }
        );
        assert_eq!(
            "Prism4PosRotate".parse::<SharedString>().unwrap(),
            SharedString::PrismPosRotate { n: 4 }
        );
        assert_eq!(
            "Effects4".parse::<SharedString>().unwrap(),
            SharedString::Effects { n: 4 }
        );
        assert_eq!(
            "Effects4Rate".parse::<SharedString>().unwrap(),
            SharedString::EffectsRate { n: 4 }
        );
        assert_eq!(
            "Effects4Fade".parse::<SharedString>().unwrap(),
            SharedString::EffectsFade { n: 4 }
        );
        assert_eq!(
            "Effects4Adjust7".parse::<SharedString>().unwrap(),
            SharedString::EffectsAdjust { n: 4, m: 7 }
        );
        assert_eq!(
            "Effects4Pos".parse::<SharedString>().unwrap(),
            SharedString::EffectsPos { n: 4 }
        );
        assert_eq!(
            "Effects4PosRotate".parse::<SharedString>().unwrap(),
            SharedString::EffectsPosRotate { n: 4 }
        );
        assert_eq!(
            "EffectsSync".parse::<SharedString>().unwrap(),
            SharedString::EffectsSync
        );
        assert_eq!(
            "BeamShaper".parse::<SharedString>().unwrap(),
            SharedString::BeamShaper
        );
        assert_eq!(
            "BeamShaperMacro".parse::<SharedString>().unwrap(),
            SharedString::BeamShaperMacro
        );
        assert_eq!(
            "BeamShaperPos".parse::<SharedString>().unwrap(),
            SharedString::BeamShaperPos
        );
        assert_eq!(
            "BeamShaperPosRotate".parse::<SharedString>().unwrap(),
            SharedString::BeamShaperPosRotate
        );
        assert_eq!("Zoom".parse::<SharedString>().unwrap(), SharedString::Zoom);
        assert_eq!(
            "ZoomModeSpot".parse::<SharedString>().unwrap(),
            SharedString::ZoomModeSpot
        );
        assert_eq!(
            "ZoomModeBeam".parse::<SharedString>().unwrap(),
            SharedString::ZoomModeBeam
        );
        assert_eq!(
            "DigitalZoom".parse::<SharedString>().unwrap(),
            SharedString::DigitalZoom
        );
        assert_eq!(
            "Focus4".parse::<SharedString>().unwrap(),
            SharedString::Focus { n: 4 }
        );
        assert_eq!(
            "Focus4Adjust".parse::<SharedString>().unwrap(),
            SharedString::FocusAdjust { n: 4 }
        );
        assert_eq!(
            "Focus4Distance".parse::<SharedString>().unwrap(),
            SharedString::FocusDistance { n: 4 }
        );
        assert_eq!(
            "Control4".parse::<SharedString>().unwrap(),
            SharedString::Control { n: 4 }
        );
        assert_eq!(
            "DimmerMode".parse::<SharedString>().unwrap(),
            SharedString::DimmerMode
        );
        assert_eq!(
            "DimmerCurve".parse::<SharedString>().unwrap(),
            SharedString::DimmerCurve
        );
        assert_eq!(
            "BlackoutMode".parse::<SharedString>().unwrap(),
            SharedString::BlackoutMode
        );
        assert_eq!(
            "LEDFrequency".parse::<SharedString>().unwrap(),
            SharedString::LedFrequency
        );
        assert_eq!(
            "LEDZoneMode".parse::<SharedString>().unwrap(),
            SharedString::LedZoneMode
        );
        assert_eq!(
            "PixelMode".parse::<SharedString>().unwrap(),
            SharedString::PixelMode
        );
        assert_eq!(
            "PanMode".parse::<SharedString>().unwrap(),
            SharedString::PanMode
        );
        assert_eq!(
            "TiltMode".parse::<SharedString>().unwrap(),
            SharedString::TiltMode
        );
        assert_eq!(
            "PanTiltMode".parse::<SharedString>().unwrap(),
            SharedString::PanTiltMode
        );
        assert_eq!(
            "PositionModes".parse::<SharedString>().unwrap(),
            SharedString::PositionModes
        );
        assert_eq!(
            "Gobo4WheelMode".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelMode { n: 4 }
        );
        assert_eq!(
            "GoboWheelShortcutMode".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelShortcutMode
        );
        assert_eq!(
            "AnimationWheel4Mode".parse::<SharedString>().unwrap(),
            SharedString::AnimationWheelMode { n: 4 }
        );
        assert_eq!(
            "AnimationWheelShortcutMode"
                .parse::<SharedString>()
                .unwrap(),
            SharedString::AnimationWheelShortcutMode
        );
        assert_eq!(
            "Color4Mode".parse::<SharedString>().unwrap(),
            SharedString::ColorMode { n: 4 }
        );
        assert_eq!(
            "ColorWheelShortcutMode".parse::<SharedString>().unwrap(),
            SharedString::ColorWheelShortcutMode
        );
        assert_eq!(
            "CyanMode".parse::<SharedString>().unwrap(),
            SharedString::CyanMode
        );
        assert_eq!(
            "MagentaMode".parse::<SharedString>().unwrap(),
            SharedString::MagentaMode
        );
        assert_eq!(
            "YellowMode".parse::<SharedString>().unwrap(),
            SharedString::YellowMode
        );
        assert_eq!(
            "ColorMixMode".parse::<SharedString>().unwrap(),
            SharedString::ColorMixMode
        );
        assert_eq!(
            "ChromaticMode".parse::<SharedString>().unwrap(),
            SharedString::ChromaticMode
        );
        assert_eq!(
            "ColorCalibrationMode".parse::<SharedString>().unwrap(),
            SharedString::ColorCalibrationMode
        );
        assert_eq!(
            "ColorConsistency".parse::<SharedString>().unwrap(),
            SharedString::ColorConsistency
        );
        assert_eq!(
            "ColorControl".parse::<SharedString>().unwrap(),
            SharedString::ColorControl
        );
        assert_eq!(
            "ColorModelMode".parse::<SharedString>().unwrap(),
            SharedString::ColorModelMode
        );
        assert_eq!(
            "ColorSettingsReset".parse::<SharedString>().unwrap(),
            SharedString::ColorSettingsReset
        );
        assert_eq!(
            "ColorUniformity".parse::<SharedString>().unwrap(),
            SharedString::ColorUniformity
        );
        assert_eq!(
            "CRIMode".parse::<SharedString>().unwrap(),
            SharedString::CriMode
        );
        assert_eq!(
            "CustomColor".parse::<SharedString>().unwrap(),
            SharedString::CustomColor
        );
        assert_eq!(
            "UVStability".parse::<SharedString>().unwrap(),
            SharedString::UvStability
        );
        assert_eq!(
            "WavelengthCorrection".parse::<SharedString>().unwrap(),
            SharedString::WavelengthCorrection
        );
        assert_eq!(
            "WhiteCount".parse::<SharedString>().unwrap(),
            SharedString::WhiteCount
        );
        assert_eq!(
            "StrobeMode".parse::<SharedString>().unwrap(),
            SharedString::StrobeMode
        );
        assert_eq!(
            "ZoomMode".parse::<SharedString>().unwrap(),
            SharedString::ZoomMode
        );
        assert_eq!(
            "FocusMode".parse::<SharedString>().unwrap(),
            SharedString::FocusMode
        );
        assert_eq!(
            "IrisMode".parse::<SharedString>().unwrap(),
            SharedString::IrisMode
        );
        assert_eq!(
            "Fan4Mode".parse::<SharedString>().unwrap(),
            SharedString::FanMode { n: 4 }
        );
        assert_eq!(
            "FollowSpotMode".parse::<SharedString>().unwrap(),
            SharedString::FollowSpotMode
        );
        assert_eq!(
            "BeamEffectIndexRotateMode".parse::<SharedString>().unwrap(),
            SharedString::BeamEffectIndexRotateMode
        );
        assert_eq!(
            "IntensityMSpeed".parse::<SharedString>().unwrap(),
            SharedString::IntensityMSpeed
        );
        assert_eq!(
            "PositionMSpeed".parse::<SharedString>().unwrap(),
            SharedString::PositionMSpeed
        );
        assert_eq!(
            "ColorMixMSpeed".parse::<SharedString>().unwrap(),
            SharedString::ColorMixMSpeed
        );
        assert_eq!(
            "ColorWheelSelectMSpeed".parse::<SharedString>().unwrap(),
            SharedString::ColorWheelSelectMSpeed
        );
        assert_eq!(
            "GoboWheel4MSpeed".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelMSpeed { n: 4 }
        );
        assert_eq!(
            "IrisMSpeed".parse::<SharedString>().unwrap(),
            SharedString::IrisMSpeed
        );
        assert_eq!(
            "Prism4MSpeed".parse::<SharedString>().unwrap(),
            SharedString::PrismMSpeed { n: 4 }
        );
        assert_eq!(
            "FocusMSpeed".parse::<SharedString>().unwrap(),
            SharedString::FocusMSpeed
        );
        assert_eq!(
            "Frost4MSpeed".parse::<SharedString>().unwrap(),
            SharedString::FrostMSpeed { n: 4 }
        );
        assert_eq!(
            "ZoomMSpeed".parse::<SharedString>().unwrap(),
            SharedString::ZoomMSpeed
        );
        assert_eq!(
            "FrameMSpeed".parse::<SharedString>().unwrap(),
            SharedString::FrameMSpeed
        );
        assert_eq!(
            "GlobalMSpeed".parse::<SharedString>().unwrap(),
            SharedString::GlobalMSpeed
        );
        assert_eq!(
            "ReflectorAdjust".parse::<SharedString>().unwrap(),
            SharedString::ReflectorAdjust
        );
        assert_eq!(
            "FixtureGlobalReset".parse::<SharedString>().unwrap(),
            SharedString::FixtureGlobalReset
        );
        assert_eq!(
            "DimmerReset".parse::<SharedString>().unwrap(),
            SharedString::DimmerReset
        );
        assert_eq!(
            "ShutterReset".parse::<SharedString>().unwrap(),
            SharedString::ShutterReset
        );
        assert_eq!(
            "BeamReset".parse::<SharedString>().unwrap(),
            SharedString::BeamReset
        );
        assert_eq!(
            "ColorMixReset".parse::<SharedString>().unwrap(),
            SharedString::ColorMixReset
        );
        assert_eq!(
            "ColorWheelReset".parse::<SharedString>().unwrap(),
            SharedString::ColorWheelReset
        );
        assert_eq!(
            "FocusReset".parse::<SharedString>().unwrap(),
            SharedString::FocusReset
        );
        assert_eq!(
            "FrameReset".parse::<SharedString>().unwrap(),
            SharedString::FrameReset
        );
        assert_eq!(
            "GoboWheelReset".parse::<SharedString>().unwrap(),
            SharedString::GoboWheelReset
        );
        assert_eq!(
            "IntensityReset".parse::<SharedString>().unwrap(),
            SharedString::IntensityReset
        );
        assert_eq!(
            "IrisReset".parse::<SharedString>().unwrap(),
            SharedString::IrisReset
        );
        assert_eq!(
            "PositionReset".parse::<SharedString>().unwrap(),
            SharedString::PositionReset
        );
        assert_eq!(
            "PanReset".parse::<SharedString>().unwrap(),
            SharedString::PanReset
        );
        assert_eq!(
            "TiltReset".parse::<SharedString>().unwrap(),
            SharedString::TiltReset
        );
        assert_eq!(
            "ZoomReset".parse::<SharedString>().unwrap(),
            SharedString::ZoomReset
        );
        assert_eq!(
            "CTBReset".parse::<SharedString>().unwrap(),
            SharedString::CtbReset
        );
        assert_eq!(
            "CTOReset".parse::<SharedString>().unwrap(),
            SharedString::CtoReset
        );
        assert_eq!(
            "CTCReset".parse::<SharedString>().unwrap(),
            SharedString::CtcReset
        );
        assert_eq!(
            "AnimationSystemReset".parse::<SharedString>().unwrap(),
            SharedString::AnimationSystemReset
        );
        assert_eq!(
            "FixtureCalibrationReset".parse::<SharedString>().unwrap(),
            SharedString::FixtureCalibrationReset
        );
        assert_eq!(
            "Function".parse::<SharedString>().unwrap(),
            SharedString::Function
        );
        assert_eq!(
            "LampControl".parse::<SharedString>().unwrap(),
            SharedString::LampControl
        );
        assert_eq!(
            "DisplayIntensity".parse::<SharedString>().unwrap(),
            SharedString::DisplayIntensity
        );
        assert_eq!(
            "DMXInput".parse::<SharedString>().unwrap(),
            SharedString::DmxInput
        );
        assert_eq!(
            "NoFeature".parse::<SharedString>().unwrap(),
            SharedString::NoFeature
        );
        assert_eq!(
            "Blower4".parse::<SharedString>().unwrap(),
            SharedString::Blower { n: 4 }
        );
        assert_eq!(
            "Fan4".parse::<SharedString>().unwrap(),
            SharedString::Fan { n: 4 }
        );
        assert_eq!(
            "Fog4".parse::<SharedString>().unwrap(),
            SharedString::Fog { n: 4 }
        );
        assert_eq!(
            "Haze4".parse::<SharedString>().unwrap(),
            SharedString::Haze { n: 4 }
        );
        assert_eq!(
            "LampPowerMode".parse::<SharedString>().unwrap(),
            SharedString::LampPowerMode
        );
        assert_eq!("Fans".parse::<SharedString>().unwrap(), SharedString::Fans);
        assert_eq!(
            "Blade4A".parse::<SharedString>().unwrap(),
            SharedString::BladeA { n: 4 }
        );
        assert_eq!(
            "Blade4B".parse::<SharedString>().unwrap(),
            SharedString::BladeB { n: 4 }
        );
        assert_eq!(
            "Blade4Rot".parse::<SharedString>().unwrap(),
            SharedString::BladeRot { n: 4 }
        );
        assert_eq!(
            "ShaperRot".parse::<SharedString>().unwrap(),
            SharedString::ShaperRot
        );
        assert_eq!(
            "ShaperMacros".parse::<SharedString>().unwrap(),
            SharedString::ShaperMacros
        );
        assert_eq!(
            "ShaperMacrosSpeed".parse::<SharedString>().unwrap(),
            SharedString::ShaperMacrosSpeed
        );
        assert_eq!(
            "BladeSoft4A".parse::<SharedString>().unwrap(),
            SharedString::BladeSoftA { n: 4 }
        );
        assert_eq!(
            "BladeSoft4B".parse::<SharedString>().unwrap(),
            SharedString::BladeSoftB { n: 4 }
        );
        assert_eq!(
            "KeyStone4A".parse::<SharedString>().unwrap(),
            SharedString::KeyStoneA { n: 4 }
        );
        assert_eq!(
            "KeyStone4B".parse::<SharedString>().unwrap(),
            SharedString::KeyStoneB { n: 4 }
        );
        assert_eq!(
            "Video".parse::<SharedString>().unwrap(),
            SharedString::Video
        );
        assert_eq!(
            "VideoEffect4Type".parse::<SharedString>().unwrap(),
            SharedString::VideoEffectType { n: 4 }
        );
        assert_eq!(
            "VideoEffect4Parameter7".parse::<SharedString>().unwrap(),
            SharedString::VideoEffectParameter { n: 4, m: 7 }
        );
        assert_eq!(
            "VideoCamera4".parse::<SharedString>().unwrap(),
            SharedString::VideoCamera { n: 4 }
        );
        assert_eq!(
            "VideoSoundVolume4".parse::<SharedString>().unwrap(),
            SharedString::VideoSoundVolume { n: 4 }
        );
        assert_eq!(
            "VideoBlendMode".parse::<SharedString>().unwrap(),
            SharedString::VideoBlendMode
        );
        assert_eq!(
            "InputSource".parse::<SharedString>().unwrap(),
            SharedString::InputSource
        );
        assert_eq!(
            "FieldOfView".parse::<SharedString>().unwrap(),
            SharedString::FieldOfView
        );
    }
}

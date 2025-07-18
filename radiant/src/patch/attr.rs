use std::str::FromStr;

/// A GDTF attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Attribute {
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

impl Attribute {
    /// Get this attribute's [FeatureGroup]. If it's a custom attribute, it will
    /// return `None`.
    pub fn feature_group(&self) -> Option<FeatureGroup> {
        match self {
            Self::Dimmer => Some(FeatureGroup::Dimmer),
            Self::Pan
            | Self::Tilt
            | Self::PanRotate
            | Self::TiltRotate
            | Self::PositionEffect
            | Self::PositionEffectRate
            | Self::PositionEffectFade
            | Self::XyzX
            | Self::XyzY
            | Self::XyzZ
            | Self::RotX
            | Self::RotY
            | Self::RotZ
            | Self::ScaleX
            | Self::ScaleY
            | Self::ScaleZ
            | Self::ScaleXYZ => Some(FeatureGroup::Position),

            Self::Gobo(_)
            | Self::GoboSelectSpin(_)
            | Self::GoboSelectShake(_)
            | Self::GoboSelectEffects(_)
            | Self::GoboWheelIndex(_)
            | Self::GoboWheelSpin(_)
            | Self::GoboWheelShake(_)
            | Self::GoboWheelRandom(_)
            | Self::GoboWheelAudio(_)
            | Self::GoboPos(_)
            | Self::GoboPosRotate(_)
            | Self::GoboPosShake(_)
            | Self::AnimationWheel(_)
            | Self::AnimationWheelAudio(_)
            | Self::AnimationWheelMacro(_)
            | Self::AnimationWheelRandom(_)
            | Self::AnimationWheelSelectEffects(_)
            | Self::AnimationWheelSelectShake(_)
            | Self::AnimationWheelSelectSpin(_)
            | Self::AnimationWheelPos(_)
            | Self::AnimationWheelPosRotate(_)
            | Self::AnimationWheelPosShake(_)
            | Self::AnimationSystem(_)
            | Self::AnimationSystemRamp(_)
            | Self::AnimationSystemShake(_)
            | Self::AnimationSystemAudio(_)
            | Self::AnimationSystemRandom(_)
            | Self::AnimationSystemPos(_)
            | Self::AnimationSystemPosRotate(_)
            | Self::AnimationSystemPosShake(_)
            | Self::AnimationSystemPosRandom(_)
            | Self::AnimationSystemPosAudio(_)
            | Self::AnimationSystemMacro(_)
            | Self::MediaFolder(_)
            | Self::MediaContent(_)
            | Self::ModelFolder(_)
            | Self::ModelContent(_)
            | Self::PlayMode
            | Self::PlayBegin
            | Self::PlayEnd
            | Self::PlaySpeed => Some(FeatureGroup::Gobo),

            Self::ColorEffects(_)
            | Self::Color(_)
            | Self::ColorWheelIndex(_)
            | Self::ColorWheelSpin(_)
            | Self::ColorWheelRandom(_)
            | Self::ColorWheelAudio(_)
            | Self::ColorAddR
            | Self::ColorAddG
            | Self::ColorAddB
            | Self::ColorAddC
            | Self::ColorAddM
            | Self::ColorAddY
            | Self::ColorAddRY
            | Self::ColorAddGY
            | Self::ColorAddGC
            | Self::ColorAddBC
            | Self::ColorAddBM
            | Self::ColorAddRM
            | Self::ColorAddW
            | Self::ColorAddWW
            | Self::ColorAddCW
            | Self::ColorAddUV
            | Self::ColorSubR
            | Self::ColorSubG
            | Self::ColorSubB
            | Self::ColorSubC
            | Self::ColorSubM
            | Self::ColorSubY
            | Self::ColorMacro(_)
            | Self::ColorMacroRate(_)
            | Self::Cto
            | Self::Ctc
            | Self::Ctb
            | Self::Tint
            | Self::HsbHue
            | Self::HsbSaturation
            | Self::HsbBrightness
            | Self::HsbQuality
            | Self::CieX
            | Self::CieY
            | Self::CieBrightness
            | Self::ColorRgbRed
            | Self::ColorRgbGreen
            | Self::ColorRgbBlue
            | Self::ColorRgbCyan
            | Self::ColorRgbMagenta
            | Self::ColorRgbYellow
            | Self::ColorRgbQuality
            | Self::VideoBoostR
            | Self::VideoBoostG
            | Self::VideoBoostB
            | Self::VideoHueShift
            | Self::VideoSaturation
            | Self::VideoBrightness
            | Self::VideoContrast
            | Self::VideoKeyColorR
            | Self::VideoKeyColorG
            | Self::VideoKeyColorB
            | Self::VideoKeyIntensity
            | Self::VideoKeyTolerance => Some(FeatureGroup::Color),

            Self::StrobeDuration
            | Self::StrobeRate
            | Self::StrobeFrequency
            | Self::StrobeModeShutter
            | Self::StrobeModeStrobe
            | Self::StrobeModePulse
            | Self::StrobeModePulseOpen
            | Self::StrobeModePulseClose
            | Self::StrobeModeRandom
            | Self::StrobeModeRandomPulse
            | Self::StrobeModeRandomPulseOpen
            | Self::StrobeModeRandomPulseClose
            | Self::StrobeModeEffect
            | Self::Shutter(_)
            | Self::ShutterStrobe(_)
            | Self::ShutterStrobePulse(_)
            | Self::ShutterStrobePulseClose(_)
            | Self::ShutterStrobePulseOpen(_)
            | Self::ShutterStrobeRandom(_)
            | Self::ShutterStrobeRandomPulse(_)
            | Self::ShutterStrobeRandomPulseClose(_)
            | Self::ShutterStrobeRandomPulseOpen(_)
            | Self::ShutterStrobeEffect(_)
            | Self::Iris
            | Self::IrisStrobe
            | Self::IrisStrobeRandom
            | Self::IrisPulseClose
            | Self::IrisPulseOpen
            | Self::IrisRandomPulseClose
            | Self::IrisRandomPulseOpen
            | Self::Frost(_)
            | Self::FrostPulseOpen(_)
            | Self::FrostPulseClose(_)
            | Self::FrostRamp(_)
            | Self::Prism(_)
            | Self::PrismSelectSpin(_)
            | Self::PrismMacro(_)
            | Self::PrismPos(_)
            | Self::PrismPosRotate(_) => Some(FeatureGroup::Beam),

            Self::Effects(_)
            | Self::EffectsRate(_)
            | Self::EffectsFade(_)
            | Self::EffectsAdjust(_, _)
            | Self::EffectsPos(_)
            | Self::EffectsPosRotate(_)
            | Self::EffectsSync
            | Self::BeamShaper
            | Self::BeamShaperMacro
            | Self::BeamShaperPos
            | Self::BeamShaperPosRotate
            | Self::Zoom
            | Self::ZoomModeSpot
            | Self::ZoomModeBeam
            | Self::DigitalZoom
            | Self::Focus(_)
            | Self::FocusAdjust(_)
            | Self::FocusDistance(_) => Some(FeatureGroup::Focus),

            Self::Control(_)
            | Self::DimmerMode
            | Self::DimmerCurve
            | Self::BlackoutMode
            | Self::LedFrequency
            | Self::LedZoneMode
            | Self::PixelMode
            | Self::PanMode
            | Self::TiltMode
            | Self::PanTiltMode
            | Self::PositionModes
            | Self::GoboWheelMode(_)
            | Self::GoboWheelShortcutMode
            | Self::AnimationWheelMode(_)
            | Self::AnimationWheelShortcutMode
            | Self::ColorMode(_)
            | Self::ColorWheelShortcutMode
            | Self::CyanMode
            | Self::MagentaMode
            | Self::YellowMode
            | Self::ColorMixMode
            | Self::ChromaticMode
            | Self::ColorCalibrationMode
            | Self::ColorConsistency
            | Self::ColorControl
            | Self::ColorModelMode
            | Self::ColorSettingsReset
            | Self::ColorUniformity
            | Self::CriMode
            | Self::CustomColor
            | Self::UvStability
            | Self::WavelengthCorrection
            | Self::WhiteCount
            | Self::StrobeMode
            | Self::ZoomMode
            | Self::FocusMode
            | Self::IrisMode
            | Self::FanMode(_)
            | Self::FollowSpotMode
            | Self::BeamEffectIndexRotateMode
            | Self::IntensityMSpeed
            | Self::PositionMSpeed
            | Self::ColorMixMSpeed
            | Self::ColorWheelSelectMSpeed
            | Self::GoboWheelMSpeed(_)
            | Self::IrisMSpeed
            | Self::PrismMSpeed(_)
            | Self::FocusMSpeed
            | Self::FrostMSpeed(_)
            | Self::ZoomMSpeed
            | Self::FrameMSpeed
            | Self::GlobalMSpeed
            | Self::ReflectorAdjust
            | Self::FixtureGlobalReset
            | Self::DimmerReset
            | Self::ShutterReset
            | Self::BeamReset
            | Self::ColorMixReset
            | Self::ColorWheelReset
            | Self::FocusReset
            | Self::FrameReset
            | Self::GoboWheelReset
            | Self::IntensityReset
            | Self::IrisReset
            | Self::PositionReset
            | Self::PanReset
            | Self::TiltReset
            | Self::ZoomReset
            | Self::CtbReset
            | Self::CtoReset
            | Self::CtcReset
            | Self::AnimationSystemReset
            | Self::FixtureCalibrationReset
            | Self::Function
            | Self::LampControl
            | Self::DisplayIntensity
            | Self::DmxInput
            | Self::NoFeature
            | Self::Blower(_)
            | Self::Fan(_)
            | Self::Fog(_)
            | Self::Haze(_)
            | Self::LampPowerMode
            | Self::Fans => Some(FeatureGroup::Control),

            Self::BladeA(_)
            | Self::BladeB(_)
            | Self::BladeRot(_)
            | Self::ShaperRot
            | Self::ShaperMacros
            | Self::ShaperMacrosSpeed
            | Self::BladeSoftA(_)
            | Self::BladeSoftB(_)
            | Self::KeyStoneA(_)
            | Self::KeyStoneB(_) => Some(FeatureGroup::Shapers),

            Self::Video
            | Self::VideoEffectType(_)
            | Self::VideoEffectParameter(_, _)
            | Self::VideoCamera(_)
            | Self::VideoSoundVolume(_)
            | Self::VideoBlendMode
            | Self::InputSource
            | Self::FieldOfView => Some(FeatureGroup::Video),

            Self::Custom(_) => None,
        }
    }
}

impl std::fmt::Display for Attribute {
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

impl FromStr for Attribute {
    type Err = ();

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Helper function to extract `n` from an attribute name.
        fn extract_attr_n(s: &str, prefix: &str, suffix: Option<&str>) -> Option<u8> {
            if let Some(rest) = s.strip_prefix(prefix) {
                if let Some(suffix) = suffix {
                    if let Some(number_part) = rest.strip_suffix(suffix) {
                        number_part.parse::<u8>().ok()
                    } else {
                        None
                    }
                } else {
                    rest.parse::<u8>().ok()
                }
            } else {
                None
            }
        }

        // Helper function to extract `n` and `m` from an attribute name.
        fn extract_attr_n_m(s: &str, prefix: &str, middle: &str, suffix: Option<&str>) -> Option<(u8, u8)> {
            // Check if it starts with the prefix
            if !s.starts_with(prefix) {
                return None;
            }

            let rest = &s[prefix.len()..];

            // Find the middle delimiter in the remaining string
            if let Some(middle_pos) = rest.find(middle) {
                // Extract the first number (n) before the middle delimiter
                let n_part = &rest[..middle_pos];
                let n = n_part.parse::<u8>().ok()?;

                // Get the part after the middle delimiter
                let after_middle = &rest[middle_pos + middle.len()..];

                // Handle optional suffix
                let m_part = if let Some(suffix) = suffix {
                    after_middle.strip_suffix(suffix)?
                } else {
                    after_middle
                };

                // Extract the second number (m)
                let m = m_part.parse::<u8>().ok()?;

                Some((n, m))
            } else {
                None
            }
        }

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

impl<'de> serde::Deserialize<'de> for Attribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct AttributeVisitor;

        impl<'de> Visitor<'de> for AttributeVisitor {
            type Value = Attribute;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an Attribute")
            }

            fn visit_str<E>(self, v: &str) -> Result<Attribute, E>
            where
                E: de::Error,
            {
                Attribute::from_str(v)
                    .map_err(|_| E::custom(format!("invalid Attribute string: {v}")))
            }
        }

        deserializer.deserialize_str(AttributeVisitor)
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use std::str::FromStr;
    use super::Attribute;

    macro_rules! test_attr {
        ($test_name:ident, $attr_name:literal, $attr_kind:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(super::Attribute::from_str($attr_name).unwrap(), $attr_kind);
            }
        };
    }

    test_attr!(dimmer, "Dimmer", Attribute::Dimmer);
    test_attr!(pan, "Pan", Attribute::Pan);
    test_attr!(tilt, "Tilt", Attribute::Tilt);
    test_attr!(pan_rotate, "PanRotate", Attribute::PanRotate);
    test_attr!(tilt_rotate, "TiltRotate", Attribute::TiltRotate);
    test_attr!(position_effect, "PositionEffect", Attribute::PositionEffect);
    test_attr!(position_effect_rate, "PositionEffectRate", Attribute::PositionEffectRate);
    test_attr!(position_effect_fade, "PositionEffectFade", Attribute::PositionEffectFade);
    test_attr!(xyz_x, "XYZ_X", Attribute::XyzX);
    test_attr!(xyz_y, "XYZ_Y", Attribute::XyzY);
    test_attr!(xyz_z, "XYZ_Z", Attribute::XyzZ);
    test_attr!(rot_x, "Rot_X", Attribute::RotX);
    test_attr!(rot_y, "Rot_Y", Attribute::RotY);
    test_attr!(rot_z, "Rot_Z", Attribute::RotZ);
    test_attr!(scale_x, "Scale_X", Attribute::ScaleX);
    test_attr!(scale_y, "Scale_Y", Attribute::ScaleY);
    test_attr!(scale_z, "Scale_Z", Attribute::ScaleZ);
    test_attr!(scale_xyz, "Scale_XYZ", Attribute::ScaleXYZ);
    test_attr!(gobo_n, "Gobo1", Attribute::Gobo(1));
    test_attr!(gobo_n_select_spin, "Gobo1SelectSpin", Attribute::GoboSelectSpin(1));
    test_attr!(gobo_n_select_shake, "Gobo1SelectShake", Attribute::GoboSelectShake(1));
    test_attr!(gobo_n_select_effects, "Gobo1SelectEffects", Attribute::GoboSelectEffects(1));
    test_attr!(gobo_n_wheel_index, "Gobo1WheelIndex", Attribute::GoboWheelIndex(1));
    test_attr!(gobo_n_wheel_spin, "Gobo1WheelSpin", Attribute::GoboWheelSpin(1));
    test_attr!(gobo_n_wheel_shake, "Gobo1WheelShake", Attribute::GoboWheelShake(1));
    test_attr!(gobo_n_wheel_random, "Gobo1WheelRandom", Attribute::GoboWheelRandom(1));
    test_attr!(gobo_n_wheel_audio, "Gobo1WheelAudio", Attribute::GoboWheelAudio(1));
    test_attr!(gobo_n_pos, "Gobo1Pos", Attribute::GoboPos(1));
    test_attr!(gobo_n_pos_rotate, "Gobo1PosRotate", Attribute::GoboPosRotate(1));
    test_attr!(gobo_n_pos_shake, "Gobo1PosShake", Attribute::GoboPosShake(1));
    test_attr!(animation_wheel_n, "AnimationWheel1", Attribute::AnimationWheel(1));
    test_attr!(animation_wheel_n_audio, "AnimationWheel1Audio", Attribute::AnimationWheelAudio(1));
    test_attr!(animation_wheel_n_macro, "AnimationWheel1Macro", Attribute::AnimationWheelMacro(1));
    test_attr!(animation_wheel_n_random, "AnimationWheel1Random", Attribute::AnimationWheelRandom(1));
    test_attr!(animation_wheel_n_select_effects, "AnimationWheel1SelectEffects", Attribute::AnimationWheelSelectEffects(1));
    test_attr!(animation_wheel_n_select_shake, "AnimationWheel1SelectShake", Attribute::AnimationWheelSelectShake(1));
    test_attr!(animation_wheel_n_select_spin, "AnimationWheel1SelectSpin", Attribute::AnimationWheelSelectSpin(1));
    test_attr!(animation_wheel_n_pos, "AnimationWheel1Pos", Attribute::AnimationWheelPos(1));
    test_attr!(animation_wheel_n_pos_rotate, "AnimationWheel1PosRotate", Attribute::AnimationWheelPosRotate(1));
    test_attr!(animation_wheel_n_pos_shake, "AnimationWheel1PosShake", Attribute::AnimationWheelPosShake(1));
    test_attr!(animation_system_n, "AnimationSystem1", Attribute::AnimationSystem(1));
    test_attr!(animation_system_n_ramp, "AnimationSystem1Ramp", Attribute::AnimationSystemRamp(1));
    test_attr!(animation_system_n_shake, "AnimationSystem1Shake", Attribute::AnimationSystemShake(1));
    test_attr!(animation_system_n_audio, "AnimationSystem1Audio", Attribute::AnimationSystemAudio(1));
    test_attr!(animation_system_n_random, "AnimationSystem1Random", Attribute::AnimationSystemRandom(1));
    test_attr!(animation_system_n_pos, "AnimationSystem1Pos", Attribute::AnimationSystemPos(1));
    test_attr!(animation_system_n_pos_rotate, "AnimationSystem1PosRotate", Attribute::AnimationSystemPosRotate(1));
    test_attr!(animation_system_n_pos_shake, "AnimationSystem1PosShake", Attribute::AnimationSystemPosShake(1));
    test_attr!(animation_system_n_pos_random, "AnimationSystem1PosRandom", Attribute::AnimationSystemPosRandom(1));
    test_attr!(animation_system_n_pos_audio, "AnimationSystem1PosAudio", Attribute::AnimationSystemPosAudio(1));
    test_attr!(animation_system_n_macro, "AnimationSystem1Macro", Attribute::AnimationSystemMacro(1));
    test_attr!(media_folder_n, "MediaFolder1", Attribute::MediaFolder(1));
    test_attr!(media_content_n, "MediaContent1", Attribute::MediaContent(1));
    test_attr!(model_folder_n, "ModelFolder1", Attribute::ModelFolder(1));
    test_attr!(model_content_n, "ModelContent1", Attribute::ModelContent(1));
    test_attr!(play_mode, "PlayMode", Attribute::PlayMode);
    test_attr!(play_begin, "PlayBegin", Attribute::PlayBegin);
    test_attr!(play_end, "PlayEnd", Attribute::PlayEnd);
    test_attr!(play_speed, "PlaySpeed", Attribute::PlaySpeed);
    test_attr!(color_effects_n, "ColorEffects1", Attribute::ColorEffects(1));
    test_attr!(color_n, "Color1", Attribute::Color(1));
    test_attr!(color_n_wheel_index, "Color1WheelIndex", Attribute::ColorWheelIndex(1));
    test_attr!(color_n_wheel_spin, "Color1WheelSpin", Attribute::ColorWheelSpin(1));
    test_attr!(color_n_wheel_random, "Color1WheelRandom", Attribute::ColorWheelRandom(1));
    test_attr!(color_n_wheel_audio, "Color1WheelAudio", Attribute::ColorWheelAudio(1));
    test_attr!(color_add_r, "ColorAdd_R", Attribute::ColorAddR);
    test_attr!(color_add_g, "ColorAdd_G", Attribute::ColorAddG);
    test_attr!(color_add_b, "ColorAdd_B", Attribute::ColorAddB);
    test_attr!(color_add_c, "ColorAdd_C", Attribute::ColorAddC);
    test_attr!(color_add_m, "ColorAdd_M", Attribute::ColorAddM);
    test_attr!(color_add_y, "ColorAdd_Y", Attribute::ColorAddY);
    test_attr!(color_add_ry, "ColorAdd_RY", Attribute::ColorAddRY);
    test_attr!(color_add_gy, "ColorAdd_GY", Attribute::ColorAddGY);
    test_attr!(color_add_gc, "ColorAdd_GC", Attribute::ColorAddGC);
    test_attr!(color_add_bc, "ColorAdd_BC", Attribute::ColorAddBC);
    test_attr!(color_add_bm, "ColorAdd_BM", Attribute::ColorAddBM);
    test_attr!(color_add_rm, "ColorAdd_RM", Attribute::ColorAddRM);
    test_attr!(color_add_w, "ColorAdd_W", Attribute::ColorAddW);
    test_attr!(color_add_ww, "ColorAdd_WW", Attribute::ColorAddWW);
    test_attr!(color_add_cw, "ColorAdd_CW", Attribute::ColorAddCW);
    test_attr!(color_add_uv, "ColorAdd_UV", Attribute::ColorAddUV);
    test_attr!(color_sub_r, "ColorSub_R", Attribute::ColorSubR);
    test_attr!(color_sub_g, "ColorSub_G", Attribute::ColorSubG);
    test_attr!(color_sub_b, "ColorSub_B", Attribute::ColorSubB);
    test_attr!(color_sub_c, "ColorSub_C", Attribute::ColorSubC);
    test_attr!(color_sub_m, "ColorSub_M", Attribute::ColorSubM);
    test_attr!(color_sub_y, "ColorSub_Y", Attribute::ColorSubY);
    test_attr!(color_macro_n, "ColorMacro1", Attribute::ColorMacro(1));
    test_attr!(color_macro_n_rate, "ColorMacro1Rate", Attribute::ColorMacroRate(1));
    test_attr!(cto, "CTO", Attribute::Cto);
    test_attr!(ctc, "CTC", Attribute::Ctc);
    test_attr!(ctb, "CTB", Attribute::Ctb);
    test_attr!(tint, "Tint", Attribute::Tint);
    test_attr!(hsb_hue, "HSB_Hue", Attribute::HsbHue);
    test_attr!(hsb_saturation, "HSB_Saturation", Attribute::HsbSaturation);
    test_attr!(hsb_brightness, "HSB_Brightness", Attribute::HsbBrightness);
    test_attr!(hsb_quality, "HSB_Quality", Attribute::HsbQuality);
    test_attr!(cie_x, "CIE_X", Attribute::CieX);
    test_attr!(cie_y, "CIE_Y", Attribute::CieY);
    test_attr!(cie_brightness, "CIE_Brightness", Attribute::CieBrightness);
    test_attr!(color_rgb_red, "ColorRGB_Red", Attribute::ColorRgbRed);
    test_attr!(color_rgb_green, "ColorRGB_Green", Attribute::ColorRgbGreen);
    test_attr!(color_rgb_blue, "ColorRGB_Blue", Attribute::ColorRgbBlue);
    test_attr!(color_rgb_cyan, "ColorRGB_Cyan", Attribute::ColorRgbCyan);
    test_attr!(color_rgb_magenta, "ColorRGB_Magenta", Attribute::ColorRgbMagenta);
    test_attr!(color_rgb_yellow, "ColorRGB_Yellow", Attribute::ColorRgbYellow);
    test_attr!(color_rgb_quality, "ColorRGB_Quality", Attribute::ColorRgbQuality);
    test_attr!(video_boost_r, "VideoBoost_R", Attribute::VideoBoostR);
    test_attr!(video_boost_g, "VideoBoost_G", Attribute::VideoBoostG);
    test_attr!(video_boost_b, "VideoBoost_B", Attribute::VideoBoostB);
    test_attr!(video_hue_shift, "VideoHueShift", Attribute::VideoHueShift);
    test_attr!(video_saturation, "VideoSaturation", Attribute::VideoSaturation);
    test_attr!(video_brightness, "VideoBrightness", Attribute::VideoBrightness);
    test_attr!(video_contrast, "VideoContrast", Attribute::VideoContrast);
    test_attr!(video_key_color_r, "VideoKeyColor_R", Attribute::VideoKeyColorR);
    test_attr!(video_key_color_g, "VideoKeyColor_G", Attribute::VideoKeyColorG);
    test_attr!(video_key_color_b, "VideoKeyColor_B", Attribute::VideoKeyColorB);
    test_attr!(video_key_intensity, "VideoKeyIntensity", Attribute::VideoKeyIntensity);
    test_attr!(video_key_tolerance, "VideoKeyTolerance", Attribute::VideoKeyTolerance);
    test_attr!(strobe_duration, "StrobeDuration", Attribute::StrobeDuration);
    test_attr!(strobe_rate, "StrobeRate", Attribute::StrobeRate);
    test_attr!(strobe_frequency, "StrobeFrequency", Attribute::StrobeFrequency);
    test_attr!(strobe_mode_shutter, "StrobeModeShutter", Attribute::StrobeModeShutter);
    test_attr!(strobe_mode_strobe, "StrobeModeStrobe", Attribute::StrobeModeStrobe);
    test_attr!(strobe_mode_pulse, "StrobeModePulse", Attribute::StrobeModePulse);
    test_attr!(strobe_mode_pulse_open, "StrobeModePulseOpen", Attribute::StrobeModePulseOpen);
    test_attr!(strobe_mode_pulse_close, "StrobeModePulseClose", Attribute::StrobeModePulseClose);
    test_attr!(strobe_mode_random, "StrobeModeRandom", Attribute::StrobeModeRandom);
    test_attr!(strobe_mode_random_pulse, "StrobeModeRandomPulse", Attribute::StrobeModeRandomPulse);
    test_attr!(strobe_mode_random_pulse_open, "StrobeModeRandomPulseOpen", Attribute::StrobeModeRandomPulseOpen);
    test_attr!(strobe_mode_random_pulse_close, "StrobeModeRandomPulseClose", Attribute::StrobeModeRandomPulseClose);
    test_attr!(strobe_mode_effect, "StrobeModeEffect", Attribute::StrobeModeEffect);
    test_attr!(shutter_n, "Shutter1", Attribute::Shutter(1));
    test_attr!(shutter_n_strobe, "Shutter1Strobe", Attribute::ShutterStrobe(1));
    test_attr!(shutter_n_strobe_pulse, "Shutter1StrobePulse", Attribute::ShutterStrobePulse(1));
    test_attr!(shutter_n_strobe_pulse_close, "Shutter1StrobePulseClose", Attribute::ShutterStrobePulseClose(1));
    test_attr!(shutter_n_strobe_pulse_open, "Shutter1StrobePulseOpen", Attribute::ShutterStrobePulseOpen(1));
    test_attr!(shutter_n_strobe_random, "Shutter1StrobeRandom", Attribute::ShutterStrobeRandom(1));
    test_attr!(shutter_n_strobe_random_pulse, "Shutter1StrobeRandomPulse", Attribute::ShutterStrobeRandomPulse(1));
    test_attr!(shutter_n_strobe_random_pulse_close, "Shutter1StrobeRandomPulseClose", Attribute::ShutterStrobeRandomPulseClose(1));
    test_attr!(shutter_n_strobe_random_pulse_open, "Shutter1StrobeRandomPulseOpen", Attribute::ShutterStrobeRandomPulseOpen(1));
    test_attr!(shutter_n_strobe_effect, "Shutter1StrobeEffect", Attribute::ShutterStrobeEffect(1));
    test_attr!(iris, "Iris", Attribute::Iris);
    test_attr!(iris_strobe, "IrisStrobe", Attribute::IrisStrobe);
    test_attr!(iris_strobe_random, "IrisStrobeRandom", Attribute::IrisStrobeRandom);
    test_attr!(iris_pulse_close, "IrisPulseClose", Attribute::IrisPulseClose);
    test_attr!(iris_pulse_open, "IrisPulseOpen", Attribute::IrisPulseOpen);
    test_attr!(iris_random_pulse_close, "IrisRandomPulseClose", Attribute::IrisRandomPulseClose);
    test_attr!(iris_random_pulse_open, "IrisRandomPulseOpen", Attribute::IrisRandomPulseOpen);
    test_attr!(frost_n, "Frost1", Attribute::Frost(1));
    test_attr!(frost_n_pulse_open, "Frost1PulseOpen", Attribute::FrostPulseOpen(1));
    test_attr!(frost_n_pulse_close, "Frost1PulseClose", Attribute::FrostPulseClose(1));
    test_attr!(frost_n_ramp, "Frost1Ramp", Attribute::FrostRamp(1));
    test_attr!(prism_n, "Prism1", Attribute::Prism(1));
    test_attr!(prism_n_select_spin, "Prism1SelectSpin", Attribute::PrismSelectSpin(1));
    test_attr!(prism_n_macro, "Prism1Macro", Attribute::PrismMacro(1));
    test_attr!(prism_n_pos, "Prism1Pos", Attribute::PrismPos(1));
    test_attr!(prism_n_pos_rotate, "Prism1PosRotate", Attribute::PrismPosRotate(1));
    test_attr!(effects_n, "Effects1", Attribute::Effects(1));
    test_attr!(effects_n_rate, "Effects1Rate", Attribute::EffectsRate(1));
    test_attr!(effects_n_fade, "Effects1Fade", Attribute::EffectsFade(1));
    test_attr!(effects_n_adjust_m, "Effects1Adjust2", Attribute::EffectsAdjust(1, 2));
    test_attr!(effects_n_pos, "Effects1Pos", Attribute::EffectsPos(1));
    test_attr!(effects_n_pos_rotate, "Effects1PosRotate", Attribute::EffectsPosRotate(1));
    test_attr!(effects_sync, "EffectsSync", Attribute::EffectsSync);
    test_attr!(beam_shaper, "BeamShaper", Attribute::BeamShaper);
    test_attr!(beam_shaper_macro, "BeamShaperMacro", Attribute::BeamShaperMacro);
    test_attr!(beam_shaper_pos, "BeamShaperPos", Attribute::BeamShaperPos);
    test_attr!(beam_shaper_pos_rotate, "BeamShaperPosRotate", Attribute::BeamShaperPosRotate);
    test_attr!(zoom, "Zoom", Attribute::Zoom);
    test_attr!(zoom_mode_spot, "ZoomModeSpot", Attribute::ZoomModeSpot);
    test_attr!(zoom_mode_beam, "ZoomModeBeam", Attribute::ZoomModeBeam);
    test_attr!(digital_zoom, "DigitalZoom", Attribute::DigitalZoom);
    test_attr!(focus_n, "Focus1", Attribute::Focus(1));
    test_attr!(focus_n_adjust, "Focus1Adjust", Attribute::FocusAdjust(1));
    test_attr!(focus_n_distance, "Focus1Distance", Attribute::FocusDistance(1));
    test_attr!(control_n, "Control1", Attribute::Control(1));
    test_attr!(dimmer_mode, "DimmerMode", Attribute::DimmerMode);
    test_attr!(dimmer_curve, "DimmerCurve", Attribute::DimmerCurve);
    test_attr!(blackout_mode, "BlackoutMode", Attribute::BlackoutMode);
    test_attr!(led_frequency, "LEDFrequency", Attribute::LedFrequency);
    test_attr!(led_zone_mode, "LEDZoneMode", Attribute::LedZoneMode);
    test_attr!(pixel_mode, "PixelMode", Attribute::PixelMode);
    test_attr!(pan_mode, "PanMode", Attribute::PanMode);
    test_attr!(tilt_mode, "TiltMode", Attribute::TiltMode);
    test_attr!(pan_tilt_mode, "PanTiltMode", Attribute::PanTiltMode);
    test_attr!(position_modes, "PositionModes", Attribute::PositionModes);
    test_attr!(gobo_n_wheel_mode, "Gobo1WheelMode", Attribute::GoboWheelMode(1));
    test_attr!(gobo_wheel_shortcut_mode, "GoboWheelShortcutMode", Attribute::GoboWheelShortcutMode);
    test_attr!(animation_wheel_n_mode, "AnimationWheel1Mode", Attribute::AnimationWheelMode(1));
    test_attr!(animation_wheel_shortcut_mode, "AnimationWheelShortcutMode", Attribute::AnimationWheelShortcutMode);
    test_attr!(color_n_mode, "Color1Mode", Attribute::ColorMode(1));
    test_attr!(color_wheel_shortcut_mode, "ColorWheelShortcutMode", Attribute::ColorWheelShortcutMode);
    test_attr!(cyan_mode, "CyanMode", Attribute::CyanMode);
    test_attr!(magenta_mode, "MagentaMode", Attribute::MagentaMode);
    test_attr!(yellow_mode, "YellowMode", Attribute::YellowMode);
    test_attr!(color_mix_mode, "ColorMixMode", Attribute::ColorMixMode);
    test_attr!(chromatic_mode, "ChromaticMode", Attribute::ChromaticMode);
    test_attr!(color_calibration_mode, "ColorCalibrationMode", Attribute::ColorCalibrationMode);
    test_attr!(color_consistency, "ColorConsistency", Attribute::ColorConsistency);
    test_attr!(color_control, "ColorControl", Attribute::ColorControl);
    test_attr!(color_model_mode, "ColorModelMode", Attribute::ColorModelMode);
    test_attr!(color_settings_reset, "ColorSettingsReset", Attribute::ColorSettingsReset);
    test_attr!(color_uniformity, "ColorUniformity", Attribute::ColorUniformity);
    test_attr!(cri_mode, "CRIMode", Attribute::CriMode);
    test_attr!(custom_color, "CustomColor", Attribute::CustomColor);
    test_attr!(uv_stability, "UVStability", Attribute::UvStability);
    test_attr!(wavelength_correction, "WavelengthCorrection", Attribute::WavelengthCorrection);
    test_attr!(white_count, "WhiteCount", Attribute::WhiteCount);
    test_attr!(strobe_mode, "StrobeMode", Attribute::StrobeMode);
    test_attr!(zoom_mode, "ZoomMode", Attribute::ZoomMode);
    test_attr!(focus_mode, "FocusMode", Attribute::FocusMode);
    test_attr!(iris_mode, "IrisMode", Attribute::IrisMode);
    test_attr!(fan_n_mode, "Fan1Mode", Attribute::FanMode(1));
    test_attr!(follow_spot_mode, "FollowSpotMode", Attribute::FollowSpotMode);
    test_attr!(beam_effect_index_rotate_mode, "BeamEffectIndexRotateMode", Attribute::BeamEffectIndexRotateMode);
    test_attr!(intensity_m_speed, "IntensityMSpeed", Attribute::IntensityMSpeed);
    test_attr!(position_m_speed, "PositionMSpeed", Attribute::PositionMSpeed);
    test_attr!(color_mix_m_speed, "ColorMixMSpeed", Attribute::ColorMixMSpeed);
    test_attr!(color_wheel_select_m_speed, "ColorWheelSelectMSpeed", Attribute::ColorWheelSelectMSpeed);
    test_attr!(gobo_wheel_n_m_speed, "GoboWheel1MSpeed", Attribute::GoboWheelMSpeed(1));
    test_attr!(iris_m_speed, "IrisMSpeed", Attribute::IrisMSpeed);
    test_attr!(prism_n_m_speed, "Prism1MSpeed", Attribute::PrismMSpeed(1));
    test_attr!(focus_m_speed, "FocusMSpeed", Attribute::FocusMSpeed);
    test_attr!(frost_n_m_speed, "Frost1MSpeed", Attribute::FrostMSpeed(1));
    test_attr!(zoom_m_speed, "ZoomMSpeed", Attribute::ZoomMSpeed);
    test_attr!(frame_m_speed, "FrameMSpeed", Attribute::FrameMSpeed);
    test_attr!(global_m_speed, "GlobalMSpeed", Attribute::GlobalMSpeed);
    test_attr!(reflector_adjust, "ReflectorAdjust", Attribute::ReflectorAdjust);
    test_attr!(fixture_global_reset, "FixtureGlobalReset", Attribute::FixtureGlobalReset);
    test_attr!(dimmer_reset, "DimmerReset", Attribute::DimmerReset);
    test_attr!(shutter_reset, "ShutterReset", Attribute::ShutterReset);
    test_attr!(beam_reset, "BeamReset", Attribute::BeamReset);
    test_attr!(color_mix_reset, "ColorMixReset", Attribute::ColorMixReset);
    test_attr!(color_wheel_reset, "ColorWheelReset", Attribute::ColorWheelReset);
    test_attr!(focus_reset, "FocusReset", Attribute::FocusReset);
    test_attr!(frame_reset, "FrameReset", Attribute::FrameReset);
    test_attr!(gobo_wheel_reset, "GoboWheelReset", Attribute::GoboWheelReset);
    test_attr!(intensity_reset, "IntensityReset", Attribute::IntensityReset);
    test_attr!(iris_reset, "IrisReset", Attribute::IrisReset);
    test_attr!(position_reset, "PositionReset", Attribute::PositionReset);
    test_attr!(pan_reset, "PanReset", Attribute::PanReset);
    test_attr!(tilt_reset, "TiltReset", Attribute::TiltReset);
    test_attr!(zoom_reset, "ZoomReset", Attribute::ZoomReset);
    test_attr!(ctb_reset, "CTBReset", Attribute::CtbReset);
    test_attr!(cto_reset, "CTOReset", Attribute::CtoReset);
    test_attr!(ctc_reset, "CTCReset", Attribute::CtcReset);
    test_attr!(animation_system_reset, "AnimationSystemReset", Attribute::AnimationSystemReset);
    test_attr!(fixture_calibration_reset, "FixtureCalibrationReset", Attribute::FixtureCalibrationReset);
    test_attr!(function, "Function", Attribute::Function);
    test_attr!(lamp_control, "LampControl", Attribute::LampControl);
    test_attr!(display_intensity, "DisplayIntensity", Attribute::DisplayIntensity);
    test_attr!(dmx_input, "DMXInput", Attribute::DmxInput);
    test_attr!(no_feature, "NoFeature", Attribute::NoFeature);
    test_attr!(blower_n, "Blower1", Attribute::Blower(1));
    test_attr!(fan_n, "Fan1", Attribute::Fan(1));
    test_attr!(fog_n, "Fog1", Attribute::Fog(1));
    test_attr!(haze_n, "Haze1", Attribute::Haze(1));
    test_attr!(lamp_power_mode, "LampPowerMode", Attribute::LampPowerMode);
    test_attr!(fans, "Fans", Attribute::Fans);
    test_attr!(blade_n_a, "Blade1A", Attribute::BladeA(1));
    test_attr!(blade_n_b, "Blade1B", Attribute::BladeB(1));
    test_attr!(blade_n_rot, "Blade1Rot", Attribute::BladeRot(1));
    test_attr!(shaper_rot, "ShaperRot", Attribute::ShaperRot);
    test_attr!(shaper_macros, "ShaperMacros", Attribute::ShaperMacros);
    test_attr!(shaper_macros_speed, "ShaperMacrosSpeed", Attribute::ShaperMacrosSpeed);
    test_attr!(blade_soft_n_a, "BladeSoft1A", Attribute::BladeSoftA(1));
    test_attr!(blade_soft_n_b, "BladeSoft1B", Attribute::BladeSoftB(1));
    test_attr!(key_stone_n_a, "KeyStone1A", Attribute::KeyStoneA(1));
    test_attr!(key_stone_n_b, "KeyStone1B", Attribute::KeyStoneB(1));
    test_attr!(video, "Video", Attribute::Video);
    test_attr!(video_effect_n_type, "VideoEffect1Type", Attribute::VideoEffectType(1));
    test_attr!(video_effect_n_parameter_m, "VideoEffect1Parameter2", Attribute::VideoEffectParameter(1, 2));
    test_attr!(video_camera_n, "VideoCamera1", Attribute::VideoCamera(1));
    test_attr!(video_sound_volume_n, "VideoSoundVolume1", Attribute::VideoSoundVolume(1));
    test_attr!(video_blend_mode, "VideoBlendMode", Attribute::VideoBlendMode);
    test_attr!(input_source, "InputSource", Attribute::InputSource);
    test_attr!(field_of_view, "FieldOfView", Attribute::FieldOfView);
    test_attr!(custom, "CustomAttribute", Attribute::Custom("CustomAttribute".to_string()));
}

/// A normalized value for lighting fixture attributes.
///
/// AttributeValue represents a floating-point value constrained to the range
/// [0.0, 1.0], commonly used for lighting parameters such as intensity, color
/// components, and other fixture attributes. All operations automatically clamp
/// values to this valid range.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(derive_more::Deref, derive_more::DerefMut)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct AttributeValue(f32);

impl AttributeValue {
    /// The minimum allowed value (0.0).
    pub const MIN: f32 = 0.0;

    /// The maximum allowed value (1.0).
    pub const MAX: f32 = 1.0;

    /// Creates a new AttributeValue with the specified value.
    ///
    /// The value is automatically clamped to the range [0.0, 1.0].
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN, Self::MAX))
    }

    /// Sets the value of this AttributeValue.
    ///
    /// The value is automatically clamped to the range [0.0, 1.0].
    #[inline]
    pub fn set(&mut self, value: f32) {
        self.0 = value.clamp(Self::MIN, Self::MAX);
    }

    /// Returns the underlying f32 value.
    ///
    /// The returned value is guaranteed to be in the range [0.0, 1.0].
    #[inline]
    pub fn as_f32(self) -> f32 {
        self.0
    }

    /// Performs linear interpolation between this value and another.
    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(Self::MIN, Self::MAX);
        Self::new(self.0 * (1.0 - t) + other.0 * t)
    }
}

impl From<f32> for AttributeValue {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<AttributeValue> for f32 {
    fn from(value: AttributeValue) -> Self {
        *value
    }
}

impl From<AttributeValue> for f64 {
    fn from(value: AttributeValue) -> Self {
        *value as f64
    }
}

impl From<gdtf::values::DmxValue> for AttributeValue {
    fn from(value: gdtf::values::DmxValue) -> Self {
        let len: u8 = value.bytes().into();
        let raw = value.to(len);
        let max_value = 2_u64.saturating_pow(len as u32 * 8) - 1;
        let floating_value = raw as f32 / max_value as f32;
        AttributeValue::new(floating_value)
    }
}

impl From<AttributeValue> for dmx::Value {
    fn from(value: AttributeValue) -> Self {
        dmx::Value((value.0 * (u8::MAX as f32)) as u8)
    }
}

impl FromStr for AttributeValue {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}

/// Represents a group of features. For example, the 'Pan' and 'Tilt' attributes
/// both control the position of a fixture, and so their feature group is
/// 'Position'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize)]
pub enum FeatureGroup {
    /// Dimmer feature group.
    Dimmer,
    /// Position feature group.
    Position,
    /// Gobo feature group.
    Gobo,
    /// Color feature group.
    Color,
    /// Beam feature group.
    Beam,
    /// Focus feature group.
    Focus,
    /// Control feature group.
    Control,
    /// Shapers feature group.
    Shapers,
    /// Video feature group.
    Video,
}

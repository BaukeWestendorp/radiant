//! # [Wheel Collect](https://gdtf.eu/gdtf/file-spec/wheel-collect/#wheel-collect)
//!
//! This section defines all physical or virtual wheels of the device.

use serde_inline_default::serde_inline_default;

use crate::{ColorCIE, Name, Node, Point, Resource, Rotation};

/// # [Wheel Collect](https://gdtf.eu/gdtf/file-spec/wheel-collect/#wheel-collect)
///
/// This section defines all physical or virtual wheels of the device. It
/// currently does not have any XML attributes (XML node `<Wheels>`).
///
/// As children wheel collect can have a list of a [Wheel].
///
/// Note 1: Physical or virtual wheels represent the changes to the light beam
/// within the device. Typically color, gobo, prism, animation, content and
/// others are described by wheels.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Wheels {
    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub wheels: Vec<Wheel>,
}

/// # [Wheel](https://gdtf.eu/gdtf/file-spec/wheel-collect/#wheel)
///
/// Each wheel describes a single physical or virtual wheel of the fixture type.
/// If the real device has wheels you can change, then all wheel configurations
/// have to be described. Wheel has the following XML node: `<Wheel>`. The
/// currently defined XML attributes of the wheel are specified in
/// [table 11](https://gdtf.eu/gdtf/file-spec/wheel-collect/#table-11-wheel-attributes).
///
/// As children, Wheel has a list of a [WheelSlot].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Wheel {
    /// The unique name of the wheel.
    #[serde(rename = "Name")]
    pub name: Name,

    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub wheel_slots: Vec<WheelSlot>,
}

/// # [Wheel Slot](https://gdtf.eu/gdtf/file-spec/wheel-collect/#wheel-slot)
///
/// The wheel slot represents a slot on the wheel (XML node `<Slot>`). The
/// currently defined XML attributes of the wheel slot are specified in
/// [table 12](https://gdtf.eu/gdtf/file-spec/wheel-collect/#table-12-wheel-slot-attributes).
///
/// Note 1: More information on the definitions of images used in wheel slots to
/// visualize gobos, animation wheels or color wheels can be found in
/// [Annex E “Wheel Slot Image Definition”](https://gdtf.eu/gdtf/annex/annex-e/).
///
/// The link between a slot and a
/// [ChannelSet](super::dmx_modes::ChannelSet) is done via the
/// wheel slot index. The wheel slot index of a slot is derived from the order
/// of a wheel’s slots. The wheel slot index is normalized to 1.
///
/// If the wheel slot has a prism, it has to have one or several children called
/// [PrismFacet]. If the wheel slot has an AnimationWheel, it has to have one
/// child called [AnimationSystem].
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct WheelSlot {
    /// The unique name of the wheel slot.
    #[serde(rename = "Name")]
    pub name: Name,

    /// Color of the wheel slot, Default value: { 0.3127, 0.3290, 100.0 }
    /// (white) For Y give relative value compared to overall output defined
    /// in property Luminous Flux of related Beam Geometry (transmissive
    /// case).
    #[serde(rename = "Color")]
    #[serde_inline_default(ColorCIE::new(0.3127, 0.3290, 100.0))]
    pub color: ColorCIE,

    /// Optional. Link to filter in the physical description; Do not define
    /// color if filter is used; Starting point: Filter Collect
    #[serde(rename = "Filter")]
    pub filter: Option<Node>,

    /// Optional. PNG file name without extension containing image for specific
    /// gobos etc.
    ///
    /// - Maximum resolution of picture: 1024x1024
    /// - Recommended resolution of gobo: 256x256
    /// - Recommended resolution of animation wheel: 256x256
    ///
    /// These resource files are located in a folder called ./wheels in the zip
    /// archive. Default value: empty.
    #[serde(rename = "MediaFileName")]
    pub media_file_name: Option<Resource>,

    /// If the wheel slot has a prism, it has to have one or several children
    /// called prism facet. If the wheel slot has an AnimationWheel, it has to
    /// have one child called Animation Wheel.
    #[serde(rename = "$value", default = "Vec::new")]
    pub content: Vec<WheelSlotContent>,
}

/// Either one or more prism facet or a single animation system.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum WheelSlotContent {
    /// Prism facet
    Facet(PrismFacet),
    /// Animation system
    AnimationSystem(AnimationSystem),
}

/// # Prism Facet
///
/// This section can only be defined for the prism wheel slot and has a
/// description for the prism facet (XML node `<Facet>`). The currently defined
/// XML attributes of the prism facet are specified in
/// [table 13](https://gdtf.eu/gdtf/file-spec/wheel-collect/#table-13-wheel-slot-attributes).
///
/// The prism facet cannot have any children.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde_inline_default]
pub struct PrismFacet {
    /// Color of prism facet, Default value: {0.3127, 0.3290, 100.0 } (white)
    #[serde(rename = "Color")]
    #[serde_inline_default(ColorCIE::new(0.3127, 0.3290, 100.0))]
    pub color: ColorCIE,

    /// Specify the rotation, translation and scaling for the facet.
    #[serde(rename = "Rotation")]
    pub rotation: Rotation,
}

/// # Animation System
///
/// This section can only be defined for the animation system disk and it
/// describes the animation system behavior (XML node `<AnimationSystem>`). The
/// currently defined XML attributes of the AnimationSystem are specified in
/// [table 14](https://gdtf.eu/gdtf/file-spec/wheel-collect/#table-14-animationsystem-attributes).
///
/// The AnimationSystem cannot have any children.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct AnimationSystem {
    /// First Point of the Spline describing the path of animation system in the
    /// beam in relation to the middle of the Media File; Array of two floats;
    /// Separator of values is “,”; First Float is X-axis and second is Y-axis.
    #[serde(rename = "P1")]
    pub p1: Point,

    /// Second Point of the Spline describing the path of animation system in
    /// the beam in relation to the middle of the Media File; Array of two
    /// floats; Separator of values is “,”; First Float is X-axis and second is
    /// Y-axis.
    #[serde(rename = "P2")]
    pub p2: Point,

    /// Third Point of the Spline describing the path of animation system in the
    /// beam in relation to the middle of the Media File; Array of two floats;
    /// Separator of values is “,”; First Float is X-axis and second is Y-axis.
    #[serde(rename = "P3")]
    pub p3: Point,

    /// Radius of the circle that defines the section of the animation system
    /// which will be shown in the beam
    #[serde(rename = "Radius")]
    pub radius: f32,
}

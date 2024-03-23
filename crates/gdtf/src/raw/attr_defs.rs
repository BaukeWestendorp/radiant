use serde_inline_default::serde_inline_default;

use crate::raw::{RawColorCIE, RawEnum, RawName, RawNode};

/// # [Attribute Definitions](https://gdtf.eu/gdtf/file-spec/attribute-definitions/)
///
/// Note 1: More information on the definitions of attributes can be found in
/// [Annex A “Attribute Definitions”](https://gdtf.eu/gdtf/annex/annex-a/).
///
/// Note 2: All currently defined Fixture Type Attributes can be found in [Annex
/// B “Attribute Listing”](https://gdtf.eu/gdtf/annex/annex-b/).
///
/// Note 3: All currently defined activation groups can be found in [Annex B
/// “Attribute Listing”](https://gdtf.eu/gdtf/annex/annex-b/).
///
/// Note 4: All currently defined feature groups can be found in [Annex B
/// “Attribute Listing”](https://gdtf.eu/gdtf/annex/annex-b/).
///
/// The current attribute definition node does not have any XML attributes (XML
/// node `<AttributeDefinitions>`).
///
/// Children of the attribute definition are specified in
/// [table 5](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#table-5).
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawAttributeDefinitions {
    /// Defines which attributes are to be activated together. For example, Pan
    /// and Tilt are in the same activation group.
    #[serde(rename = "ActivationGroups")]
    pub activation_groups: Option<RawActivationGroups>,

    /// Describes the logical grouping of attributes. For example, Gobo 1 and
    /// Gobo 2 are grouped in the feature Gobo of the feature group Gobo.
    #[serde(rename = "FeatureGroups")]
    pub feature_groups: RawFeatureGroups,

    /// List of Fixture Type Attributes that are used. Predefindes fixtury type
    /// attributes can be found in [Annex A](https://gdtf.eu/gdtf/annex/annex-a/).
    #[serde(rename = "Attributes")]
    pub attributes: RawAttributes,
}

/// # [Activation Groups](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#activation-groups)
///
/// This section defines groups of Fixture Type Attributes that are intended to
/// be used together.
///
/// Example: Usually Pan and Tilt are Fixture Type Attributes that shall be
/// activated together to be able to store and recreate any position.
///
/// The current activation groups node does not have any XML attributes (XML
/// node `<ActivationGroups>`).
///
/// As children it can have a list of a [RawActivationGroup].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawActivationGroups {
    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub groups: Vec<RawActivationGroup>,
}

/// # [Activation Group](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#activation-group)
///
/// This section defines the activation group Attributes (XML node
/// `<ActivationGroup>`). Currently defined XML attributes of the activation
/// group are specified in
/// [table 6](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#table-6-activation-group-attributes).
///
/// The activation group does not have any children.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawActivationGroup {
    /// The unique name of the activation group.
    #[serde(rename = "Name")]
    pub name: RawName,
}

/// # [Feature Groups](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#feature-groups)
///
/// This section defines the logical grouping of Fixture Type Attributes (XML
/// node `<FeatureGroups>`).
///
/// Note 1: A feature group can contain more than one
/// logical control unit. A feature group Position shall contain PanTilt and XYZ
/// as separate Feature.
///
/// Note 2: Usually Pan and Tilt create a logical unit to
/// enable position control, so they must be grouped in a Feature PanTilt.
///
/// As children the feature groups has a list of a [RawFeatureGroup].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawFeatureGroups {
    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub groups: Vec<RawFeatureGroup>,
}

/// # [Feature Group](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#feature-group)
///
/// This section defines the feature group (XML node `<FeatureGroup>`). The
/// currently defined XML attributes of the feature group are specified in
/// [table 7](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#table-7-feature-group-attributes).
///
/// As children the feature group has a list of a [RawFeature].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawFeatureGroup {
    /// The unique name of the feature group.
    #[serde(rename = "Name")]
    pub name: RawName,

    /// The pretty name of the feature group.
    #[serde(rename = "Pretty")]
    pub pretty: Option<String>,

    #[serde(rename = "$value", default = "Vec::new")]
    pub features: Vec<RawFeature>,
}

/// # [Feature](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#table-8-feature-attributes)
///
/// This section defines the feature (XML node `<Feature>`). The currently
/// defined XML attributes of the feature are specified in
/// [table 8](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#table-8-feature-attributes).
///
/// The feature does not have any children.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawFeature {
    /// The unique name of the feature.
    #[serde(rename = "Name")]
    pub name: RawName,
}

/// # [Attributes](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#attributes)
///
/// This section defines the Fixture Type Attributes (XML node `<Attributes>`).
///
/// As children the attributes node has a list of a [RawAttribute].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawAttributes {
    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub attributes: Vec<RawAttribute>,
}

/// # [Attribute](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#attribute)
///
/// This section defines the Fixture Type Attribute (XML node `<Attribute>`).
/// The currently defined XML attributes of the attribute Node are specified in
/// [table 9](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#table-9-xml-attributes-of-the-attribute).
///
/// As children the attribute node has a list of a [RawSubphysicalUnit].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawAttribute {
    /// The unique name of the attribute.
    #[serde(rename = "Name")]
    pub name: RawName,

    /// The pretty name of the attribute.
    #[serde(rename = "Pretty")]
    pub pretty: Option<String>,

    /// Optional link to the activation group. The starting point is the
    /// [ActivationGroups] node.
    #[serde(rename = "ActivationGroup")]
    pub activation_group: Option<RawNode>,

    /// Link to the corresponding feature. The starting point is the
    /// [FeatureGroups] node.
    #[serde(rename = "Feature")]
    pub feature: RawNode,

    /// Optional link to the main attribute. The starting point is the
    /// [Attribute] node.
    #[serde(rename = "MainAttribute")]
    pub main_attribute: Option<RawNode>,

    /// Default value: None
    #[serde(rename = "PhysicalUnit")]
    pub physical_unit: RawEnum,

    /// Optional. Defines the color for the attribute.
    #[serde(rename = "Color")]
    pub color: Option<RawColorCIE>,

    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub subphysical_units: Vec<RawSubphysicalUnit>,
}

/// # [Subphysical Unit](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#table-10-xml-attributes-of-the-subphysical-unit)
///
/// This section defines the Attribute Subphysical Unit (XML node
/// `<SubPhysicalUnit>`). The currently defined XML attributes of the
/// subphysical unit are specified in
/// [table 10](https://gdtf.eu/gdtf/file-spec/attribute-definitions/#table-10-xml-attributes-of-the-subphysical-unit).
///
/// The subphysical unit does not have any children.
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawSubphysicalUnit {
    /// The subphysical unit type.
    #[serde(rename = "Type")]
    pub r#type: RawEnum,

    /// Default value: None
    #[serde(rename = "PhysicalUnit")]
    pub physical_unit: RawEnum,

    /// The default physical from of the subphysical unit; Unit: as defined in
    /// PhysicalUnit; Default value: 0
    #[serde(rename = "PhysicalFrom")]
    #[serde_inline_default(0.0)]
    pub physical_from: f32,

    /// The default physical to of the subphysical unit; Unit: as defined in
    /// PhysicalUnit; Default value: 1
    #[serde(rename = "PhysicalTo")]
    #[serde_inline_default(1.0)]
    pub physical_to: f32,
}

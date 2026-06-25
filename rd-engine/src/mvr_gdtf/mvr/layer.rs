use core::str;
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    path::PathBuf,
    str::FromStr as _,
};

use uuid::Uuid;

use crate::mvr_gdtf::{
    CieColor, DmxAddress, gdtf,
    mvr::{
        self, NodeId, ResourceKey,
        aux::{Class, MappingDefinition, Position},
        bundle::{self, FromBundle as _},
        geo::Geometry,
    },
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    id: mvr::NodeId<Layer>,
    name: String,
    local_transform: glam::Affine3A,

    objects: Vec<Object>,
}

impl Layer {
    pub fn id(&self) -> NodeId<Self> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
}

impl bundle::FromBundle for Layer {
    type Source = bundle::Layer;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let uuid: Uuid = Uuid::from_str(&source.uuid).unwrap();
        let objects = source
            .child_list
            .as_ref()
            .map(|cl| cl.content.iter().map(|child| build_object(child, bundle)).collect())
            .unwrap_or_default();

        Layer {
            id: uuid.into(),
            name: source.name.clone(),
            local_transform: util::parse_affine3a_or_identity(source.matrix.as_deref()),
            objects,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    id: mvr::NodeId<Object>,
    name: String,
    class: Option<mvr::NodeId<Class>>,
    local_transform: glam::Affine3A,

    kind: ObjectKind,
}

impl Object {
    pub fn id(&self) -> NodeId<Self> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> Option<mvr::NodeId<Class>> {
        self.class
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn kind(&self) -> &ObjectKind {
        &self.kind
    }

    pub fn as_scene_object(&self) -> Option<&SceneObject> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_group_object(&self) -> Option<&GroupObject> {
        match &self.kind {
            ObjectKind::GroupObject(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_focus_point_object(&self) -> Option<&FocusPointObject> {
        match &self.kind {
            ObjectKind::FocusPoint(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_fixture_object(&self) -> Option<&FixtureObject> {
        match &self.kind {
            ObjectKind::Fixture(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_support_object(&self) -> Option<&SupportObject> {
        match &self.kind {
            ObjectKind::Support(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_truss_object(&self) -> Option<&TrussObject> {
        match &self.kind {
            ObjectKind::Truss(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_video_screen_object(&self) -> Option<&VideoScreenObject> {
        match &self.kind {
            ObjectKind::VideoScreen(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_projector_object(&self) -> Option<&ProjectorObject> {
        match &self.kind {
            ObjectKind::Projector(v) => Some(v),
            _ => None,
        }
    }

    pub fn identifier(&self) -> Option<&ObjectIdentifier> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.id()),
            ObjectKind::Fixture(v) => Some(v.id()),
            ObjectKind::Support(v) => Some(v.id()),
            ObjectKind::Truss(v) => Some(v.id()),
            ObjectKind::VideoScreen(v) => Some(v.id()),
            ObjectKind::Projector(v) => Some(v.id()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn gdtf_info(&self) -> Option<&GdtfInfo> {
        match &self.kind {
            ObjectKind::SceneObject(v) => v.gdtf(),
            ObjectKind::Fixture(v) => v.gdtf(),
            ObjectKind::Support(v) => v.gdtf(),
            ObjectKind::Truss(v) => v.gdtf(),
            ObjectKind::VideoScreen(v) => v.gdtf(),
            ObjectKind::Projector(v) => v.gdtf(),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn geometries(&self) -> Option<&[Geometry]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.geometries()),
            ObjectKind::FocusPoint(v) => Some(v.geometries()),
            ObjectKind::Support(v) => Some(v.geometries()),
            ObjectKind::Projector(v) => Some(v.geometries()),
            ObjectKind::Truss(v) => Some(v.geometries()),
            ObjectKind::VideoScreen(v) => Some(v.geometries()),
            ObjectKind::GroupObject(_) | ObjectKind::Fixture(_) => None,
        }
    }

    pub fn child_objects(&self) -> Option<&[Object]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.child_objects()),
            ObjectKind::GroupObject(v) => Some(v.child_objects()),
            ObjectKind::Fixture(v) => Some(v.child_objects()),
            ObjectKind::Support(v) => Some(v.child_objects()),
            ObjectKind::Truss(v) => Some(v.child_objects()),
            ObjectKind::VideoScreen(v) => Some(v.child_objects()),
            ObjectKind::Projector(v) => Some(v.child_objects()),
            ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn has_children(&self) -> bool {
        self.child_objects().is_some_and(|c| !c.is_empty())
    }

    pub fn dmx_addresses(&self) -> Option<&[DmxAddress]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.dmx_addresses()),
            ObjectKind::Fixture(v) => Some(v.dmx_addresses()),
            ObjectKind::Support(v) => Some(v.dmx_addresses()),
            ObjectKind::Truss(v) => Some(v.dmx_addresses()),
            ObjectKind::VideoScreen(v) => Some(v.dmx_addresses()),
            ObjectKind::Projector(v) => Some(v.dmx_addresses()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn network_addresses(&self) -> Option<&[NetworkAddress]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.network_addresses()),
            ObjectKind::Fixture(v) => Some(v.network_addresses()),
            ObjectKind::Support(v) => Some(v.network_addresses()),
            ObjectKind::Truss(v) => Some(v.network_addresses()),
            ObjectKind::VideoScreen(v) => Some(v.network_addresses()),
            ObjectKind::Projector(v) => Some(v.network_addresses()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn alignments(&self) -> Option<&[Alignment]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.alignments()),
            ObjectKind::Fixture(v) => Some(v.alignments()),
            ObjectKind::Support(v) => Some(v.alignments()),
            ObjectKind::Truss(v) => Some(v.alignments()),
            ObjectKind::VideoScreen(v) => Some(v.alignments()),
            ObjectKind::Projector(v) => Some(v.alignments()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn custom_commands(&self) -> Option<&[CustomCommand]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.custom_commands()),
            ObjectKind::Fixture(v) => Some(v.custom_commands()),
            ObjectKind::Support(v) => Some(v.custom_commands()),
            ObjectKind::Truss(v) => Some(v.custom_commands()),
            ObjectKind::VideoScreen(v) => Some(v.custom_commands()),
            ObjectKind::Projector(v) => Some(v.custom_commands()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn overwrites(&self) -> Option<&[Overwrite]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.overwrites()),
            ObjectKind::Fixture(v) => Some(v.overwrites()),
            ObjectKind::Support(v) => Some(v.overwrites()),
            ObjectKind::Truss(v) => Some(v.overwrites()),
            ObjectKind::VideoScreen(v) => Some(v.overwrites()),
            ObjectKind::Projector(v) => Some(v.overwrites()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn connections(&self) -> Option<&[Connection]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.connections()),
            ObjectKind::Fixture(v) => Some(v.connections()),
            ObjectKind::Support(v) => Some(v.connections()),
            ObjectKind::Truss(v) => Some(v.connections()),
            ObjectKind::VideoScreen(v) => Some(v.connections()),
            ObjectKind::Projector(v) => Some(v.connections()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectKind {
    SceneObject(SceneObject),
    GroupObject(GroupObject),
    FocusPoint(FocusPointObject),
    Fixture(FixtureObject),
    Support(SupportObject),
    Truss(TrussObject),
    VideoScreen(VideoScreenObject),
    Projector(ProjectorObject),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneObject {
    id: ObjectIdentifier,
    gdtf: Option<GdtfInfo>,

    cast_shadow: bool,
    unit_number: Option<i32>,

    dmx_addresses: Vec<DmxAddress>,
    network_addresses: Vec<NetworkAddress>,
    alignments: Vec<Alignment>,
    custom_commands: Vec<CustomCommand>,
    overwrites: Vec<Overwrite>,
    connections: Vec<Connection>,

    geometries: Vec<Geometry>,
    child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl SceneObject {
    // FIXME: Find a better way to get the data from ObjectIdentifier
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

impl bundle::FromBundle for SceneObject {
    type Source = bundle::SceneObject;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let id = build_id_from_multipatch(
            &source.multipatch,
            source.fixture_id.to_owned(),
            source.fixture_id_numeric,
            source.custom_id,
            source.custom_id_type,
        );

        Self {
            id,
            gdtf: build_gdtf_info(&source.gdtf_spec, &source.gdtf_mode),
            cast_shadow: source.cast_shadow.unwrap_or_default(),
            unit_number: source.unit_number,
            dmx_addresses: build_dmx_addresses(source.addresses.as_ref(), bundle),
            network_addresses: build_network_addresses(source.addresses.as_ref(), bundle),
            alignments: build_alignments(source.alignments.as_ref(), bundle),
            custom_commands: build_custom_commands(source.custom_commands.as_ref()),
            overwrites: build_overwrites(source.overwrites.as_ref(), bundle),
            connections: build_connections(source.connections.as_ref(), bundle),
            geometries: mvr::build_geometries(
                &source.geometries.geometry_3d,
                &source.geometries.symbol,
                bundle,
            ),
            child_objects: build_child_objects(source.child_list.as_deref(), bundle),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupObject {
    child_objects: Vec<Object>,
}

impl GroupObject {
    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

impl bundle::FromBundle for GroupObject {
    type Source = bundle::GroupObject;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            child_objects: source
                .child_list
                .content
                .iter()
                .map(|child| build_object(child, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FocusPointObject {
    geometries: Vec<Geometry>,
}

impl FocusPointObject {
    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }
}

impl bundle::FromBundle for FocusPointObject {
    type Source = bundle::FocusPoint;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        FocusPointObject {
            geometries: mvr::build_geometries(
                &source.geometries.geometry_3d,
                &source.geometries.symbol,
                bundle,
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureObject {
    id: ObjectIdentifier,
    gdtf: Option<GdtfInfo>,

    cast_shadow: bool,
    child_position: Option<gdtf::NodePath>,
    color: Option<CieColor>,
    dmx_invert_pan: bool,
    dmx_invert_tilt: bool,
    focus: Option<mvr::NodeId<Object>>,
    function: Option<String>,
    gobo: Option<Gobo>,
    mappings: Vec<Mapping>,
    position: Option<mvr::NodeId<Position>>,
    protocols: Vec<Protocol>,
    unit_number: Option<i32>,

    dmx_addresses: Vec<DmxAddress>,
    network_addresses: Vec<NetworkAddress>,
    alignments: Vec<Alignment>,
    custom_commands: Vec<CustomCommand>,
    overwrites: Vec<Overwrite>,
    connections: Vec<Connection>,

    child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl FixtureObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn id_u32(&self) -> anyhow::Result<u32> {
        match &self.id {
            ObjectIdentifier::Multipatch(_) => {
                anyhow::bail!("Multipatch node IDs are not yet supported for fixture");
            }
            ObjectIdentifier::Single {
                fixture_id,
                fixture_id_numeric,
                custom_id,
                custom_id_type: _,
            } => {
                if let Some(numeric_id) = fixture_id_numeric {
                    if *numeric_id < 0 {
                        anyhow::bail!("Negative numeric fixture ID for fixture");
                    }
                    let numeric_id_u32 = u32::try_from(*numeric_id).map_err(|_| {
                        anyhow::anyhow!("Fixture ID out of range for u32 for fixture.")
                    })?;
                    Ok(numeric_id_u32)
                } else if let Some(id_str) = fixture_id {
                    match id_str.parse::<u32>() {
                        Ok(parsed_id) => Ok(parsed_id),
                        Err(_) => {
                            anyhow::bail!("Could not parse fixture_id '{}' as u32.", id_str);
                        }
                    }
                } else if let Some(custom_id) = custom_id {
                    if *custom_id < 0 {
                        anyhow::bail!("Negative numeric custom ID for fixture");
                    }
                    let custom_id_u32 = u32::try_from(*custom_id).map_err(|_| {
                        anyhow::anyhow!("Custom ID out of range for u32 for fixture.")
                    })?;
                    Ok(custom_id_u32)
                } else {
                    anyhow::bail!("No valid fixture ID found for fixture");
                }
            }
        }
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    // FIXME: Add direct getter from `Mvr` for this.
    pub fn child_position(&self) -> Option<&gdtf::NodePath> {
        self.child_position.as_ref()
    }

    pub fn color(&self) -> Option<CieColor> {
        self.color
    }

    pub fn dmx_invert_pan(&self) -> bool {
        self.dmx_invert_pan
    }

    pub fn dmx_invert_tilt(&self) -> bool {
        self.dmx_invert_tilt
    }

    pub fn focus_point(&self) -> Option<mvr::NodeId<Object>> {
        self.focus
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn gobo(&self) -> Option<&Gobo> {
        self.gobo.as_ref()
    }

    pub fn mappings(&self) -> &[Mapping] {
        &self.mappings
    }

    pub fn position(&self) -> Option<mvr::NodeId<Position>> {
        self.position
    }

    pub fn protocols(&self) -> &[Protocol] {
        &self.protocols
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

impl bundle::FromBundle for FixtureObject {
    type Source = bundle::Fixture;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            id: build_id_from_multipatch(
                &source.multipatch,
                source.fixture_id.clone(),
                source.fixture_id_numeric,
                source.custom_id,
                source.custom_id_type,
            ),
            gdtf: build_gdtf_info(&source.gdtf_spec, &source.gdtf_mode),
            cast_shadow: source.cast_shadow.unwrap_or_default(),
            child_position: source
                .child_position
                .as_ref()
                .map(|s| gdtf::NodePath::from_str(s).unwrap()),
            color: source.color.as_ref().map(|s| CieColor::from_str(s).unwrap()),
            dmx_invert_pan: source.dmx_invert_pan.unwrap_or(false),
            dmx_invert_tilt: source.dmx_invert_tilt.unwrap_or(false),
            focus: source.focus.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            function: source.function.clone(),
            gobo: source.gobo.as_ref().map(|g| Gobo::from_bundle(g, bundle)),
            mappings: source
                .mappings
                .as_ref()
                .map(|mappings| {
                    mappings.mapping.iter().map(|m| Mapping::from_bundle(m, bundle)).collect()
                })
                .unwrap_or_default(),
            position: source.position.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            protocols: source
                .protocols
                .as_ref()
                .map(|protocols| {
                    protocols.protocol.iter().map(|p| Protocol::from_bundle(p, bundle)).collect()
                })
                .unwrap_or_default(),
            unit_number: Some(source.unit_number),
            dmx_addresses: build_dmx_addresses(source.addresses.as_ref(), bundle),
            network_addresses: build_network_addresses(source.addresses.as_ref(), bundle),
            alignments: build_alignments(source.alignments.as_ref(), bundle),
            custom_commands: build_custom_commands(source.custom_commands.as_ref()),
            overwrites: build_overwrites(source.overwrites.as_ref(), bundle),
            connections: build_connections(source.connections.as_ref(), bundle),
            child_objects: source
                .child_list
                .as_ref()
                .map(|cl| cl.content.iter().map(|child| build_object(child, bundle)).collect())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportObject {
    gdtf: Option<GdtfInfo>,
    id: ObjectIdentifier,

    cast_shadow: bool,
    chain_length: f32,
    function: Option<String>,
    position: Option<mvr::NodeId<Position>>,
    unit_number: Option<i32>,

    dmx_addresses: Vec<DmxAddress>,
    network_addresses: Vec<NetworkAddress>,
    alignments: Vec<Alignment>,
    custom_commands: Vec<CustomCommand>,
    overwrites: Vec<Overwrite>,
    connections: Vec<Connection>,

    geometries: Vec<Geometry>,
    child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl SupportObject {
    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    pub fn chain_length(&self) -> f32 {
        self.chain_length
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn position(&self) -> Option<mvr::NodeId<Position>> {
        self.position
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

impl bundle::FromBundle for SupportObject {
    type Source = bundle::Support;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            gdtf: build_gdtf_info(&source.gdtf_spec, &source.gdtf_mode),
            id: build_id_from_multipatch(
                &source.multipatch,
                source.fixture_id.clone(),
                source.fixture_id_numeric,
                source.custom_id,
                source.custom_id_type,
            ),
            cast_shadow: source.cast_shadow.unwrap_or_default(),
            chain_length: source.chain_length,
            function: source.function.clone(),
            position: source.position.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            unit_number: source.unit_number,
            dmx_addresses: build_dmx_addresses(source.addresses.as_ref(), bundle),
            network_addresses: build_network_addresses(source.addresses.as_ref(), bundle),
            alignments: build_alignments(source.alignments.as_ref(), bundle),
            custom_commands: build_custom_commands(source.custom_commands.as_ref()),
            overwrites: build_overwrites(source.overwrites.as_ref(), bundle),
            connections: build_connections(source.connections.as_ref(), bundle),
            geometries: mvr::build_geometries(
                &source.geometries.geometry_3d,
                &source.geometries.symbol,
                bundle,
            ),
            child_objects: source
                .child_list
                .as_ref()
                .map(|cl| cl.content.iter().map(|child| build_object(child, bundle)).collect())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrussObject {
    id: ObjectIdentifier,
    gdtf: Option<GdtfInfo>,

    cast_shadow: bool,
    child_position: Option<gdtf::NodePath>,
    function: Option<String>,
    position: Option<mvr::NodeId<Position>>,
    unit_number: Option<i32>,

    dmx_addresses: Vec<DmxAddress>,
    network_addresses: Vec<NetworkAddress>,
    alignments: Vec<Alignment>,
    custom_commands: Vec<CustomCommand>,
    overwrites: Vec<Overwrite>,
    connections: Vec<Connection>,

    geometries: Vec<Geometry>,
    child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl TrussObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    // FIXME: Add direct getter from `Mvr` for this.
    pub fn child_position(&self) -> Option<&gdtf::NodePath> {
        self.child_position.as_ref()
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn position(&self) -> Option<mvr::NodeId<Position>> {
        self.position
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

impl bundle::FromBundle for TrussObject {
    type Source = bundle::Truss;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            gdtf: build_gdtf_info(&source.gdtf_spec, &source.gdtf_mode),
            id: build_id_from_multipatch(
                &source.multipatch,
                source.fixture_id.clone(),
                source.fixture_id_numeric,
                source.custom_id,
                source.custom_id_type,
            ),
            cast_shadow: source.cast_shadow.unwrap_or_default(),
            child_position: source
                .child_position
                .as_ref()
                .map(|s| gdtf::NodePath::from_str(s).unwrap()),
            function: source.function.clone(),
            position: source.position.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            unit_number: source.unit_number,
            dmx_addresses: build_dmx_addresses(source.addresses.as_ref(), bundle),
            network_addresses: build_network_addresses(source.addresses.as_ref(), bundle),
            alignments: build_alignments(source.alignments.as_ref(), bundle),
            custom_commands: build_custom_commands(source.custom_commands.as_ref()),
            overwrites: build_overwrites(source.overwrites.as_ref(), bundle),
            connections: build_connections(source.connections.as_ref(), bundle),
            geometries: mvr::build_geometries(
                &source.geometries.geometry_3d,
                &source.geometries.symbol,
                bundle,
            ),
            child_objects: source
                .child_list
                .as_ref()
                .map(|cl| cl.content.iter().map(|child| build_object(child, bundle)).collect())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoScreenObject {
    id: ObjectIdentifier,
    gdtf: Option<GdtfInfo>,

    function: Option<String>,
    sources: Vec<Source>,
    cast_shadow: bool,

    dmx_addresses: Vec<DmxAddress>,
    network_addresses: Vec<NetworkAddress>,
    alignments: Vec<Alignment>,
    custom_commands: Vec<CustomCommand>,
    overwrites: Vec<Overwrite>,
    connections: Vec<Connection>,

    geometries: Vec<Geometry>,
    child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl VideoScreenObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn sources(&self) -> &[Source] {
        &self.sources
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

impl bundle::FromBundle for VideoScreenObject {
    type Source = bundle::VideoScreen;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            gdtf: build_gdtf_info(&source.gdtf_spec, &source.gdtf_mode),
            id: build_id_from_multipatch(
                &source.multipatch,
                source.fixture_id.clone(),
                source.fixture_id_numeric,
                source.custom_id,
                source.custom_id_type,
            ),
            function: source.function.clone(),
            sources: source
                .sources
                .as_ref()
                .map(|sources| {
                    sources.source.iter().map(|s| Source::from_bundle(s, bundle)).collect()
                })
                .unwrap_or_default(),
            dmx_addresses: build_dmx_addresses(source.addresses.as_ref(), bundle),
            network_addresses: build_network_addresses(source.addresses.as_ref(), bundle),
            alignments: build_alignments(source.alignments.as_ref(), bundle),
            custom_commands: build_custom_commands(source.custom_commands.as_ref()),
            overwrites: build_overwrites(source.overwrites.as_ref(), bundle),
            connections: build_connections(source.connections.as_ref(), bundle),
            cast_shadow: source.cast_shadow.unwrap_or_default(),
            geometries: mvr::build_geometries(
                &source.geometries.geometry_3d,
                &source.geometries.symbol,
                bundle,
            ),
            child_objects: source
                .child_list
                .as_ref()
                .map(|cl| cl.content.iter().map(|child| build_object(child, bundle)).collect())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectorObject {
    id: ObjectIdentifier,
    gdtf: Option<GdtfInfo>,

    cast_shadow: bool,
    projections: Vec<Projection>,
    unit_number: Option<i32>,

    dmx_addresses: Vec<DmxAddress>,
    network_addresses: Vec<NetworkAddress>,
    alignments: Vec<Alignment>,
    custom_commands: Vec<CustomCommand>,
    overwrites: Vec<Overwrite>,
    connections: Vec<Connection>,

    geometries: Vec<Geometry>,
    child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl ProjectorObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    pub fn projections(&self) -> &[Projection] {
        &self.projections
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

impl bundle::FromBundle for ProjectorObject {
    type Source = bundle::Projector;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            gdtf: build_gdtf_info(&source.gdtf_spec, &source.gdtf_mode),
            id: build_id_from_multipatch(
                &source.multipatch,
                source.fixture_id.clone(),
                source.fixture_id_numeric,
                source.custom_id,
                source.custom_id_type,
            ),
            cast_shadow: source.cast_shadow.unwrap_or_default(),
            projections: source
                .projections
                .projection
                .iter()
                .filter_map(|p| {
                    let source = p.source.as_ref().map(|s| Source::from_bundle(s, bundle));
                    let scale_handling = p
                        .scale_handeling
                        .map(|sh| ScaleHandling::from_bundle(&sh, bundle))
                        .unwrap_or_default();
                    Some(Projection { source, scale_handling })
                })
                .collect(),
            unit_number: source.unit_number,
            dmx_addresses: build_dmx_addresses(source.addresses.as_ref(), bundle),
            network_addresses: build_network_addresses(source.addresses.as_ref(), bundle),
            alignments: build_alignments(source.alignments.as_ref(), bundle),
            custom_commands: build_custom_commands(source.custom_commands.as_ref()),
            overwrites: build_overwrites(source.overwrites.as_ref(), bundle),
            connections: build_connections(source.connections.as_ref(), bundle),
            geometries: mvr::build_geometries(
                &source.geometries.geometry_3d,
                &source.geometries.symbol,
                bundle,
            ),
            child_objects: source
                .child_list
                .as_ref()
                .map(|cl| cl.content.iter().map(|child| build_object(child, bundle)).collect())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectIdentifier {
    Multipatch(mvr::NodeId<Object>),
    Single {
        fixture_id: Option<String>,
        fixture_id_numeric: Option<i32>,
        custom_id: Option<i32>,
        custom_id_type: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct GdtfInfo {
    resource: ResourceKey,
    mode: String,
}

impl GdtfInfo {
    pub fn new(resource: ResourceKey, mode: impl Into<String>) -> Self {
        Self { resource, mode: mode.into() }
    }

    pub fn resource(&self) -> &ResourceKey {
        &self.resource
    }

    pub fn mode(&self) -> &str {
        &self.mode
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkAddress {
    geometry: gdtf::NodePath,
    ipv4: Option<Ipv4Addr>,
    subnetmask: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
    dhcp: bool,
    hostname: Option<String>,
}

impl NetworkAddress {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn geometry(&self) -> &gdtf::NodePath {
        &self.geometry
    }

    pub fn ipv4(&self) -> Option<Ipv4Addr> {
        self.ipv4
    }

    pub fn subnetmask(&self) -> Option<Ipv4Addr> {
        self.subnetmask
    }

    pub fn ipv6(&self) -> Option<Ipv6Addr> {
        self.ipv6
    }

    pub fn dhcp(&self) -> bool {
        self.dhcp
    }

    pub fn hostname(&self) -> Option<&str> {
        self.hostname.as_deref()
    }
}

impl bundle::FromBundle for NetworkAddress {
    type Source = bundle::Network;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            geometry: gdtf::NodePath::from_str(&source.geometry).unwrap(),
            ipv4: source.ipv_4.as_ref().map(|s| Ipv4Addr::from_str(s).unwrap()),
            subnetmask: source.subnetmask.as_ref().map(|s| Ipv4Addr::from_str(s).unwrap()),
            ipv6: source.ipv_6.as_ref().map(|s| Ipv6Addr::from_str(s).unwrap()),
            dhcp: source.dhcp.as_ref().is_some_and(|s| s == "on"),
            hostname: source.hostname.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alignment {
    geometry: gdtf::NodePath,
    up: glam::Vec3A,
    direction: glam::Vec3A,
}

impl Alignment {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn geometry(&self) -> &gdtf::NodePath {
        &self.geometry
    }

    pub fn up(&self) -> glam::Vec3A {
        self.up
    }

    pub fn direction(&self) -> glam::Vec3A {
        self.direction
    }
}

impl bundle::FromBundle for Alignment {
    type Source = bundle::Alignment;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            geometry: gdtf::NodePath::from_str(
                source.geometry.as_ref().expect("FIXME: handle missing geometry"),
            )
            .unwrap(),
            up: util::parse_vec3(&source.up),
            direction: util::parse_vec3(&source.direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomCommand(String);

impl CustomCommand {
    pub fn command(&self) -> &str {
        &self.0
    }
}

impl str::FromStr for CustomCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomCommand(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Overwrite {
    universal: gdtf::NodePath,
    target: Option<gdtf::NodePath>,
}

impl Overwrite {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn universal(&self) -> &gdtf::NodePath {
        &self.universal
    }

    // FIXME: Add direct getter from `Mvr` for this.
    pub fn target(&self) -> Option<&gdtf::NodePath> {
        self.target.as_ref()
    }
}

impl bundle::FromBundle for Overwrite {
    type Source = bundle::Overwrite;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            universal: gdtf::NodePath::from_str(&source.universal).unwrap(),
            target: Some(gdtf::NodePath::from_str(&source.target).unwrap()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    own: gdtf::NodePath,
    other: gdtf::NodePath,
    to_object: mvr::NodeId<Object>,
}

impl Connection {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn own(&self) -> &gdtf::NodePath {
        &self.own
    }

    // FIXME: Add direct getter from `Mvr` for this.
    pub fn other(&self) -> &gdtf::NodePath {
        &self.other
    }

    pub fn to_object(&self) -> mvr::NodeId<Object> {
        self.to_object
    }
}

impl bundle::FromBundle for Connection {
    type Source = bundle::Connection;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            own: gdtf::NodePath::from_str(&source.own).unwrap(),
            other: gdtf::NodePath::from_str(&source.other).unwrap(),
            to_object: mvr::NodeId::from_str(&source.to_object).unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gobo {
    resource: ResourceKey,
    rotation: f32,
}

impl bundle::FromBundle for Gobo {
    type Source = bundle::Gobo;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self { resource: ResourceKey::new(&source.file_name), rotation: source.rotation }
    }
}

impl Gobo {
    pub fn resource(&self) -> &ResourceKey {
        &self.resource
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Protocol {
    geometry: gdtf::NodePath,
    name: String,
    r#type: Option<String>,
    version: Option<String>,
    transmission: Option<Transmission>,
}

impl Protocol {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn geometry(&self) -> &gdtf::NodePath {
        &self.geometry
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_(&self) -> Option<&str> {
        self.r#type.as_deref()
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn transmission(&self) -> Option<Transmission> {
        self.transmission
    }
}

impl bundle::FromBundle for Protocol {
    type Source = bundle::Protocol;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            geometry: gdtf::NodePath::from_str(&source.geometry)
                .unwrap_or_else(|_| gdtf::NodePath::from_str("NetworkInOut_1").unwrap()),
            name: source.name.clone(),
            r#type: if source.r#type.is_empty() { None } else { Some(source.r#type.clone()) },
            version: if source.version.is_empty() { None } else { Some(source.version.clone()) },
            transmission: source
                .transmission
                .as_ref()
                .map(|t| Transmission::from_bundle(&t, bundle)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transmission {
    Unicast,
    Multicast,
    Broadcast,
    Anycast,
}

impl bundle::FromBundle for Transmission {
    type Source = bundle::Transmission;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::Transmission::Unicast => Transmission::Unicast,
            bundle::Transmission::Multicast => Transmission::Multicast,
            bundle::Transmission::Broadcast => Transmission::Broadcast,
            bundle::Transmission::Anycast => Transmission::Anycast,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    linked_def: mvr::NodeId<MappingDefinition>,
    ux: i32,
    uy: i32,
    ox: i32,
    oy: i32,
    rz: f32,
}

impl Mapping {
    pub fn linked_def(&self) -> mvr::NodeId<MappingDefinition> {
        self.linked_def
    }

    pub fn ux(&self) -> i32 {
        self.ux
    }

    pub fn uy(&self) -> i32 {
        self.uy
    }

    pub fn ox(&self) -> i32 {
        self.ox
    }

    pub fn oy(&self) -> i32 {
        self.oy
    }

    pub fn rz(&self) -> f32 {
        self.rz
    }
}

impl bundle::FromBundle for Mapping {
    type Source = bundle::Mapping;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            linked_def: mvr::NodeId::from_str(&source.linked_def).unwrap(),
            ux: source.ux.unwrap_or_default(),
            uy: source.uy.unwrap_or_default(),
            ox: source.ox.unwrap_or_default(),
            oy: source.oy.unwrap_or_default(),
            rz: source.rz.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    linked_geometry: gdtf::NodePath,
    r#type: SourceType,
    value: String,
}

impl Source {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn linked_geometry(&self) -> &gdtf::NodePath {
        &self.linked_geometry
    }

    pub fn r#type(&self) -> SourceType {
        self.r#type
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl bundle::FromBundle for Source {
    type Source = bundle::Source;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            linked_geometry: gdtf::NodePath::from_str(&source.linked_geometry).unwrap(),
            r#type: SourceType::from_bundle(&source.r#type, bundle),
            value: source.content.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceType {
    Ndi,
    File,
    Citp,
    CaptureDevice,
}

impl bundle::FromBundle for SourceType {
    type Source = bundle::SourceType;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::SourceType::Ndi => SourceType::Ndi,
            bundle::SourceType::File => SourceType::File,
            bundle::SourceType::Citp => SourceType::Citp,
            bundle::SourceType::CaptureDevice => SourceType::CaptureDevice,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScaleHandling {
    #[default]
    ScaleKeepRatio,
    ScaleIgnoreRatio,
    KeepSizeCenter,
}

impl bundle::FromBundle for ScaleHandling {
    type Source = bundle::ScaleHandeling;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source.r#enum {
            bundle::Scale::ScaleKeepRatio => ScaleHandling::ScaleKeepRatio,
            bundle::Scale::ScaleIgnoreRatio => ScaleHandling::ScaleIgnoreRatio,
            bundle::Scale::KeepSizeCenter => ScaleHandling::KeepSizeCenter,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Projection {
    source: Option<Source>,
    scale_handling: ScaleHandling,
}

impl Projection {
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    pub fn scale_handling(&self) -> ScaleHandling {
        self.scale_handling
    }
}

impl bundle::FromBundle for Projection {
    type Source = bundle::Projection;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let src = source.source.as_ref().map(|s| Source::from_bundle(s, bundle));
        let scale_handling = source
            .scale_handeling
            .as_ref()
            .map(|sh| ScaleHandling::from_bundle(sh, bundle))
            .unwrap_or_default();
        Self { source: src, scale_handling }
    }
}

fn build_object(child: &bundle::ChildListContent, bundle: &bundle::Bundle) -> Object {
    let (uuid_str, name, matrix, classing, kind) = match child {
        bundle::ChildListContent::SceneObject(c) => (
            &c.uuid,
            &c.name,
            c.matrix.as_deref(),
            c.classing.as_ref(),
            ObjectKind::SceneObject(SceneObject::from_bundle(c, bundle)),
        ),
        bundle::ChildListContent::GroupObject(c) => (
            &c.uuid,
            &c.name,
            c.matrix.as_deref(),
            c.classing.as_ref(),
            ObjectKind::GroupObject(GroupObject::from_bundle(c, bundle)),
        ),
        bundle::ChildListContent::FocusPoint(c) => (
            &c.uuid,
            &c.name,
            c.matrix.as_deref(),
            c.classing.as_ref(),
            ObjectKind::FocusPoint(FocusPointObject::from_bundle(c, bundle)),
        ),
        bundle::ChildListContent::Fixture(c) => (
            &c.uuid,
            &c.name,
            c.matrix.as_deref(),
            c.classing.as_ref(),
            ObjectKind::Fixture(FixtureObject::from_bundle(c, bundle)),
        ),
        bundle::ChildListContent::Support(c) => (
            &c.uuid,
            &c.name,
            c.matrix.as_deref(),
            c.classing.as_ref(),
            ObjectKind::Support(SupportObject::from_bundle(c, bundle)),
        ),
        bundle::ChildListContent::Truss(c) => (
            &c.uuid,
            &c.name,
            c.matrix.as_deref(),
            c.classing.as_ref(),
            ObjectKind::Truss(TrussObject::from_bundle(c, bundle)),
        ),
        bundle::ChildListContent::VideoScreen(c) => (
            &c.uuid,
            &c.name,
            c.matrix.as_deref(),
            c.classing.as_ref(),
            ObjectKind::VideoScreen(VideoScreenObject::from_bundle(c, bundle)),
        ),
        bundle::ChildListContent::Projector(c) => (
            &c.uuid,
            &c.name,
            c.matrix.as_deref(),
            c.classing.as_ref(),
            ObjectKind::Projector(ProjectorObject::from_bundle(c, bundle)),
        ),
    };

    Object {
        id: Uuid::from_str(uuid_str).unwrap().into(),
        name: name.to_string(),
        class: classing.map(|classing| NodeId::new(Uuid::from_str(classing).unwrap())),
        local_transform: util::parse_affine3a_or_identity(matrix),
        kind,
    }
}

fn build_child_objects(
    child_list: Option<&bundle::ChildList>,
    bundle: &bundle::Bundle,
) -> Vec<Object> {
    child_list
        .map(|cl| cl.content.iter().map(|child| build_object(child, bundle)).collect())
        .unwrap_or_default()
}

fn build_dmx_addresses(
    addresses: Option<&bundle::Addresses>,
    bundle: &bundle::Bundle,
) -> Vec<DmxAddress> {
    addresses
        .map(|addrs| addrs.address.iter().map(|a| DmxAddress::from_bundle(a, bundle)).collect())
        .unwrap_or_default()
}

fn build_network_addresses(
    addresses: Option<&bundle::Addresses>,
    bundle: &bundle::Bundle,
) -> Vec<NetworkAddress> {
    addresses
        .map(|addrs| addrs.network.iter().map(|n| NetworkAddress::from_bundle(n, bundle)).collect())
        .unwrap_or_default()
}

fn build_alignments(
    alignments: Option<&bundle::Alignments>,
    bundle: &bundle::Bundle,
) -> Vec<Alignment> {
    alignments
        .map(|alignments| {
            alignments.alignment.iter().map(|a| Alignment::from_bundle(a, bundle)).collect()
        })
        .unwrap_or_default()
}

fn build_custom_commands(commands: Option<&bundle::CustomCommands>) -> Vec<CustomCommand> {
    commands
        .map(|commands| commands.custom_command.iter().map(|c| c.parse().unwrap()).collect())
        .unwrap_or_default()
}

fn build_overwrites(
    overwrites: Option<&bundle::Overwrites>,
    bundle: &bundle::Bundle,
) -> Vec<Overwrite> {
    overwrites
        .map(|ows| ows.overwrite.iter().map(|o| Overwrite::from_bundle(o, bundle)).collect())
        .unwrap_or_default()
}

fn build_connections(
    connections: Option<&bundle::Connections>,
    bundle: &bundle::Bundle,
) -> Vec<Connection> {
    connections
        .map(|conns| conns.connection.iter().map(|c| Connection::from_bundle(c, bundle)).collect())
        .unwrap_or_default()
}

fn build_gdtf_info(gdtf_spec: &Option<String>, gdtf_mode: &Option<String>) -> Option<GdtfInfo> {
    match (gdtf_spec, gdtf_mode) {
        (Some(spec), Some(mode)) => {
            let spec = spec.trim();
            let mode = mode.trim();

            if spec.is_empty() {
                return None;
            }

            Some(GdtfInfo {
                resource: ResourceKey::new(PathBuf::from(spec).with_extension("gdtf")),
                mode: mode.to_owned(),
            })
        }
        _ => None,
    }
}

fn build_id_from_multipatch(
    multipatch: &str,
    fixture_id: Option<String>,
    fixture_id_numeric: Option<i32>,
    custom_id: Option<i32>,
    custom_id_type: Option<i32>,
) -> ObjectIdentifier {
    match Uuid::from_str(multipatch) {
        Ok(uuid) => ObjectIdentifier::Multipatch(uuid.into()),
        Err(_) => {
            ObjectIdentifier::Single { fixture_id, fixture_id_numeric, custom_id, custom_id_type }
        }
    }
}

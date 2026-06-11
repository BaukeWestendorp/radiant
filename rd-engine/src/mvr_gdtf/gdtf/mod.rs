use std::{
    collections::HashMap,
    path::{Component, Path, PathBuf},
    str::{self, FromStr as _},
};

use uuid::Uuid;

use crate::mvr_gdtf::gdtf::{
    attr::AttributeName,
    bundle::FromBundle as _,
    proto::Protocols,
    resource::{ModelResource, ResourceKey, Resources, ThumbnailResource, WheelResource},
};

mod bundle;

pub mod attr;
pub mod dmx;
pub mod geo;
pub mod model;
pub mod phys;
pub mod proto;
pub mod resource;
pub mod rev;
pub mod wheel;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gdtf {
    version: Version,

    name: String,
    short_name: Option<String>,
    long_name: Option<String>,
    manufacturer: String,
    description: String,
    fixture_type_id: FixtureTypeId,
    reference_fixture_type_id: Option<FixtureTypeId>,
    thumbnail_offset: glam::I16Vec2,
    can_have_children: bool,

    activation_groups: Vec<attr::ActivationGroup>,
    activation_groups_by_name: HashMap<Name, usize>,

    feature_groups: Vec<attr::FeatureGroup>,
    feature_groups_by_name: HashMap<Name, usize>,

    attributes: Vec<attr::Attribute>,
    attributes_by_name: HashMap<AttributeName, usize>,

    wheels: Vec<wheel::Wheel>,
    wheels_by_name: HashMap<Name, usize>,

    emitters: Vec<phys::Emitter>,
    emitters_by_name: HashMap<Name, usize>,

    filters: Vec<phys::Filter>,
    filters_by_name: HashMap<Name, usize>,

    color_space: Option<phys::ColorSpace>,

    additional_color_spaces: Vec<phys::ColorSpace>,
    additional_color_spaces_by_name: HashMap<Name, usize>,

    gamuts: Vec<phys::Gamut>,
    gamuts_by_name: HashMap<Name, usize>,

    dmx_profiles: Vec<phys::DmxProfile>,
    dmx_profiles_by_name: HashMap<Name, usize>,

    cri_groups: Vec<phys::CriGroup>,

    properties: phys::Properties,

    models: Vec<model::Model>,
    models_by_name: HashMap<Name, usize>,

    geometries: Vec<geo::Geometry>,
    geometries_by_name: HashMap<Name, usize>,

    dmx_modes: Vec<dmx::DmxMode>,
    dmx_modes_by_name: HashMap<Name, dmx::DmxMode>,

    revisions: Vec<rev::Revision>,

    protocols: Protocols,

    resources: Resources,
}

impl Gdtf {
    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        Self::from(&bundle::Bundle::from_folder(path))
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        Self::from(&bundle::Bundle::from_archive(path))
    }

    pub fn from_archive_bytes(bytes: &[u8]) -> Self {
        Self::from(&bundle::Bundle::from_archive_bytes(bytes))
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }

    pub fn long_name(&self) -> Option<&str> {
        self.long_name.as_deref()
    }

    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn fixture_type_id(&self) -> FixtureTypeId {
        self.fixture_type_id
    }

    pub fn reference_fixture_type_id(&self) -> Option<FixtureTypeId> {
        self.reference_fixture_type_id
    }

    pub fn thumbnail_offset(&self) -> glam::I16Vec2 {
        self.thumbnail_offset
    }

    pub fn can_have_children(&self) -> bool {
        self.can_have_children
    }

    pub fn activation_groups(&self) -> &[attr::ActivationGroup] {
        &self.activation_groups
    }

    pub fn activation_group(&self, name: &Name) -> Option<&attr::ActivationGroup> {
        let ix = self.activation_groups_by_name.get(name)?;
        Some(&self.activation_groups[*ix])
    }

    pub fn feature_groups(&self) -> &[attr::FeatureGroup] {
        &self.feature_groups
    }

    pub fn feature_group(&self, name: &Name) -> Option<&attr::FeatureGroup> {
        let ix = self.feature_groups_by_name.get(name)?;
        Some(&self.feature_groups[*ix])
    }

    pub fn attributes(&self) -> &[attr::Attribute] {
        &self.attributes
    }

    pub fn attribute(&self, name: &AttributeName) -> Option<&attr::Attribute> {
        let ix = self.attributes_by_name.get(name)?;
        Some(&self.attributes[*ix])
    }

    pub fn wheels(&self) -> &[wheel::Wheel] {
        &self.wheels
    }

    pub fn wheel(&self, name: &Name) -> Option<&wheel::Wheel> {
        let ix = self.wheels_by_name.get(name)?;
        Some(&self.wheels[*ix])
    }

    pub fn emitters(&self) -> &[phys::Emitter] {
        &self.emitters
    }

    pub fn emitter(&self, name: &Name) -> Option<&phys::Emitter> {
        let ix = self.emitters_by_name.get(name)?;
        Some(&self.emitters[*ix])
    }

    pub fn filters(&self) -> &[phys::Filter] {
        &self.filters
    }

    pub fn filter(&self, name: &Name) -> Option<&phys::Filter> {
        let ix = self.filters_by_name.get(name)?;
        Some(&self.filters[*ix])
    }

    pub fn color_space(&self) -> Option<&phys::ColorSpace> {
        self.color_space.as_ref()
    }

    pub fn additional_color_spaces(&self) -> &[phys::ColorSpace] {
        &self.additional_color_spaces
    }

    pub fn additional_color_space(&self, name: &Name) -> Option<&phys::ColorSpace> {
        let ix = self.additional_color_spaces_by_name.get(name)?;
        Some(&self.additional_color_spaces[*ix])
    }

    pub fn gamuts(&self) -> &[phys::Gamut] {
        &self.gamuts
    }

    pub fn gamut(&self, name: &Name) -> Option<&phys::Gamut> {
        let ix = self.gamuts_by_name.get(name)?;
        Some(&self.gamuts[*ix])
    }

    pub fn dmx_profiles(&self) -> &[phys::DmxProfile] {
        &self.dmx_profiles
    }

    pub fn dmx_profile(&self, name: &Name) -> Option<&phys::DmxProfile> {
        let ix = self.dmx_profiles_by_name.get(name)?;
        Some(&self.dmx_profiles[*ix])
    }

    pub fn cri_groups(&self) -> &[phys::CriGroup] {
        &self.cri_groups
    }

    pub fn properties(&self) -> &phys::Properties {
        &self.properties
    }

    pub fn models(&self) -> &[model::Model] {
        &self.models
    }

    pub fn model(&self, name: &Name) -> Option<&model::Model> {
        let ix = self.models_by_name.get(name)?;
        Some(&self.models[*ix])
    }

    pub fn root_geometries(&self) -> &[geo::Geometry] {
        &self.geometries
    }

    pub fn geometry(&self, name: &Name) -> Option<&geo::Geometry> {
        fn find<'a>(geometries: &'a [geo::Geometry], name: &Name) -> Option<&'a geo::Geometry> {
            for geometry in geometries {
                if geometry.name() == name {
                    return Some(geometry);
                }
                if let Some(found) = find(geometry.children(), name) {
                    return Some(found);
                }
            }
            None
        }

        find(&self.geometries, name)
    }

    pub fn dmx_modes(&self) -> &[dmx::DmxMode] {
        &self.dmx_modes
    }

    pub fn dmx_mode(&self, name: &Name) -> Option<&dmx::DmxMode> {
        self.dmx_modes_by_name.get(name)
    }

    pub fn revisions(&self) -> &[rev::Revision] {
        &self.revisions
    }

    pub fn revisions_mut(&mut self) -> &mut Vec<rev::Revision> {
        &mut self.revisions
    }

    pub fn protocols(&self) -> &proto::Protocols {
        &self.protocols
    }

    pub fn protocols_mut(&mut self) -> &mut proto::Protocols {
        &mut self.protocols
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }
}

impl From<&bundle::Bundle> for Gdtf {
    fn from(bundle: &bundle::Bundle) -> Self {
        let mut gdtf = Gdtf::default();

        let ft = &bundle.description().fixture_type;

        gdtf.version = bundle.description().data_version.as_str().into();
        gdtf.name = ft.name.clone();
        gdtf.short_name = ft.short_name.clone();
        gdtf.long_name = ft.long_name.clone();
        gdtf.manufacturer = ft.manufacturer.clone();
        gdtf.description = ft.description.clone();
        gdtf.fixture_type_id = FixtureTypeId::from_str(&ft.fixture_type_id).unwrap();
        gdtf.can_have_children = ft.can_have_children == bundle::YesNo::Yes;

        gdtf.reference_fixture_type_id =
            ft.ref_ft.as_deref().and_then(|s| FixtureTypeId::from_str(s).ok());

        gdtf.thumbnail_offset = glam::I16Vec2::new(
            ft.thumbnail_offset_x.unwrap_or(0) as i16,
            ft.thumbnail_offset_y.unwrap_or(0) as i16,
        );

        if let Some(ags) = &ft.attribute_definitions.activation_groups {
            for ag in &ags.activation_groups {
                let ag = attr::ActivationGroup::from_bundle(ag, bundle);
                let ix = gdtf.activation_groups.len();
                gdtf.activation_groups_by_name.insert(ag.name().clone(), ix);
                gdtf.activation_groups.push(ag);
            }
        }

        for fg in &ft.attribute_definitions.feature_groups.feature_groups {
            let fg = attr::FeatureGroup::from_bundle(fg, bundle);
            let ix = gdtf.feature_groups.len();
            gdtf.feature_groups_by_name.insert(fg.name().clone(), ix);
            gdtf.feature_groups.push(fg);
        }

        for attr in &ft.attribute_definitions.attributes.attributes {
            let attr = attr::Attribute::from_bundle(attr, bundle);
            let ix = gdtf.attributes.len();
            gdtf.attributes_by_name.insert(attr.name().clone(), ix);
            gdtf.attributes.push(attr);
        }

        if let Some(w) = &ft.wheels {
            for wheel in &w.wheels {
                let wheel = wheel::Wheel::from_bundle(wheel, bundle);
                let ix = gdtf.wheels.len();
                if let Some(name) = wheel.name() {
                    gdtf.wheels_by_name.insert(name.clone(), ix);
                }
                gdtf.wheels.push(wheel);
            }
        };

        if let Some(pd) = &ft.physical_descriptions {
            if let Some(e) = &pd.emitters {
                for emitter in &e.emitters {
                    let emitter = phys::Emitter::from_bundle(emitter, bundle);
                    let ix = gdtf.emitters.len();
                    gdtf.emitters_by_name.insert(emitter.name().clone(), ix);
                    gdtf.emitters.push(emitter);
                }
            }

            if let Some(f) = &pd.filters {
                for f in &f.filters {
                    let f = phys::Filter::from_bundle(f, bundle);
                    let ix = gdtf.filters.len();
                    gdtf.filters_by_name.insert(f.name().clone(), ix);
                    gdtf.filters.push(f);
                }
            }

            if let Some(cs) = &pd.color_space {
                gdtf.color_space = Some(phys::ColorSpace::from_bundle(cs, bundle));
            }

            if let Some(acs) = &pd.additional_color_spaces {
                for cs in &acs.color_spaces {
                    let cs = phys::ColorSpace::from_bundle(cs, bundle);
                    let ix = gdtf.additional_color_spaces.len();
                    if let Some(name) = cs.name() {
                        gdtf.additional_color_spaces_by_name.insert(name.clone(), ix);
                    }
                    gdtf.additional_color_spaces.push(cs);
                }
            }

            if let Some(gs) = &pd.gamuts {
                for g in &gs.gamuts {
                    let g = phys::Gamut::from_bundle(g, bundle);
                    let ix = gdtf.gamuts.len();
                    if let Some(name) = g.name() {
                        gdtf.gamuts_by_name.insert(name.clone(), ix);
                    }
                    gdtf.gamuts.push(g);
                }
            }

            if let Some(dps) = &pd.dmx_profiles {
                for dp in &dps.dmx_profiles {
                    let dp = phys::DmxProfile::from_bundle(dp, bundle);
                    let ix = gdtf.dmx_profiles.len();
                    if let Some(name) = dp.name() {
                        gdtf.dmx_profiles_by_name.insert(name.clone(), ix);
                    }
                    gdtf.dmx_profiles.push(dp);
                }
            }

            if let Some(cris) = &pd.cr_is {
                for cri in &cris.cri_groups {
                    let cri = phys::CriGroup::from_bundle(cri, bundle);
                    gdtf.cri_groups.push(cri);
                }
            }

            if let Some(p) = &pd.properties {
                gdtf.properties = phys::Properties::from_bundle(p, bundle);
            }
        }

        if let Some(ms) = &ft.models {
            for model in &ms.models {
                let m = model::Model::from_bundle(model, bundle);
                let ix = gdtf.models.len();
                gdtf.models_by_name.insert(m.name().clone(), ix);
                gdtf.models.push(m);
            }
        };

        for g in &ft.geometries.children {
            let g = geo::Geometry::from_bundle(g, bundle);
            let ix = gdtf.geometries.len();
            gdtf.geometries_by_name.insert(g.name().clone(), ix);
            gdtf.geometries.push(g);
        }

        for dm in &ft.dmx_modes.dmx_modes {
            let dm = dmx::DmxMode::from_bundle(dm, bundle);
            gdtf.dmx_modes_by_name.insert(dm.name().clone(), dm.clone());
            gdtf.dmx_modes.push(dm);
        }

        if let Some(r) = &ft.revisions {
            for revision in &r.revisions {
                gdtf.revisions_mut().push(rev::Revision::from_bundle(revision, bundle));
            }
        }

        if let Some(p) = &ft.protocols {
            gdtf.protocols = proto::Protocols::from_bundle(p, bundle);
        }

        for (path, bytes) in bundle.resources() {
            if let Some(first_component) = path.components().next() {
                if first_component == Component::Normal("wheels".as_ref()) {
                    gdtf.resources
                        .wheels
                        .insert(ResourceKey::new(path), WheelResource::new(bytes.clone()));
                } else if first_component == Component::Normal("models".as_ref()) {
                    let Some(resource) = ModelResource::new(path, bytes.clone()) else { continue };
                    gdtf.resources.models.insert(ResourceKey::new(path), resource);
                }
            }
        }

        if let Some(thumbnail_png) = bundle.resources().get(Path::new("thumbnail.png")) {
            gdtf.resources.thumbnail_png = Some(ThumbnailResource::new(thumbnail_png.clone()));
        }

        if let Some(thumbnail_svg) = bundle.resources().get(Path::new("thumbnail.svg")) {
            gdtf.resources.thumbnail_svg = Some(ThumbnailResource::new(thumbnail_svg.clone()));
        }

        gdtf
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureTypeId(Uuid);

impl FixtureTypeId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for FixtureTypeId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl str::FromStr for FixtureTypeId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(Uuid::from_str(s)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    major: u32,
    minor: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

impl Default for Version {
    fn default() -> Self {
        Self { major: 1, minor: 2 }
    }
}

impl From<(u32, u32)> for Version {
    fn from((major, minor): (u32, u32)) -> Self {
        Self { major, minor }
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        let mut parts = value.splitn(2, '.');
        let major = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        let minor = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        Self { major, minor }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NodePath(Vec<Name>);

impl NodePath {
    pub fn new(root: impl Into<Name>) -> Self {
        Self(vec![root.into()])
    }

    pub fn join(mut self, part: impl Into<Name>) -> Self {
        self.0.push(part.into());
        self
    }

    pub fn join_path(mut self, path: impl Into<NodePath>) -> Self {
        let path = path.into();
        for part in path.0 {
            self.0.push(part);
        }
        self
    }

    pub fn parts(&self) -> &[Name] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_dotted_string(&self) -> String {
        self.0.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(".")
    }
}

impl std::fmt::Display for NodePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_dotted_string())
    }
}

impl str::FromStr for NodePath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(NodePath(Vec::new()));
        }
        let parts: Vec<Name> =
            s.split('.').map(|part| Name::new(part.trim().to_string())).collect();
        Ok(NodePath(parts))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Name(String);

impl Name {
    pub fn new(value: impl Into<String>) -> Self {
        // FIXME: Validate name.
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn parse_optional_name(s: Option<&str>) -> Option<Name> {
    let name = s.as_ref().map(|s| s.trim());
    match &name {
        Some(s) if s.is_empty() => None,
        Some(s) => Some(Name::new(*s)),
        None => None,
    }
}

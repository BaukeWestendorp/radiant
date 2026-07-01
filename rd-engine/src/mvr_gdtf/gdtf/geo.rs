use std::{collections::HashMap, str::FromStr as _};

use crate::mvr_gdtf::{
    DmxAddress,
    gdtf::{Gdtf, Name, NodePath, bundle, model::Model, phys::Emitter},
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Geometry {
    Basic(BasicGeometry),
    Axis(AxisGeometry),
    FilterBeam(FilterBeamGeometry),
    FilterColor(FilterColorGeometry),
    FilterGobo(FilterGoboGeometry),
    FilterShaper(FilterShaperGeometry),
    Beam(BeamGeometry),
    MediaServerLayer(MediaServerLayerGeometry),
    MediaServerCamera(MediaServerCameraGeometry),
    MediaServerMaster(MediaServerMasterGeometry),
    Display(DisplayGeometry),
    GeometryReference(ReferenceGeometry),
    Laser(LaserGeometry),
    WiringObject(WiringObjectGeometry),
    Inventory(InventoryGeometry),
    Structure(StructureGeometry),
    Support(SupportGeometry),
    Magnet(MagnetGeometry),
}

impl Geometry {
    pub fn name(&self) -> &Name {
        match self {
            Geometry::Basic(v) => v.name(),
            Geometry::Axis(v) => v.name(),
            Geometry::FilterBeam(v) => v.name(),
            Geometry::FilterColor(v) => v.name(),
            Geometry::FilterGobo(v) => v.name(),
            Geometry::FilterShaper(v) => v.name(),
            Geometry::Beam(v) => v.name(),
            Geometry::MediaServerLayer(v) => v.name(),
            Geometry::MediaServerCamera(v) => v.name(),
            Geometry::MediaServerMaster(v) => v.name(),
            Geometry::Display(v) => v.name(),
            Geometry::GeometryReference(v) => v.name(),
            Geometry::Laser(v) => v.name(),
            Geometry::WiringObject(v) => v.name(),
            Geometry::Inventory(v) => v.name(),
            Geometry::Structure(v) => v.name(),
            Geometry::Support(v) => v.name(),
            Geometry::Magnet(v) => v.name(),
        }
    }

    pub fn model_name(&self) -> Option<&Name> {
        match self {
            Geometry::Basic(v) => v.model_name(),
            Geometry::Axis(v) => v.model_name(),
            Geometry::FilterBeam(v) => v.model_name(),
            Geometry::FilterColor(v) => v.model_name(),
            Geometry::FilterGobo(v) => v.model_name(),
            Geometry::FilterShaper(v) => v.model_name(),
            Geometry::Beam(v) => v.model_name(),
            Geometry::MediaServerLayer(v) => v.model_name(),
            Geometry::MediaServerCamera(v) => v.model_name(),
            Geometry::MediaServerMaster(v) => v.model_name(),
            Geometry::Display(v) => v.model_name(),
            Geometry::GeometryReference(v) => v.model_name(),
            Geometry::Laser(v) => v.model_name(),
            Geometry::WiringObject(v) => v.model_name(),
            Geometry::Inventory(v) => v.model_name(),
            Geometry::Structure(v) => v.model_name(),
            Geometry::Support(v) => v.model_name(),
            Geometry::Magnet(v) => v.model_name(),
        }
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        match self {
            Geometry::Basic(v) => v.model(gdtf),
            Geometry::Axis(v) => v.model(gdtf),
            Geometry::FilterBeam(v) => v.model(gdtf),
            Geometry::FilterColor(v) => v.model(gdtf),
            Geometry::FilterGobo(v) => v.model(gdtf),
            Geometry::FilterShaper(v) => v.model(gdtf),
            Geometry::Beam(v) => v.model(gdtf),
            Geometry::MediaServerLayer(v) => v.model(gdtf),
            Geometry::MediaServerCamera(v) => v.model(gdtf),
            Geometry::MediaServerMaster(v) => v.model(gdtf),
            Geometry::Display(v) => v.model(gdtf),
            Geometry::GeometryReference(v) => v.model(gdtf),
            Geometry::Laser(v) => v.model(gdtf),
            Geometry::WiringObject(v) => v.model(gdtf),
            Geometry::Inventory(v) => v.model(gdtf),
            Geometry::Structure(v) => v.model(gdtf),
            Geometry::Support(v) => v.model(gdtf),
            Geometry::Magnet(v) => v.model(gdtf),
        }
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        match self {
            Geometry::Basic(v) => v.local_transform,
            Geometry::Axis(v) => v.local_transform,
            Geometry::FilterBeam(v) => v.local_transform,
            Geometry::FilterColor(v) => v.local_transform,
            Geometry::FilterGobo(v) => v.local_transform,
            Geometry::FilterShaper(v) => v.local_transform,
            Geometry::Beam(v) => v.local_transform,
            Geometry::MediaServerLayer(v) => v.local_transform,
            Geometry::MediaServerCamera(v) => v.local_transform,
            Geometry::MediaServerMaster(v) => v.local_transform,
            Geometry::Display(v) => v.local_transform,
            Geometry::GeometryReference(v) => v.local_transform,
            Geometry::Laser(v) => v.local_transform,
            Geometry::WiringObject(v) => v.local_transform,
            Geometry::Inventory(v) => v.local_transform,
            Geometry::Structure(v) => v.local_transform,
            Geometry::Support(v) => v.local_transform,
            Geometry::Magnet(v) => v.local_transform,
        }
    }

    pub fn children(&self) -> &[Geometry] {
        match self {
            Geometry::Basic(v) => v.children(),
            Geometry::Axis(v) => v.children(),
            Geometry::FilterBeam(v) => v.children(),
            Geometry::FilterColor(v) => v.children(),
            Geometry::FilterGobo(v) => v.children(),
            Geometry::FilterShaper(v) => v.children(),
            Geometry::Beam(v) => v.children(),
            Geometry::MediaServerLayer(v) => v.children(),
            Geometry::MediaServerCamera(v) => v.children(),
            Geometry::MediaServerMaster(v) => v.children(),
            Geometry::Display(v) => v.children(),
            Geometry::GeometryReference(v) => v.children(),
            Geometry::Laser(v) => v.children(),
            Geometry::WiringObject(v) => v.children(),
            Geometry::Inventory(v) => v.children(),
            Geometry::Structure(v) => v.children(),
            Geometry::Support(v) => v.children(),
            Geometry::Magnet(v) => v.children(),
        }
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        match self {
            Geometry::Basic(v) => v.child(name),
            Geometry::Axis(v) => v.child(name),
            Geometry::FilterBeam(v) => v.child(name),
            Geometry::FilterColor(v) => v.child(name),
            Geometry::FilterGobo(v) => v.child(name),
            Geometry::FilterShaper(v) => v.child(name),
            Geometry::Beam(v) => v.child(name),
            Geometry::MediaServerLayer(v) => v.child(name),
            Geometry::MediaServerCamera(v) => v.child(name),
            Geometry::MediaServerMaster(v) => v.child(name),
            Geometry::Display(v) => v.child(name),
            Geometry::GeometryReference(v) => v.child(name),
            Geometry::Laser(v) => v.child(name),
            Geometry::WiringObject(v) => v.child(name),
            Geometry::Inventory(v) => v.child(name),
            Geometry::Structure(v) => v.child(name),
            Geometry::Support(v) => v.child(name),
            Geometry::Magnet(v) => v.child(name),
        }
    }
}

impl<'src> bundle::FromBundle<'src> for Geometry {
    type Source = bundle::Geometry;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::Geometry::Geometry(v) => Geometry::Basic(BasicGeometry::from_bundle(v, bundle)),
            bundle::Geometry::Axis(v) => Geometry::Axis(AxisGeometry::from_bundle(v, bundle)),
            bundle::Geometry::FilterBeam(v) => {
                Geometry::FilterBeam(FilterBeamGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::FilterColor(v) => {
                Geometry::FilterColor(FilterColorGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::FilterGobo(v) => {
                Geometry::FilterGobo(FilterGoboGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::FilterShaper(v) => {
                Geometry::FilterShaper(FilterShaperGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::Beam(v) => Geometry::Beam(BeamGeometry::from_bundle(v, bundle)),
            bundle::Geometry::MediaServerLayer(v) => {
                Geometry::MediaServerLayer(MediaServerLayerGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::MediaServerCamera(v) => {
                Geometry::MediaServerCamera(MediaServerCameraGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::MediaServerMaster(v) => {
                Geometry::MediaServerMaster(MediaServerMasterGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::Display(v) => {
                Geometry::Display(DisplayGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::Laser(v) => Geometry::Laser(LaserGeometry::from_bundle(v, bundle)),
            bundle::Geometry::GeometryReference(v) => {
                Geometry::GeometryReference(ReferenceGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::WiringObject(v) => {
                Geometry::WiringObject(WiringObjectGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::Inventory(v) => {
                Geometry::Inventory(InventoryGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::Structure(v) => {
                Geometry::Structure(StructureGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::Support(v) => {
                Geometry::Support(SupportGeometry::from_bundle(v, bundle))
            }
            bundle::Geometry::Magnet(v) => Geometry::Magnet(MagnetGeometry::from_bundle(v, bundle)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl BasicGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for BasicGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AxisGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl AxisGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for AxisGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterBeamGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl FilterBeamGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for FilterBeamGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterColorGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl FilterColorGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for FilterColorGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterGoboGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl FilterGoboGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for FilterGoboGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterShaperGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl FilterShaperGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for FilterShaperGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BeamGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    lamp_type: LampType,
    power_consumption: f32,
    luminous_flux: f32,
    color_temperature: f32,
    beam_angle: f32,
    field_angle: f32,
    throw_ratio: f32,
    rectangle_ratio: f32,
    beam_radius: f32,
    beam_type: BeamType,
    cri: u8,
    emitter_spectrum: Option<NodePath>,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl BeamGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn lamp_type(&self) -> LampType {
        self.lamp_type
    }

    pub fn power_consumption(&self) -> f32 {
        self.power_consumption
    }

    pub fn luminous_flux(&self) -> f32 {
        self.luminous_flux
    }

    pub fn color_temperature(&self) -> f32 {
        self.color_temperature
    }

    pub fn beam_angle(&self) -> f32 {
        self.beam_angle
    }

    pub fn field_angle(&self) -> f32 {
        self.field_angle
    }

    pub fn throw_ratio(&self) -> f32 {
        self.throw_ratio
    }

    pub fn rectangle_ratio(&self) -> f32 {
        self.rectangle_ratio
    }

    pub fn beam_radius(&self) -> f32 {
        self.beam_radius
    }

    pub fn beam_type(&self) -> BeamType {
        self.beam_type
    }

    pub fn cri(&self) -> u8 {
        self.cri
    }

    pub fn emitter_spectrum_node(&self) -> Option<&NodePath> {
        self.emitter_spectrum.as_ref()
    }

    pub fn emitter_spectrum<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Emitter> {
        let emitter_name = self.emitter_spectrum_node()?.parts().get(0)?;
        gdtf.emitter(emitter_name)
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for BeamGeometry {
    type Source = bundle::Beam;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            lamp_type: source
                .lamp_type
                .as_ref()
                .map(|t| LampType::from_bundle(t, bundle))
                .unwrap_or(LampType::Led),
            power_consumption: source.power_consumption.unwrap_or_default(),
            luminous_flux: source.luminous_flux.unwrap_or_default(),
            color_temperature: source.color_temperature.unwrap_or_default(),
            beam_angle: source.beam_angle.unwrap_or_default(),
            field_angle: source.field_angle.unwrap_or_default(),
            throw_ratio: source.throw_ratio.unwrap_or_default(),
            rectangle_ratio: source.rectangle_ratio.unwrap_or_default(),
            beam_radius: source.beam_radius.unwrap_or_default(),
            beam_type: source
                .beam_type
                .as_ref()
                .map(|t| BeamType::from_bundle(t, bundle))
                .unwrap_or(BeamType::None),
            cri: source.color_rendering_index.unwrap_or(100),
            emitter_spectrum: source.emitter_spectrum.as_ref().map(|s| s.parse().unwrap()),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LampType {
    Discharge,
    Tungsten,
    Halogen,
    Led,
}

impl<'src> bundle::FromBundle<'src> for LampType {
    type Source = bundle::LampType;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::LampType::Discharge => Self::Discharge,
            bundle::LampType::Tungsten => Self::Tungsten,
            bundle::LampType::Halogen => Self::Halogen,
            bundle::LampType::Led => Self::Led,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeamType {
    None,
    Wash,
    Spot,
    Rectangle,
    Pc,
    Fresnel,
    Glow,
}

impl<'src> bundle::FromBundle<'src> for BeamType {
    type Source = bundle::BeamType;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::BeamType::None => Self::None,
            bundle::BeamType::Wash => Self::Wash,
            bundle::BeamType::Spot => Self::Spot,
            bundle::BeamType::Rectangle => Self::Rectangle,
            bundle::BeamType::Pc => Self::Pc,
            bundle::BeamType::Fresnel => Self::Fresnel,
            bundle::BeamType::Glow => Self::Glow,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MediaServerLayerGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl MediaServerLayerGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for MediaServerLayerGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MediaServerCameraGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl MediaServerCameraGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for MediaServerCameraGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MediaServerMasterGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl MediaServerMasterGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for MediaServerMasterGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    texture: String,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl DisplayGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn texture_name(&self) -> &str {
        &self.texture
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for DisplayGeometry {
    type Source = bundle::Display;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            texture: source.texture.clone().unwrap(),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    geometry: NodePath,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,

    breaks: Vec<DmxAddress>,
}

impl ReferenceGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn geometry_node(&self) -> &NodePath {
        &self.geometry
    }

    pub fn geometry<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Geometry> {
        let geometry_name = self.geometry_node().parts().get(0)?;
        gdtf.geometry(geometry_name)
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }

    pub fn breaks(&self) -> &[DmxAddress] {
        &self.breaks
    }
}

impl<'src> bundle::FromBundle<'src> for ReferenceGeometry {
    type Source = bundle::GeometryReference;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            geometry: match &source.geometry {
                Some(name) => NodePath::from_str(name).unwrap(),
                None => {
                    let first_geometry_name = bundle
                        .description()
                        .fixture_type
                        .geometries
                        .children
                        .first()
                        .expect("FIXME: Find out what to do in this cade")
                        .name();
                    NodePath::from_str(first_geometry_name).unwrap()
                }
            },
            children,
            children_by_name,
            breaks: source
                .breaks
                .iter()
                .map(|b| DmxAddress::new(b.dmx_offset).with_dmx_break(b.dmx_break))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LaserGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    color_type: LaserColorType,
    output_strength: f32,
    emitter: Option<NodePath>,
    beam_diameter: f32,
    beam_divergence_min: f32,
    beam_divergence_max: f32,
    scan_angle_pan: f32,
    scan_angle_tilt: f32,
    scan_speed: f32,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,

    protocols: Vec<Protocol>,
}

impl LaserGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn color_type(&self) -> LaserColorType {
        self.color_type
    }

    pub fn output_strength(&self) -> f32 {
        self.output_strength
    }

    pub fn emitter_node(&self) -> Option<&NodePath> {
        self.emitter.as_ref()
    }

    pub fn emitter<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Emitter> {
        let emitter_name = self.emitter_node()?.parts().get(0)?;
        gdtf.emitter(emitter_name)
    }

    pub fn beam_diameter(&self) -> f32 {
        self.beam_diameter
    }

    pub fn beam_divergence_min(&self) -> f32 {
        self.beam_divergence_min
    }

    pub fn beam_divergence_max(&self) -> f32 {
        self.beam_divergence_max
    }

    pub fn scan_angle_pan(&self) -> f32 {
        self.scan_angle_pan
    }

    pub fn scan_angle_tilt(&self) -> f32 {
        self.scan_angle_tilt
    }

    pub fn scan_speed(&self) -> f32 {
        self.scan_speed
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }

    pub fn protocols(&self) -> &[Protocol] {
        &self.protocols
    }
}

impl<'src> bundle::FromBundle<'src> for LaserGeometry {
    type Source = bundle::Laser;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            color_type: match source.color_type {
                bundle::LaserColorType::Rgb => LaserColorType::Rgb,
                bundle::LaserColorType::SingleWaveLength => {
                    LaserColorType::SingleWaveLength { color: source.color.unwrap() }
                }
            },
            output_strength: source.output_strength.unwrap_or_default(),
            emitter: source.emitter.as_ref().map(|s| s.parse().unwrap()),
            beam_diameter: source.beam_diameter.unwrap_or_default(),
            beam_divergence_min: source.beam_divergence_min.unwrap_or_default(),
            beam_divergence_max: source.beam_divergence_max.unwrap_or_default(),
            scan_angle_pan: source.scan_angle_pan.unwrap_or_default(),
            scan_angle_tilt: source.scan_angle_tilt.unwrap_or_default(),
            scan_speed: source.scan_speed.unwrap_or_default(),
            children,
            children_by_name,
            protocols: source
                .protocols
                .iter()
                .map(|p| Protocol { name: p.name.clone().unwrap() })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LaserColorType {
    Rgb,
    SingleWaveLength { color: f32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Protocol {
    name: String,
}

impl Protocol {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WiringObjectGeometry {
    name: Name,
    model: Option<Name>,
    connector_type: Name,
    local_transform: glam::Affine3A,
    component_type: WiringComponentType,
    details: WiringObjectDetails,
    signal_layer: i32,
    orientation: PinOrientation,
    wire_group: String,
    signal_type: String,
    pin_count: u32,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,

    pin_patches: Vec<PinPatch>,
}

impl WiringObjectGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn connector_type(&self) -> &Name {
        &self.connector_type
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn component_type(&self) -> WiringComponentType {
        self.component_type
    }

    pub fn details(&self) -> WiringObjectDetails {
        self.details
    }

    pub fn signal_layer(&self) -> i32 {
        self.signal_layer
    }

    pub fn orientation(&self) -> PinOrientation {
        self.orientation
    }

    pub fn wire_group(&self) -> &str {
        &self.wire_group
    }

    pub fn signal_type(&self) -> &str {
        &self.signal_type
    }

    pub fn pin_count(&self) -> u32 {
        self.pin_count
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }

    pub fn pin_patches(&self) -> &[PinPatch] {
        &self.pin_patches
    }
}

impl<'src> bundle::FromBundle<'src> for WiringObjectGeometry {
    type Source = bundle::WiringObject;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let component_type = source
            .component_type
            .as_ref()
            .map(|t| WiringComponentType::from_bundle(t, bundle))
            .unwrap();

        let details = match component_type {
            WiringComponentType::Input => WiringObjectDetails::Input,
            WiringComponentType::Output => WiringObjectDetails::Output,
            WiringComponentType::NetworkProvider => WiringObjectDetails::NetworkProvider,
            WiringComponentType::NetworkInput => WiringObjectDetails::NetworkInput,
            WiringComponentType::NetworkOutput => WiringObjectDetails::NetworkOutput,
            WiringComponentType::NetworkInOut => WiringObjectDetails::NetworkInOut,
            WiringComponentType::PowerSource => {
                let max_payload = source.max_pay_load.unwrap();
                let voltage = source.voltage.unwrap();

                WiringObjectDetails::PowerSource { max_payload, voltage }
            }
            WiringComponentType::Consumer => {
                let electrical_payload = source.electrical_pay_load.unwrap();

                let voltage_range_min = source.voltage_range_min.unwrap();
                let voltage_range_max = source.voltage_range_max.unwrap();

                let frequency_range_min = source.frequency_range_min.unwrap();
                let frequency_range_max = source.frequency_range_max.unwrap();

                let cos_phi = source.cos_phi.unwrap();

                WiringObjectDetails::Consumer {
                    electrical_payload,
                    voltage_range_max,
                    voltage_range_min,
                    frequency_range_max,
                    frequency_range_min,
                    cos_phi,
                }
            }
            WiringComponentType::Fuse => {
                let fuse_current = source.fuse_current.unwrap();
                let fuse_rating = source
                    .fuse_rating
                    .as_ref()
                    .map(|r| FuseRating::from_bundle(r, bundle))
                    .unwrap();

                WiringObjectDetails::Fuse { fuse_current, fuse_rating }
            }
        };

        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            connector_type: Name::new(source.connector_type.as_ref().unwrap()),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            component_type,
            details,
            signal_layer: source.signal_layer.unwrap_or_default().max(0),
            orientation: source
                .orientation
                .as_ref()
                .map(|o| PinOrientation::from_bundle(o, bundle))
                .unwrap(),
            wire_group: source.wire_group.clone().unwrap_or_default(),
            signal_type: source.signal_type.clone().unwrap_or_default(),
            pin_count: source.pin_count.unwrap() as u32,
            children,
            children_by_name,
            pin_patches: source
                .pin_patches
                .iter()
                .map(|p| PinPatch::from_bundle(p, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WiringObjectDetails {
    Input,
    Output,
    PowerSource {
        max_payload: f32,
        voltage: f32,
    },
    Consumer {
        electrical_payload: f32,
        voltage_range_max: f32,
        voltage_range_min: f32,
        frequency_range_max: f32,
        frequency_range_min: f32,
        cos_phi: f32,
    },
    Fuse {
        fuse_current: f32,
        fuse_rating: FuseRating,
    },
    NetworkProvider,
    NetworkInput,
    NetworkOutput,
    NetworkInOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WiringComponentType {
    Input,
    Output,
    PowerSource,
    Consumer,
    Fuse,
    NetworkProvider,
    NetworkInput,
    NetworkOutput,
    NetworkInOut,
}

impl<'src> bundle::FromBundle<'src> for WiringComponentType {
    type Source = bundle::WiringComponentType;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::WiringComponentType::Input => Self::Input,
            bundle::WiringComponentType::Output => Self::Output,
            bundle::WiringComponentType::PowerSource => Self::PowerSource,
            bundle::WiringComponentType::Consumer => Self::Consumer,
            bundle::WiringComponentType::Fuse => Self::Fuse,
            bundle::WiringComponentType::NetworkProvider => Self::NetworkProvider,
            bundle::WiringComponentType::NetworkInput => Self::NetworkInput,
            bundle::WiringComponentType::NetworkOutput => Self::NetworkOutput,
            bundle::WiringComponentType::NetworkInOut => Self::NetworkInOut,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FuseRating {
    B,
    C,
    D,
    K,
    Z,
}

impl<'src> bundle::FromBundle<'src> for FuseRating {
    type Source = bundle::WiringFuseRating;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::WiringFuseRating::B => Self::B,
            bundle::WiringFuseRating::C => Self::C,
            bundle::WiringFuseRating::D => Self::D,
            bundle::WiringFuseRating::K => Self::K,
            bundle::WiringFuseRating::Z => Self::Z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PinOrientation {
    Left,
    Right,
    Top,
    Bottom,
}

impl<'src> bundle::FromBundle<'src> for PinOrientation {
    type Source = bundle::WiringOrientation;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::WiringOrientation::Left => Self::Left,
            bundle::WiringOrientation::Right => Self::Right,
            bundle::WiringOrientation::Top => Self::Top,
            bundle::WiringOrientation::Bottom => Self::Bottom,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PinPatch {
    to_wiring_object: Name,
    from_pin: i32,
    to_pin: i32,
}

impl PinPatch {
    pub fn to_wiring_object_name(&self) -> &Name {
        &self.to_wiring_object
    }

    pub fn to_wiring_object<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Geometry> {
        gdtf.geometry(self.to_wiring_object_name())
    }

    pub fn from_pin(&self) -> i32 {
        self.from_pin
    }

    pub fn to_pin(&self) -> i32 {
        self.to_pin
    }
}

impl<'src> bundle::FromBundle<'src> for PinPatch {
    type Source = bundle::PinPatch;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            to_wiring_object: Name::new(source.to_wiring_object.as_ref().unwrap()),
            from_pin: source.from_pin.unwrap(),
            to_pin: source.to_pin.unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InventoryGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    count: u32,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl InventoryGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for InventoryGeometry {
    type Source = bundle::Inventory;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            count: source.count.unwrap_or(0),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructureGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    linked_geometry: Name,
    structure_type: StructureType,
    cross_section_type: CrossSectionType,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl StructureGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn linked_geometry_name(&self) -> &Name {
        &self.linked_geometry
    }

    pub fn linked_geometry<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Geometry> {
        gdtf.geometry(self.linked_geometry_name())
    }

    pub fn structure_type(&self) -> &StructureType {
        &self.structure_type
    }

    pub fn cross_section_type(&self) -> &CrossSectionType {
        &self.cross_section_type
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for StructureGeometry {
    type Source = bundle::Structure;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            linked_geometry: Name::new(source.linked_geometry.as_ref().unwrap()),
            structure_type: source
                .structure_type
                .as_ref()
                .map(|t| StructureType::from_bundle(t, bundle))
                .unwrap(),
            cross_section_type: match source
                .cross_section_type
                .as_ref()
                .unwrap_or(&bundle::CrossSectionType::TrussFramework)
            {
                bundle::CrossSectionType::TrussFramework => CrossSectionType::Truss {
                    cross_section: source.truss_cross_section.clone().unwrap(),
                },
                bundle::CrossSectionType::Tube => CrossSectionType::Tube {
                    cross_section_height: source.cross_section_height.unwrap(),
                    cross_section_wall_thickness: source.cross_section_wall_thickness.unwrap(),
                },
            },
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructureType {
    CenterLineBased,
    Detail,
}

impl<'src> bundle::FromBundle<'src> for StructureType {
    type Source = bundle::StructureType;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::StructureType::CenterLineBased => Self::CenterLineBased,
            bundle::StructureType::Detail => Self::Detail,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrossSectionType {
    Tube { cross_section_height: f32, cross_section_wall_thickness: f32 },
    Truss { cross_section: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    support_type: SupportType,
    capacity_force: glam::Vec3A,
    capacity_moment: glam::Vec3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl SupportGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn support_type(&self) -> &SupportType {
        &self.support_type
    }

    pub fn capacity_force(&self) -> glam::Vec3A {
        self.capacity_force
    }

    pub fn capacity_moment(&self) -> glam::Vec3A {
        self.capacity_moment
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for SupportGeometry {
    type Source = bundle::Support;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            support_type: match source.support_type {
                bundle::SupportType::Rope => SupportType::Rope {
                    cross_section: source.rope_cross_section.clone().unwrap(),
                    offset: util::parse_vec3(source.rope_offset.as_ref().unwrap()),
                },
                bundle::SupportType::GroundSupport => SupportType::GroundSupport {
                    resistance_along: glam::Vec3A::new(
                        source.resistance_x.unwrap_or(0.0),
                        source.resistance_y.unwrap_or(0.0),
                        source.resistance_z.unwrap_or(0.0),
                    ),
                    resistance_around: glam::Vec3A::new(
                        source.resistance_xx.unwrap_or(0.0),
                        source.resistance_yy.unwrap_or(0.0),
                        source.resistance_zz.unwrap_or(0.0),
                    ),
                },
            },
            capacity_force: glam::Vec3A::new(
                source.capacity_x.unwrap_or(0.0),
                source.capacity_y.unwrap_or(0.0),
                source.capacity_z.unwrap_or(0.0),
            ),
            capacity_moment: glam::Vec3A::new(
                source.capacity_xx.unwrap_or(0.0),
                source.capacity_yy.unwrap_or(0.0),
                source.capacity_zz.unwrap_or(0.0),
            ),
            children,
            children_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SupportType {
    Rope { cross_section: String, offset: glam::Vec3A },
    GroundSupport { resistance_along: glam::Vec3A, resistance_around: glam::Vec3A },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MagnetGeometry {
    name: Name,
    model: Option<Name>,
    local_transform: glam::Affine3A,
    children: Vec<Geometry>,
    children_by_name: HashMap<Name, usize>,
}

impl MagnetGeometry {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn model_name(&self) -> Option<&Name> {
        self.model.as_ref()
    }

    pub fn model<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Model> {
        let model_name = self.model_name()?;
        gdtf.model(model_name)
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn children(&self) -> &[Geometry] {
        &self.children
    }

    pub fn child(&self, name: &Name) -> Option<&Geometry> {
        let ix = self.children_by_name.get(name)?;
        Some(&self.children[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for MagnetGeometry {
    type Source = bundle::BasicGeometryType;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let children: Vec<Geometry> =
            source.children.iter().map(|child| Geometry::from_bundle(child, bundle)).collect();
        let children_by_name: HashMap<Name, usize> =
            children.iter().enumerate().map(|(ix, child)| (child.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            model: source.model.as_ref().map(Name::new),
            local_transform: util::parse_affine3a_from_mat4(&source.position),
            children,
            children_by_name,
        }
    }
}

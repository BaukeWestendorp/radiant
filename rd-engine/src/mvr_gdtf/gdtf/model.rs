use crate::mvr_gdtf::gdtf::{Name, ResourceKey, bundle};

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    name: Name,
    length: f32,
    width: f32,
    height: f32,
    primitive_type: PrimitiveType,
    resources: Vec<ResourceKey>,
    svg_offset: glam::Vec2,
    svg_side_offset: glam::Vec2,
    svg_front_offset: glam::Vec2,
}

impl Model {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn primitive_type(&self) -> &PrimitiveType {
        &self.primitive_type
    }

    pub fn resources(&self) -> &[ResourceKey] {
        &self.resources
    }

    pub fn svg_offset(&self) -> glam::Vec2 {
        self.svg_offset
    }

    pub fn svg_side_offset(&self) -> glam::Vec2 {
        self.svg_side_offset
    }

    pub fn svg_front_offset(&self) -> glam::Vec2 {
        self.svg_front_offset
    }
}

impl<'src> bundle::FromBundle<'src> for Model {
    type Source = bundle::Model;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let files = bundle
            .resources()
            .keys()
            .filter(|path| {
                path.starts_with("models")
                    && path.file_name().is_some_and(|f| f.to_string_lossy().contains(&source.file))
            })
            .map(|path| ResourceKey::new(path))
            .collect();

        Self {
            name: Name::new(source.name.to_owned()),
            length: source.length,
            width: source.width,
            height: source.height,
            primitive_type: (&source.primitive_type).into(),
            resources: files,
            svg_offset: glam::Vec2::new(
                source.svg_offset_x.unwrap_or(0.0),
                source.svg_offset_y.unwrap_or(0.0),
            ),
            svg_side_offset: glam::Vec2::new(
                source.svg_offset_x.unwrap_or(0.0),
                source.svg_offset_y.unwrap_or(0.0),
            ),
            svg_front_offset: glam::Vec2::new(
                source.svg_offset_x.unwrap_or(0.0),
                source.svg_offset_y.unwrap_or(0.0),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Undefined,
    Cube,
    Cylinder,
    Sphere,
    Base,
    Yoke,
    Head,
    Scanner,
    Conventional,
    Pigtail,
    Base11,
    Scanner11,
    Conventional11,
}

impl From<&bundle::PrimitiveType> for PrimitiveType {
    fn from(value: &bundle::PrimitiveType) -> Self {
        match value {
            bundle::PrimitiveType::Undefined => PrimitiveType::Undefined,
            bundle::PrimitiveType::Cube => PrimitiveType::Cube,
            bundle::PrimitiveType::Cylinder => PrimitiveType::Cylinder,
            bundle::PrimitiveType::Sphere => PrimitiveType::Sphere,
            bundle::PrimitiveType::Base => PrimitiveType::Base,
            bundle::PrimitiveType::Yoke => PrimitiveType::Yoke,
            bundle::PrimitiveType::Head => PrimitiveType::Head,
            bundle::PrimitiveType::Scanner => PrimitiveType::Scanner,
            bundle::PrimitiveType::Conventional => PrimitiveType::Conventional,
            bundle::PrimitiveType::Pigtail => PrimitiveType::Pigtail,
            bundle::PrimitiveType::Base11 => PrimitiveType::Base11,
            bundle::PrimitiveType::Scanner11 => PrimitiveType::Scanner11,
            bundle::PrimitiveType::Conventional11 => PrimitiveType::Conventional11,
        }
    }
}

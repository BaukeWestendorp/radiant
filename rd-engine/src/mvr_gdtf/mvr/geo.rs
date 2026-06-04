use crate::mvr_gdtf::{
    mvr::{ResourceKey, bundle},
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    pub(in crate::mvr_gdtf::mvr) local_transform: glam::Affine3A,

    pub(in crate::mvr_gdtf::mvr) model: ResourceKey,
}

impl Geometry {
    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn model(&self) -> &ResourceKey {
        &self.model
    }
}

impl bundle::FromBundle for Geometry {
    type Source = bundle::Geometry3D;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Geometry {
            local_transform: util::parse_affine3a_or_identity(source.matrix.as_deref()),
            model: ResourceKey::new(&source.file_name),
        }
    }
}

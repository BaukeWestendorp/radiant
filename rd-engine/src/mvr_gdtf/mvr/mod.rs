use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    str::{self, FromStr as _},
};

use crate::mvr_gdtf::{
    gdtf::Gdtf,
    mvr::{
        bundle::FromBundle,
        resource::{ModelResource, ResourceKey, Resources, TextureResource},
    },
    util,
};

mod bundle;

pub mod aux;
pub mod geo;
pub mod layer;
pub mod resource;

mod node;

pub use node::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Mvr {
    version: Version,
    provider: Provider,
    path: Option<PathBuf>,

    resources: Resources,

    symdefs: HashMap<NodeId<aux::Symdef>, Rc<aux::Symdef>>,
    classes: HashMap<NodeId<aux::Class>, Rc<aux::Class>>,
    mapping_definitions: HashMap<NodeId<aux::MappingDefinition>, Rc<aux::MappingDefinition>>,
    positions: HashMap<NodeId<aux::Position>, Rc<aux::Position>>,

    layers: HashMap<NodeId<layer::Layer>, Rc<layer::Layer>>,
}

impl Mvr {
    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        Self::from(&bundle::Bundle::from_folder(path))
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        Self::from(&bundle::Bundle::from_archive(path))
    }

    pub fn from_archive_bytes(bytes: &[u8]) -> Self {
        Self::from(&bundle::Bundle::from_archive_bytes(bytes))
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }
    pub fn symdefs(&self) -> impl Iterator<Item = &aux::Symdef> {
        self.symdefs.values().map(|v| v.as_ref())
    }

    pub fn symdef(&self, id: &NodeId<aux::Symdef>) -> Option<&aux::Symdef> {
        self.symdefs.get(id).map(|v| v.as_ref())
    }

    pub fn classes(&self) -> impl Iterator<Item = &aux::Class> {
        self.classes.values().map(|v| v.as_ref())
    }

    pub fn class(&self, id: &NodeId<aux::Class>) -> Option<&aux::Class> {
        self.classes.get(id).map(|v| v.as_ref())
    }

    pub fn positions(&self) -> impl Iterator<Item = &aux::Position> {
        self.positions.values().map(|v| v.as_ref())
    }

    pub fn position(&self, id: &NodeId<aux::Position>) -> Option<&aux::Position> {
        self.positions.get(id).map(|v| v.as_ref())
    }

    pub fn mapping_definitions(&self) -> impl Iterator<Item = &aux::MappingDefinition> {
        self.mapping_definitions.values().map(|v| v.as_ref())
    }

    pub fn mapping_definition(
        &self,
        id: &NodeId<aux::MappingDefinition>,
    ) -> Option<&aux::MappingDefinition> {
        self.mapping_definitions.get(id).map(|v| v.as_ref())
    }

    pub fn layers(&self) -> impl Iterator<Item = &layer::Layer> {
        self.layers.values().map(|v| v.as_ref())
    }

    pub fn layer(&self, id: &NodeId<layer::Layer>) -> Option<&layer::Layer> {
        self.layers.get(id).map(|v| v.as_ref())
    }
}

impl Default for Mvr {
    fn default() -> Self {
        Self {
            version: Version { major: 1, minor: 6 },
            provider: Provider {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            path: None,
            resources: Resources::default(),
            symdefs: HashMap::default(),
            classes: HashMap::default(),
            mapping_definitions: HashMap::default(),
            positions: HashMap::default(),
            layers: HashMap::default(),
        }
    }
}

impl From<&bundle::Bundle> for Mvr {
    fn from(bundle: &bundle::Bundle) -> Self {
        let mut mvr = Self {
            version: Version {
                major: bundle.description().ver_major as u32,
                minor: bundle.description().ver_minor as u32,
            },
            provider: Provider {
                name: bundle.description().provider.clone().unwrap_or_default(),
                version: bundle.description().provider_version.clone().unwrap_or_default(),
            },
            path: bundle.path().map(ToOwned::to_owned),
            resources: Resources {
                gdtfs: HashMap::new(),
                models: HashMap::new(),
                textures: HashMap::new(),
            },
            symdefs: HashMap::new(),
            classes: HashMap::new(),
            mapping_definitions: HashMap::new(),
            positions: HashMap::new(),
            layers: HashMap::new(),
        };

        for (file_name, bytes) in bundle.resources() {
            match file_name.extension().unwrap().to_str().unwrap() {
                "gdtf" => {
                    let gdtf = Gdtf::from_archive_bytes(&bytes);
                    mvr.resources.gdtfs.insert(ResourceKey::new(file_name), Rc::new(gdtf));
                }
                "3ds" | "gltf" | "glb" => {
                    mvr.resources.models.insert(
                        ResourceKey::new(file_name),
                        Rc::new(ModelResource::new(file_name, bytes.clone())),
                    );
                }
                "png" | "jpg" | "jpeg" | "svg" => {
                    mvr.resources.textures.insert(
                        ResourceKey::new(file_name),
                        Rc::new(TextureResource::new(file_name, bytes.clone())),
                    );
                }
                _ => {}
            };
        }

        let aux_data = bundle.description().scene.aux_data.as_ref().unwrap_or({
            static EMPTY_AUX_DATA: bundle::AuxData = bundle::AuxData {
                class: Vec::new(),
                symdef: Vec::new(),
                position: Vec::new(),
                mapping_definition: Vec::new(),
            };
            &EMPTY_AUX_DATA
        });

        for class in &aux_data.class {
            let class = aux::Class::from_bundle(&class, &bundle);
            mvr.classes.insert(class.id(), Rc::new(class));
        }

        for position in &aux_data.position {
            let position = aux::Position::from_bundle(&position, &bundle);
            mvr.positions.insert(position.id(), Rc::new(position));
        }

        for symdef in &aux_data.symdef {
            let symdef = aux::Symdef::from_bundle(&symdef, &bundle);
            mvr.symdefs.insert(symdef.id(), Rc::new(symdef));
        }

        for mapping_definition in &aux_data.mapping_definition {
            let mapping_definition =
                aux::MappingDefinition::from_bundle(&mapping_definition, &bundle);
            mvr.mapping_definitions.insert(mapping_definition.id(), Rc::new(mapping_definition));
        }

        for layer in &bundle.description().scene.layers.layer {
            let layer = layer::Layer::from_bundle(&layer, &bundle);
            mvr.layers.insert(layer.id(), Rc::new(layer));
        }

        mvr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    major: u32,
    minor: u32,
}

impl Version {
    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provider {
    name: String,
    version: String,
}

impl Provider {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

fn build_geometries(
    geometry_3ds: &[bundle::Geometry3D],
    symbols: &[bundle::Symbol],
    bundle: &bundle::Bundle,
) -> Vec<geo::Geometry> {
    let mut geometries = Vec::new();

    for geo3d in geometry_3ds {
        geometries.push(geo::Geometry::from_bundle(geo3d, bundle));
    }

    let Some(aux_data) = &bundle.description().scene.aux_data else {
        return geometries;
    };

    for symbol in symbols {
        let symbol_transform = util::parse_affine3a_or_identity(symbol.matrix.as_deref());
        let symdef = aux_data.symdef.iter().find(|s| s.uuid == symbol.symdef).unwrap();
        let nested_symdef = aux::Symdef::from_bundle(&symdef, bundle);
        for mut geo in nested_symdef.geometries().to_owned() {
            geo.local_transform = geo.local_transform() * symbol_transform;
            geometries.push(geo);
        }
    }

    geometries
}

impl bundle::FromBundle for super::DmxAddress {
    type Source = bundle::Address;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        super::DmxAddress::from_str(&source.content).unwrap()
    }
}

use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

#[derive(Clone, PartialEq, Default)]
pub struct Resources {
    pub(in crate::mvr_gdtf::gdtf) models: HashMap<ResourceKey, ModelResource>,
    pub(in crate::mvr_gdtf::gdtf) wheels: HashMap<ResourceKey, WheelResource>,
    pub(in crate::mvr_gdtf::gdtf) thumbnail_png: Option<ThumbnailResource>,
    pub(in crate::mvr_gdtf::gdtf) thumbnail_svg: Option<ThumbnailResource>,
}

impl Resources {
    pub fn models(&self) -> impl Iterator<Item = (&ResourceKey, &ModelResource)> {
        self.models.iter()
    }

    pub fn model(&self, key: &ResourceKey) -> Option<&ModelResource> {
        self.models.get(key)
    }

    pub fn wheels(&self) -> impl Iterator<Item = (&ResourceKey, &WheelResource)> {
        self.wheels.iter()
    }

    pub fn wheel(&self, key: &ResourceKey) -> Option<&WheelResource> {
        self.wheels.get(key)
    }

    pub fn thumbnail_png(&self) -> Option<&ThumbnailResource> {
        self.thumbnail_png.as_ref()
    }

    pub fn thumbnail_svg(&self) -> Option<&ThumbnailResource> {
        self.thumbnail_svg.as_ref()
    }
}

impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resources")
            .field("models", &self.models)
            .field("wheels", &self.wheels)
            .field("thumbnail_svg", &self.thumbnail_svg)
            .field("thumbnail_png", &self.thumbnail_png)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceKey(PathBuf);

impl ResourceKey {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    pub fn relative_path(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Clone, PartialEq)]
pub struct ModelResource {
    bytes: Vec<u8>,
    kind: ModelKind,
}

impl ModelResource {
    pub fn new(path: &Path, bytes: Vec<u8>) -> Option<Self> {
        let ext = path.extension().and_then(|s| s.to_str()).map(|s| s.to_ascii_lowercase());
        let parent =
            path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()).unwrap_or("");
        let grandparent = path
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");

        let kind = match ext.as_deref() {
            Some("3ds") => match (grandparent, parent) {
                ("models", "3ds_low") => ModelKind::ThreeDs { lod: ModelLod::Low },
                ("models", "3ds_high") => ModelKind::ThreeDs { lod: ModelLod::High },
                ("models", folder) if folder.starts_with("3ds") => {
                    ModelKind::ThreeDs { lod: ModelLod::Default }
                }
                _ => ModelKind::ThreeDs { lod: ModelLod::Default },
            },
            Some("gltf") => match (grandparent, parent) {
                ("models", "gltf_low") => ModelKind::Gltf { lod: ModelLod::Low },
                ("models", "gltf_high") => ModelKind::Gltf { lod: ModelLod::High },
                ("models", folder) if folder.starts_with("gltf") => {
                    ModelKind::Gltf { lod: ModelLod::Default }
                }
                _ => ModelKind::Gltf { lod: ModelLod::Default },
            },
            Some("glb") => match (grandparent, parent) {
                ("models", "gltf_low") => ModelKind::Glb { lod: ModelLod::Low },
                ("models", "gltf_high") => ModelKind::Glb { lod: ModelLod::High },
                ("models", folder) if folder.starts_with("gltf") => {
                    ModelKind::Glb { lod: ModelLod::Default }
                }
                _ => ModelKind::Glb { lod: ModelLod::Default },
            },
            Some("svg") => match (grandparent, parent) {
                ("models", "svg") => ModelKind::Svg { view: SvgView::Top },
                ("models", "svg_bottom") => ModelKind::Svg { view: SvgView::Bottom },
                ("models", "svg_front") => ModelKind::Svg { view: SvgView::Front },
                ("models", "svg_side") => ModelKind::Svg { view: SvgView::Side },
                _ => ModelKind::Svg { view: SvgView::Top },
            },
            _ => return None,
        };

        Some(Self { bytes, kind })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn kind(&self) -> &ModelKind {
        &self.kind
    }
}

impl fmt::Debug for ModelResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModelResource")
            .field("bytes", &"Vec(...)")
            .field("kind", &self.kind)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelKind {
    ThreeDs { lod: ModelLod },
    Gltf { lod: ModelLod },
    Glb { lod: ModelLod },
    Svg { view: SvgView },
}

impl ModelKind {
    pub fn extension(&self) -> &str {
        match self {
            ModelKind::ThreeDs { .. } => "3ds",
            ModelKind::Gltf { .. } => "gltf",
            ModelKind::Glb { .. } => "glb",
            ModelKind::Svg { .. } => "svg",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelLod {
    Low,
    Default,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SvgView {
    Top,
    Bottom,
    Front,
    Side,
}

#[derive(Clone, PartialEq)]
pub struct WheelResource {
    bytes: Vec<u8>,
}

impl WheelResource {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for WheelResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextureResource").field("bytes", &"Vec(...)").finish()
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct ThumbnailResource {
    bytes: Vec<u8>,
}

impl ThumbnailResource {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for ThumbnailResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThumbnailResource").field("bytes", &"Vec(...)").finish()
    }
}

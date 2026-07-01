use std::{collections::HashMap, str::FromStr as _};

use crate::mvr_gdtf::{
    CieColor,
    gdtf::{Gdtf, Name, NodePath, ResourceKey, bundle, parse_optional_name, phys::Filter},
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Wheel {
    name: Option<Name>,

    slots: Vec<WheelSlot>,
    slots_by_name: HashMap<Name, usize>,
}

impl Wheel {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn slots(&self) -> &[WheelSlot] {
        &self.slots
    }

    pub fn slot(&self, name: &Name) -> Option<&WheelSlot> {
        let ix = self.slots_by_name.get(name)?;
        Some(&self.slots[*ix])
    }
}

impl<'src> bundle::FromBundle<'src> for Wheel {
    type Source = bundle::Wheel;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let mut slots = Vec::new();
        let mut slots_by_name = HashMap::new();
        for (ix, slot) in source.slots.iter().enumerate() {
            let slot = WheelSlot::from_bundle(slot, bundle);
            slots_by_name.insert(slot.name().clone(), ix);
            slots.push(slot);
        }

        Self { name: parse_optional_name(source.name.as_deref()), slots, slots_by_name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WheelSlot {
    name: Name,
    color: SlotColor,
    file: Option<ResourceKey>,
    content: Option<WheelSlotContent>,
}

impl WheelSlot {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn color(&self) -> &SlotColor {
        &self.color
    }

    pub fn file(&self) -> Option<&ResourceKey> {
        self.file.as_ref()
    }

    pub fn content(&self) -> Option<&WheelSlotContent> {
        self.content.as_ref()
    }
}

impl<'src> bundle::FromBundle<'src> for WheelSlot {
    type Source = bundle::Slot;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let facets = source
            .content
            .iter()
            .filter_map(|c| if let bundle::SlotContent::Facet(f) = c { Some(f) } else { None })
            .collect::<Vec<_>>();

        let animation = source.content.iter().find_map(|c| {
            if let bundle::SlotContent::AnimationSystem(a) = c { Some(a) } else { None }
        });

        let content = if !facets.is_empty() {
            Some(WheelSlotContent::Prism(
                facets.iter().map(|f| PrismFacet::from_bundle(f, bundle)).collect(),
            ))
        } else if let Some(anim) = animation {
            Some(WheelSlotContent::AnimationSystem(AnimationSystem::from_bundle(&anim, bundle)))
        } else {
            None
        };

        let file = if source.media_file_name.is_empty() {
            None
        } else {
            bundle
                .resources()
                .keys()
                .find(|path| {
                    path.starts_with("wheels")
                        && path
                            .file_name()
                            .is_some_and(|f| f.to_string_lossy().contains(&source.media_file_name))
                })
                .map(|path| ResourceKey::new(path))
        };

        let color = if let Some(f) = source.filter.as_deref().filter(|f| !f.is_empty()) {
            SlotColor::Filter(NodePath::from_str(f).unwrap())
        } else {
            SlotColor::Cie(
                source
                    .color
                    .as_deref()
                    .and_then(|c| CieColor::from_str(c).ok())
                    .unwrap_or_default(),
            )
        };

        Self { name: Name::new(source.name.clone()), color, file, content }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SlotColor {
    Filter(NodePath),
    Cie(CieColor),
}

impl SlotColor {
    pub fn filter<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Filter> {
        match self {
            SlotColor::Filter(filter_node) => {
                let filter_name = filter_node.parts().get(0)?;
                gdtf.filter(filter_name)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WheelSlotContent {
    Prism(Vec<PrismFacet>),
    AnimationSystem(AnimationSystem),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrismFacet {
    color: CieColor,
    rotation: glam::Mat3,
}

impl PrismFacet {
    pub fn color(&self) -> CieColor {
        self.color
    }

    pub fn rotation(&self) -> glam::Mat3 {
        self.rotation
    }
}

impl<'src> bundle::FromBundle<'src> for PrismFacet {
    type Source = bundle::Facet;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            color: source
                .color
                .as_deref()
                .and_then(|c| CieColor::from_str(c).ok())
                .unwrap_or_default(),
            rotation: util::parse_mat3(&source.rotation),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationSystem {
    p1: glam::Vec2,
    p2: glam::Vec2,
    p3: glam::Vec2,
    radius: f32,
}

impl AnimationSystem {
    pub fn p1(&self) -> glam::Vec2 {
        self.p1
    }

    pub fn p2(&self) -> glam::Vec2 {
        self.p2
    }

    pub fn p3(&self) -> glam::Vec2 {
        self.p3
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl<'src> bundle::FromBundle<'src> for AnimationSystem {
    type Source = bundle::AnimationSystem;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            p1: util::parse_vec2(&source.p1),
            p2: util::parse_vec2(&source.p2),
            p3: util::parse_vec2(&source.p3),
            radius: source.radius,
        }
    }
}

use crate::effect_graph::EffectGraph;
use crate::fixture::FixtureId;
use crate::patch::Patch;
use std::net::Ipv4Addr;

pub mod effect_graph;
pub mod fixture;
pub mod patch;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default)]
pub struct Show {
    #[cfg_attr(feature = "serde", serde(default))]
    patch: Patch,
    #[cfg_attr(feature = "serde", serde(default))]
    dmx_protocols: DmxProtocols,

    #[cfg_attr(feature = "serde", serde(default))]
    effect_graph: EffectGraph,
}

impl Show {
    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn patch_mut(&mut self) -> &mut Patch {
        &mut self.patch
    }

    pub fn effect_graph(&self) -> &EffectGraph {
        &self.effect_graph
    }

    pub fn effect_graph_mut(&mut self) -> &mut EffectGraph {
        &mut self.effect_graph
    }

    pub fn dmx_protocols(&self) -> &DmxProtocols {
        &self.dmx_protocols
    }

    #[cfg(feature = "serde")]
    pub fn read_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let show_json = std::fs::read_to_string(path)?;
        let show: Self = serde_json::from_str(&show_json)?;
        Ok(show)
    }

    #[cfg(feature = "serde")]
    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let show_json = serde_json::to_string_pretty(self)?;

        std::fs::write(path, show_json)?;

        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct DmxProtocols {
    artnet: Vec<ArtnetNodeSettings>,
}

impl DmxProtocols {
    pub fn artnet(&self) -> &[ArtnetNodeSettings] {
        &self.artnet
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ArtnetNodeSettings {
    pub destination_ip: Ipv4Addr,
    pub universe: u16,
}

#[derive(Clone, Default)]
pub struct FixtureGroup {
    fixtures: Vec<FixtureId>,
}

impl FixtureGroup {
    pub fn new(fixtures: Vec<FixtureId>) -> Self {
        Self { fixtures }
    }

    pub fn fixtures(&self) -> &[FixtureId] {
        &self.fixtures
    }

    pub fn push_fixture(&mut self, fixture: FixtureId) {
        self.fixtures.push(fixture);
    }

    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }
}

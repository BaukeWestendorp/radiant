use std::{collections::HashMap, str::FromStr as _};

use crate::mvr_gdtf::gdtf::{Gdtf, NodePath, bundle, dmx::DmxMode};

fn parse_hex_u16(s: &str) -> Option<u16> {
    let s = s.trim();
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u16::from_str_radix(s, 16).ok()
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Protocols {
    rdm: Option<Rdm>,
    art_net: Option<ArtNet>,
    sacn: Option<SAcn>,
    posi_stage_net: Option<PosiStageNet>,
    open_sound_control: Option<OpenSoundControl>,
    ctip: Option<Citp>,
}

impl Protocols {
    pub fn new() -> Self {
        Self {
            rdm: None,
            art_net: None,
            sacn: None,
            posi_stage_net: None,
            open_sound_control: None,
            ctip: None,
        }
    }

    pub fn rdm(&self) -> Option<&Rdm> {
        self.rdm.as_ref()
    }

    pub fn art_net(&self) -> Option<&ArtNet> {
        self.art_net.as_ref()
    }

    pub fn sacn(&self) -> Option<&SAcn> {
        self.sacn.as_ref()
    }

    pub fn posi_stage_net(&self) -> Option<&PosiStageNet> {
        self.posi_stage_net.as_ref()
    }

    pub fn open_sound_control(&self) -> Option<&OpenSoundControl> {
        self.open_sound_control.as_ref()
    }

    pub fn ctip(&self) -> Option<&Citp> {
        self.ctip.as_ref()
    }
}

impl bundle::FromBundle for Protocols {
    type Source = bundle::Protocols;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            rdm: source.ftrdm.as_ref().map(|ftrdm| Rdm::from_bundle(ftrdm, bundle)),
            art_net: source.art_net.as_ref().map(|a| ArtNet::from_bundle(a, bundle)),
            sacn: source.s_acn.as_ref().map(|s| SAcn::from_bundle(s, bundle)),
            posi_stage_net: source
                .posi_stage_net
                .as_ref()
                .map(|p| PosiStageNet::from_bundle(p, bundle)),
            open_sound_control: source
                .open_sound_control
                .as_ref()
                .map(|o| OpenSoundControl::from_bundle(o, bundle)),
            ctip: source.citp.as_ref().map(|c| Citp::from_bundle(c, bundle)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rdm {
    manufacturer: Option<u16>,
    device_model: Option<u16>,
    versions: Vec<SoftwareVersion>,
}

impl Rdm {
    pub fn manufacturer(&self) -> Option<u16> {
        self.manufacturer
    }

    pub fn device_model(&self) -> Option<u16> {
        self.device_model
    }

    pub fn versions(&self) -> &[SoftwareVersion] {
        &self.versions
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoftwareVersion {
    id: Option<u16>,

    personalities: Vec<DmxPersonality>,
}

impl SoftwareVersion {
    pub fn id(&self) -> Option<u16> {
        self.id
    }

    pub fn personalities(&self) -> &[DmxPersonality] {
        &self.personalities
    }
}

impl bundle::FromBundle for Rdm {
    type Source = bundle::Ftrdm;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let manufacturer = source.manufacturer_id.as_ref().and_then(|id| parse_hex_u16(id));
        let device_model = source.device_model_id.as_ref().and_then(|id| parse_hex_u16(id));

        let mut versions = Vec::new();
        for sv in &source.software_version_ids {
            let id = sv.value.as_ref().and_then(|id| parse_hex_u16(id));
            let personalities = sv
                .dmx_personalities
                .iter()
                .map(|p| DmxPersonality::from_bundle(p, bundle))
                .collect();
            versions.push(SoftwareVersion { id, personalities });
        }
        Self { manufacturer, device_model, versions }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxPersonality {
    value: Option<u16>,
    dmx_mode: Option<NodePath>,
}

impl DmxPersonality {
    pub fn value(&self) -> Option<u16> {
        self.value
    }

    pub fn dmx_mode_node(&self) -> Option<&NodePath> {
        self.dmx_mode.as_ref()
    }

    pub fn dmx_mode<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a DmxMode> {
        let dmx_mode_name = self.dmx_mode_node()?.parts().get(0)?;
        gdtf.dmx_mode(dmx_mode_name)
    }
}

impl bundle::FromBundle for DmxPersonality {
    type Source = bundle::DmxPersonality;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        let value = source.value.as_ref().and_then(|id| parse_hex_u16(id));
        Self { value, dmx_mode: source.dmx_mode.as_ref().map(|s| NodePath::from_str(s).unwrap()) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArtNet {
    mapping: HashMap<u8, u8>,
}

impl ArtNet {
    pub fn mapping(&self) -> &HashMap<u8, u8> {
        &self.mapping
    }
}

impl bundle::FromBundle for ArtNet {
    type Source = bundle::ArtNet;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        let mut mapping = HashMap::new();
        for m in &source.map {
            mapping.insert(m.key as u8, m.value as u8);
        }
        Self { mapping }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SAcn {
    mapping: HashMap<u8, u8>,
}

impl SAcn {
    pub fn mapping(&self) -> &HashMap<u8, u8> {
        &self.mapping
    }
}

impl bundle::FromBundle for SAcn {
    type Source = bundle::SAcn;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        let mut mapping = HashMap::new();
        for m in &source.map {
            mapping.insert(m.key as u8, m.value as u8);
        }
        Self { mapping }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PosiStageNet {}

impl bundle::FromBundle for PosiStageNet {
    type Source = bundle::PosiStageNet;
    fn from_bundle(_source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenSoundControl {}

impl bundle::FromBundle for OpenSoundControl {
    type Source = bundle::OpenSoundControl;
    fn from_bundle(_source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Citp {}

impl bundle::FromBundle for Citp {
    type Source = bundle::Citp;
    fn from_bundle(_source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {}
    }
}

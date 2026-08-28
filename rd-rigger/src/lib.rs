pub mod gdtf;
pub mod mvr;
mod util;

use std::str;

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CieColor {
    x: f32,
    y: f32,
    yy: f32,
}

impl CieColor {
    pub fn new(x: f32, y: f32, yy: f32) -> Self {
        Self { x, y, yy }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn yy(&self) -> f32 {
        self.yy
    }
}

impl Default for CieColor {
    fn default() -> Self {
        Self { x: 0.3127, y: 0.3290, yy: 100.0 }
    }
}

impl str::FromStr for CieColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_start_matches('{').trim_end_matches('}');
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(());
        }
        let x = parts[0].trim().parse::<f32>().map_err(|_| ())?;
        let y = parts[1].trim().parse::<f32>().map_err(|_| ())?;
        let yy = parts[2].trim().parse::<f32>().map_err(|_| ())?;
        Ok(CieColor { x, y, yy })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxAddress {
    absolute: u32,
    dmx_break: u8,
}

impl DmxAddress {
    pub fn new(absolute: u32) -> Self {
        Self { absolute, dmx_break: 1 }
    }

    pub fn absolute(&self) -> u32 {
        self.absolute
    }

    pub fn set_absolute(&mut self, absolute: u32) {
        self.absolute = absolute
    }

    pub fn dmx_break(&self) -> u8 {
        self.dmx_break
    }

    pub fn set_dmx_brak(&mut self, dmx_break: u8) {
        self.dmx_break = dmx_break;
    }

    pub fn with_dmx_break(mut self, dmx_break: u8) -> Self {
        self.dmx_break = dmx_break;
        self
    }
}

impl std::str::FromStr for DmxAddress {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let absolute_address = if let Some(dot) = s.find('.') {
            let (universe_str, channel_str) = s.split_at(dot);
            let universe = universe_str.parse::<u32>().map_err(|_| ())?;
            let channel = channel_str[1..].parse::<u32>().map_err(|_| ())?;
            (universe - 1) * 512 + channel
        } else {
            s.parse::<u32>().map_err(|_| ())?
        };
        Ok(DmxAddress { dmx_break: 1, absolute: absolute_address })
    }
}

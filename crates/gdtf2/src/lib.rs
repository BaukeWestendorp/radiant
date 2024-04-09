pub mod attribute_definitions;
pub mod dmx_mode;
pub mod fixture_type;
mod raw;

use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use fixture_type::FixtureType;

/// # A GDTF archive file.
pub struct GdtfArchive {
    /// The GDTF file descriptor.
    pub description: GdtfDescription,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GdtfDescription {
    pub data_version: DataVersion,
    pub fixture_type: FixtureType,
}

impl GdtfDescription {
    /// Create a new GDTF file from a .gdtf archive file.
    pub fn from_archive_reader<R>(reader: R) -> Result<GdtfDescription>
    where
        R: std::io::Read + std::io::Seek,
    {
        let mut archive = zip::ZipArchive::new(reader)
            .map_err(|err| anyhow!("Failed to unzip archive: {err}"))?;

        let description = archive
            .by_name("description.xml")
            .map_err(|err| anyhow!("Missing description file: {err}"))?;

        let description_reader = std::io::BufReader::new(description);

        let raw_gdtf: raw::RawGdtfDescription = serde_xml_rs::from_reader(description_reader)
            .map_err(|err| anyhow!("Failed to parse GDTF description: {err}"))?;
        let gdtf: GdtfDescription = raw_gdtf.try_into()?;

        Ok(gdtf)
    }

    /// Create a new GDTF file from .gdtf archive bytes.
    pub fn from_archive_bytes(bytes: &[u8]) -> Result<GdtfDescription> {
        let bytes = std::io::Cursor::new(bytes);
        let reader = std::io::BufReader::new(bytes);
        Self::from_archive_reader(reader)
    }

    /// Create a new GDTF file from a descriptor file.
    pub fn from_file(file: &std::fs::File) -> Result<GdtfDescription> {
        let file_reader = std::io::BufReader::new(file);
        let raw_gdtf: raw::RawGdtfDescription = serde_xml_rs::from_reader(file_reader)
            .map_err(|err| anyhow!("Failed to parse GDTF description: {err}"))?;
        let gdtf: GdtfDescription = raw_gdtf.try_into()?;

        Ok(gdtf)
    }

    /// Create a new GDTF file from a descriptor file.
    pub fn from_str(s: &str) -> Result<GdtfDescription> {
        let raw_gdtf: raw::RawGdtfDescription = serde_xml_rs::from_str(s)
            .map_err(|err| anyhow!("Failed to parse GDTF description: {err}"))?;
        let gdtf: GdtfDescription = raw_gdtf.try_into()?;

        Ok(gdtf)
    }
}

impl TryFrom<raw::RawGdtfDescription> for GdtfDescription {
    type Error = Error;

    fn try_from(value: raw::RawGdtfDescription) -> Result<Self, Self::Error> {
        Ok(GdtfDescription {
            data_version: value.data_version.parse()?,
            fixture_type: FixtureType::from_raw(value.fixture_type)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataVersion {
    pub major: u32,
    pub minor: u32,
}

impl FromStr for DataVersion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "Invalid data version: '{}'. Expected 2 parts, but found {}",
                s,
                parts.len()
            ));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| anyhow!("Failed to parse data version major: '{}'", parts[0]))?;

        let minor = parts[1]
            .parse()
            .map_err(|_| anyhow!("Failed to parse data version minor: '{}'", parts[1]))?;

        Ok(Self { major, minor })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxValue {
    value: u64,
    byte_specifier: u8,
    byte_shifting: bool,
}

impl DmxValue {
    pub fn value(&self, channel_resolution: ChannelBitResolution) -> Result<u64> {
        // Check if the ByteSpecifier is different to the ChannelResolution.
        let mut result = self.value;
        if self.byte_shifting {
            if self.byte_specifier != channel_resolution as u8 {
                let shift: i32 = 8 * (channel_resolution as i32 - self.byte_specifier as i32);
                if shift >= 0 {
                    result <<= shift;
                } else {
                    result >>= -shift;
                }
            } else {
                // We can take the value as it is defined in the document without scaling it to
                // another BitResolution.
                result = self.value;
            }
        } else if self.byte_specifier != channel_resolution as u8 {
            let max_resolution = get_channel_max_dmx(self.byte_specifier.into());
            let max_channel_unit = get_channel_max_dmx(channel_resolution);

            let percentage = self.value as f64 / max_resolution as f64;

            result = (percentage * max_channel_unit as f64) as u64;
        } else {
            // We can take the value as it is defined in the document without scaling it to
            // another BitResolution.
            result = self.value;
        }

        if get_channel_max_dmx(channel_resolution) < result {
            return Err(anyhow!(
                "DMX value of {} is out of range for the channel resolution: {:?}",
                result,
                channel_resolution
            ));
        }

        Ok(result)
    }

    pub fn bytes(&self, channel_resolution: ChannelBitResolution) -> Result<Vec<u8>> {
        let value = self.value(channel_resolution)?;

        let mut bytes = Vec::new();
        match channel_resolution {
            ChannelBitResolution::Bit8 => {
                bytes.push(value as u8);
            }
            ChannelBitResolution::Bit16 => {
                bytes.push((value >> 8) as u8);
                bytes.push(value as u8);
            }
            ChannelBitResolution::Bit24 => {
                bytes.push((value >> 16) as u8);
                bytes.push((value >> 8) as u8);
                bytes.push(value as u8);
            }
            ChannelBitResolution::Bit32 => {
                bytes.push((value >> 24) as u8);
                bytes.push((value >> 16) as u8);
                bytes.push((value >> 8) as u8);
                bytes.push(value as u8);
            }
        }

        Ok(bytes)
    }

    pub fn byte_specifier(&self) -> u8 {
        self.byte_specifier
    }
}

fn get_channel_max_dmx(channel_resolution: ChannelBitResolution) -> u64 {
    let max_val = match channel_resolution {
        ChannelBitResolution::Bit8 => 256,
        ChannelBitResolution::Bit16 => 256 * 256,
        ChannelBitResolution::Bit24 => 256 * 256 * 256,
        ChannelBitResolution::Bit32 => 256 * 256 * 256 * 256,
    };

    max_val - 1
}

impl FromStr for DmxValue {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(anyhow!("expected a non-empty string in the DMX value."));
        }

        let (first, mut second) = match value.split_once('/') {
            Some((first, second)) => (first.to_string(), second.to_string()),
            None => return Err(anyhow!("expected a '/' in the DMX value.")),
        };

        let mut byte_shifting = false;
        if second.ends_with('s') {
            byte_shifting = true;
            second.pop();
        }

        let mut value: u64 = match first.parse() {
            Ok(value) => value,
            Err(_) => return Err(anyhow!("expected a number in the DMX value.")),
        };

        let mut byte_specifier: i32 = match second.parse() {
            Ok(value) => value,
            Err(_) => return Err(anyhow!("expected a number in byte specifier")),
        };

        if !(first.parse::<u64>().is_ok() && second.parse::<u64>().is_ok()) {
            eprintln!("Error: DMX value has an invalid value");
            value = 0;
            byte_specifier = 1;
        }

        Ok(DmxValue {
            value,
            byte_specifier: byte_specifier as u8,
            byte_shifting,
        })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelBitResolution {
    Bit8 = 1,
    Bit16 = 2,
    Bit24 = 3,
    Bit32 = 4,
}

impl From<u8> for ChannelBitResolution {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Bit8,
            2 => Self::Bit16,
            3 => Self::Bit24,
            4 => Self::Bit32,
            _ => panic!("invalid channel bit resolution"),
        }
    }
}

pub(crate) fn parse_name(name: String) -> Result<String> {
    // FIXME: Validate name with a regex.
    Ok(name.to_string())
}

pub(crate) fn parse_guid(name: String) -> Result<String> {
    // FIXME: Validate GUID with a regex.
    Ok(name.to_string())
}

pub(crate) fn parse_yes_no(s: String) -> Result<bool> {
    match s.as_str() {
        "Yes" => Ok(true),
        "No" => Ok(false),
        other => return Err(anyhow::anyhow!("expected 'Yes' or 'No'. Found: {other}")),
    }
}

pub(crate) fn parse_int_array(s: String) -> Result<Vec<i32>> {
    let parts: Vec<&str> = s.split(',').collect();
    let mut array = Vec::with_capacity(parts.len());
    for part in parts {
        let value = part
            .parse()
            .map_err(|_| anyhow!("Failed to parse int array value: '{}'", part))?;
        array.push(value);
    }
    Ok(array)
}

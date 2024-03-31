//! # GDTF
//!
//! [File Format Definition](https://gdtf.eu/gdtf/file-spec/file-format-definition/#file-format-definition)

use std::{fs::File, io, str::FromStr};

use error::Error;

pub mod attr_defs;
pub mod dmx_modes;
pub mod error;
pub mod fixture_type;

pub(crate) mod raw;

pub use attr_defs::*;
pub use dmx_modes::*;
pub use fixture_type::*;

pub(crate) fn parse_name(name: raw::RawName) -> Result<String, Error> {
    // FIXME: Validate name with a regex.

    Ok(name.to_string())
}

pub type Node = Vec<String>;

pub(crate) fn parse_node(node: raw::RawNode) -> Result<Node, Error> {
    let split = node.split('.').map(String::from).collect::<Vec<_>>();

    if split.is_empty() {
        return Err(Error::EmptyNode);
    }

    Ok(split)
}

pub type ColorCIE = [f32; 3];

pub(crate) fn parse_color_cie(color_cie: raw::RawColorCIE) -> Result<ColorCIE, Error> {
    let parts: Vec<&str> = color_cie.split(',').collect();
    if parts.len() != 3 {
        return Err(Error::ParseError(format!(
            "Invalid CIE color: '{}'. Expected 3 parts, but found {}",
            color_cie,
            parts.len()
        )));
    }

    let x = parts[0]
        .parse()
        .map_err(|_| Error::ParseError(format!("Failed to parse CIE color X: '{}'", parts[0])))?;

    let y = parts[1]
        .parse()
        .map_err(|_| Error::ParseError(format!("Failed to parse CIE color Y: '{}'", parts[1])))?;

    let large_y = parts[2].parse().map_err(|_| {
        Error::ParseError(format!("Failed to parse CIE color large Y: '{}'", parts[2]))
    })?;

    Ok([x, y, large_y])
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxValue {
    value: u64,
    byte_specifier: u8,
    byte_shifting: bool,
}

impl DmxValue {
    pub fn value(&self, channel_resolution: ChannelBitResolution) -> Result<u64, error::Error> {
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
            return Err(error::Error::ParseError(
                format!(
                    "DMX value of {} is out of range for the channel resolution: {:?}",
                    result, channel_resolution
                )
                .to_string(),
            ));
        }

        Ok(result)
    }

    pub fn bytes(&self, channel_resolution: ChannelBitResolution) -> Result<Vec<u8>, error::Error> {
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
    type Err = error::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(error::Error::ParseError(
                "expected a non-empty string in the DMX value.".to_string(),
            ));
        }

        let (first, mut second) = match value.split_once('/') {
            Some((first, second)) => (first.to_string(), second.to_string()),
            None => {
                return Err(error::Error::ParseError(
                    "expected a '/' in the DMX value.".to_string(),
                ))
            }
        };

        let mut byte_shifting = false;
        if second.ends_with('s') {
            byte_shifting = true;
            second.pop();
        }

        let mut value: u64 = match first.parse() {
            Ok(value) => value,
            Err(_) => {
                return Err(error::Error::ParseError(
                    "expected a number in the DMX value.".to_string(),
                ))
            }
        };

        let mut byte_specifier: i32 = match second.parse() {
            Ok(value) => value,
            Err(_) => {
                return Err(error::Error::ParseError(
                    "expected a number in byte specifier".to_string(),
                ))
            }
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

pub type Guid = String;

pub(crate) fn parse_guid(guid: raw::RawGuid) -> Result<String, Error> {
    if guid.is_empty() {
        return Err(Error::InvalidGuid(guid.to_string()));
    }

    // FIXME: Validate GUID with a regex.

    Ok(guid.to_string())
}

pub(crate) fn parse_optional_guid(guid: Option<raw::RawGuid>) -> Result<Option<String>, Error> {
    match guid {
        Some(guid) => match guid.is_empty() {
            true => Ok(None),
            false => Ok(Some(parse_guid(guid)?)),
        },
        None => Ok(None),
    }
}

pub type Resource = String;

pub(crate) fn parse_int_array(value: &str) -> Result<Vec<i32>, Error> {
    value
        .split(',')
        .map(|s| {
            s.parse()
                .map_err(|_| Error::ParseError(format!("Invalid integer: {}", s)))
        })
        .collect()
}

pub(crate) fn parse_yes_no(value: &str) -> Result<bool, Error> {
    match value {
        "Yes" => Ok(true),
        "No" => Ok(false),
        _ => Err(Error::ParseError(format!(
            "Invalid value for 'Yes'/'No': {}",
            value
        ))),
    }
}

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
    pub fn from_archive_reader<R>(reader: R) -> Result<GdtfDescription, Error>
    where
        R: io::Read + io::Seek,
    {
        let mut archive = zip::ZipArchive::new(reader).map_err(|_| Error::UnzipError)?;

        let description = archive
            .by_name("description.xml")
            .map_err(|_| Error::MissingDescription)?;

        let description_reader = io::BufReader::new(description);

        let raw_gdtf: raw::RawGdtfDescription = serde_xml_rs::from_reader(description_reader)
            .map_err(|e| Error::ParseError(format!("failed to parse GDTF description: {}", e)))?;
        let gdtf: GdtfDescription = raw_gdtf.try_into()?;

        Ok(gdtf)
    }

    /// Create a new GDTF file from .gdtf archive bytes.
    pub fn from_archive_bytes(bytes: &[u8]) -> Result<GdtfDescription, Error> {
        let bytes = std::io::Cursor::new(bytes);
        let reader = std::io::BufReader::new(bytes);
        Self::from_archive_reader(reader)
    }

    /// Create a new GDTF file from a descriptor file.
    pub fn from_file(file: &File) -> Result<GdtfDescription, Error> {
        let file_reader = io::BufReader::new(file);
        let raw_gdtf: raw::RawGdtfDescription = serde_xml_rs::from_reader(file_reader)
            .map_err(|e| Error::ParseError(format!("failed to parse GDTF description: {}", e)))?;
        let gdtf: GdtfDescription = raw_gdtf.try_into()?;

        Ok(gdtf)
    }
}

impl TryFrom<raw::RawGdtfDescription> for GdtfDescription {
    type Error = Error;

    fn try_from(value: raw::RawGdtfDescription) -> Result<Self, Self::Error> {
        Ok(GdtfDescription {
            data_version: value.data_version.parse()?,
            fixture_type: value.fixture_type.try_into()?,
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
            return Err(Error::ParseError(format!(
                "Invalid data version: '{}'. Expected 2 parts, but found {}",
                s,
                parts.len()
            )));
        }

        let major = parts[0].parse().map_err(|_| {
            Error::ParseError(format!(
                "Failed to parse data version major: '{}'",
                parts[0]
            ))
        })?;

        let minor = parts[1].parse().map_err(|_| {
            Error::ParseError(format!(
                "Failed to parse data version minor: '{}'",
                parts[1]
            ))
        })?;

        Ok(Self { major, minor })
    }
}

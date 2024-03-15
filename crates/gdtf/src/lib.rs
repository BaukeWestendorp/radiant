//! # GDTF
//!
//! [File Format Definition](https://gdtf.eu/gdtf/file-spec/file-format-definition/#file-format-definition)

#![warn(missing_docs)]

use std::fs::File;
use std::io;

use fixture_type::FixtureType;
use once_cell::sync::Lazy;
use regex::Regex;

#[allow(missing_docs)]
pub mod error;

pub mod fixture_type;

/// # A GDTF archive file.
pub struct GdtfArchive {
    /// The GDTF file descriptor.
    pub description: GdtfDescription,
}

/// # A GDTF description.
///
/// [GDTF Node Attributes](https://gdtf.eu/gdtf/file-spec/file-format-definition/#table-2-gdtf-node-attributes)
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GdtfDescription {
    /// The DataVersion attribute defines the minimal version of compatibility.
    /// The Version format is “Major.Minor”, where major and minor is Uint with
    /// size 1 byte.
    #[serde(rename = "DataVersion")]
    pub data_version: String,

    /// The FixtureType node is the starting point of the description of the
    /// fixture.
    #[serde(rename = "FixtureType")]
    pub fixture_type: FixtureType,
}

impl GdtfDescription {
    /// Create a new GDTF file from a .gdtf archive file.
    pub fn from_archive_reader<R>(reader: R) -> Result<GdtfDescription, Box<dyn std::error::Error>>
    where
        R: io::Read + io::Seek,
    {
        let mut archive = zip::ZipArchive::new(reader).unwrap();

        let description = archive
            .by_name("description.xml")
            .expect("expected 'description.xml'");

        let description_reader = io::BufReader::new(description);

        let gdtf: GdtfDescription = serde_xml_rs::from_reader(description_reader).unwrap();

        Ok(gdtf)
    }

    /// Create a new GDTF file from .gdtf archive bytes.
    pub fn from_archive_bytes(bytes: &[u8]) -> Result<GdtfDescription, Box<dyn std::error::Error>> {
        let bytes = std::io::Cursor::new(bytes);
        let reader = std::io::BufReader::new(bytes);
        Self::from_archive_reader(reader)
    }

    /// Create a new GDTF file from a descriptor file.
    pub fn from_file(file: &File) -> Result<GdtfDescription, Box<dyn std::error::Error>> {
        let file_reader = io::BufReader::new(file);
        let gdtf: GdtfDescription = serde_xml_rs::from_reader(file_reader).unwrap();

        Ok(gdtf)
    }
}

/// Unique object names; The allowed characters are listed in Annex C Default
/// value: object type with an index in parent.
#[derive(Debug, Clone, PartialEq)]
pub struct Name(String);

impl Name {
    /// Create a new name.
    pub fn new(name: String) -> Result<Self, error::Error> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"^[ "%%'()*+\-\/#0-9:;<=>@A-Z_a-z`]*$"#).unwrap());

        match REGEX.is_match(&name) {
            true => Ok(Self(name)),
            false => Err(error::Error::ParseError("invalid name".to_string())),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Name, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;

        match Name::new(s) {
            Ok(name) => Ok(name),
            Err(_) => Err(serde::de::Error::custom("invalid name")),
        }
    }
}

/// Link to an element: “Name” is the value of the attribute “Name” of a defined
/// XML node. The starting point defines each attribute separately.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    references: Vec<String>,
}

impl Node {
    /// Create a new node.
    pub fn new(references: Vec<String>) -> Self {
        Self { references }
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let references = value.split('.').map(|s| s.to_string()).collect();
        Self { references }
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl<'de> serde::Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Node, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        let references = s.split('.').map(|s| s.to_string()).collect();
        Ok(Node { references })
    }
}

/// CIE color representation xyY 1931
#[derive(Debug, Clone, PartialEq)]
pub struct ColorCIE {
    /// x value
    pub x: f32,

    /// y value
    pub y: f32,

    /// Y value
    pub large_y: f32,
}

impl ColorCIE {
    /// Create a new CIE color representation.
    #[allow(non_snake_case)]
    pub fn new(x: f32, y: f32, large_y: f32) -> Self {
        Self { x, y, large_y }
    }
}

impl<'de> serde::Deserialize<'de> for ColorCIE {
    fn deserialize<D>(deserializer: D) -> Result<ColorCIE, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values: Vec<f32> = deserialize_float_array(deserializer)?;

        Ok(ColorCIE {
            x: values[0],
            y: values[1],
            large_y: values[2],
        })
    }
}

/// Rotation matrix, consist of 3*3 floats. Stored as row-major matrix, i.e.
/// each row of the matrix is stored as a 3-component vector. Mathematical
/// definition of the matrix is column-major, i.e. the matrix rotation is stored
/// in the three columns. Metric system, right-handed Cartesian coordinates XYZ:
///
/// X – from left (-X) to right (+X),
///
/// Y – from the outside of the monitor (-Y) to the inside of the monitor (+Y),
///
/// Z – from the bottom (-Z) to the top (+Z).
/// TODO: Implement a matrix type.
pub type Rotation = String;

/// Special type to define DMX value where n is the byte count. The byte count
/// can be individually specified without depending on the resolution of the DMX
/// Channel.
///
/// By default byte mirroring is used for the conversion. So 255/1 in a 16 bit
/// channel will result in 65535.
///
/// You can use the byte shifting operator to use byte shifting for the
/// conversion. So 255/1s in a 16 bit channel will result in 65280.
#[derive(Debug, Clone, PartialEq)]
pub struct DmxValue(u64);

impl DmxValue {
    /// Create a new DMX value from a string.
    pub fn try_from_string(
        value: &str,
        channel_resolution: Option<ChannelBitResolution>,
    ) -> Result<Self, error::Error> {
        let channel_resolution = match channel_resolution {
            Some(value) => value,
            None => ChannelBitResolution::Bit8,
        };

        if value.is_empty() {
            return Ok(Self(0));
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

        let mut dmx_value_raw: u64 = match first.parse() {
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
            dmx_value_raw = 0;
            byte_specifier = 1;
        }

        // Check if the ByteSpecifier is different to the ChannelResolution.
        let mut result = dmx_value_raw;
        if byte_shifting {
            if byte_specifier != channel_resolution as i32 {
                let shift: i32 = 8 * (channel_resolution as i32 - byte_specifier);
                if shift >= 0 {
                    result <<= shift;
                } else {
                    result >>= -shift;
                }
            } else {
                // We can take the value as it is defined in the document without scaling it to
                // another BitResolution.
                result = dmx_value_raw;
            }
        } else {
            if byte_specifier != channel_resolution as i32 {
                let max_resolution = get_channel_max_dmx((byte_specifier as u8).into());
                let max_channel_unit = get_channel_max_dmx(channel_resolution);

                let percentage = dmx_value_raw as f64 / max_resolution.0 as f64;

                result = (percentage * max_channel_unit.0 as f64) as u64;
            } else {
                // We can take the value as it is defined in the document without scaling it to
                // another BitResolution.
                result = dmx_value_raw;
            }
        }

        if get_channel_max_dmx(channel_resolution).0 < result {
            return Err(error::Error::ParseError(
                format!(
                    "DMX value of {} is out of range for the channel resolution: {:?}",
                    result, channel_resolution
                )
                .to_string(),
            ));
        }

        Ok(Self(result))
    }
}

fn get_channel_max_dmx(channel_resolution: ChannelBitResolution) -> DmxValue {
    let max_val = match channel_resolution {
        ChannelBitResolution::Bit8 => 256,
        ChannelBitResolution::Bit16 => 256 * 256,
        ChannelBitResolution::Bit24 => 256 * 256 * 256,
        ChannelBitResolution::Bit32 => 256 * 256 * 256 * 256,
    };

    DmxValue(max_val - 1)
}

impl From<u64> for DmxValue {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for DmxValue {
    type Error = error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let channel_resolution = ChannelBitResolution::Bit8;
        DmxValue::try_from_string(value.as_str(), Some(channel_resolution))
    }
}

impl TryFrom<&str> for DmxValue {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        value.try_into()
    }
}

impl<'de> serde::Deserialize<'de> for DmxValue {
    fn deserialize<D>(deserializer: D) -> Result<DmxValue, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;

        s.try_into()
            .map_err(|error: error::Error| serde::de::Error::custom(error.message()))
    }
}

/// The resolution of the channel in bits.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelBitResolution {
    /// 8 bit resolution.
    Bit8 = 1,
    /// 16 bit resolution.
    Bit16 = 2,
    /// 24 bit resolution.
    Bit24 = 3,
    /// 32 bit resolution.
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

/// Unique ID corresponding to RFC 4122: X–1 digit in hexadecimal notation.
///
/// Example: “308EA87D-7164-42DE-8106-A6D273F57A51”.
#[derive(Debug, Clone, PartialEq)]
pub struct Guid(String);

impl<'de> serde::Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Guid, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;

        Ok(Guid(s))
    }
}

/// File name of the resource file without extension and without subfolder.
#[derive(Debug, Clone, PartialEq)]
pub struct Resource(String);

impl<'de> serde::Deserialize<'de> for Resource {
    fn deserialize<D>(deserializer: D) -> Result<Resource, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;

        Ok(Resource(s))
    }
}

/// A point in 2D space.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    #[allow(missing_docs)]
    pub x: f32,

    #[allow(missing_docs)]
    pub y: f32,
}

impl<'de> serde::Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values: Vec<f32> = deserialize_float_array(deserializer)?;

        Ok(Point {
            x: values[0],
            y: values[1],
        })
    }
}

pub(crate) fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "Yes" => Ok(true),
        "No" => Ok(false),
        _ => Err(serde::de::Error::custom("expected 'Yes' or 'No'")),
    }
}

pub(crate) fn deserialize_float_array<'de, D>(deserializer: D) -> Result<Vec<f32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;

    let split = s.split(',');
    let floats: Vec<f32> = split.map(|s| s.parse().unwrap()).collect();
    Ok(floats)
}

pub(crate) fn deserialize_optional_int_array<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<i32>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;

    if s.is_empty() {
        return Ok(Some(Vec::new()));
    }

    match s.as_str() {
        "None" => Ok(None),
        _ => {
            let split = s.split(',');
            let ints: Vec<i32> = split.map(|s| s.parse().unwrap()).collect();
            Ok(Some(ints))
        }
    }
}

pub(crate) fn deserialize_optional_dmx_value<'de, D>(
    deserializer: D,
) -> Result<Option<DmxValue>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "None" => Ok(None),
        // TODO: We currently ignore the bit resolution
        _ => match s.try_into() {
            Ok(v) => Ok(Some(v)),
            Err(error) => Err(serde::de::Error::custom(format!(
                "Invalid DmxValue value: {}",
                error.message()
            ))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_gdtf_file() {
        let root = std::env::current_dir().unwrap();
        let file = std::fs::File::open(root.join("tests/test_fixture.gdtf")).unwrap();
        let reader = io::BufReader::new(file);
        let gdtf = GdtfDescription::from_archive_reader(reader);

        assert!(gdtf.is_ok())
    }

    #[test]
    fn dmx_value_conversion() {
        macro_rules! test_dmx_value {
            ($value:expr, $resolution:expr, $expected:expr) => {
                let dmx_value = DmxValue::try_from_string($value, Some($resolution)).unwrap();
                assert_eq!(dmx_value.0, $expected);
            };
        }

        // --------------------------
        // Byte Mirroring
        test_dmx_value!("255/1", ChannelBitResolution::Bit8, 255);
        test_dmx_value!("255/1", ChannelBitResolution::Bit16, 65535);
        test_dmx_value!("255/1", ChannelBitResolution::Bit24, 16777215);
        test_dmx_value!("255/1", ChannelBitResolution::Bit32, 4294967295);

        test_dmx_value!("65535/2", ChannelBitResolution::Bit8, 255);
        test_dmx_value!("65535/2", ChannelBitResolution::Bit16, 65535);
        test_dmx_value!("65535/2", ChannelBitResolution::Bit24, 16777215);
        test_dmx_value!("65535/2", ChannelBitResolution::Bit32, 4294967295);

        test_dmx_value!("16777215/3", ChannelBitResolution::Bit8, 255);
        test_dmx_value!("16777215/3", ChannelBitResolution::Bit16, 65535);
        test_dmx_value!("16777215/3", ChannelBitResolution::Bit24, 16777215);
        test_dmx_value!("16777215/3", ChannelBitResolution::Bit32, 4294967295);

        test_dmx_value!("4294967295/4", ChannelBitResolution::Bit8, 255);
        test_dmx_value!("4294967295/4", ChannelBitResolution::Bit16, 65535);
        test_dmx_value!("4294967295/4", ChannelBitResolution::Bit24, 16777215);
        test_dmx_value!("4294967295/4", ChannelBitResolution::Bit32, 4294967295);

        // First Value
        test_dmx_value!("1/1", ChannelBitResolution::Bit8, 1);
        test_dmx_value!("1/1", ChannelBitResolution::Bit16, 257);
        test_dmx_value!("1/1", ChannelBitResolution::Bit24, 65793);
        test_dmx_value!("1/1", ChannelBitResolution::Bit32, 16843009);

        // --------------------------
        // Byte shifting
        test_dmx_value!("255/1s", ChannelBitResolution::Bit8, 255);
        test_dmx_value!("255/1s", ChannelBitResolution::Bit16, 65280);
        test_dmx_value!("255/1s", ChannelBitResolution::Bit24, 16711680);
        test_dmx_value!("255/1s", ChannelBitResolution::Bit32, 4278190080);

        test_dmx_value!("65535/2s", ChannelBitResolution::Bit8, 255);
        test_dmx_value!("65535/2s", ChannelBitResolution::Bit16, 65535);
        test_dmx_value!("65535/2s", ChannelBitResolution::Bit24, 16776960);
        test_dmx_value!("65535/2s", ChannelBitResolution::Bit32, 4294901760);

        test_dmx_value!("16777215/3s", ChannelBitResolution::Bit8, 255);
        test_dmx_value!("16777215/3s", ChannelBitResolution::Bit16, 65535);
        test_dmx_value!("16777215/3s", ChannelBitResolution::Bit24, 16777215);
        test_dmx_value!("16777215/3s", ChannelBitResolution::Bit32, 4294967040);

        test_dmx_value!("4294967295/4s", ChannelBitResolution::Bit8, 255);
        test_dmx_value!("4294967295/4s", ChannelBitResolution::Bit16, 65535);
        test_dmx_value!("4294967295/4s", ChannelBitResolution::Bit24, 16777215);
        test_dmx_value!("4294967295/4s", ChannelBitResolution::Bit32, 4294967295);

        // First Value
        test_dmx_value!("1/1s", ChannelBitResolution::Bit8, 1);
        test_dmx_value!("1/1s", ChannelBitResolution::Bit16, 256);
        test_dmx_value!("1/1s", ChannelBitResolution::Bit24, 65536);
        test_dmx_value!("1/1s", ChannelBitResolution::Bit32, 16777216);
    }
}

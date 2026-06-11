use std::{collections::BTreeMap, fmt, num, str};

use crate::{
    dmx::{self, Address},
    mvr_gdtf::gdtf::{self, attr::AttributeName},
    patch::FixtureId,
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AttributeValue {
    Clamped(ClampedValue),
    Physical(f32),
}

impl AttributeValue {
    pub fn to_clamped_value(&self, min: AttributeValue, max: AttributeValue) -> ClampedValue {
        match self {
            AttributeValue::Clamped(v) => *v,
            AttributeValue::Physical(v) => {
                let (mut min, mut max) = match (min, max) {
                    (AttributeValue::Physical(min), AttributeValue::Physical(max)) => (min, max),
                    _ => (0.0, 1.0),
                };

                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }

                let range = max - min;
                if range == 0.0 {
                    return ClampedValue::new(0.0);
                }

                let normalized = ((*v - min) / range).clamp(0.0, 1.0);
                ClampedValue::new(normalized)
            }
        }
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            AttributeValue::Clamped(v) => v.as_f32(),
            AttributeValue::Physical(v) => *v,
        }
    }
}

impl From<ClampedValue> for AttributeValue {
    fn from(value: ClampedValue) -> Self {
        Self::Clamped(value)
    }
}

impl From<AttributeValue> for f32 {
    fn from(value: AttributeValue) -> Self {
        value.as_f32()
    }
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeValue::Clamped(val) => write!(f, "Clamped({})", val),
            AttributeValue::Physical(value) => {
                write!(f, "Physical({})", value)
            }
        }
    }
}

/// Stores [`AttributeValue`]s for each [`FixtureId`]'s [`Attribute`].
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AttributeValues {
    values: BTreeMap<FixtureId, BTreeMap<AttributeName, AttributeValue>>,
}

impl Default for AttributeValues {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeValues {
    pub fn new() -> Self {
        Self { values: BTreeMap::new() }
    }

    pub fn set(
        &mut self,
        fixture_id: FixtureId,
        attribute: AttributeName,
        value: impl Into<AttributeValue>,
    ) {
        self.values.entry(fixture_id).or_default().insert(attribute, value.into());
    }

    pub fn values(&self) -> impl Iterator<Item = (&FixtureId, &AttributeName, &AttributeValue)> {
        self.values.iter().flat_map(
            |(fixture_id, attrs): (&FixtureId, &BTreeMap<AttributeName, AttributeValue>)| {
                attrs.iter().map(move |(attr, val): (&AttributeName, &AttributeValue)| {
                    (fixture_id, attr, val)
                })
            },
        )
    }

    pub fn get(&self, id: &FixtureId, attribute: &AttributeName) -> Option<AttributeValue> {
        self.values
            .get(id)
            .and_then(|attrs: &BTreeMap<AttributeName, AttributeValue>| attrs.get(attribute))
            .copied()
    }

    pub fn contains(&self, id: &FixtureId, attribute: &AttributeName) -> bool {
        self.values.get(id).map(|attrs| attrs.contains_key(attribute)).unwrap_or(false)
    }

    pub fn extend(&mut self, other: AttributeValues) {
        for (fixture_id, attrs) in other.values {
            self.values.entry(fixture_id).or_default().extend(attrs);
        }
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

/// A clamped value between `0.0..=1.0`.
///
/// Represents a floating-point value constrained to the range
/// `0.0..=1.0`. All operations automatically clamp values to this valid range.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ClampedValue(f32);

impl ClampedValue {
    /// The minimum allowed value (0.0).
    pub const MIN: f32 = 0.0;

    /// The maximum allowed value (1.0).
    pub const MAX: f32 = 1.0;

    /// Creates a new [`ClampedValue`] with the specified value.
    ///
    /// The value is automatically clamped to the range `0.0..=1.0`.
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN, Self::MAX))
    }

    /// Sets the value of this [`ClampedValue`].
    ///
    /// The value is automatically clamped to the range `0.0..=1.0`.
    #[inline]
    pub fn set(&mut self, value: f32) {
        self.0 = value.clamp(Self::MIN, Self::MAX);
    }

    /// Returns the underlying `f32` value.
    ///
    /// The returned value is guaranteed to be in the range `0.0..=1.0`.
    #[inline]
    pub fn as_f32(self) -> f32 {
        self.0
    }

    /// Performs linear interpolation between this value and another.
    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(Self::MIN, Self::MAX);
        Self::new(self.0 * (1.0 - t) + other.0 * t)
    }

    /// Converts the value to a 1-byte representation ([`u8`]).
    #[inline]
    pub fn to_u8(&self) -> u8 {
        (self.0 * 255.0).round().clamp(0.0, 255.0) as u8
    }

    /// Converts the value to a 2-byte representation (`[u8; 2]`), big-endian.
    #[inline]
    pub fn to_u16_bytes(&self) -> [u8; 2] {
        let val = (self.0 * 65535.0).round().clamp(0.0, 65535.0) as u16;
        val.to_be_bytes()
    }

    /// Converts the value to a 3-byte representation (`[u8; 3]`), big-endian.
    #[inline]
    pub fn to_u24_bytes(&self) -> [u8; 3] {
        let val = (self.0 * 16777215.0).round().clamp(0.0, 16777215.0) as u32;
        [((val >> 16) & 0xFF) as u8, ((val >> 8) & 0xFF) as u8, (val & 0xFF) as u8]
    }

    /// Converts the value to a 4-byte representation (`[u8; 4]`), big-endian.
    #[inline]
    pub fn to_u32_bytes(&self) -> [u8; 4] {
        let val = (self.0 * 4294967295.0).round().clamp(0.0, 4294967295.0) as u32;
        val.to_be_bytes()
    }

    /// Converts the value to values directly mappable at addresses.
    pub fn to_address_values(&self, addresses: &[Address]) -> Vec<(Address, dmx::Value)> {
        let bytes: Vec<u8> = match addresses.len() {
            1 => vec![self.to_u8()],
            2 => self.to_u16_bytes().to_vec(),
            3 => self.to_u24_bytes().to_vec(),
            4 => self.to_u32_bytes().to_vec(),
            _ => {
                log::warn!(
                    "cannot set DMX channel value for fixture: unsupported address length {}",
                    addresses.len()
                );
                return Vec::new();
            }
        };

        addresses.iter().copied().zip(bytes.into_iter().map(dmx::Value::from)).collect()
    }
}

impl fmt::Display for ClampedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f32> for ClampedValue {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<ClampedValue> for f32 {
    fn from(value: ClampedValue) -> Self {
        value.0
    }
}

impl From<ClampedValue> for f64 {
    fn from(value: ClampedValue) -> Self {
        value.0 as f64
    }
}

impl From<ClampedValue> for dmx::Value {
    fn from(value: ClampedValue) -> Self {
        dmx::Value((value.0 * (u8::MAX as f32)) as u8)
    }
}

impl From<gdtf::dmx::DmxValue> for ClampedValue {
    fn from(value: gdtf::dmx::DmxValue) -> Self {
        ClampedValue::new(value.to_normalized() as f32)
    }
}

impl str::FromStr for ClampedValue {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}

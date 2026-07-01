use std::{cmp, fmt, num::NonZeroU32, str};

use anyhow::Context as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct FixtureIdPart(NonZeroU32);

impl FixtureIdPart {
    pub fn new(id: u32) -> anyhow::Result<Self> {
        match NonZeroU32::new(id) {
            Some(id) => Ok(FixtureIdPart(id)),
            None => anyhow::bail!("FixtureIdPart must be nonzero (got {})", id),
        }
    }

    pub fn as_u32(&self) -> u32 {
        self.0.into()
    }

    pub fn offset(self, offset: i32) -> anyhow::Result<Self> {
        let base = self.as_u32() as i64;
        let id = base + offset as i64;
        if id <= 0 || id > u32::MAX as i64 {
            anyhow::bail!("FixtureIdPart offset out of range: {} + {} = {}", base, offset, id);
        }
        Ok(FixtureIdPart(NonZeroU32::new(id as u32).unwrap()))
    }
}

impl Default for FixtureIdPart {
    fn default() -> Self {
        Self(NonZeroU32::new(1).unwrap())
    }
}

impl fmt::Display for FixtureIdPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_u32())
    }
}

impl str::FromStr for FixtureIdPart {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .parse::<u32>()
            .map_err(|e| anyhow::anyhow!("Failed to parse FixtureIdPart from '{}': {}", s, e))?;
        FixtureIdPart::new(id)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixtureId {
    ids: [FixtureIdPart; Self::MAX_LEN],
    len: u8,
}

impl FixtureId {
    pub const MAX_LEN: usize = 8;

    pub fn new(root_part: FixtureIdPart) -> Self {
        let mut ids = [FixtureIdPart::new(1).unwrap(); Self::MAX_LEN];
        ids[0] = root_part;
        FixtureId { ids, len: 1 }
    }

    pub fn push(&mut self, part: FixtureIdPart) {
        self.try_push(part).expect("FixtureId capacity exceeded");
    }

    pub fn try_push(&mut self, part: FixtureIdPart) -> anyhow::Result<()> {
        let len = self.len();
        if len >= Self::MAX_LEN {
            anyhow::bail!("FixtureId too long (max {} parts)", Self::MAX_LEN);
        }
        self.ids[len] = part;
        self.len = (len + 1) as u8;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_root(&self) -> bool {
        self.len == 1
    }

    pub fn sub_len(&self) -> usize {
        assert!(!self.is_empty(), "FixtureId must have at least a root");
        self.len() - 1
    }

    pub fn root(&self) -> FixtureIdPart {
        self.ids[0]
    }

    pub fn last(&self) -> FixtureIdPart {
        let l = self.len();
        assert!(l >= 1, "FixtureId must have at least a root");
        self.ids[l - 1]
    }

    pub fn as_slice(&self) -> &[FixtureIdPart] {
        &self.ids[..self.len()]
    }

    pub fn iter(&self) -> std::slice::Iter<'_, FixtureIdPart> {
        self.as_slice().iter()
    }

    pub fn replace_last(&mut self, sub_part: FixtureIdPart) {
        let l = self.len();
        assert!(l >= 1, "FixtureId must have at least a root");
        self.ids[l - 1] = sub_part;
    }

    pub fn extended_with(mut self, part: FixtureIdPart) -> FixtureId {
        self.push(part);
        self
    }

    pub fn starts_with_fixture_id(&self, prefix: &FixtureId) -> bool {
        let prefix_len = prefix.len();
        if prefix_len > self.len() {
            return false;
        }
        &self.as_slice()[..prefix_len] == prefix.as_slice()
    }

    pub fn contains(&self, other: &FixtureId) -> bool {
        self.starts_with_fixture_id(other)
    }
}

impl AsRef<[FixtureIdPart]> for FixtureId {
    fn as_ref(&self) -> &[FixtureIdPart] {
        self.as_slice()
    }
}

impl From<FixtureIdPart> for FixtureId {
    fn from(part: FixtureIdPart) -> Self {
        FixtureId::new(part)
    }
}

impl From<&[FixtureIdPart]> for FixtureId {
    fn from(slice: &[FixtureIdPart]) -> Self {
        assert!(
            slice.len() <= FixtureId::MAX_LEN,
            "FixtureId slice length {} exceeds capacity {}",
            slice.len(),
            FixtureId::MAX_LEN
        );
        let mut ids = [FixtureIdPart::new(1).unwrap(); FixtureId::MAX_LEN];
        for (i, v) in slice.iter().enumerate() {
            ids[i] = *v;
        }
        FixtureId { ids, len: slice.len() as u8 }
    }
}

pub struct FixtureIdIntoIter {
    id: FixtureId,
    idx: u8,
}

impl Iterator for FixtureIdIntoIter {
    type Item = FixtureIdPart;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.id.len as usize;
        let idx = self.idx as usize;
        if idx >= len {
            return None;
        }
        self.idx += 1;
        Some(self.id.ids[idx])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.id.len as usize).saturating_sub(self.idx as usize);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for FixtureIdIntoIter {}

impl IntoIterator for FixtureId {
    type Item = FixtureIdPart;
    type IntoIter = FixtureIdIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        FixtureIdIntoIter { id: self, idx: 0 }
    }
}

impl<'a> IntoIterator for &'a FixtureId {
    type Item = &'a FixtureIdPart;
    type IntoIter = std::slice::Iter<'a, FixtureIdPart>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl cmp::PartialOrd for FixtureId {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for FixtureId {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let a = self.as_slice();
        let b = other.as_slice();
        for (x, y) in a.iter().zip(b.iter()) {
            let ord = x.cmp(y);
            if ord != cmp::Ordering::Equal {
                return ord;
            }
        }
        a.len().cmp(&b.len())
    }
}

impl fmt::Display for FixtureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for part in self.as_slice() {
            if !first {
                write!(f, ".")?;
            }
            write!(f, "{}", part)?;
            first = false;
        }
        Ok(())
    }
}

impl fmt::Debug for FixtureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FixtureId(")?;
        fmt::Display::fmt(self, f)?;
        write!(f, ")")
    }
}

impl str::FromStr for FixtureId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            anyhow::bail!("FixtureId string is empty");
        }

        let mut ids = [FixtureIdPart::new(1).unwrap(); FixtureId::MAX_LEN];
        let mut len: usize = 0;

        for (i, part) in s.split('.').enumerate() {
            if i >= FixtureId::MAX_LEN {
                anyhow::bail!("FixtureId has too many parts (max {})", FixtureId::MAX_LEN);
            }
            ids[i] = FixtureIdPart::from_str(part)
                .with_context(|| format!("Invalid FixtureIdPart at position {}: '{}", i, part))?;
            len += 1;
        }

        if len == 0 {
            anyhow::bail!("FixtureId string is empty after parsing");
        }

        Ok(FixtureId { ids, len: len as u8 })
    }
}

impl serde::Serialize for FixtureId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for FixtureId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FixtureIdVisitor;

        impl<'de> serde::de::Visitor<'de> for FixtureIdVisitor {
            type Value = FixtureId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a FixtureId")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::str::FromStr;
                FixtureId::from_str(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(FixtureIdVisitor)
    }
}

pub trait IntoFixtureId {
    fn into_fixture_id(self) -> Option<FixtureId>;
}

impl IntoFixtureId for FixtureId {
    fn into_fixture_id(self) -> Option<FixtureId> {
        Some(self)
    }
}

impl IntoFixtureId for &FixtureId {
    fn into_fixture_id(self) -> Option<FixtureId> {
        Some(self.clone())
    }
}

impl IntoFixtureId for &str {
    fn into_fixture_id(self) -> Option<FixtureId> {
        self.parse().ok()
    }
}

impl IntoFixtureId for String {
    fn into_fixture_id(self) -> Option<FixtureId> {
        self.parse().ok()
    }
}

impl IntoFixtureId for u8 {
    fn into_fixture_id(self) -> Option<FixtureId> {
        FixtureIdPart::new(self as u32).ok().map(FixtureId::from)
    }
}

impl IntoFixtureId for i8 {
    fn into_fixture_id(self) -> Option<FixtureId> {
        if self > 0 { FixtureIdPart::new(self as u32).ok().map(FixtureId::from) } else { None }
    }
}

impl IntoFixtureId for u16 {
    fn into_fixture_id(self) -> Option<FixtureId> {
        FixtureIdPart::new(self as u32).ok().map(FixtureId::from)
    }
}

impl IntoFixtureId for i16 {
    fn into_fixture_id(self) -> Option<FixtureId> {
        if self > 0 { FixtureIdPart::new(self as u32).ok().map(FixtureId::from) } else { None }
    }
}

impl IntoFixtureId for u32 {
    fn into_fixture_id(self) -> Option<FixtureId> {
        FixtureIdPart::new(self).ok().map(FixtureId::from)
    }
}

impl IntoFixtureId for i32 {
    fn into_fixture_id(self) -> Option<FixtureId> {
        if self > 0 { FixtureIdPart::new(self as u32).ok().map(FixtureId::from) } else { None }
    }
}

impl IntoFixtureId for u64 {
    fn into_fixture_id(self) -> Option<FixtureId> {
        FixtureIdPart::new(self as u32).ok().map(FixtureId::from)
    }
}

impl IntoFixtureId for i64 {
    fn into_fixture_id(self) -> Option<FixtureId> {
        if self > 0 && self <= u32::MAX as i64 {
            FixtureIdPart::new(self as u32).ok().map(FixtureId::from)
        } else {
            None
        }
    }
}

impl IntoFixtureId for usize {
    fn into_fixture_id(self) -> Option<FixtureId> {
        if self > 0 && self <= u32::MAX as usize {
            FixtureIdPart::new(self as u32).ok().map(FixtureId::from)
        } else {
            None
        }
    }
}

pub trait IntoFixtureIds {
    fn into_fixture_ids(self) -> Box<dyn Iterator<Item = FixtureId>>;
}

impl<'a, I, T> IntoFixtureIds for I
where
    I: IntoIterator<Item = T>,
    T: IntoFixtureId,
    <I as IntoIterator>::IntoIter: 'static,
{
    fn into_fixture_ids(self) -> Box<dyn Iterator<Item = FixtureId>> {
        Box::new(self.into_iter().filter_map(|item| item.into_fixture_id()))
    }
}

impl IntoFixtureIds for FixtureId {
    fn into_fixture_ids(self) -> Box<dyn Iterator<Item = FixtureId>> {
        Box::new(std::iter::once(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part(n: u32) -> FixtureIdPart {
        FixtureIdPart::new(n).unwrap()
    }

    #[test]
    fn test_into_fixture_id_from_fixture_id() {
        let id = FixtureId::from(part(42));
        let result = id.clone().into_fixture_id();
        assert_eq!(result, Some(id));
    }

    #[test]
    fn test_into_fixture_id_from_ref_fixture_id() {
        let id = FixtureId::from(part(7));
        let result = (&id).into_fixture_id();
        assert_eq!(result, Some(id));
    }

    #[test]
    fn test_into_fixture_id_from_str_valid() {
        let s = "1.2.3";
        let id = FixtureId::from(&[part(1), part(2), part(3)][..]);
        let result = s.into_fixture_id();
        assert_eq!(result, Some(id));
    }

    #[test]
    fn test_into_fixture_id_from_str_invalid() {
        let s = "";
        let result = s.into_fixture_id();
        assert_eq!(result, None);

        let s = "0.2";
        let result = s.into_fixture_id();
        assert_eq!(result, None);

        let s = "1.2.3.4.5.6.7.8.9"; // Too long
        let result = s.into_fixture_id();
        assert_eq!(result, None);
    }

    #[test]
    fn test_into_fixture_id_from_string() {
        let s = String::from("5.6");
        let id = FixtureId::from(&[part(5), part(6)][..]);
        let result = s.into_fixture_id();
        assert_eq!(result, Some(id));
    }

    #[test]
    fn test_into_fixture_ids_from_vec_of_str() {
        let v = vec!["1.2", "3", "bad", "4.5.6"];
        let ids: Vec<_> = v.into_fixture_ids().collect();
        let expected = vec![
            FixtureId::from(&[part(1), part(2)][..]),
            FixtureId::from(part(3)),
            FixtureId::from(&[part(4), part(5), part(6)][..]),
        ];
        assert_eq!(ids, expected);
    }

    #[test]
    fn test_into_fixture_ids_from_vec_of_fixture_id() {
        let ids_in = vec![FixtureId::from(part(1)), FixtureId::from(&[part(2), part(3)][..])];
        let ids: Vec<_> = ids_in.clone().into_fixture_ids().collect();
        assert_eq!(ids, ids_in);
    }

    #[test]
    fn test_into_fixture_ids_from_vec_of_string() {
        let v = vec!["1.2".to_string(), "3".to_string()];
        let ids: Vec<_> = v.into_fixture_ids().collect();
        let expected = vec![FixtureId::from(&[part(1), part(2)][..]), FixtureId::from(part(3))];
        assert_eq!(ids, expected);
    }

    #[test]
    fn test_into_fixture_ids_from_fixture_id() {
        let id = FixtureId::from(&[part(9), part(8)][..]);
        let ids: Vec<_> = id.into_fixture_ids().collect();
        assert_eq!(ids, vec![FixtureId::from(&[part(9), part(8)][..])]);
    }

    #[test]
    fn test_into_fixture_ids_from_slice_of_str() {
        let arr = ["1", "2.3", "bad"];
        let ids: Vec<_> = arr.into_fixture_ids().collect();
        let expected = vec![FixtureId::from(part(1)), FixtureId::from(&[part(2), part(3)][..])];
        assert_eq!(ids, expected);
    }

    #[test]
    fn test_into_fixture_ids_from_slice_of_fixture_id() {
        let arr = [FixtureId::from(part(1)), FixtureId::from(&[part(2), part(3)][..])];
        let ids: Vec<_> = arr.into_fixture_ids().collect();
        assert_eq!(ids, arr);
    }
}

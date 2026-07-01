use crate::mvr_gdtf::gdtf::bundle;

#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl DateTime {
    pub fn parse_from_str(s: &str) -> Option<Self> {
        // YYYY-MM-DDTHH:MM:SS
        let parts: Vec<&str> = s.split('T').collect();
        if parts.len() != 2 {
            return None;
        }
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        if date_parts.len() != 3 || time_parts.len() != 3 {
            return None;
        }
        Some(Self {
            year: date_parts[0].parse().ok()?,
            month: date_parts[1].parse().ok()?,
            day: date_parts[2].parse().ok()?,
            hour: time_parts[0].parse().ok()?,
            minute: time_parts[1].parse().ok()?,
            second: time_parts[2].parse().ok()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Revision {
    pub text: String,
    pub date: Option<DateTime>,
    pub user_id: i32,
    pub modified_by: String,
}

impl<'src> bundle::FromBundle<'src> for Revision {
    type Source = bundle::Revision;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            text: source.text.trim().to_string(),
            date: source.date.as_ref().and_then(|s| DateTime::parse_from_str(s)),
            user_id: source.user_id,
            modified_by: source.modified_by.trim().to_string(),
        }
    }
}

//! # Show
//!
//! The show module contains the show struct and its sub-structs.

use crate::dmx::DmxChannel;
use gdtf::GdtfDescription;
use gdtf_share::GdtfShare;
use lazy_static::lazy_static;
use std::{
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
};

const FIXTURE_CACHE_PATH: &str = "radiant/fixtures";

#[cfg(test)]
const GDTF_SHARE_API_USER: &str = "TEST_GDTF_SHARE_API_USER";
#[cfg(test)]
const GDTF_SHARE_API_PASSWORD: &str = "TEST_GDTF_SHARE_API_PASSWORD";

#[cfg(not(test))]
const GDTF_SHARE_API_USER: &str = "GDTF_SHARE_API_USER";
#[cfg(not(test))]
const GDTF_SHARE_API_PASSWORD: &str = "GDTF_SHARE_API_PASSWORD";

lazy_static! {
    static ref BASE_DIRS: xdg::BaseDirectories = xdg::BaseDirectories::new().unwrap();
    static ref GDTF_SHARE: Result<GdtfShare, Error> = {
        let user =
            std::env::var(GDTF_SHARE_API_USER).map_err(|_| Error::GdtfShareAuthenticationFailed)?;
        let password = std::env::var(GDTF_SHARE_API_PASSWORD)
            .map_err(|_| Error::GdtfShareAuthenticationFailed)?;
        match futures_lite::future::block_on(GdtfShare::auth(user, password)) {
            Ok(gdtf_share) => {
                log::info!("Authenticated with GDTF Share API");
                Ok(gdtf_share)
            }
            Err(err) => Err(err.into()),
        }
    };
}

/// An id that uniquely identifies a fixture.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct FixtureId(usize);

impl FixtureId {
    /// Create a new fixture id.
    ///
    /// Will return None if the id is zero.
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

/// The revision id of a GDTF file.
pub type RevisionId = i32;

/// # Show
///
/// The show struct contains all information related to a show.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Show {
    patchlist: Patchlist,
}

impl Show {
    /// Create a new show.
    pub fn new() -> Self {
        Self {
            patchlist: Patchlist::new(),
        }
    }

    /// Get the patchlist.
    pub fn patchlist(&self) -> &Patchlist {
        &self.patchlist
    }

    /// Get the patchlist mutably.
    pub fn patchlist_mut(&mut self) -> &mut Patchlist {
        &mut self.patchlist
    }
}

impl<'de> serde::Deserialize<'de> for Show {
    fn deserialize<D>(deserializer: D) -> Result<Show, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let intermediate = ShowIntermediate::deserialize(deserializer)?;
        intermediate.try_into().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct ShowIntermediate {
    patchlist: PatchlistIntermediate,
}

impl TryInto<Show> for ShowIntermediate {
    type Error = Error;

    fn try_into(self) -> Result<Show, Error> {
        let patchlist = self.patchlist.try_into()?;
        Ok(Show { patchlist })
    }
}

/// # Patchlist
///
/// The patchlist struct contains all information about the fixtures in the show.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Patchlist {
    fixtures: Vec<Fixture>,
    gdtf_descriptions: Vec<Rc<GdtfDescription>>,
}

impl Patchlist {
    /// Create a new patchlist.
    pub fn new() -> Self {
        Self {
            fixtures: Vec::new(),
            gdtf_descriptions: Vec::new(),
        }
    }

    /// Get the fixtures.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    /// Get a fixture with the given id.
    pub fn fixture(&self, id: &FixtureId) -> Option<&Fixture> {
        self.fixtures().iter().find(|fixture| fixture.id == *id)
    }

    /// Patch a new fixture into the patchlist.
    ///
    /// # Errors
    ///
    /// This function can error if the GDTF file cannot be downloaded or if the GDTF file is invalid.
    pub async fn patch_fixture(
        &mut self,
        id: FixtureId,
        label: String,
        revision_id: RevisionId,
        channel: DmxChannel,
        mode: String,
    ) -> Result<(), Error> {
        let cached_file_path = BASE_DIRS
            .place_cache_file(Path::new(FIXTURE_CACHE_PATH).join(format!("{revision_id}.gdtf")))
            .map_err(|_| Error::GdtfFileCacheFailed)?;

        let description = match self.get_cached_description(&cached_file_path)? {
            Some(cached_description) => cached_description,
            None => {
                let gdtf_share = match GDTF_SHARE.as_ref() {
                    Ok(gdtf_share) => gdtf_share,
                    Err(err) => return Err(err.clone()),
                };

                let description_file = gdtf_share.download_file(revision_id).await?;
                let reader = std::io::Cursor::new(description_file.clone());
                let description = GdtfDescription::from_archive_reader(reader)
                    .map_err(|_| Error::GdtfFileInvalid)?;

                let mut file = std::fs::File::create_new(cached_file_path.clone())
                    .map_err(|_| Error::GdtfFileCacheFailed)?;
                file.write_all(&description_file)
                    .map_err(|_| Error::GdtfFileCacheFailed)?;
                log::info!("Cached GDTF file '{}'", cached_file_path.display());
                description
            }
        };

        let description = Rc::new(description);
        self.gdtf_descriptions.push(description.clone());

        self.fixtures.push(Fixture {
            id,
            label,
            revision_id,
            description,
            channel,
            mode,
        });

        Ok(())
    }

    fn get_cached_description(
        &self,
        file_path: &PathBuf,
    ) -> Result<Option<GdtfDescription>, Error> {
        match std::fs::read(file_path) {
            Ok(cached_file) => {
                let cached_description = GdtfDescription::from_archive_bytes(&cached_file)
                    .map_err(|_| Error::GdtfFileInvalid)?;
                log::info!("Using cached GDTF file '{}'", file_path.display());
                Ok(Some(cached_description))
            }
            _ => Ok(None),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Patchlist {
    fn deserialize<D>(deserializer: D) -> Result<Patchlist, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let intermediate = PatchlistIntermediate::deserialize(deserializer)?;
        intermediate.try_into().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct PatchlistIntermediate {
    pub fixtures: Vec<FixtureIntermediate>,
}

impl TryInto<Patchlist> for PatchlistIntermediate {
    type Error = Error;

    fn try_into(self) -> Result<Patchlist, Error> {
        let mut patchlist = Patchlist::new();
        for fixture in self.fixtures {
            // FIXME: We should make this non-blocking.
            futures_lite::future::block_on(patchlist.patch_fixture(
                fixture.id,
                fixture.label,
                fixture.revision_id,
                fixture.channel,
                fixture.mode,
            ))?;
        }
        Ok(patchlist)
    }
}

/// # Fixture
///
/// The fixture struct contains all information about a fixture in the show.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fixture {
    /// The fixture ID.
    pub id: FixtureId,
    /// A custom label for the fixture.
    pub label: String,
    /// The revision id of the fixture. This should correspond to the revision id of the GDTF description.
    pub revision_id: i32,
    /// The GDTF description of the fixture.
    pub description: Rc<GdtfDescription>,
    /// The channel on which the fixture is patched.
    pub channel: DmxChannel,
    /// The DMX mode used. The mode must be one of the modes in the GDTF description.
    pub mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct FixtureIntermediate {
    pub id: FixtureId,
    pub label: String,
    pub revision_id: i32,
    pub channel: DmxChannel,
    pub mode: String,
}

#[cfg(test)]
mod tests {
    use crate::show::FixtureId;

    use super::Show;

    #[test]
    fn deserialize_show() {
        let json = r#"{
            "patchlist": {
                "fixtures": [
                    {
                        "id": 1,
                        "label": "Test",
                        "revision_id": 31143,
                        "channel": [1, 2],
                        "mode": "Mode"
                    }
                ]
            }
        }"#;
        let show: Show = serde_json::from_str(json).unwrap();
        assert!(show.patchlist().fixture(&FixtureId::new(1)).is_some())
    }
}

/// Error type for errors related to the show.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Error when trying to authenticate to the GDTF share.
    #[error("GDTF share authentication failed")]
    GdtfShareAuthenticationFailed,
    /// Error when trying to cache a GDTF file.
    #[error("GDTF file caching failed")]
    GdtfFileCacheFailed,
    /// Error when trying to download a GDTF file.
    #[error("GDTF file download failed")]
    GdtfFileDownloadFailed,
    /// Error when trying to parse a GDTF file.
    #[error("GDTF file invalid")]
    GdtfFileInvalid,
    /// Error related to the GDTF share.
    #[error("{0}")]
    GdtfError(#[from] gdtf_share::Error),
}

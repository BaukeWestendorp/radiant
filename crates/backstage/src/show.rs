use anyhow::Result;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;

use dmx::{DmxChannel, DmxOutput};
use gdtf::GdtfDescription;
use gdtf_share::GdtfShare;

use crate::command::{Command, Instruction};
use crate::playback_engine::PlaybackEngine;
use crate::showfile::Showfile;

#[derive(Debug, Clone, PartialEq)]
pub struct Show {
    patchlist: Patchlist,

    programmer: Programmer,

    playback_engine: PlaybackEngine,
}

impl Show {
    pub async fn new(showfile: Showfile, gdtf_share: GdtfShare) -> Self {
        let mut patchlist = Patchlist::new();

        for f in showfile.patch_list.fixtures.into_iter() {
            let rid = f.gdtf_share_revision_id;
            let gdtf_description = match patchlist.get_gdtf_description(rid) {
                Some(description) => description,
                None => {
                    let description_file = gdtf_share.download_file(rid).await.unwrap();
                    let reader = Cursor::new(description_file);
                    let description = GdtfDescription::from_archive_reader(reader).unwrap();
                    patchlist.register_gdtf_description(rid, description)
                }
            };

            let fixture = Fixture {
                id: f.id,
                label: f.label,
                gdtf_description,
                channel: f.channel,
                mode: f.mode,
            };

            patchlist.patch_fixture(fixture);
        }

        let programmer = Programmer {
            selection: showfile.programmer.selection,
        };

        Self {
            patchlist,
            programmer,
            playback_engine: PlaybackEngine::new(),
        }
    }

    pub fn execute_command_str(&mut self, s: &str) -> Result<()> {
        let command = Command::parse(s)?;
        self.execute_command(command)?;
        Ok(())
    }

    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        // FIXME: This command execution is quite ad-hoc for now.
        match command.instructions.get(0) {
            Some(instr) => match instr {
                Instruction::Clear => {}
                Instruction::Group(id) => todo!("Select Group {}", id),
                Instruction::Fixture(id) => {
                    if !self.fixture_is_selected(*id) {
                        if self.fixture_exists(*id) {
                            self.programmer.selection.push(*id);
                            log::info!("Selected Fixture {id}")
                        } else {
                            log::error!("Failed to select Fixture {id}: Fixture not found")
                        }
                    }
                }
            },
            None => {}
        }
        Ok(())
    }

    pub fn fixture_is_selected(&self, id: usize) -> bool {
        self.programmer.selection.contains(&id)
    }

    pub fn fixture_exists(&self, id: usize) -> bool {
        self.get_fixture(id).is_some()
    }

    pub fn get_fixture(&self, id: usize) -> Option<&Fixture> {
        self.patchlist.fixtures.iter().find(|f| f.id == id)
    }

    pub fn get_stage_output(&mut self) -> DmxOutput {
        let playback = self.playback_engine.determine_dmx_output();
        playback
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Patchlist {
    fixtures: Vec<Fixture>,
    gdtf_descriptions: HashMap<i32, Rc<GdtfDescription>>,
}

impl Patchlist {
    pub fn new() -> Self {
        Self {
            fixtures: Vec::new(),
            gdtf_descriptions: HashMap::new(),
        }
    }

    pub fn patch_fixture(&mut self, fixture: Fixture) {
        self.fixtures.push(fixture);
    }

    pub fn register_gdtf_description(
        &mut self,
        rid: i32,
        gdtf_description: GdtfDescription,
    ) -> Rc<GdtfDescription> {
        let gdtf_description = Rc::new(gdtf_description);
        self.gdtf_descriptions.insert(rid, gdtf_description.clone());
        gdtf_description
    }

    pub fn get_gdtf_description(&self, rid: i32) -> Option<Rc<GdtfDescription>> {
        self.gdtf_descriptions.get(&rid).cloned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fixture {
    pub id: usize,
    pub label: String,
    pub gdtf_description: Rc<GdtfDescription>,
    pub channel: DmxChannel,
    pub mode: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Programmer {
    selection: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use std::env;

    use gdtf_share::GdtfShare;

    use crate::showfile::Showfile;

    use super::Show;

    #[tokio::test]
    async fn from_empty_showfile() {
        dotenv::dotenv().ok();
        let user = env::var("GDTF_SHARE_API_USER").unwrap();
        let password = env::var("GDTF_SHARE_API_PASSWORD").unwrap();

        let showfile: Showfile = serde_json::from_str(r#"{}"#).unwrap();
        let gdtf_share = GdtfShare::auth(user, password).await.unwrap();

        Show::new(showfile, gdtf_share).await;
    }

    #[tokio::test]
    async fn from_showfile_with_fixture() {
        dotenv::dotenv().ok();
        let user = env::var("GDTF_SHARE_API_USER").unwrap();
        let password = env::var("GDTF_SHARE_API_PASSWORD").unwrap();

        let showfile: Showfile = serde_json::from_str(
            r#"{
            "patch_list": {
                "fixtures": [
                    {
                        "id": 420,
                        "gdtf_share_revision_id": 60124,
                        "label": "Front Wash 1",
                        "mode": "Basic",
                        "channel": {
                            "address": 1,
                            "universe": 2
                        }
                    },
                    {
                        "id": 12,
                        "label": "Wash 2",
                        "gdtf_share_revision_id": 60124,
                        "mode": "Turned to 11",
                        "channel": {
                            "address": 5,
                            "universe": 8
                        }
                    }
                ]
            }
        }"#,
        )
        .unwrap();
        let gdtf_share = GdtfShare::auth(user, password).await.unwrap();

        let show = Show::new(showfile, gdtf_share).await;

        assert_eq!(
            show.patchlist.fixtures[0].gdtf_description.fixture_type.id,
            "DB42C9F0-3236-4251-8436-D9CBE92F4021".to_string()
        );

        assert_eq!(show.patchlist.gdtf_descriptions.len(), 1);
    }

    #[tokio::test]
    async fn test_selecting_fixture_with_command() {
        dotenv::dotenv().ok();
        let user = env::var("GDTF_SHARE_API_USER").unwrap();
        let password = env::var("GDTF_SHARE_API_PASSWORD").unwrap();

        let showfile: Showfile = serde_json::from_str(
            r#"{
            "patch_list": {
                "fixtures": [
                    {
                        "id": 420,
                        "gdtf_share_revision_id": 60124,
                        "label": "Front Wash 1",
                        "mode": "Basic",
                        "channel": {
                            "address": 1,
                            "universe": 2
                        }
                    },
                    {
                        "id": 12,
                        "label": "Wash 2",
                        "gdtf_share_revision_id": 60124,
                        "mode": "Turned to 11",
                        "channel": {
                            "address": 5,
                            "universe": 8
                        }
                    }
                ]
            }
        }"#,
        )
        .unwrap();
        let gdtf_share = GdtfShare::auth(user, password).await.unwrap();

        let mut show = Show::new(showfile, gdtf_share).await;
        show.execute_command_str("Fixture 420").unwrap();

        assert_eq!(*show.programmer.selection.first().unwrap(), 420);
    }
}

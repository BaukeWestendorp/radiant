use std::iter::Peekable;
use std::str::FromStr;

use eyre::{Context, ContextCompat};

use super::lexer::{Lexer, Token};
use crate::cmd::{
    Command, CueCommand, ExecutorButtonCommand, ExecutorCommand, ExecutorFaderCommand,
    FixtureGroupCommand, PatchCommand, PresetCommand, ProgrammerCommand, ProgrammerSetCommand,
    SequenceCommand,
};
use crate::error::Result;
use crate::object::{
    AnyObjectId, AnyPresetId, CueId, ExecutorButtonMode, ExecutorFaderMode, ExecutorId,
    FixtureGroupId, Recipe, RecipeContent, SequenceId,
};
use crate::patch::{Attribute, AttributeValue, DmxMode, FixtureId};

const ERRMSG_UNEXPECTED_EOL: &str = "unexpected End Of Line";

// Functions.
const PATCH: &str = "patch";
const PROGRAMMER: &str = "programmer";
const CREATE: &str = "create";
const RENAME: &str = "rename";
const REMOVE: &str = "remove";

// Objects.
const FIXTURE_GROUP: &str = "fixture_group";
const EXECUTOR: &str = "executor";
const SEQUENCE: &str = "sequence";
const CUE: &str = "cue";
const PRESET: &str = "preset";
const DIMMER: &str = "dimmer";
const COLOR: &str = "color";

pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { lexer: Lexer::new(source).peekable() }
    }

    pub fn parse(mut self) -> Result<Command> {
        let command = match self.parse_one_of_idents(&[PATCH, PROGRAMMER, CREATE, REMOVE, RENAME]) {
            Ok(PATCH) => {
                Command::Patch(match self.parse_one_of_idents(&["add", "set", "remove"])? {
                    "add" => PatchCommand::Add {
                        id: self.parse_fixture_id()?,
                        address: self.parse_dmx_address()?,
                        gdtf_file_name: self.parse_string()?.to_string(),
                        mode: DmxMode::new(self.parse_string()?.to_string()),
                    },
                    "set" => {
                        match self.parse_one_of_idents(&["address", "mode", "gdtf_file_name"])? {
                            "address" => PatchCommand::SetAddress {
                                id: self.parse_fixture_id()?,
                                address: self.parse_dmx_address()?,
                            },
                            "mode" => PatchCommand::SetMode {
                                id: self.parse_fixture_id()?,
                                mode: DmxMode::new(self.parse_string()?),
                            },
                            "gdtf_file_name" => PatchCommand::SetGdtfFileName {
                                id: self.parse_fixture_id()?,
                                name: self.parse_string()?.to_string(),
                            },
                            _ => unreachable!(),
                        }
                    }
                    "remove" => PatchCommand::Remove { id: self.parse_fixture_id()? },
                    _ => unreachable!(),
                })
            }
            Ok(PROGRAMMER) => {
                Command::Programmer(match self.parse_one_of_idents(&["set", "clear"])? {
                    "set" => ProgrammerCommand::Set(
                        match self.parse_one_of_idents(&["direct", "attribute"])? {
                            "direct" => ProgrammerSetCommand::Direct {
                                address: self.parse_dmx_address()?,
                                value: self.parse_dmx_value()?,
                            },
                            "attribute" => ProgrammerSetCommand::Attribute {
                                id: self.parse_fixture_id()?,
                                attribute: self.parse_attribute()?,
                                value: self.parse_attribute_value()?,
                            },
                            _ => eyre::bail!("unexpected subcommand"),
                        },
                    ),
                    "clear" => ProgrammerCommand::Clear,
                    _ => unreachable!(),
                })
            }
            Ok(CREATE) => Command::Create {
                id: self.parse_object_id()?,
                name: self.parse_string().ok().map(String::from),
            },
            Ok(REMOVE) => Command::Remove { id: self.parse_object_id()? },
            Ok(RENAME) => Command::Rename {
                id: self.parse_object_id()?,
                name: self.parse_string()?.to_string(),
            },
            _ => match self.parse_object_id()? {
                AnyObjectId::FixtureGroup(id) => Command::FixtureGroup(
                    id,
                    match self.parse_one_of_idents(&[
                        "add",
                        "replace_at",
                        "remove",
                        "remove_at",
                        "clear",
                    ])? {
                        "add" => FixtureGroupCommand::Add {
                            ids: self.parse_list(|this| this.parse_fixture_id())?,
                        },
                        "replace_at" => FixtureGroupCommand::ReplaceAt {
                            index: self.parse_index()?,
                            id: self.parse_fixture_id()?,
                        },
                        "remove" => FixtureGroupCommand::Remove { id: self.parse_fixture_id()? },
                        "remove_at" => FixtureGroupCommand::RemoveAt { index: self.parse_index()? },
                        "clear" => FixtureGroupCommand::Clear,
                        _ => unreachable!(),
                    },
                ),
                AnyObjectId::Executor(id) => Command::Executor(
                    id,
                    match self.parse_one_of_idents(&["button", "fader", "set_sequence", "clear"])? {
                        "button" => ExecutorCommand::Button(
                            match self
                                .parse_one_of_idents(&["press", "press", "release", "mode"])?
                            {
                                "press" => ExecutorButtonCommand::Press,
                                "release" => ExecutorButtonCommand::Release,
                                "mode" => ExecutorButtonCommand::SetMode {
                                    mode: ExecutorButtonMode::from_str(self.parse_ident()?)
                                        .context("invalid executor button mode")?,
                                },
                                _ => unreachable!(),
                            },
                        ),
                        "fader" => ExecutorCommand::Fader(
                            match self.parse_one_of_idents(&["mode", "level"])? {
                                "level" => ExecutorFaderCommand::SetLevel {
                                    level: self.parse_float()? as f32,
                                },
                                "mode" => ExecutorFaderCommand::SetMode {
                                    mode: ExecutorFaderMode::from_str(self.parse_ident()?)
                                        .context("invalid executor button mode")?,
                                },
                                _ => unreachable!(),
                            },
                        ),
                        "set_sequence" => ExecutorCommand::SetSequence {
                            sequence_id: self
                                .parse_object_id()?
                                .try_into()
                                .wrap_err("failed to parse sequence id")?,
                        },
                        "clear" => ExecutorCommand::Clear,
                        _ => unreachable!(),
                    },
                ),
                AnyObjectId::Sequence(id) => Command::Sequence(
                    id,
                    match self.parse_one_of_idents(&[
                        "add",
                        "replace_at",
                        "remove",
                        "remove_at",
                        "clear",
                    ])? {
                        "add" => SequenceCommand::Add {
                            cue_ids: self.parse_list(|this| {
                                this.parse_object_id()?
                                    .try_into()
                                    .wrap_err("failed to parse cue id")
                            })?,
                        },
                        "replace_at" => SequenceCommand::ReplaceAt {
                            index: self.parse_index()?,
                            cue_id: self
                                .parse_object_id()?
                                .try_into()
                                .wrap_err("failed to parse cue id")?,
                        },
                        "remove" => SequenceCommand::Remove {
                            cue_id: self
                                .parse_object_id()?
                                .try_into()
                                .wrap_err("failed to parse cue id")?,
                        },
                        "remove_at" => SequenceCommand::RemoveAt { index: self.parse_index()? },
                        "clear" => SequenceCommand::Clear,
                        _ => unreachable!(),
                    },
                ),
                AnyObjectId::Cue(id) => Command::Cue(
                    id,
                    match self.parse_one_of_idents(&["add", "replace_at", "remove_at", "clear"])? {
                        "add" => CueCommand::Add {
                            recipes: self.parse_list(|this| this.parse_recipe())?,
                        },
                        "replace_at" => CueCommand::ReplaceAt {
                            index: self.parse_index()?,
                            recipe: self.parse_recipe()?,
                        },
                        "remove_at" => CueCommand::RemoveAt { index: self.parse_index()? },
                        "clear" => CueCommand::Clear,
                        _ => unreachable!(),
                    },
                ),
                AnyObjectId::Preset(id) => Command::Preset(
                    id,
                    match self.parse_one_of_idents(&["store", "clear"])? {
                        "store" => PresetCommand::Store,
                        "clear" => PresetCommand::Clear,
                        _ => unreachable!(),
                    },
                ),
            },
        };

        eyre::ensure!(
            self.lexer.peek().is_none(),
            "expected End Of Line, found: '{}'",
            self.lexer.peek().unwrap()
        );

        Ok(command)
    }

    fn parse_string(&mut self) -> Result<&str> {
        match self.next_token()? {
            Token::String(string) => Ok(string),
            other => eyre::bail!("expected a string, found: '{other}'"),
        }
    }

    fn parse_int(&mut self) -> Result<i64> {
        match self.next_token()? {
            Token::Integer(int) => Ok(int),
            other => eyre::bail!("expected an integer, found: '{other}'"),
        }
    }

    fn parse_float(&mut self) -> Result<f64> {
        match self.next_token()? {
            Token::Float(float) => Ok(float),
            other => eyre::bail!("expected a float, found: '{other}'"),
        }
    }

    fn parse_ident(&mut self) -> Result<&str> {
        match self.next_token()? {
            Token::Ident(ident) => Ok(ident),
            other => eyre::bail!("expected an identifier, found: '{other}'"),
        }
    }

    fn parse_list<R, F: FnMut(&mut Self) -> Result<R>>(&mut self, mut f: F) -> Result<Vec<R>> {
        let mut items = Vec::new();

        loop {
            let item = f(self)?;
            items.push(item);

            if self.parse_token(&Token::Comma).is_err() {
                return Ok(items);
            }
        }
    }

    fn parse_object_id(&mut self) -> Result<AnyObjectId> {
        match self.parse_one_of_idents(&[FIXTURE_GROUP, EXECUTOR, SEQUENCE, CUE, PRESET])? {
            FIXTURE_GROUP => Ok(FixtureGroupId(self.parse_positive_int()?).into()),
            EXECUTOR => Ok(ExecutorId(self.parse_positive_int()?).into()),
            SEQUENCE => Ok(SequenceId(self.parse_positive_int()?).into()),
            CUE => Ok(CueId(self.parse_positive_int()?).into()),
            PRESET => {
                self.parse_token(&Token::Colon)?;
                self.parse_token(&Token::Colon)?;

                match self.parse_one_of_idents(&[DIMMER, COLOR])? {
                    DIMMER => {
                        let preset_id = AnyPresetId::Dimmer(self.parse_positive_int()?.into());
                        Ok(AnyObjectId::Preset(preset_id))
                    }
                    COLOR => {
                        let preset_id = AnyPresetId::Color(self.parse_positive_int()?.into());
                        Ok(AnyObjectId::Preset(preset_id))
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_recipe(&mut self) -> Result<Recipe> {
        Ok(Recipe {
            fixture_group: self
                .parse_object_id()?
                .try_into()
                .wrap_err("a recipe requires a fixture group id")?,
            content: match self.parse_object_id()? {
                AnyObjectId::Preset(preset_id) => RecipeContent::Preset(preset_id),
                other => {
                    eyre::bail!("a recipe cannot be created from '{other}'")
                }
            },
        })
    }

    fn parse_fixture_id(&mut self) -> Result<FixtureId> {
        Ok(FixtureId(self.parse_positive_int()?))
    }

    fn parse_attribute(&mut self) -> Result<Attribute> {
        let attribute = Attribute::from_str(self.parse_string()?).unwrap();
        Ok(attribute)
    }

    fn parse_attribute_value(&mut self) -> Result<AttributeValue> {
        let attribute = AttributeValue::new(self.parse_float().wrap_err("")? as f32);
        Ok(attribute)
    }

    fn parse_dmx_address(&mut self) -> Result<dmx::Address> {
        let str = self.parse_string()?;
        dmx::Address::from_str(str).wrap_err("failed to parse DMX address")
    }

    fn parse_dmx_value(&mut self) -> Result<dmx::Value> {
        let n = self.parse_positive_int()?;
        let byte: u8 =
            n.try_into().wrap_err("dmx value should be in the range of 0..=255, found: '{n}'")?;
        Ok(dmx::Value(byte))
    }

    fn parse_index(&mut self) -> Result<usize> {
        Ok(self.parse_positive_int()? as usize)
    }

    fn parse_positive_int(&mut self) -> Result<u32> {
        let n = self.parse_int()?;
        eyre::ensure!(n >= 0, "expected a positive integer, found: '{n}'");
        Ok(n as u32)
    }

    fn parse_token(&mut self, token: &Token) -> Result<()> {
        let next = self.expect_peek()?;
        eyre::ensure!(next == token, "unexpected token: '{next}'");
        self.next_token()?;
        Ok(())
    }

    fn parse_one_of_idents<'a>(&mut self, items: &[&'a str]) -> Result<&'a str> {
        let next = self.expect_peek()?;
        match items.iter().find(|item| next == &Token::Ident(item)) {
            Some(item) => {
                self.next_token()?;
                Ok(item)
            }
            None => eyre::bail!("expected one of {items:?}, found: '{next}'"),
        }
    }

    fn expect_peek(&mut self) -> Result<&Token<'_>> {
        self.lexer.peek().wrap_err(ERRMSG_UNEXPECTED_EOL)
    }

    fn next_token(&mut self) -> Result<Token<'_>> {
        self.lexer.next().wrap_err(ERRMSG_UNEXPECTED_EOL)
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd;
    use crate::cmd::parser::Parser;
    use crate::cmd::{
        Command, CueCommand, ExecutorButtonCommand, ExecutorCommand, ExecutorFaderCommand,
        FixtureGroupCommand, PatchCommand, PresetCommand, ProgrammerCommand, ProgrammerSetCommand,
        SequenceCommand,
    };
    use crate::object::{
        AnyObjectId, AnyPresetId, ColorPresetId, DimmerPresetId, ExecutorButtonMode,
        ExecutorFaderMode, ExecutorId, Recipe, RecipeContent,
    };
    use crate::patch::{Attribute, AttributeValue, DmxMode, FixtureId};

    #[test]
    fn whitespace() {
        assert_eq!(
            cmd!(r#"  create    executor        2 "  With Name" "#),
            Command::Create { id: ExecutorId(2).into(), name: Some("  With Name".to_string()) }
        );
    }

    #[test]
    fn parse_patch_add() {
        assert_eq!(
            cmd!(r#"patch add 1 "1.2" "Generic@Dimmer@Generic.gdtf" "Default""#),
            Command::Patch(PatchCommand::Add {
                id: FixtureId(1),
                address: dmx::Address::new(
                    dmx::UniverseId::new(1).unwrap(),
                    dmx::Channel::new(2).unwrap()
                ),
                gdtf_file_name: "Generic@Dimmer@Generic.gdtf".to_string(),
                mode: DmxMode::new("Default")
            })
        );
    }

    #[test]
    fn parse_patch_set_address() {
        assert_eq!(
            cmd!(r#"patch set address 1 "1.2""#),
            Command::Patch(PatchCommand::SetAddress {
                id: FixtureId(1),
                address: dmx::Address::new(
                    dmx::UniverseId::new(1).unwrap(),
                    dmx::Channel::new(2).unwrap()
                ),
            })
        );
    }

    #[test]
    fn parse_patch_set_mode() {
        assert_eq!(
            cmd!(r#"patch set mode 1 "Default""#),
            Command::Patch(PatchCommand::SetMode {
                id: FixtureId(1),
                mode: DmxMode::new("Default"),
            })
        );
    }

    #[test]
    fn parse_patch_set_gdtf_file_name() {
        assert_eq!(
            cmd!(r#"patch set gdtf_file_name 1 "Generic@Dimmer@Generic.gdtf""#),
            Command::Patch(PatchCommand::SetGdtfFileName {
                id: FixtureId(1),
                name: "Generic@Dimmer@Generic.gdtf".to_string()
            })
        );
    }

    #[test]
    fn parse_patch_remove() {
        assert_eq!(
            cmd!(r#"patch remove 1"#),
            Command::Patch(PatchCommand::Remove { id: FixtureId(1) })
        );
    }

    #[test]
    fn parse_programmer_set_direct() {
        assert_eq!(
            cmd!(r#"programmer set direct "1.2" 42"#),
            Command::Programmer(ProgrammerCommand::Set(ProgrammerSetCommand::Direct {
                address: dmx::Address::new(
                    dmx::UniverseId::new(1).unwrap(),
                    dmx::Channel::new(2).unwrap()
                ),
                value: dmx::Value(42),
            }))
        );
    }

    #[test]
    fn parse_programmer_set_attribute() {
        assert_eq!(
            cmd!(r#"programmer set attribute 1 "Dimmer" 0.25"#),
            Command::Programmer(ProgrammerCommand::Set(ProgrammerSetCommand::Attribute {
                id: FixtureId(1),
                attribute: Attribute::Dimmer,
                value: AttributeValue::new(0.25),
            }))
        );
    }

    #[test]
    fn parse_programmer_clear() {
        assert_eq!(cmd!(r#"programmer clear"#), Command::Programmer(ProgrammerCommand::Clear));
    }

    #[test]
    fn parse_object_id() {
        assert_eq!(
            Parser::new("fixture_group 0").parse_object_id().unwrap(),
            AnyObjectId::FixtureGroup(0.into())
        );

        assert_eq!(
            Parser::new("executor 0").parse_object_id().unwrap(),
            AnyObjectId::Executor(0.into())
        );

        assert_eq!(
            Parser::new("sequence 0").parse_object_id().unwrap(),
            AnyObjectId::Sequence(0.into())
        );

        assert_eq!(Parser::new("cue 0").parse_object_id().unwrap(), AnyObjectId::Cue(0.into()));

        assert_eq!(
            Parser::new("preset::dimmer 0").parse_object_id().unwrap(),
            AnyObjectId::Preset(DimmerPresetId(0).into())
        );

        assert_eq!(
            Parser::new("preset::color 0").parse_object_id().unwrap(),
            AnyObjectId::Preset(ColorPresetId(0).into())
        );

        assert_eq!(
            Parser::new("executor 1").parse_object_id().unwrap(),
            AnyObjectId::Executor(1.into())
        );

        assert!(Parser::new("executor -1").parse_object_id().is_err());

        assert!(Parser::new("invalid_object 0").parse_object_id().is_err());
    }

    #[test]
    fn create_with_name() {
        assert_eq!(
            cmd!(r#"create executor 0 "Example Executor""#),
            Command::Create {
                id: AnyObjectId::Executor(0.into()),
                name: Some("Example Executor".to_string()),
            },
        );
    }

    #[test]
    fn create_without_name() {
        assert_eq!(
            cmd!(r#"create executor 0"#),
            Command::Create { id: AnyObjectId::Executor(0.into()), name: None },
        );
    }

    #[test]
    fn parse_remove_command() {
        assert_eq!(cmd!(r#"remove cue 1"#), Command::Remove { id: AnyObjectId::Cue(1.into()) });
    }

    #[test]
    fn parse_rename_command() {
        assert_eq!(
            cmd!(r#"rename cue 1 "New Name""#),
            Command::Rename { id: AnyObjectId::Cue(1.into()), name: "New Name".to_string() }
        );
    }

    #[test]
    fn parse_fixture_group_add_single() {
        assert_eq!(
            cmd!(r#"fixture_group 1 add 2"#),
            Command::FixtureGroup(1.into(), FixtureGroupCommand::Add { ids: vec![FixtureId(2)] })
        );
    }

    #[test]
    fn parse_fixture_group_add_multiple() {
        assert_eq!(
            cmd!(r#"fixture_group 1 add 2, 3, 4"#),
            Command::FixtureGroup(
                1.into(),
                FixtureGroupCommand::Add { ids: vec![FixtureId(2), FixtureId(3), FixtureId(4)] }
            )
        );
    }

    #[test]
    fn parse_fixture_group_replace_at() {
        assert_eq!(
            cmd!(r#"fixture_group 1 replace_at 1 2"#),
            Command::FixtureGroup(
                1.into(),
                FixtureGroupCommand::ReplaceAt { index: 1, id: FixtureId(2) }
            )
        );
    }

    #[test]
    fn parse_fixture_group_remove() {
        assert_eq!(
            cmd!(r#"fixture_group 1 remove 2"#),
            Command::FixtureGroup(1.into(), FixtureGroupCommand::Remove { id: FixtureId(2) })
        );
    }

    #[test]
    fn parse_fixture_group_remove_at() {
        assert_eq!(
            cmd!(r#"fixture_group 1 remove_at 1"#),
            Command::FixtureGroup(1.into(), FixtureGroupCommand::RemoveAt { index: 1 })
        );
    }

    #[test]
    fn parse_fixture_group_clear() {
        assert_eq!(
            cmd!(r#"fixture_group 1 clear"#),
            Command::FixtureGroup(1.into(), FixtureGroupCommand::Clear)
        );
    }

    #[test]
    fn parse_executor_button_mode() {
        assert_eq!(
            cmd!(r#"executor 1 button mode go"#),
            Command::Executor(
                1.into(),
                ExecutorCommand::Button(ExecutorButtonCommand::SetMode {
                    mode: ExecutorButtonMode::Go
                })
            )
        );
    }

    #[test]
    fn parse_executor_button_press() {
        assert_eq!(
            cmd!(r#"executor 1 button press"#),
            Command::Executor(1.into(), ExecutorCommand::Button(ExecutorButtonCommand::Press))
        );
    }

    #[test]
    fn parse_executor_button_release() {
        assert_eq!(
            cmd!(r#"executor 1 button release"#),
            Command::Executor(1.into(), ExecutorCommand::Button(ExecutorButtonCommand::Release))
        );
    }

    #[test]
    fn parse_executor_set_fader_mode() {
        assert_eq!(
            cmd!(r#"executor 1 fader mode master"#),
            Command::Executor(
                1.into(),
                ExecutorCommand::Fader(ExecutorFaderCommand::SetMode {
                    mode: ExecutorFaderMode::Master
                })
            )
        );
    }

    #[test]
    fn parse_executor_set_fader_level() {
        assert_eq!(
            cmd!(r#"executor 1 fader level 0.5"#),
            Command::Executor(
                1.into(),
                ExecutorCommand::Fader(ExecutorFaderCommand::SetLevel { level: 0.5 })
            )
        );
    }

    #[test]
    fn parse_executor_set_sequence_id() {
        assert_eq!(
            cmd!(r#"executor 1 set_sequence sequence 2"#),
            Command::Executor(1.into(), ExecutorCommand::SetSequence { sequence_id: 2.into() })
        );
    }

    #[test]
    fn parse_executor_clear() {
        assert_eq!(
            cmd!(r#"executor 1 clear"#),
            Command::Executor(1.into(), ExecutorCommand::Clear)
        );
    }

    #[test]
    fn parse_sequence_add_single() {
        assert_eq!(
            cmd!(r#"sequence 1 add cue 2"#),
            Command::Sequence(1.into(), SequenceCommand::Add { cue_ids: vec![2.into()] })
        );
    }

    #[test]
    fn parse_sequence_add_multiple() {
        assert_eq!(
            cmd!(r#"sequence 1 add cue 2, cue 3, cue 4"#),
            Command::Sequence(
                1.into(),
                SequenceCommand::Add { cue_ids: vec![2.into(), 3.into(), 4.into()] }
            )
        );
    }

    #[test]
    fn parse_sequence_replace_at() {
        assert_eq!(
            cmd!(r#"sequence 1 replace_at 1 cue 2"#),
            Command::Sequence(1.into(), SequenceCommand::ReplaceAt { index: 1, cue_id: 2.into() })
        );
    }

    #[test]
    fn parse_sequence_remove() {
        assert_eq!(
            cmd!(r#"sequence 1 remove cue 2"#),
            Command::Sequence(1.into(), SequenceCommand::Remove { cue_id: 2.into() })
        );
    }

    #[test]
    fn parse_sequence_remove_at() {
        assert_eq!(
            cmd!(r#"sequence 1 remove_at 2"#),
            Command::Sequence(1.into(), SequenceCommand::RemoveAt { index: 2 })
        );
    }

    #[test]
    fn parse_sequence_clear() {
        assert_eq!(
            cmd!(r#"sequence 1 clear"#),
            Command::Sequence(1.into(), SequenceCommand::Clear)
        );
    }

    #[test]
    fn parse_cue_add_single() {
        assert_eq!(
            cmd!(r#"cue 1 add fixture_group 2 preset::dimmer 3"#),
            Command::Cue(
                1.into(),
                CueCommand::Add {
                    recipes: vec![Recipe {
                        fixture_group: 2.into(),
                        content: RecipeContent::Preset(AnyPresetId::Dimmer(3.into()))
                    }]
                }
            )
        );
    }

    #[test]
    fn parse_cue_add_multiple() {
        assert_eq!(
            cmd!(r#"cue 1 add fixture_group 2 preset::dimmer 3, fixture_group 4 preset::dimmer 5"#),
            Command::Cue(
                1.into(),
                CueCommand::Add {
                    recipes: vec![
                        Recipe {
                            fixture_group: 2.into(),
                            content: RecipeContent::Preset(AnyPresetId::Dimmer(3.into()))
                        },
                        Recipe {
                            fixture_group: 4.into(),
                            content: RecipeContent::Preset(AnyPresetId::Dimmer(5.into()))
                        }
                    ]
                }
            )
        );
    }

    #[test]
    fn parse_cue_replace_at() {
        assert_eq!(
            cmd!(r#"cue 1 replace_at 2 fixture_group 2 preset::dimmer 3"#),
            Command::Cue(
                1.into(),
                CueCommand::ReplaceAt {
                    index: 2,
                    recipe: Recipe {
                        fixture_group: 2.into(),
                        content: RecipeContent::Preset(AnyPresetId::Dimmer(3.into()))
                    }
                }
            )
        );
    }

    #[test]
    fn parse_cue_remove_at() {
        assert_eq!(
            cmd!(r#"cue 1 remove_at 2"#),
            Command::Cue(1.into(), CueCommand::RemoveAt { index: 2 })
        );
    }

    #[test]
    fn parse_cue_clear() {
        assert_eq!(cmd!(r#"cue 1 clear"#), Command::Cue(1.into(), CueCommand::Clear));
    }

    #[test]
    fn parse_preset_store_dimmer() {
        assert_eq!(
            cmd!(r#"preset::dimmer 1 store"#),
            Command::Preset(AnyPresetId::Dimmer(1.into()), PresetCommand::Store)
        );
    }

    #[test]
    fn parse_preset_store_color() {
        assert_eq!(
            cmd!(r#"preset::color 1 store"#),
            Command::Preset(AnyPresetId::Color(1.into()), PresetCommand::Store)
        );
    }

    #[test]
    fn parse_preset_clear() {
        assert_eq!(
            cmd!(r#"preset::dimmer 1 clear"#),
            Command::Preset(AnyPresetId::Dimmer(1.into()), PresetCommand::Clear)
        );
    }
}

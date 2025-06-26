use std::iter::Peekable;
use std::str::FromStr;

use eyre::{Context, ContextCompat};

use super::lexer::{Lexer, Token};
use crate::backend::{
    ActivationMode, AnyObjectId, AnyPresetId, Attribute, AttributeValue, Command, CueCommand,
    CueId, DmxMode, ExecutorCommand, ExecutorId, FixtureGroupCommand, FixtureGroupId, FixtureId,
    PatchCommand, PresetCommand, ProgrammerCommand, ProgrammerSetCommand, Recipe, RecipeContent,
    SequenceCommand, SequenceId, TerminationMode,
};
use crate::dmx;

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

pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { lexer: Lexer::new(source).peekable() }
    }

    pub fn parse(mut self) -> eyre::Result<Command> {
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
                        "add" => FixtureGroupCommand::Add { id: self.parse_fixture_id()? },
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
                    match self.parse_one_of_idents(&["set", "clear"])? {
                        "set" => match self.parse_one_of_idents(&[
                            "activation_mode",
                            "termination_mode",
                            "sequence",
                        ])? {
                            "activation_mode" => ExecutorCommand::SetActivationMode {
                                mode: ActivationMode::from_str(self.parse_string()?)?,
                            },
                            "termination_mode" => ExecutorCommand::SetTerminationMode {
                                mode: TerminationMode::from_str(self.parse_string()?)?,
                            },
                            "sequence" => ExecutorCommand::SetSequence {
                                sequence_id: self
                                    .parse_object_id()?
                                    .try_into()
                                    .wrap_err("failed to parse sequence id")?,
                            },
                            _ => unreachable!(),
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
                            cue_id: self
                                .parse_object_id()?
                                .try_into()
                                .wrap_err("failed to parse cue id")?,
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
                        "add" => CueCommand::Add { recipe: self.parse_recipe()? },
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

    fn parse_string(&mut self) -> eyre::Result<&str> {
        match self.next_token()? {
            Token::String(string) => Ok(string),
            other => eyre::bail!("expected a string, found: '{other}'"),
        }
    }

    fn parse_int(&mut self) -> eyre::Result<i64> {
        match self.next_token()? {
            Token::Integer(int) => Ok(int),
            other => eyre::bail!("expected an integer, found: '{other}'"),
        }
    }

    fn parse_float(&mut self) -> eyre::Result<f64> {
        match self.next_token()? {
            Token::Float(float) => Ok(float),
            other => eyre::bail!("expected a float, found: '{other}'"),
        }
    }

    fn parse_object_id(&mut self) -> eyre::Result<AnyObjectId> {
        match self.parse_one_of_idents(&[FIXTURE_GROUP, EXECUTOR, SEQUENCE, CUE, PRESET])? {
            FIXTURE_GROUP => Ok(FixtureGroupId(self.parse_positive_int()?).into()),
            EXECUTOR => Ok(ExecutorId(self.parse_positive_int()?).into()),
            SEQUENCE => Ok(SequenceId(self.parse_positive_int()?).into()),
            CUE => Ok(CueId(self.parse_positive_int()?).into()),
            PRESET => {
                self.parse_token(&Token::Colon)?;
                self.parse_token(&Token::Colon)?;

                match self.parse_one_of_idents(&[DIMMER])? {
                    DIMMER => {
                        let preset_id = AnyPresetId::Dimmer(self.parse_positive_int()?.into());
                        Ok(AnyObjectId::Preset(preset_id))
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_recipe(&mut self) -> eyre::Result<Recipe> {
        Ok(Recipe {
            fixture_group_id: self
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

    fn parse_fixture_id(&mut self) -> eyre::Result<FixtureId> {
        Ok(FixtureId(self.parse_positive_int()?))
    }

    fn parse_attribute(&mut self) -> eyre::Result<Attribute> {
        let attribute = Attribute::from_str(self.parse_string()?).unwrap();
        Ok(attribute)
    }

    fn parse_attribute_value(&mut self) -> eyre::Result<AttributeValue> {
        let attribute = AttributeValue::new(self.parse_float().wrap_err("")? as f32);
        Ok(attribute)
    }

    fn parse_dmx_address(&mut self) -> eyre::Result<dmx::Address> {
        let str = self.parse_string()?;
        dmx::Address::from_str(str).wrap_err("failed to parse DMX address")
    }

    fn parse_dmx_value(&mut self) -> eyre::Result<dmx::Value> {
        let n = self.parse_positive_int()?;
        let byte: u8 =
            n.try_into().wrap_err("dmx value should be in the range of 0..=255, found: '{n}'")?;
        Ok(dmx::Value(byte))
    }

    fn parse_index(&mut self) -> eyre::Result<usize> {
        Ok(self.parse_positive_int()? as usize)
    }

    fn parse_positive_int(&mut self) -> eyre::Result<u32> {
        let n = self.parse_int()?;
        eyre::ensure!(n >= 0, "expected a positive integer, found: '{n}'");
        Ok(n as u32)
    }

    fn parse_token(&mut self, token: &Token) -> eyre::Result<()> {
        let next = self.expect_peek()?;
        eyre::ensure!(next == token, "unexpected token: '{next}'");
        self.next_token()?;
        Ok(())
    }

    fn parse_one_of_idents<'a>(&mut self, items: &[&'a str]) -> eyre::Result<&'a str> {
        let next = self.expect_peek()?;
        match items.iter().find(|item| next == &Token::Ident(item)) {
            Some(item) => {
                self.next_token()?;
                Ok(item)
            }
            None => eyre::bail!("expected one of {items:?}, found: '{next}'"),
        }
    }

    fn expect_peek(&mut self) -> eyre::Result<&Token<'_>> {
        self.lexer.peek().wrap_err(ERRMSG_UNEXPECTED_EOL)
    }

    fn next_token(&mut self) -> eyre::Result<Token<'_>> {
        self.lexer.next().wrap_err(ERRMSG_UNEXPECTED_EOL)
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::engine::cmd::{Command, parser::Parser};
    use crate::backend::engine::cmd::{
        CueCommand, ExecutorCommand, FixtureGroupCommand, PatchCommand, PresetCommand,
        ProgrammerCommand, ProgrammerSetCommand, SequenceCommand,
    };
    use crate::backend::object::{
        ActivationMode, AnyObjectId, AnyPresetId, DimmerPresetId, ExecutorId, Recipe,
        RecipeContent, TerminationMode,
    };
    use crate::backend::patch::attr::{Attribute, AttributeValue};
    use crate::backend::patch::fixture::{DmxMode, FixtureId};
    use crate::{cmd, dmx};

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
    fn parse_fixture_group_add() {
        assert_eq!(
            cmd!(r#"fixture_group 1 add 2"#),
            Command::FixtureGroup(1.into(), FixtureGroupCommand::Add { id: FixtureId(2) })
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
    fn parse_executor_set_activation_mode() {
        assert_eq!(
            cmd!(r#"executor 1 set activation_mode "instant""#),
            Command::Executor(
                1.into(),
                ExecutorCommand::SetActivationMode { mode: ActivationMode::Instant }
            )
        );
    }

    #[test]
    fn parse_executor_set_termination_mode() {
        assert_eq!(
            cmd!(r#"executor 1 set termination_mode "never""#),
            Command::Executor(
                1.into(),
                ExecutorCommand::SetTerminationMode { mode: TerminationMode::Never }
            )
        );
    }

    #[test]
    fn parse_executor_set_sequence_id() {
        assert_eq!(
            cmd!(r#"executor 1 set sequence sequence 2"#),
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
    fn parse_sequence_add() {
        assert_eq!(
            cmd!(r#"sequence 1 add cue 2"#),
            Command::Sequence(1.into(), SequenceCommand::Add { cue_id: 2.into() })
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
    fn parse_cue_add() {
        assert_eq!(
            cmd!(r#"cue 1 add fixture_group 2 preset::dimmer 3"#),
            Command::Cue(
                1.into(),
                CueCommand::Add {
                    recipe: Recipe {
                        fixture_group_id: 2.into(),
                        content: RecipeContent::Preset(AnyPresetId::Dimmer(3.into()))
                    }
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
                        fixture_group_id: 2.into(),
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
    fn parse_preset_store() {
        assert_eq!(
            cmd!(r#"preset::dimmer 1 store"#),
            Command::Preset(AnyPresetId::Dimmer(1.into()), PresetCommand::Store)
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

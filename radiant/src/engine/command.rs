use eyre::{Context, ContextCompat};

use crate::error::Result;
use crate::show::{AnyPoolId, FixtureId, ObjectId};

#[derive(Debug, Clone)]
pub enum Command {
    Select { selection: Selection },
    ClearSelection,

    Store { destination: AnyPoolId },
    Update { object: AnyPoolId },
    Remove { object: AnyPoolId },
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Select,
    ClearSelection,

    Store,
    Update,
    Remove,
}

#[derive(Debug, Clone)]
#[derive(derive_more::TryInto)]
pub enum Parameter {
    Selection(Selection),
    Object(AnyPoolId),
}

#[derive(Debug, Clone)]
pub enum Selection {
    FixtureId(FixtureId),
    Group(ObjectId),
    All,
    None,
}

#[derive(Debug, Clone, Default)]
pub struct CommandBuilder {
    keyword: Option<Keyword>,
    parameters: Vec<Parameter>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_input(&mut self, input: impl Into<CommandInput>) -> Result<()> {
        match input.into() {
            CommandInput::Keyword(keyword) => {
                if self.keyword.is_none() {
                    self.keyword = Some(keyword)
                } else {
                    eyre::bail!(
                        "can't process another command keyword, as a command is being built already"
                    )
                }
            }
            CommandInput::ParameterValue(parameter) => self.parameters.push(parameter),
        }

        Ok(())
    }

    pub fn resolve(&mut self) -> Result<Option<Command>> {
        let Some(keyword) = &self.keyword else { return Ok(None) };
        let mut params = self.parameters.clone().into_iter();

        let command = match keyword {
            Keyword::Select => {
                let selection = params
                    .next()
                    .wrap_err("missing selection")?
                    .try_into()
                    .wrap_err("expected selection")?;

                Command::Select { selection }
            }
            Keyword::ClearSelection => Command::ClearSelection,
            Keyword::Store => {
                let destination = params
                    .next()
                    .wrap_err("missing destination")?
                    .try_into()
                    .wrap_err("expected destination")?;

                Command::Store { destination }
            }
            Keyword::Update => {
                let object = params
                    .next()
                    .wrap_err("missing object")?
                    .try_into()
                    .wrap_err("expected object")?;

                Command::Update { object }
            }
            Keyword::Remove => {
                let object = params
                    .next()
                    .wrap_err("missing object")?
                    .try_into()
                    .wrap_err("expected object")?;

                Command::Remove { object }
            }
        };

        *self = Self::default();

        Ok(Some(command))
    }
}

#[derive(Debug, Clone)]
#[derive(derive_more::From)]
pub enum CommandInput {
    Keyword(Keyword),
    ParameterValue(Parameter),
}

use std::num::NonZeroU32;

use eyre::{Context, ContextCompat};

use crate::error::Result;
use crate::show::{
    Attribute, AttributeValue, Executor, FixtureId, Group, Object, ObjectId, ObjectKind, PoolId,
    PresetBeam, PresetColor, PresetControl, PresetDimmer, PresetFocus, PresetGobo, PresetPosition,
    PresetShapers, PresetVideo, Sequence, Show,
};

#[derive(Debug, Clone)]
pub enum Command {
    Select { selection: Selection },
    Clear,

    Store { destination: ObjectReference },
    Update { object: ObjectReference },
    Delete { object: ObjectReference },
    Rename { object: ObjectReference, name: String },

    Go { executor: ObjectReference },

    SetAttribute { fid: FixtureId, attribute: Attribute, value: AttributeValue },

    Save,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Select { selection } => write!(f, "select {selection}"),
            Command::Clear => write!(f, "clear"),

            Command::Store { destination } => write!(f, "store {destination}"),
            Command::Update { object } => write!(f, "update {object}"),
            Command::Delete { object } => write!(f, "delete {object}"),
            Command::Rename { object, name } => write!(f, "label {object} \"{name}\""),

            Command::Go { executor } => write!(f, "go {executor}"),

            Command::SetAttribute { fid: _, attribute: _, value: _ } => todo!(),

            Command::Save => write!(f, "save"),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(derive_more::Display)]
pub enum Keyword {
    #[display("select")]
    Select,
    #[display("clear")]
    Clear,

    #[display("store")]
    Store,
    #[display("update")]
    Update,
    #[display("delete")]
    Delete,
    #[display("rename")]
    Rename,

    #[display("go")]
    Go,

    #[display("set")]
    Set,
    #[display("at")]
    At,

    #[display("attribute")]
    Attribute,

    #[display("save")]
    Save,
}

#[derive(Debug, Clone)]
#[derive(derive_more::Display, derive_more::From, derive_more::TryInto)]
pub enum Parameter {
    #[try_into]
    Keyword(Keyword),
    #[try_into]
    Selection(Selection),
    #[try_into]
    ObjectKind(ObjectKind),
    #[try_into]
    Integer(i32),
    #[try_into]
    String(String),
    PoolId(PoolId),
}

impl TryInto<PoolId> for Parameter {
    type Error = crate::error::Error;

    fn try_into(self) -> std::result::Result<PoolId, Self::Error> {
        match self {
            Parameter::Integer(i) if i > 0 => Ok(PoolId::new(NonZeroU32::new(i as u32).unwrap())),
            Parameter::PoolId(pool_id) => Ok(pool_id),
            _ => Err(eyre::eyre!("Parameter cannot be converted to PoolId")),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(derive_more::Display)]
pub enum Selection {
    FixtureId(FixtureId),
    Object(ObjectReference),
    #[display("all")]
    All,
    #[display("none")]
    None,
}

#[derive(Debug, Clone)]
pub struct ObjectReference {
    pub kind: ObjectKind,
    pub pool_id: PoolId,
}

impl ObjectReference {
    pub fn object_id(&self, show: &Show) -> Option<ObjectId> {
        let o = show.objects();
        let id = match self.kind {
            ObjectKind::Group => o.get_by_pool_id::<Group>(self.pool_id)?.id(),
            ObjectKind::Executor => o.get_by_pool_id::<Executor>(self.pool_id)?.id(),
            ObjectKind::Sequence => o.get_by_pool_id::<Sequence>(self.pool_id)?.id(),
            ObjectKind::PresetDimmer => o.get_by_pool_id::<PresetDimmer>(self.pool_id)?.id(),
            ObjectKind::PresetPosition => o.get_by_pool_id::<PresetPosition>(self.pool_id)?.id(),
            ObjectKind::PresetGobo => o.get_by_pool_id::<PresetGobo>(self.pool_id)?.id(),
            ObjectKind::PresetColor => o.get_by_pool_id::<PresetColor>(self.pool_id)?.id(),
            ObjectKind::PresetBeam => o.get_by_pool_id::<PresetBeam>(self.pool_id)?.id(),
            ObjectKind::PresetFocus => o.get_by_pool_id::<PresetFocus>(self.pool_id)?.id(),
            ObjectKind::PresetControl => o.get_by_pool_id::<PresetControl>(self.pool_id)?.id(),
            ObjectKind::PresetShapers => o.get_by_pool_id::<PresetShapers>(self.pool_id)?.id(),
            ObjectKind::PresetVideo => o.get_by_pool_id::<PresetVideo>(self.pool_id)?.id(),
        };
        Some(id)
    }
}

impl std::fmt::Display for ObjectReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind, self.pool_id)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommandBuilder {
    parameters: Vec<Parameter>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_param(&mut self, parameter: impl Into<Parameter>) -> Result<()> {
        self.parameters.push(parameter.into());
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        let len = self.parameters.len();
        let Some(first) = self.parameters.first() else { return false };
        match (len, first) {
            (1, Parameter::Keyword(Keyword::Clear)) => true,
            (1, Parameter::Keyword(Keyword::Save)) => true,
            _ => false,
        }
    }

    pub fn first_keyword(&self) -> Option<&Keyword> {
        match self.parameters.first() {
            Some(Parameter::Keyword(kw)) => Some(kw),
            _ => None,
        }
    }

    pub fn resolve(&mut self) -> Result<Option<Command>> {
        let mut params = self.parameters.clone().into_iter();

        let Some(first) = params.next() else {
            return Ok(None);
        };

        let parse_obj_ref =
            |params: &mut std::vec::IntoIter<Parameter>| -> Result<ObjectReference> {
                let kind = params
                    .next()
                    .wrap_err("missing object kind")?
                    .try_into()
                    .wrap_err("expected object kind")?;
                let pool_id = params
                    .next()
                    .wrap_err("missing pool id")?
                    .try_into()
                    .wrap_err("expected pool id")?;
                Ok(ObjectReference { kind, pool_id })
            };

        let parse_selection = |params: &mut std::vec::IntoIter<Parameter>| -> Result<Selection> {
            let selection = params
                .next()
                .wrap_err("missing selection")?
                .try_into()
                .wrap_err("expected selection")?;
            Ok(selection)
        };

        let parse_string = |params: &mut std::vec::IntoIter<Parameter>| -> Result<String> {
            let string =
                params.next().wrap_err("missing string")?.try_into().wrap_err("expected string")?;
            Ok(string)
        };

        let command = match first {
            Parameter::Keyword(Keyword::Select) => {
                Command::Select { selection: parse_selection(&mut params)? }
            }
            Parameter::Keyword(Keyword::Clear) => Command::Clear,
            Parameter::Keyword(Keyword::Store) => {
                Command::Store { destination: parse_obj_ref(&mut params)? }
            }
            Parameter::Keyword(Keyword::Update) => {
                Command::Update { object: parse_obj_ref(&mut params)? }
            }
            Parameter::Keyword(Keyword::Delete) => {
                Command::Delete { object: parse_obj_ref(&mut params)? }
            }
            Parameter::Keyword(Keyword::Rename) => Command::Rename {
                object: parse_obj_ref(&mut params)?,
                name: parse_string(&mut params)?,
            },
            Parameter::Keyword(Keyword::Go) => {
                Command::Go { executor: parse_obj_ref(&mut params)? }
            }
            Parameter::Keyword(Keyword::Save) => Command::Save,
            _ => eyre::bail!("unexpected start of command: {first}"),
        };

        Ok(Some(command))
    }

    pub fn clear(&mut self) {
        self.parameters = Vec::new();
    }
}

impl std::fmt::Display for CommandBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self.parameters.iter().map(ToString::to_string).collect::<Vec<_>>().join(" ");
        write!(f, "{}", params)
    }
}

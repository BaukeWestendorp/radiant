use std::{fmt, str};

use eyre::ContextCompat;

use crate::attr::{Attribute, AttributeValue};
use crate::builtin::{
    Fixture, FixtureId, FixtureReference, GdtfFixtureTypeId, Object, ObjectId, ObjectKind,
    ObjectType, PoolId,
};
use crate::engine::Engine;
use crate::engine::event::EngineEvent;
use crate::error::Result;

#[derive(Clone)]
pub enum Command {
    Patch(PatchCommand),

    Select { fid: FixtureId },
    Clear { mode: Option<ClearMode> },

    ProgrammerSetValue { fid: FixtureId, attribute: Attribute, value: AttributeValue },

    Create { r#type: ObjectType, pool_id: PoolId, name: Option<String> },
    Remove { object_ref: ObjectReference },
}

#[derive(Clone)]
pub enum PatchCommand {
    Edit,
    Save,
    Discard,
    AddFixture {
        fid: Option<FixtureId>,
        fixture_type_id: GdtfFixtureTypeId,
        address: Option<dmx::Address>,
        dmx_mode: String,
        name: Option<String>,
    },
    RemoveFixture {
        fixture_ref: FixtureReference,
    },
    SetName {
        fixture_ref: FixtureReference,
        name: String,
    },
    SetFixtureId {
        fixture_ref: FixtureReference,
        new_fid: Option<FixtureId>,
    },
    SetAddress {
        fixture_ref: FixtureReference,
        address: Option<dmx::Address>,
    },
    SetFixtureTypeId {
        fixture_ref: FixtureReference,
        fixture_type_id: GdtfFixtureTypeId,
        dmx_mode: String,
    },
}

impl PatchCommand {
    fn exec(self, engine: &mut Engine) -> Result<()> {
        engine.emit(EngineEvent::PatchChanged);

        match self {
            PatchCommand::Edit => {
                engine.patch().update(|patch| patch.start_edit());
            }
            PatchCommand::Save => {
                engine.patch().update(|patch| patch.save_edit())?;
            }
            PatchCommand::Discard => {
                engine.patch().update(|patch| patch.discard_edit());
            }
            PatchCommand::AddFixture { fid, fixture_type_id, address, dmx_mode, name } => {
                engine.patch().update(|patch| {
                    patch.add_fixture(Fixture::new(
                        fid,
                        fixture_type_id,
                        address,
                        dmx_mode,
                        name.unwrap_or("New Fixture".to_string()),
                    ))
                })?;
            }
            PatchCommand::RemoveFixture { fixture_ref } => {
                engine.patch().update(|patch| patch.remove_fixture(fixture_ref))?;
            }
            PatchCommand::SetName { fixture_ref, name } => {
                engine.patch().update(|patch| {
                    if let Some(fixture) = patch.fixture_mut(fixture_ref)? {
                        fixture.name = name;
                    } else {
                        eyre::bail!("fixture with reference {fixture_ref} not found");
                    }

                    Ok(())
                })?;
            }
            PatchCommand::SetFixtureId { fixture_ref, new_fid } => {
                engine.patch().update(|patch| {
                    if let Some(new_fid) = new_fid {
                        if let Some(conflicting_fixture) = patch.fixture_mut(new_fid)? {
                            conflicting_fixture.fid = None;
                        }
                    }

                    if let Some(fixture) = patch.fixture_mut(fixture_ref)? {
                        fixture.fid = new_fid;
                    } else {
                        eyre::bail!("fixture with reference {fixture_ref} not found");
                    }

                    Ok(())
                })?;
            }
            PatchCommand::SetAddress { fixture_ref, address } => {
                engine.patch().update(|patch| {
                    if let Some(address) = address {
                        if let Some(conflicting_fixture) = patch.fixture_mut(address)? {
                            conflicting_fixture.address = None;
                        }
                    }

                    if let Some(fixture) = patch.fixture_mut(fixture_ref)? {
                        fixture.address = address;
                    } else {
                        eyre::bail!("fixture with reference {fixture_ref} not found");
                    }

                    Ok(())
                })?;
            }
            PatchCommand::SetFixtureTypeId { fixture_ref, fixture_type_id, dmx_mode } => {
                engine.patch().update(|patch| {
                    if let Some(fixture) = patch.fixture_mut(fixture_ref)? {
                        fixture.fixture_type_id = fixture_type_id;
                        fixture.dmx_mode = dmx_mode;
                    } else {
                        eyre::bail!("fixture with reference {fixture_ref} not found");
                    }

                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}

impl Command {
    pub(crate) fn exec(self, engine: &mut Engine) -> Result<()> {
        match self {
            Command::Patch(cmd) => {
                cmd.exec(engine)?;
            }

            Command::Select { fid } => {
                engine.programmer().update(|p| p.select(fid));
            }
            Command::Clear { mode } => {
                let mode = mode.unwrap_or_default();

                match mode {
                    ClearMode::ProgrammerSelection => {
                        engine.programmer().update(|p| p.clear_selection());
                    }
                    ClearMode::ProgrammerValues => {
                        engine.programmer().update(|p| p.clear_values());
                    }
                    ClearMode::Progressive => {
                        if engine.programmer().read(|p| p.has_selection()) {
                            engine.programmer().update(|p| p.clear_selection());
                        } else if engine.programmer().read(|p| p.has_values()) {
                            engine.programmer().update(|p| p.clear_values());
                        }
                    }
                    ClearMode::All => {
                        engine.programmer().update(|p| {
                            p.clear_selection();
                            p.clear_values();
                        });
                    }
                }
            }

            Command::ProgrammerSetValue { fid, attribute, value } => {
                engine.programmer().update(|p| p.set_value(fid, attribute, value));
            }

            Command::Create { r#type, pool_id, name } => {
                let kind = ObjectKind::default_for_type(r#type);

                let mut object = Object::new(kind);

                if let Some(name) = name {
                    object.name = name;
                }

                let object_id = object.id();
                engine.objects().update(|objects| objects.insert(object));

                engine.pools().update(|pools| pools.pool_mut(r#type).insert(pool_id, object_id));
            }
            Command::Remove { object_ref } => {
                let object_id = object_ref.object_id(engine)?;

                engine.objects().update(|objects| objects.remove(object_id));
            }
        }

        Ok(())
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[inline]
        fn push_keyword(parts: &mut Vec<String>, keyword: &str) {
            parts.push(keyword.to_string());
        }

        #[inline]
        fn push_argument(parts: &mut Vec<String>, value: impl ToString) {
            parts.push(value.to_string());
        }

        #[inline]
        fn push_optional_argument(
            parts: &mut Vec<String>,
            name: &str,
            value: &Option<impl ToString>,
        ) {
            if let Some(value) = value.as_ref().map(|value| value.to_string()) {
                parts.push(format!("{name}={value}"));
            }
        }

        let mut parts = Vec::new();

        match self {
            Command::Patch(PatchCommand::Edit) => push_keyword(&mut parts, "patch_edit"),
            Command::Patch(PatchCommand::Save) => push_keyword(&mut parts, "patch_save"),
            Command::Patch(PatchCommand::Discard) => push_keyword(&mut parts, "patch_discard"),
            Command::Patch(PatchCommand::AddFixture {
                fid,
                fixture_type_id,
                address,
                dmx_mode,
                name,
            }) => {
                push_keyword(&mut parts, "patch_add_fixture");
                push_argument(
                    &mut parts,
                    fid.map(|fid| fid.to_string()).unwrap_or("none".to_string()),
                );
                push_argument(&mut parts, fixture_type_id);
                push_argument(
                    &mut parts,
                    address.map(|address| address.to_string()).unwrap_or("none".to_string()),
                );
                push_argument(&mut parts, dmx_mode);
                push_optional_argument(&mut parts, "name", name);
            }
            Command::Patch(PatchCommand::RemoveFixture { fixture_ref }) => {
                push_keyword(&mut parts, "patch_remove_fixture");
                push_argument(&mut parts, fixture_ref);
            }
            Command::Patch(PatchCommand::SetName { fixture_ref, name }) => {
                push_keyword(&mut parts, "patch_set_name");
                push_argument(&mut parts, fixture_ref);
                push_argument(&mut parts, name);
            }
            Command::Patch(PatchCommand::SetFixtureId { fixture_ref, new_fid }) => {
                push_keyword(&mut parts, "patch_set_fixture_id");
                push_argument(&mut parts, fixture_ref);
                push_argument(
                    &mut parts,
                    new_fid.map(|fid| fid.to_string()).unwrap_or("none".to_string()),
                );
            }
            Command::Patch(PatchCommand::SetAddress { fixture_ref, address }) => {
                push_keyword(&mut parts, "patch_set_address");
                push_argument(&mut parts, fixture_ref);
                push_argument(
                    &mut parts,
                    address.map(|address| address.to_string()).unwrap_or("none".to_string()),
                );
            }
            Command::Patch(PatchCommand::SetFixtureTypeId {
                fixture_ref,
                fixture_type_id,
                dmx_mode,
            }) => {
                push_keyword(&mut parts, "patch_set_fixture_type_id");
                push_argument(&mut parts, fixture_ref);
                push_argument(&mut parts, fixture_type_id);
                push_argument(&mut parts, dmx_mode);
            }
            Command::Select { fid } => {
                push_keyword(&mut parts, "select");
                push_argument(&mut parts, fid);
            }
            Command::Clear { mode } => {
                push_keyword(&mut parts, "clear");
                push_optional_argument(&mut parts, "mode", mode);
            }
            Command::ProgrammerSetValue { fid, attribute, value } => {
                push_keyword(&mut parts, "programmer_set_value");
                push_argument(&mut parts, fid);
                push_argument(&mut parts, attribute);
                push_argument(&mut parts, value);
            }
            Command::Create { r#type, pool_id, name } => {
                push_keyword(&mut parts, "create");
                push_argument(&mut parts, r#type);
                push_argument(&mut parts, pool_id);
                push_optional_argument(&mut parts, "name", name);
            }
            Command::Remove { object_ref } => {
                push_keyword(&mut parts, "remove");
                push_argument(&mut parts, object_ref);
            }
        }

        write!(f, "{}", parts.join(" "))
    }
}

#[derive(Clone)]
pub enum ObjectReference {
    PoolItem(ObjectType, PoolId),
    ObjectId(ObjectId),
}

impl ObjectReference {
    pub fn object_id(&self, engine: &Engine) -> Result<ObjectId> {
        match self {
            ObjectReference::PoolItem(object_type, pool_id) => {
                engine.pools().read(|pools| pools.get(*object_type, *pool_id)).wrap_err_with(|| {
                    format!("could not find object for pool id {pool_id} of type {object_type}")
                })
            }
            ObjectReference::ObjectId(object_id) => Ok(*object_id),
        }
    }
}

impl fmt::Display for ObjectReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectReference::PoolItem(object_type, pool_id) => write!(f, "{object_type} {pool_id}"),
            ObjectReference::ObjectId(object_id) => write!(f, "{object_id}"),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum ClearMode {
    ProgrammerSelection,
    ProgrammerValues,
    #[default]
    Progressive,
    All,
}

impl fmt::Display for ClearMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClearMode::ProgrammerSelection => write!(f, "selection"),
            ClearMode::ProgrammerValues => write!(f, "programmer"),
            ClearMode::Progressive => write!(f, "progressive"),
            ClearMode::All => write!(f, "all"),
        }
    }
}

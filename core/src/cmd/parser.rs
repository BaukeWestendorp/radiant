use std::str::FromStr;

use chumsky::prelude::*;
use chumsky::text::{ident, keyword};

use crate::cmd::{
    Command, CueCommand, ExecutorCommand, FixtureGroupCommand, ObjectCommand, PatchCommand,
    PresetCommand, ProgrammerCommand, SequenceCommand,
};
use crate::object::{
    AnyObjectId, AnyPresetId, BeamPresetId, ColorPresetId, ControlPresetId, CueId, DimmerPresetId,
    ExecutorButtonMode, ExecutorFaderMode, ExecutorId, FixtureGroupId, FocusPresetId, GoboPresetId,
    PositionPresetId, Recipe, RecipeContent, SequenceId, ShapersPresetId, VideoPresetId,
};
use crate::patch::{Attribute, AttributeValue, DmxMode, FixtureId};

type ErrKind<'src> = extra::Full<Rich<'src, char>, (), ()>;

pub fn parser<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    choice((patch(), programmer(), create(), remove(), rename(), object()))
}

fn patch<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    let add = keyword("add")
        .padded()
        .ignore_then(
            fid()
                .padded()
                .then(dmx_addr())
                .padded()
                .then(quoted_string())
                .padded()
                .then(quoted_string()),
        )
        .map(|(((fid, address), gdtf), mode)| {
            Command::Patch(PatchCommand::Add { fid, address, gdtf, mode: DmxMode::new(mode) })
        });

    let set = keyword("set").padded().ignore_then(choice((
        keyword("address")
            .padded()
            .ignore_then(fid().padded().then(dmx_addr()))
            .map(|(fid, address)| Command::Patch(PatchCommand::SetAddress { fid, address })),
        keyword("gdtf")
            .padded()
            .ignore_then(fid().padded().then(quoted_string()))
            .map(|(fid, name)| Command::Patch(PatchCommand::SetGdtf { fid, name })),
        keyword("mode").padded().ignore_then(fid().padded().then(quoted_string())).map(
            |(fid, mode)| Command::Patch(PatchCommand::SetMode { fid, mode: DmxMode::new(mode) }),
        ),
    )));

    let remove = keyword("remove")
        .padded()
        .ignore_then(fid())
        .map(|fid| Command::Patch(PatchCommand::Remove { fid }));

    keyword("patch").padded().ignore_then(choice((add, set, remove)))
}

fn programmer<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    let attribute = keyword("attribute")
        .padded()
        .ignore_then(fid().padded().then(attribute()).padded().then(attribute_value()))
        .map(|((fid, attribute), value)| {
            Command::Programmer(ProgrammerCommand::SetAttribute { fid, attribute, value })
        });

    let address = keyword("address")
        .padded()
        .ignore_then(dmx_addr().padded().then(dmx_value()))
        .map(|(address, value)| {
            Command::Programmer(ProgrammerCommand::SetAddress { address, value })
        });

    let clear = keyword("clear").padded().to(Command::Programmer(ProgrammerCommand::Clear));

    keyword("programmer").padded().ignore_then(choice((attribute, address, clear)))
}

fn create<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("create")
        .padded()
        .ignore_then(any_object_id().padded().then(quoted_string().padded().or_not()))
        .map(|(id, name)| Command::Create { id, name })
}

fn remove<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("remove").padded().ignore_then(any_object_id()).map(|id| Command::Remove { id })
}

fn rename<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("rename").padded().ignore_then(
        any_object_id()
            .padded()
            .then(quoted_string())
            .map(|(id, name)| Command::Rename { id, name }),
    )
}

fn object<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    let executor_subcmd = choice((
        keyword("button").padded().ignore_then(choice((
            keyword("mode").padded().ignore_then(
                keyword("go").to(ExecutorCommand::ButtonSetMode { mode: ExecutorButtonMode::Go }),
            ),
            keyword("press").to(ExecutorCommand::ButtonPress),
            keyword("release").to(ExecutorCommand::ButtonRelease),
        ))),
        keyword("fader").padded().ignore_then(choice((
            keyword("mode").padded().ignore_then(
                keyword("master")
                    .to(ExecutorCommand::FaderSetMode { mode: ExecutorFaderMode::Master }),
            ),
            keyword("level")
                .padded()
                .ignore_then(float())
                .map(|level| ExecutorCommand::FaderSetLevel { level }),
        ))),
        keyword("sequence")
            .padded()
            .ignore_then(sequence_id())
            .map(|sequence_id| ExecutorCommand::SetSequence { sequence_id }),
        keyword("clear").to(ExecutorCommand::Clear),
    ));

    let sequence_subcmd = choice((
        keyword("add").padded().ignore_then(cue_id()).map(|cue_id| SequenceCommand::Add { cue_id }),
        keyword("replace_at")
            .padded()
            .ignore_then(index())
            .padded()
            .then(cue_id())
            .map(|(index, cue_id)| SequenceCommand::ReplaceAt { index, cue_id }),
        keyword("remove")
            .padded()
            .ignore_then(cue_id())
            .map(|cue_id| SequenceCommand::Remove { cue_id }),
        keyword("remove_at")
            .padded()
            .ignore_then(index())
            .map(|index| SequenceCommand::RemoveAt { index }),
        keyword("clear").to(SequenceCommand::Clear),
    ));

    let cue_subcmd = choice((
        keyword("add").padded().ignore_then(recipe()).map(|recipe| CueCommand::Add { recipe }),
        keyword("replace_at")
            .padded()
            .ignore_then(index())
            .padded()
            .then(recipe())
            .map(|(index, recipe)| CueCommand::ReplaceAt { index, recipe }),
        keyword("remove_at")
            .padded()
            .ignore_then(index())
            .map(|index| CueCommand::RemoveAt { index }),
        keyword("clear").to(CueCommand::Clear),
    ));

    let fixture_group_subcmd = choice((
        keyword("add").padded().ignore_then(fid()).map(|fid| FixtureGroupCommand::Add { fid }),
        keyword("replace_at")
            .padded()
            .ignore_then(index())
            .padded()
            .then(fid())
            .map(|(index, fid)| FixtureGroupCommand::ReplaceAt { index, fid }),
        keyword("remove")
            .padded()
            .ignore_then(fid())
            .map(|fid| FixtureGroupCommand::Remove { fid }),
        keyword("remove_at")
            .padded()
            .ignore_then(index())
            .map(|index| FixtureGroupCommand::RemoveAt { index }),
        keyword("clear").to(FixtureGroupCommand::Clear),
    ));

    let preset_subcmd = choice((
        keyword("store").to(PresetCommand::Store),
        keyword("clear").to(PresetCommand::Clear),
    ));

    choice((
        executor_id()
            .padded()
            .then(executor_subcmd)
            .map(|(id, cmd)| Command::Object(ObjectCommand::Executor(id, cmd))),
        sequence_id()
            .padded()
            .then(sequence_subcmd)
            .map(|(id, cmd)| Command::Object(ObjectCommand::Sequence(id, cmd))),
        cue_id()
            .padded()
            .then(cue_subcmd)
            .map(|(id, cmd)| Command::Object(ObjectCommand::Cue(id, cmd))),
        fixture_group_id()
            .padded()
            .then(fixture_group_subcmd)
            .map(|(id, cmd)| Command::Object(ObjectCommand::FixtureGroup(id, cmd))),
        any_preset_id()
            .padded()
            .then(preset_subcmd)
            .map(|(id, cmd)| Command::Object(ObjectCommand::Preset(id.into(), cmd))),
    ))
}

fn fid<'src>() -> impl Parser<'src, &'src str, FixtureId, ErrKind<'src>> {
    text::int(10).from_str().unwrapped().map(FixtureId)
}

fn executor_id<'src>() -> impl Parser<'src, &'src str, ExecutorId, ErrKind<'src>> {
    keyword("executor")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| ExecutorId::from_str(s).unwrap())
}

fn sequence_id<'src>() -> impl Parser<'src, &'src str, SequenceId, ErrKind<'src>> {
    keyword("sequence")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| SequenceId::from_str(s).unwrap())
}

fn cue_id<'src>() -> impl Parser<'src, &'src str, CueId, ErrKind<'src>> {
    keyword("cue").padded().ignore_then(text::int(10)).map(|s: &str| CueId::from_str(s).unwrap())
}

fn fixture_group_id<'src>() -> impl Parser<'src, &'src str, FixtureGroupId, ErrKind<'src>> {
    keyword("fixture_group")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| FixtureGroupId::from_str(s).unwrap())
}

fn preset_position_id<'src>() -> impl Parser<'src, &'src str, PositionPresetId, ErrKind<'src>> {
    just("preset::position")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| PositionPresetId::from_str(s).unwrap())
}

fn preset_gobo_id<'src>() -> impl Parser<'src, &'src str, GoboPresetId, ErrKind<'src>> {
    just("preset::gobo")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| GoboPresetId::from_str(s).unwrap())
}

fn preset_dimmer_id<'src>() -> impl Parser<'src, &'src str, DimmerPresetId, ErrKind<'src>> {
    just("preset::dimmer")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| DimmerPresetId::from_str(s).unwrap())
}

fn preset_color_id<'src>() -> impl Parser<'src, &'src str, ColorPresetId, ErrKind<'src>> {
    just("preset::color")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| ColorPresetId::from_str(s).unwrap())
}

fn preset_beam_id<'src>() -> impl Parser<'src, &'src str, BeamPresetId, ErrKind<'src>> {
    just("preset::beam")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| BeamPresetId::from_str(s).unwrap())
}

fn preset_focus_id<'src>() -> impl Parser<'src, &'src str, FocusPresetId, ErrKind<'src>> {
    just("preset::focus")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| FocusPresetId::from_str(s).unwrap())
}

fn preset_control_id<'src>() -> impl Parser<'src, &'src str, ControlPresetId, ErrKind<'src>> {
    just("preset::control")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| ControlPresetId::from_str(s).unwrap())
}

fn preset_shapers_id<'src>() -> impl Parser<'src, &'src str, ShapersPresetId, ErrKind<'src>> {
    just("preset::shapers")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| ShapersPresetId::from_str(s).unwrap())
}

fn preset_video_id<'src>() -> impl Parser<'src, &'src str, VideoPresetId, ErrKind<'src>> {
    just("preset::video")
        .padded()
        .ignore_then(text::int(10))
        .map(|s: &str| VideoPresetId::from_str(s).unwrap())
}

fn any_preset_id<'src>() -> impl Parser<'src, &'src str, AnyPresetId, ErrKind<'src>> {
    choice((
        preset_dimmer_id().map(AnyPresetId::from),
        preset_position_id().map(AnyPresetId::from),
        preset_gobo_id().map(AnyPresetId::from),
        preset_color_id().map(AnyPresetId::from),
        preset_beam_id().map(AnyPresetId::from),
        preset_focus_id().map(AnyPresetId::from),
        preset_control_id().map(AnyPresetId::from),
        preset_shapers_id().map(AnyPresetId::from),
        preset_video_id().map(AnyPresetId::from),
    ))
}

fn any_object_id<'src>() -> impl Parser<'src, &'src str, AnyObjectId, ErrKind<'src>> {
    choice((
        executor_id().map(AnyObjectId::from),
        sequence_id().map(AnyObjectId::from),
        cue_id().map(AnyObjectId::from),
        fixture_group_id().map(AnyObjectId::from),
        any_preset_id().map(AnyObjectId::from),
    ))
}

fn dmx_addr<'src>() -> impl Parser<'src, &'src str, dmx::Address, ErrKind<'src>> {
    text::int(10)
        .from_str()
        .validate(|u, e, emitter| match u {
            Ok(u) => match dmx::UniverseId::new(u) {
                Ok(universe_id) => universe_id,
                Err(_) => {
                    emitter.emit(Rich::custom(e.span(), dmx::Error::InvalidUniverseId(u)));
                    dmx::UniverseId::default()
                }
            },
            Err(err) => {
                emitter.emit(Rich::custom(e.span(), err));
                dmx::UniverseId::default()
            }
        })
        .then_ignore(just('.').or_not().validate(|dot, e, emitter| {
            if dot.is_none() {
                emitter.emit(Rich::custom(e.span(), "expected '.' between universe and channel"));
            }
        }))
        .then(text::int(10).from_str().validate(|c, e, emitter| match c {
            Ok(c) => match dmx::Channel::new(c) {
                Ok(c) => c,
                Err(_) => {
                    emitter.emit(Rich::custom(e.span(), dmx::Error::InvalidChannel(c)));
                    dmx::Channel::default()
                }
            },
            Err(err) => {
                emitter.emit(Rich::custom(e.span(), err));
                dmx::Channel::default()
            }
        }))
        .map(|(u, c)| dmx::Address::new(u, c))
}

fn dmx_value<'src>() -> impl Parser<'src, &'src str, dmx::Value, ErrKind<'src>> {
    text::int(10).from_str().validate(|value, e, emitter| match value {
        Ok(value) => dmx::Value(value),
        Err(err) => {
            emitter.emit(Rich::custom(e.span(), err));
            dmx::Value::default()
        }
    })
}

fn attribute<'src>() -> impl Parser<'src, &'src str, Attribute, ErrKind<'src>> {
    ident().from_str().unwrapped()
}

fn attribute_value<'src>() -> impl Parser<'src, &'src str, AttributeValue, ErrKind<'src>> {
    float().map(AttributeValue::new)
}

fn recipe<'src>() -> impl Parser<'src, &'src str, Recipe, ErrKind<'src>> {
    // FIXME: Parse level_effect.
    fixture_group_id()
        .padded()
        .then(any_preset_id().map(|id| RecipeContent::Preset(id)))
        .map(|(fixture_group, content)| Recipe { fixture_group, content, level_effect: None })
}

fn quoted_string<'src>() -> impl Parser<'src, &'src str, String, ErrKind<'src>> {
    just('"').ignore_then(none_of("\"").repeated().collect::<String>()).then_ignore(just('"'))
}

fn index<'src>() -> impl Parser<'src, &'src str, usize, ErrKind<'src>> {
    text::int(10).from_str().unwrapped()
}

fn float<'src>() -> impl Parser<'src, &'src str, f32, ErrKind<'src>> {
    let digits = text::digits(10).to_slice();

    let frac = just('.').then(digits);

    just('-')
        .or_not()
        .then(text::int(10))
        .then(frac.or_not())
        .to_slice()
        .map(|s: &str| s.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::cmd::parser::parser;
    use crate::cmd::{
        Command, CueCommand, ExecutorCommand, FixtureGroupCommand, ObjectCommand, PatchCommand,
        PresetCommand, ProgrammerCommand, SequenceCommand,
    };
    use crate::object::{
        AnyObjectId, AnyPresetId, BeamPresetId, ColorPresetId, ControlPresetId, CueId,
        DimmerPresetId, ExecutorButtonMode, ExecutorFaderMode, ExecutorId, FixtureGroupId,
        FocusPresetId, GoboPresetId, PositionPresetId, Recipe, RecipeContent, SequenceId,
        ShapersPresetId, VideoPresetId,
    };
    use crate::patch::{Attribute, AttributeValue, DmxMode, FixtureId};

    macro_rules! should_error {
        ($test:ident, $cmd:literal) => {
            #[test]
            fn $test() {
                assert!(parser().parse($cmd).has_errors())
            }
        };
    }

    macro_rules! expect {
        ($test:ident, $cmd:literal, $expected:expr) => {
            #[test]
            fn $test() {
                assert_eq!(parser().parse($cmd).into_result(), Ok($expected))
            }
        };
    }

    // --- Patch Command Tests ---
    expect!(
        patch_add,
        r#"patch add 1 2.3 "GDTF_FILE" "MODE""#,
        Command::Patch(PatchCommand::Add {
            fid: FixtureId(1),
            address: "2.3".parse().unwrap(),
            gdtf: "GDTF_FILE".to_string(),
            mode: DmxMode::new("MODE")
        })
    );

    expect!(
        patch_set_address,
        r#"patch set address 42 5.12"#,
        Command::Patch(PatchCommand::SetAddress {
            fid: FixtureId(42),
            address: "5.12".parse().unwrap()
        })
    );

    expect!(
        patch_set_gdtf,
        r#"patch set gdtf 7 "SomeGDTF""#,
        Command::Patch(PatchCommand::SetGdtf { fid: FixtureId(7), name: "SomeGDTF".to_string() })
    );

    expect!(
        patch_set_mode,
        r#"patch set mode 3 "SuperMode""#,
        Command::Patch(PatchCommand::SetMode {
            fid: FixtureId(3),
            mode: DmxMode::new("SuperMode")
        })
    );

    expect!(
        patch_remove,
        r#"patch remove 99"#,
        Command::Patch(PatchCommand::Remove { fid: FixtureId(99) })
    );

    should_error!(patch_no_subcommand, "patch");
    should_error!(patch_unknown_subcommand, "patch foo");
    should_error!(patch_add_missing_fid, r#"patch add 2.3 "GDTF_FILE" "MODE""#);
    should_error!(patch_add_missing_address, r#"patch add 1 "GDTF_FILE" "MODE""#);
    should_error!(patch_add_missing_gdtf, r#"patch add 1 2.3 "MODE""#);
    should_error!(patch_add_missing_mode, r#"patch add 1 2.3 "GDTF_FILE""#);
    should_error!(patch_add_malformed_address, r#"patch add 1 2-3 "GDTF_FILE" "MODE""#);
    should_error!(patch_add_unquoted_gdtf, r#"patch add 1 2.3 GDTF_FILE "MODE""#);
    should_error!(patch_add_unquoted_mode, r#"patch add 1 2.3 "GDTF_FILE" MODE"#);
    should_error!(patch_add_extra_args, r#"patch add 2.3 "GDTF_FILE" "MODE" "EXTRA""#);
    should_error!(patch_address_missing_fid, r#"patch address 2.3"#);
    should_error!(patch_address_missing_address, r#"patch address 1 "#);
    should_error!(patch_gdtf_missing_fid, r#"patch gdtf "GDTF_FILE""#);
    should_error!(patch_gdtf_missing_gdtf, r#"patch gdtf 1 "#);
    should_error!(patch_gdtf_unquoted, r#"patch gdtf 1 GDTF_FILE"#);
    should_error!(patch_mode_missing_fid, r#"patch mode "MODE""#);
    should_error!(patch_mode_missing_mode, r#"patch mode 1 "#);
    should_error!(patch_mode_unquoted, r#"patch mode 1 MODE"#);
    should_error!(patch_address_extra_args, r#"patch address 1 2.3 4"#);
    should_error!(patch_gdtf_extra_args, r#"patch gdtf 1 "GDTF_FILE" "EXTRA""#);
    should_error!(patch_mode_extra_args, r#"patch mode 1 "MODE" "EXTRA""#);
    should_error!(patch_remove_missing_fid, r#"patch remove "#);
    should_error!(patch_remove_non_integer, r#"patch remove abc"#);

    // --- Programmer Command Tests ---
    expect!(
        programmer_attribute,
        r#"programmer attribute 1 Dimmer 1.0"#,
        Command::Programmer(ProgrammerCommand::SetAttribute {
            fid: FixtureId(1),
            attribute: Attribute::Dimmer,
            value: AttributeValue::new(1.0)
        })
    );

    expect!(
        programmer_address,
        r#"programmer address 2.10 255"#,
        Command::Programmer(ProgrammerCommand::SetAddress {
            address: "2.10".parse().unwrap(),
            value: dmx::Value(255)
        })
    );

    expect!(programmer_clear, r#"programmer clear"#, Command::Programmer(ProgrammerCommand::Clear));

    should_error!(programmer_no_subcommand, "programmer");
    should_error!(programmer_unknown_subcommand, "programmer foo");
    should_error!(programmer_attribute_missing_fid, r#"programmer attribute Dimmer 1.0"#);
    should_error!(programmer_attribute_missing_attribute, r#"programmer attribute 1 1.0"#);
    should_error!(programmer_attribute_missing_value, r#"programmer attribute 1 Dimmer"#);
    should_error!(programmer_attribute_non_numeric_value, r#"programmer attribute 1 Dimmer foo"#);
    should_error!(programmer_address_missing_address, r#"programmer address 0.5"#);
    should_error!(programmer_address_missing_value, r#"programmer address 2.10"#);
    should_error!(programmer_address_invalid_address, r#"programmer address 2-10 0.5"#);
    should_error!(programmer_address_non_numeric_value, r#"programmer address 2.10 foo"#);
    should_error!(programmer_clear_extra_args, r#"programmer clear foo"#);
    should_error!(programmer_attribute_extra_args, r#"programmer attribute 1 Dimmer, 0.5 extra"#);
    should_error!(programmer_address_extra_args, r#"programmer address 2.10 0.5 extra"#);

    // --- Create/Remove/Rename Command Tests ---
    expect!(
        create_executor,
        r#"create executor 1"#,
        Command::Create { id: AnyObjectId::from(ExecutorId(1)), name: None }
    );
    expect!(
        create_executor_with_name,
        r#"create executor 1 "Test Executor""#,
        Command::Create {
            id: AnyObjectId::from(ExecutorId(1)),
            name: Some("Test Executor".to_string())
        }
    );
    should_error!(create_missing_type, r#"create"#);
    should_error!(create_missing_id, r#"create executor"#);
    should_error!(create_executor_invalid_id, r#"create executor foo"#);

    expect!(
        remove_executor,
        r#"remove executor 1"#,
        Command::Remove { id: AnyObjectId::from(ExecutorId(1)) }
    );
    should_error!(remove_missing_type, r#"remove"#);
    should_error!(remove_missing_id, r#"remove executor"#);
    should_error!(remove_executor_invalid_id, r#"remove executor foo"#);

    expect!(
        rename_executor,
        r#"rename executor 1 "New Name""#,
        Command::Rename { id: AnyObjectId::from(ExecutorId(1)), name: "New Name".to_string() }
    );
    should_error!(rename_missing_type, r#"rename"#);
    should_error!(rename_missing_id, r#"rename executor"#);
    should_error!(rename_missing_name, r#"rename executor 1"#);
    should_error!(rename_executor_invalid_id, r#"rename executor foo \"Name\""#);

    // --- Fixture Group Command Tests ---
    expect!(
        fixture_group_add,
        r#"fixture_group 1 add 2"#,
        Command::Object(ObjectCommand::FixtureGroup(
            FixtureGroupId(1),
            FixtureGroupCommand::Add { fid: FixtureId(2) }
        ))
    );
    expect!(
        fixture_group_replace_at,
        r#"fixture_group 1 replace_at 1 2"#,
        Command::Object(ObjectCommand::FixtureGroup(
            FixtureGroupId(1),
            FixtureGroupCommand::ReplaceAt { index: 1, fid: FixtureId(2) }
        ))
    );
    expect!(
        fixture_group_remove,
        r#"fixture_group 1 remove 2"#,
        Command::Object(ObjectCommand::FixtureGroup(
            FixtureGroupId(1),
            FixtureGroupCommand::Remove { fid: FixtureId(2) }
        ))
    );
    expect!(
        fixture_group_clear,
        r#"fixture_group 1 clear"#,
        Command::Object(ObjectCommand::FixtureGroup(1.into(), FixtureGroupCommand::Clear))
    );
    should_error!(fixture_group_add_missing_fid, r#"fixture_group 1 add"#);
    should_error!(fixture_group_replace_at_missing_args, r#"fixture_group 1 replace_at 1"#);
    should_error!(fixture_group_remove_missing_fid, r#"fixture_group 1 remove"#);

    // --- Executor Command Tests ---
    expect!(
        executor_button_mode,
        r#"executor 1 button mode go"#,
        Command::Object(ObjectCommand::Executor(
            ExecutorId(1),
            ExecutorCommand::ButtonSetMode { mode: ExecutorButtonMode::Go }
        ))
    );
    expect!(
        executor_button_press,
        r#"executor 1 button press"#,
        Command::Object(ObjectCommand::Executor(ExecutorId(1), ExecutorCommand::ButtonPress))
    );
    expect!(
        executor_button_release,
        r#"executor 1 button release"#,
        Command::Object(ObjectCommand::Executor(ExecutorId(1), ExecutorCommand::ButtonRelease))
    );
    should_error!(executor_button_mode_missing_mode, r#"executor 1 button mode"#);
    should_error!(executor_button_unknown_subcmd, r#"executor 1 button foo"#);

    expect!(
        executor_fadermode,
        r#"executor 1 fader mode master"#,
        Command::Object(ObjectCommand::Executor(
            ExecutorId(1),
            ExecutorCommand::FaderSetMode { mode: ExecutorFaderMode::Master }
        ))
    );
    expect!(
        executor_faderlevel,
        r#"executor 1 fader level 0.5"#,
        Command::Object(ObjectCommand::Executor(
            ExecutorId(1),
            ExecutorCommand::FaderSetLevel { level: 0.5 }
        ))
    );
    should_error!(executor_fader_mode_missing_mode, r#"executor 1 fader mode"#);
    should_error!(executor_fader_level_missing_value, r#"executor 1 fader level"#);

    expect!(
        executor_set_sequence,
        r#"executor 1 sequence sequence 2"#,
        Command::Object(ObjectCommand::Executor(
            ExecutorId(1),
            ExecutorCommand::SetSequence { sequence_id: SequenceId(2) }
        ))
    );
    should_error!(executor_set_sequence_missing_id, r#"executor 1 sequence sequence"#);

    expect!(
        executor_clear,
        r#"executor 1 clear"#,
        Command::Object(ObjectCommand::Executor(ExecutorId(1), ExecutorCommand::Clear))
    );

    // --- Sequence Command Tests ---
    expect!(
        sequence_add_cue,
        r#"sequence 1 add cue 2"#,
        Command::Object(ObjectCommand::Sequence(
            SequenceId(1),
            SequenceCommand::Add { cue_id: CueId(2) }
        ))
    );
    expect!(
        sequence_replace_at,
        r#"sequence 1 replace_at 1 cue 2"#,
        Command::Object(ObjectCommand::Sequence(
            SequenceId(1),
            SequenceCommand::ReplaceAt { index: 1, cue_id: CueId(2) }
        ))
    );
    expect!(
        sequence_remove_cue,
        r#"sequence 1 remove cue 2"#,
        Command::Object(ObjectCommand::Sequence(
            SequenceId(1),
            SequenceCommand::Remove { cue_id: CueId(2) }
        ))
    );
    expect!(
        sequence_remove_at,
        r#"sequence 1 remove_at 1"#,
        Command::Object(ObjectCommand::Sequence(
            SequenceId(1),
            SequenceCommand::RemoveAt { index: 1 }
        ))
    );
    expect!(
        sequence_clear,
        r#"sequence 1 clear"#,
        Command::Object(ObjectCommand::Sequence(SequenceId(1), SequenceCommand::Clear))
    );
    should_error!(sequence_add_cue_missing_id, r#"sequence 1 add"#);
    should_error!(sequence_replace_at_missing_args, r#"sequence 1 replace_at 1"#);
    should_error!(sequence_remove_cue_missing_id, r#"sequence 1 remove"#);
    should_error!(sequence_remove_at_missing_index, r#"sequence 1 remove_at"#);

    expect!(
        cue_add,
        r#"cue 1 add fixture_group 1 preset::dimmer 2"#,
        Command::Object(ObjectCommand::Cue(
            CueId(1),
            CueCommand::Add {
                recipe: Recipe {
                    fixture_group: FixtureGroupId(1),
                    content: RecipeContent::Preset(AnyPresetId::from(DimmerPresetId(2))),
                    level_effect: None,
                }
            }
        ))
    );
    expect!(
        cue_replace_at,
        r#"cue 1 replace_at 1 fixture_group 2 preset::dimmer 3"#,
        Command::Object(ObjectCommand::Cue(
            CueId(1),
            CueCommand::ReplaceAt {
                index: 1,
                recipe: Recipe {
                    fixture_group: FixtureGroupId(2),
                    content: RecipeContent::Preset(AnyPresetId::from(DimmerPresetId(3))),
                    level_effect: None,
                }
            }
        ))
    );
    expect!(
        cue_remove_at,
        r#"cue 1 remove_at 1"#,
        Command::Object(ObjectCommand::Cue(CueId(1), CueCommand::RemoveAt { index: 1 }))
    );
    expect!(
        cue_clear,
        r#"cue 1 clear"#,
        Command::Object(ObjectCommand::Cue(CueId::from(1), CueCommand::Clear))
    );
    should_error!(cue_add_missing_args, r#"cue 1 add"#);
    should_error!(
        cue_replace_at_missing_args,
        r#"cue 1 replace_at 1 fixture_group 2 preset::dimmer"#
    );
    should_error!(cue_remove_at_missing_index, r#"cue 1 remove_at"#);

    expect!(
        preset_dimmer_store,
        r#"preset::dimmer 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(DimmerPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_dimmer_clear,
        r#"preset::dimmer 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(DimmerPresetId(1)),
            PresetCommand::Clear
        ))
    );

    expect!(
        preset_position_store,
        r#"preset::position 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(PositionPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_position_clear,
        r#"preset::position 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(PositionPresetId(1)),
            PresetCommand::Clear
        ))
    );

    expect!(
        preset_gobo_store,
        r#"preset::gobo 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(GoboPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_gobo_clear,
        r#"preset::gobo 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(GoboPresetId(1)),
            PresetCommand::Clear
        ))
    );

    expect!(
        preset_color_store,
        r#"preset::color 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(ColorPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_color_clear,
        r#"preset::color 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(ColorPresetId(1)),
            PresetCommand::Clear
        ))
    );

    expect!(
        preset_beam_store,
        r#"preset::beam 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(BeamPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_beam_clear,
        r#"preset::beam 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(BeamPresetId(1)),
            PresetCommand::Clear
        ))
    );

    expect!(
        preset_focus_store,
        r#"preset::focus 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(FocusPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_focus_clear,
        r#"preset::focus 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(FocusPresetId(1)),
            PresetCommand::Clear
        ))
    );

    expect!(
        preset_control_store,
        r#"preset::control 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(ControlPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_control_clear,
        r#"preset::control 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(ControlPresetId(1)),
            PresetCommand::Clear
        ))
    );

    expect!(
        preset_shapers_store,
        r#"preset::shapers 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(ShapersPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_shapers_clear,
        r#"preset::shapers 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(ShapersPresetId(1)),
            PresetCommand::Clear
        ))
    );

    expect!(
        preset_video_store,
        r#"preset::video 1 store"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(VideoPresetId(1)),
            PresetCommand::Store
        ))
    );
    expect!(
        preset_video_clear,
        r#"preset::video 1 clear"#,
        Command::Object(ObjectCommand::Preset(
            AnyPresetId::from(VideoPresetId(1)),
            PresetCommand::Clear
        ))
    );
}

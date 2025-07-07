use chumsky::prelude::*;
use chumsky::text::{ident, keyword};

use crate::cmd::{Command, PatchCommand, ProgrammerCommand, ProgrammerSetCommand};
use crate::patch::{Attribute, AttributeValue, DmxMode, FixtureId};

type ErrKind<'src> = extra::Full<Rich<'src, char>, (), ()>;

pub fn parser<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    choice((
        patch(),
        programmer(),
        create(),
        remove(),
        rename(),
        fixture_group(),
        executor(),
        cue(),
        preset(),
    ))
}

fn patch<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    let add = keyword("add")
        .padded()
        .ignore_then(
            fid()
                .then_ignore(just(',').padded())
                .then(dmx_addr())
                .then_ignore(just(',').padded())
                .then(quoted_string())
                .then_ignore(just(',').padded())
                .then(quoted_string()),
        )
        .map(|(((fid, address), gdtf_file_name), mode)| {
            Command::Patch(PatchCommand::Add {
                fid,
                address,
                gdtf_file_name,
                mode: DmxMode::new(mode),
            })
        });

    let set = keyword("set").padded().ignore_then(choice((
        keyword("address")
            .padded()
            .ignore_then(fid().then_ignore(just(',').padded()).then(dmx_addr()))
            .map(|(fid, address)| Command::Patch(PatchCommand::SetAddress { fid, address })),
        keyword("gdtf")
            .padded()
            .ignore_then(fid().then_ignore(just(',').padded()).then(quoted_string()))
            .map(|(fid, name)| Command::Patch(PatchCommand::SetGdtfFileName { fid, name })),
        keyword("mode")
            .padded()
            .ignore_then(fid().then_ignore(just(',').padded()).then(quoted_string()))
            .map(|(fid, mode)| {
                Command::Patch(PatchCommand::SetMode { fid, mode: DmxMode::new(mode) })
            }),
    )));

    let remove = keyword("remove")
        .padded()
        .ignore_then(fid())
        .map(|fid| Command::Patch(PatchCommand::Remove { fid }));

    keyword("patch")
        .labelled("expected a command")
        .padded()
        .ignore_then(choice((add, set, remove)).labelled("expected a subcommand"))
        .boxed()
}

fn programmer<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    let set = keyword("set").padded().ignore_then(choice((
        keyword("attribute")
            .padded()
            .ignore_then(
                fid()
                    .then_ignore(just(',').padded())
                    .then(attribute())
                    .then_ignore(just(',').padded())
                    .then(attribute_value()),
            )
            .map(|((fid, attribute), value)| {
                Command::Programmer(ProgrammerCommand::Set(ProgrammerSetCommand::Attribute {
                    fid,
                    attribute,
                    value,
                }))
            }),
        keyword("direct")
            .padded()
            .ignore_then(dmx_addr().then_ignore(just(',').padded()).then(dmx_value()))
            .map(|(address, value)| {
                Command::Programmer(ProgrammerCommand::Set(ProgrammerSetCommand::Direct {
                    address,
                    value,
                }))
            }),
    )));

    let clear = keyword("clear").padded().to(Command::Programmer(ProgrammerCommand::Clear));

    keyword("programmer")
        .labelled("expected a command")
        .padded()
        .ignore_then(choice((set, clear)).labelled("expected a subcommand"))
        .boxed()
}

fn create<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("create")
        .labelled("expected a command")
        .padded()
        .ignore_then(todo().labelled("expected a subcommand"))
        .boxed()
}

fn remove<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("remove")
        .labelled("expected a command")
        .padded()
        .ignore_then(todo().labelled("expected a subcommand"))
        .boxed()
}

fn rename<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("rename")
        .labelled("expected a command")
        .padded()
        .ignore_then(todo().labelled("expected a subcommand"))
        .boxed()
}

fn fixture_group<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("fixture_group")
        .labelled("expected a command")
        .padded()
        .ignore_then(todo().labelled("expected a subcommand"))
        .boxed()
}

fn executor<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("executor")
        .labelled("expected a command")
        .padded()
        .ignore_then(todo().labelled("expected a subcommand"))
        .boxed()
}

fn cue<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("cue")
        .labelled("expected a command")
        .padded()
        .ignore_then(todo().labelled("expected a subcommand"))
        .boxed()
}

fn preset<'src>() -> impl Parser<'src, &'src str, Command, ErrKind<'src>> {
    keyword("preset")
        .labelled("expected a command")
        .padded()
        .ignore_then(todo().labelled("expected a subcommand"))
        .boxed()
}

fn fid<'src>() -> impl Parser<'src, &'src str, FixtureId, ErrKind<'src>> {
    text::int(10).from_str().unwrapped().map(FixtureId).boxed()
}

fn dmx_addr<'src>() -> impl Parser<'src, &'src str, dmx::Address, ErrKind<'src>> {
    text::int(10)
        .from_str()
        .unwrapped()
        .then_ignore(just('.'))
        .then(text::int(10).from_str().unwrapped())
        .map(|(u, c): (dmx::UniverseId, dmx::Channel)| dmx::Address::new(u, c))
        .padded()
        .boxed()
}

fn dmx_value<'src>() -> impl Parser<'src, &'src str, dmx::Value, ErrKind<'src>> {
    text::int(10).from_str().unwrapped().padded().boxed()
}

fn attribute<'src>() -> impl Parser<'src, &'src str, Attribute, ErrKind<'src>> {
    ident().from_str().unwrapped().boxed()
}

fn attribute_value<'src>() -> impl Parser<'src, &'src str, AttributeValue, ErrKind<'src>> {
    float().map(AttributeValue::new).boxed()
}

fn quoted_string<'src>() -> impl Parser<'src, &'src str, String, ErrKind<'src>> {
    just('"')
        .ignore_then(none_of("\"").repeated().collect::<String>())
        .then_ignore(just('"'))
        .boxed()
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
        .boxed()
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::cmd::parser::parser;
    use crate::cmd::{Command, PatchCommand, ProgrammerCommand, ProgrammerSetCommand};
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

    expect!(
        patch_add,
        r#"patch add 1, 2.3, "GDTF_FILE", "MODE""#,
        Command::Patch(PatchCommand::Add {
            fid: FixtureId(1),
            address: "2.3".parse().unwrap(),
            gdtf_file_name: "GDTF_FILE".to_string(),
            mode: DmxMode::new("MODE")
        })
    );

    expect!(
        patch_set_address,
        r#"patch set address 42, 5.12"#,
        Command::Patch(PatchCommand::SetAddress {
            fid: FixtureId(42),
            address: "5.12".parse().unwrap()
        })
    );

    expect!(
        patch_set_gdtf,
        r#"patch set gdtf 7, "SomeGDTF""#,
        Command::Patch(PatchCommand::SetGdtfFileName {
            fid: FixtureId(7),
            name: "SomeGDTF".to_string()
        })
    );

    expect!(
        patch_set_mode,
        r#"patch set mode 3, "SuperMode""#,
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
    should_error!(patch_add_missing_fid, r#"patch add , 2.3, "GDTF_FILE", "MODE""#);
    should_error!(patch_add_missing_address, r#"patch add 1, , "GDTF_FILE", "MODE""#);
    should_error!(patch_add_missing_gdtf, r#"patch add 1, 2.3, , "MODE""#);
    should_error!(patch_add_missing_mode, r#"patch add 1, 2.3, "GDTF_FILE", "#);
    should_error!(patch_add_missing_comma, r#"patch add 1 2.3, "GDTF_FILE", "MODE""#);
    should_error!(patch_add_malformed_address, r#"patch add 1, 2-3, "GDTF_FILE", "MODE""#);
    should_error!(patch_add_unquoted_gdtf, r#"patch add 1, 2.3, GDTF_FILE, "MODE""#);
    should_error!(patch_add_unquoted_mode, r#"patch add 1, 2.3, "GDTF_FILE", MODE"#);
    should_error!(patch_add_extra_args, r#"patch add 1, 2.3, "GDTF_FILE", "MODE", "EXTRA""#);
    should_error!(patch_set_address_missing_fid, r#"patch set address , 2.3"#);
    should_error!(patch_set_address_missing_address, r#"patch set address 1, "#);
    should_error!(patch_set_gdtf_missing_fid, r#"patch set gdtf , "GDTF_FILE""#);
    should_error!(patch_set_gdtf_missing_gdtf, r#"patch set gdtf 1, "#);
    should_error!(patch_set_gdtf_unquoted, r#"patch set gdtf 1, GDTF_FILE"#);
    should_error!(patch_set_mode_missing_fid, r#"patch set mode , "MODE""#);
    should_error!(patch_set_mode_missing_mode, r#"patch set mode 1, "#);
    should_error!(patch_set_mode_unquoted, r#"patch set mode 1, MODE"#);
    should_error!(patch_set_address_extra_args, r#"patch set address 1, 2.3, 4"#);
    should_error!(patch_set_gdtf_extra_args, r#"patch set gdtf 1, "GDTF_FILE", "EXTRA""#);
    should_error!(patch_set_mode_extra_args, r#"patch set mode 1, "MODE", "EXTRA""#);
    should_error!(patch_remove_extra_args, r#"patch remove 1, 2"#);
    should_error!(patch_remove_missing_fid, r#"patch remove "#);

    expect!(
        programmer_set_attribute,
        r#"programmer set attribute 1, Dimmer, 1.0"#,
        Command::Programmer(ProgrammerCommand::Set(ProgrammerSetCommand::Attribute {
            fid: FixtureId(1),
            attribute: Attribute::Dimmer,
            value: AttributeValue::new(1.0)
        }))
    );

    // Valid: programmer set direct
    expect!(
        programmer_set_direct,
        r#"programmer set direct 2.10, 255"#,
        Command::Programmer(ProgrammerCommand::Set(ProgrammerSetCommand::Direct {
            address: "2.10".parse().unwrap(),
            value: dmx::Value(255)
        }))
    );

    expect!(programmer_clear, r#"programmer clear"#, Command::Programmer(ProgrammerCommand::Clear));

    should_error!(programmer_no_subcommand, "programmer");
    should_error!(programmer_unknown_subcommand, "programmer foo");
    should_error!(
        programmer_set_attribute_missing_fid,
        r#"programmer set attribute , Dimmer, 1.0"#
    );
    should_error!(
        programmer_set_attribute_missing_attribute,
        r#"programmer set attribute 1, , 1.0"#
    );
    should_error!(
        programmer_set_attribute_missing_value,
        r#"programmer set attribute 1, Dimmer, "#
    );
    should_error!(
        programmer_set_attribute_non_numeric_value,
        r#"programmer set attribute 1, Dimmer, foo"#
    );
    should_error!(programmer_set_direct_missing_address, r#"programmer set direct , 0.5"#);
    should_error!(programmer_set_direct_missing_value, r#"programmer set direct 2.10, "#);
    should_error!(programmer_set_direct_invalid_address, r#"programmer set direct 2-10, 0.5"#);
    should_error!(programmer_set_direct_non_numeric_value, r#"programmer set direct 2.10, foo"#);
    should_error!(programmer_clear_extra_args, r#"programmer clear foo"#);
    should_error!(
        programmer_set_attribute_extra_args,
        r#"programmer set attribute 1, Dimmer, 0.5, extra"#
    );
    should_error!(programmer_set_direct_extra_args, r#"programmer set direct 2.10, 0.5, extra"#);
    should_error!(patch_remove_non_integer, r#"patch remove abc"#);
}

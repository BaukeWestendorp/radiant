use std::{path::Path, str::FromStr};

use neo_radiant::{
    backend::{
        engine::Engine,
        object::{AnyPreset, DimmerPresetId},
        patch::fixture::{DmxMode, FixtureId},
    },
    cmd, dmx,
    showfile::Showfile,
};

fn init_engine() -> Engine {
    let showfile = Showfile::load_folder(Path::new("tests/test_showfile")).unwrap();
    Engine::new(showfile).unwrap()
}

#[test]
fn patch_add() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 "2.3" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    let f = &engine.show().patch().fixtures()[0];

    assert_eq!(engine.show().patch().fixtures().len(), 1);

    assert_eq!(f.id(), FixtureId(1));
    assert_eq!(f.address(), &dmx::Address::from_str("2.3").unwrap());
    assert_eq!(f.dmx_mode(), &DmxMode::new("Default"));
    assert_eq!(f.gdtf_file_name(), "Generic@Dimmer@Generic.gdtf".to_string());

    engine.exec_cmd(cmd!(r#"patch add 4 "5.6" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 7 "8.9" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    assert_eq!(engine.show().patch().fixtures().len(), 3);

    let f = &engine.show().patch().fixtures()[0];
    assert_eq!(f.id(), FixtureId(1));
    assert_eq!(f.address(), &dmx::Address::from_str("2.3").unwrap());

    let f = &engine.show().patch().fixtures()[1];
    assert_eq!(f.id(), FixtureId(4));
    assert_eq!(f.address(), &dmx::Address::from_str("5.6").unwrap());

    let f = &engine.show().patch().fixtures()[2];
    assert_eq!(f.id(), FixtureId(7));
    assert_eq!(f.address(), &dmx::Address::from_str("8.9").unwrap());
}

#[test]
fn patch_set_address() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 "1.1" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 2 "2.2" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    assert_eq!(
        engine.show().patch().fixture(1).unwrap().address(),
        &dmx::Address::from_str("1.1").unwrap(),
    );

    assert_eq!(
        engine.show().patch().fixture(2).unwrap().address(),
        &dmx::Address::from_str("2.2").unwrap(),
    );

    engine.exec_cmd(cmd!(r#"patch set address 1 "5.5""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch set address 2 "6.6""#)).unwrap();

    assert_eq!(
        engine.show().patch().fixture(1).unwrap().address(),
        &dmx::Address::from_str("5.5").unwrap(),
    );

    assert_eq!(
        engine.show().patch().fixture(2).unwrap().address(),
        &dmx::Address::from_str("6.6").unwrap(),
    );
}

#[test]
fn patch_set_mode() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 "1.1" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 2 "2.2" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    assert_eq!(engine.show().patch().fixture(1).unwrap().dmx_mode(), &DmxMode::new("Default"));
    assert_eq!(engine.show().patch().fixture(2).unwrap().dmx_mode(), &DmxMode::new("Default"));

    engine.exec_cmd(cmd!(r#"patch set mode 1 "Other Mode""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch set mode 2 "Other Mode""#)).unwrap();

    assert_eq!(engine.show().patch().fixture(1).unwrap().dmx_mode(), &DmxMode::new("Other Mode"));
    assert_eq!(engine.show().patch().fixture(2).unwrap().dmx_mode(), &DmxMode::new("Other Mode"));

    assert!(engine.exec_cmd(cmd!(r#"patch set mode 1 "Invalid Mode""#)).is_err());
    assert!(engine.exec_cmd(cmd!(r#"patch set mode 2 "Invalid Mode""#)).is_err());
}

#[test]
fn patch_set_gdtf_file_name() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 "1.1" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 2 "2.2" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    assert_eq!(
        engine.show().patch().fixture(1).unwrap().gdtf_file_name(),
        "Generic@Dimmer@Generic.gdtf".to_string()
    );
    assert_eq!(
        engine.show().patch().fixture(2).unwrap().gdtf_file_name(),
        "Generic@Dimmer@Generic.gdtf".to_string()
    );

    engine.exec_cmd(cmd!(r#"patch set gdtf_file_name 1 "Generic@Dimmer2@Generic.gdtf""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch set gdtf_file_name 2 "Generic@Dimmer2@Generic.gdtf""#)).unwrap();

    assert_eq!(
        engine.show().patch().fixture(1).unwrap().gdtf_file_name(),
        "Generic@Dimmer2@Generic.gdtf".to_string()
    );
    assert_eq!(
        engine.show().patch().fixture(2).unwrap().gdtf_file_name(),
        "Generic@Dimmer2@Generic.gdtf".to_string()
    );

    assert!(
        engine
            .exec_cmd(cmd!(r#"patch set gdtf_file_name 1 "Generic@Invalid@Generic.gdtf""#))
            .is_err()
    );
    assert!(
        engine
            .exec_cmd(cmd!(r#"patch set gdtf_file_name 2 "Generic@Invalid@Generic.gdtf""#))
            .is_err()
    );
}

#[test]
fn patch_remove() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 "1.1" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 2 "2.2" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    assert_eq!(engine.show().patch().fixtures().len(), 2);

    engine.exec_cmd(cmd!(r#"patch remove 1 "#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch remove 2 "#)).unwrap();

    assert_eq!(engine.show().patch().fixtures().len(), 0);
}

#[test]
fn programmer_set_direct() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"programmer set direct "1.1" 42"#)).unwrap();

    engine.resolve_dmx();

    assert_eq!(
        engine.output_multiverse().get_value(&dmx::Address::from_str("1.1").unwrap()),
        dmx::Value(42)
    );
}

#[test]
fn programmer_set_attribute() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 "1.1" "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer set attribute 1 "Dimmer" 0.5"#)).unwrap();

    engine.resolve_dmx();

    assert_eq!(
        engine.output_multiverse().get_value(&dmx::Address::from_str("1.1").unwrap()),
        dmx::Value(128)
    );
}

#[test]
fn programmer_clear() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"programmer set direct "1.1" 42"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer set direct "1.2" 69"#)).unwrap();

    engine.resolve_dmx();

    engine.exec_cmd(cmd!(r#"programmer clear"#)).unwrap();

    engine.resolve_dmx();

    assert_eq!(
        engine.output_multiverse().get_value(&dmx::Address::from_str("1.1").unwrap()),
        dmx::Value(0)
    );

    assert_eq!(
        engine.output_multiverse().get_value(&dmx::Address::from_str("1.2").unwrap()),
        dmx::Value(0)
    );
}

#[test]
fn create_with_name() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().name, "Test Name".to_string());
}

#[test]
fn create_executor() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert!(engine.show().executor(1).is_some());
}

#[test]
fn create_sequence() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1"#)).unwrap();
    assert!(engine.show().sequence(1).is_some());
}

#[test]
fn create_cue() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create cue 1"#)).unwrap();
    assert!(engine.show().cue(1).is_some());
}

#[test]
fn create_fixture_group() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1"#)).unwrap();
    assert!(engine.show().fixture_group(1).is_some());
}

#[test]
fn create_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1"#)).unwrap();
    assert!(engine.show().preset(DimmerPresetId(1)).is_some());
}

#[test]
fn remove_executor() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert!(engine.show().executor(1).is_some());
    engine.exec_cmd(cmd!(r#"remove executor 1"#)).unwrap();
    assert!(engine.show().executor(1).is_none());
}

#[test]
fn remove_sequence() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1"#)).unwrap();
    assert!(engine.show().sequence(1).is_some());
    engine.exec_cmd(cmd!(r#"remove sequence 1"#)).unwrap();
    assert!(engine.show().sequence(1).is_none());
}

#[test]
fn remove_cue() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create cue 1"#)).unwrap();
    assert!(engine.show().cue(1).is_some());
    engine.exec_cmd(cmd!(r#"remove cue 1"#)).unwrap();
    assert!(engine.show().cue(1).is_none());
}

#[test]
fn remove_fixture_group() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1"#)).unwrap();
    assert!(engine.show().fixture_group(1).is_some());
    engine.exec_cmd(cmd!(r#"remove fixture_group 1"#)).unwrap();
    assert!(engine.show().fixture_group(1).is_none());
}

#[test]
fn remove_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1"#)).unwrap();
    assert!(engine.show().preset(DimmerPresetId(1)).is_some());
    engine.exec_cmd(cmd!(r#"remove preset::dimmer 1"#)).unwrap();
    assert!(engine.show().preset(DimmerPresetId(1)).is_none());
}

#[test]
fn rename_executor() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().name, "Test Name".to_string());
    engine.exec_cmd(cmd!(r#"rename executor 1 "Other Name"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().name, "Other Name".to_string());
}

#[test]
fn rename_sequence() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().sequence(1).unwrap().name, "Test Name".to_string());
    engine.exec_cmd(cmd!(r#"rename sequence 1 "Other Name"#)).unwrap();
    assert_eq!(engine.show().sequence(1).unwrap().name, "Other Name".to_string());
}

#[test]
fn rename_cue() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create cue 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().cue(1).unwrap().name, "Test Name".to_string());
    engine.exec_cmd(cmd!(r#"rename cue 1 "Other Name"#)).unwrap();
    assert_eq!(engine.show().cue(1).unwrap().name, "Other Name".to_string());
}

#[test]
fn rename_fixture_group() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().fixture_group(1).unwrap().name, "Test Name".to_string());
    engine.exec_cmd(cmd!(r#"rename fixture_group 1 "Other Name"#)).unwrap();
    assert_eq!(engine.show().fixture_group(1).unwrap().name, "Other Name".to_string());
}

#[test]
fn rename_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1 "Test Name""#)).unwrap();

    let preset = engine.show().preset(DimmerPresetId(1)).unwrap();
    let AnyPreset::Dimmer(dimmer) = preset;
    assert_eq!(dimmer.name, "Test Name".to_string());

    engine.exec_cmd(cmd!(r#"rename preset::dimmer 1 "Other Name""#)).unwrap();

    let preset = engine.show().preset(DimmerPresetId(1)).unwrap();
    let AnyPreset::Dimmer(dimmer) = preset;
    assert_eq!(dimmer.name, "Other Name".to_string());
}

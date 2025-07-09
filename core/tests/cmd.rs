use std::path::Path;
use std::str::FromStr;

use radiant_core::cmd;
use radiant_core::engine::Engine;
use radiant_core::object::{
    AnyPreset, AnyPresetId, ColorPresetId, CueId, DimmerPresetId, ExecutorButtonMode,
    ExecutorFaderMode, PresetContent, Recipe, RecipeContent, SelectivePreset, SequenceId,
    UniversalPreset,
};
use radiant_core::patch::{Attribute, AttributeValue, DmxMode, FeatureGroup, FixtureId};
use radiant_core::showfile::Showfile;

fn init_engine() -> Engine {
    let showfile = Showfile::load_folder(Path::new("tests/test_showfile")).unwrap();
    Engine::new(showfile).unwrap()
}

#[test]
fn patch_add() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 2.3 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    let f = &engine.show().patch().fixtures()[0];

    assert_eq!(engine.show().patch().fixtures().len(), 1);

    assert_eq!(f.id(), FixtureId(1));
    assert_eq!(f.address(), &dmx::Address::from_str("2.3").unwrap());
    assert_eq!(f.dmx_mode(), &DmxMode::new("Default"));
    assert_eq!(f.gdtf(), "Generic@Dimmer@Generic.gdtf".to_string());

    engine.exec_cmd(cmd!(r#"patch add 4 5.6 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 7 8.9 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

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

    engine.exec_cmd(cmd!(r#"patch add 1 1.1 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 2 2.2 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    assert_eq!(
        engine.show().patch().fixture(1).unwrap().address(),
        &dmx::Address::from_str("1.1").unwrap(),
    );

    assert_eq!(
        engine.show().patch().fixture(2).unwrap().address(),
        &dmx::Address::from_str("2.2").unwrap(),
    );

    engine.exec_cmd(cmd!(r#"patch set address 1 5.5"#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch set address 2 6.6"#)).unwrap();

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

    engine.exec_cmd(cmd!(r#"patch add 1 1.1 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 2 2.2 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

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
fn patch_set_gdtf() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 1.1 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 2 2.2 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    assert_eq!(
        engine.show().patch().fixture(1).unwrap().gdtf(),
        "Generic@Dimmer@Generic.gdtf".to_string()
    );
    assert_eq!(
        engine.show().patch().fixture(2).unwrap().gdtf(),
        "Generic@Dimmer@Generic.gdtf".to_string()
    );

    engine.exec_cmd(cmd!(r#"patch set gdtf 1 "Generic@Dimmer2@Generic.gdtf""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch set gdtf 2 "Generic@Dimmer2@Generic.gdtf""#)).unwrap();

    assert_eq!(
        engine.show().patch().fixture(1).unwrap().gdtf(),
        "Generic@Dimmer2@Generic.gdtf".to_string()
    );
    assert_eq!(
        engine.show().patch().fixture(2).unwrap().gdtf(),
        "Generic@Dimmer2@Generic.gdtf".to_string()
    );

    assert!(engine.exec_cmd(cmd!(r#"patch set gdtf 1 "Generic@Invalid@Generic.gdtf""#)).is_err());
    assert!(engine.exec_cmd(cmd!(r#"patch set gdtf 2 "Generic@Invalid@Generic.gdtf""#)).is_err());
}

#[test]
fn patch_remove() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 1.1 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch add 2 2.2 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();

    assert_eq!(engine.show().patch().fixtures().len(), 2);

    engine.exec_cmd(cmd!(r#"patch remove 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"patch remove 2"#)).unwrap();

    assert_eq!(engine.show().patch().fixtures().len(), 0);
}

#[test]
fn programmer_set_address() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"programmer address 1.1 42"#)).unwrap();

    engine.resolve_dmx();

    assert_eq!(
        engine.output_multiverse().get_value(&dmx::Address::from_str("1.1").unwrap()),
        dmx::Value(42)
    );
}

#[test]
fn programmer_attribute() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"patch add 1 1.1 "Generic@Dimmer@Generic.gdtf" "Default""#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 1 Dimmer 0.5"#)).unwrap();

    engine.resolve_dmx();

    assert_eq!(
        engine.output_multiverse().get_value(&dmx::Address::from_str("1.1").unwrap()),
        dmx::Value(128)
    );
}

#[test]
fn programmer_clear() {
    let mut engine = init_engine();

    engine.exec_cmd(cmd!(r#"programmer address 1.1 42"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer address 1.2 69"#)).unwrap();

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
    assert_eq!(engine.show().executor(1).unwrap().name(), "Test Name".to_string());
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
fn create_dimmer_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1"#)).unwrap();
    assert!(engine.show().preset(DimmerPresetId(1)).is_some());
}

#[test]
fn create_color_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::color 1"#)).unwrap();
    assert!(engine.show().preset(ColorPresetId(1)).is_some());
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
fn remove_dimmer_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1"#)).unwrap();
    assert!(engine.show().preset(DimmerPresetId(1)).is_some());
    engine.exec_cmd(cmd!(r#"remove preset::dimmer 1"#)).unwrap();
    assert!(engine.show().preset(DimmerPresetId(1)).is_none());
}

#[test]
fn remove_color_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::color 1"#)).unwrap();
    assert!(engine.show().preset(ColorPresetId(1)).is_some());
    engine.exec_cmd(cmd!(r#"remove preset::color 1"#)).unwrap();
    assert!(engine.show().preset(ColorPresetId(1)).is_none());
}

#[test]
fn rename_executor() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().name(), "Test Name".to_string());
    engine.exec_cmd(cmd!(r#"rename executor 1 "Other Name""#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().name(), "Other Name".to_string());
}

#[test]
fn rename_sequence() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().sequence(1).unwrap().name(), "Test Name".to_string());
    engine.exec_cmd(cmd!(r#"rename sequence 1 "Other Name""#)).unwrap();
    assert_eq!(engine.show().sequence(1).unwrap().name(), "Other Name".to_string());
}

#[test]
fn rename_cue() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create cue 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().cue(1).unwrap().name(), "Test Name".to_string());
    engine.exec_cmd(cmd!(r#"rename cue 1 "Other Name""#)).unwrap();
    assert_eq!(engine.show().cue(1).unwrap().name(), "Other Name".to_string());
}

#[test]
fn rename_fixture_group() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1 "Test Name""#)).unwrap();
    assert_eq!(engine.show().fixture_group(1).unwrap().name(), "Test Name".to_string());
    engine.exec_cmd(cmd!(r#"rename fixture_group 1 "Other Name""#)).unwrap();
    assert_eq!(engine.show().fixture_group(1).unwrap().name(), "Other Name".to_string());
}

#[test]
fn rename_dimmer_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1 "Test Name""#)).unwrap();

    let preset = engine.show().preset(DimmerPresetId(1)).unwrap();
    let AnyPreset::Dimmer(dimmer) = preset else { panic!() };
    assert_eq!(dimmer.name(), "Test Name".to_string());

    engine.exec_cmd(cmd!(r#"rename preset::dimmer 1 "Other Name""#)).unwrap();

    let preset = engine.show().preset(DimmerPresetId(1)).unwrap();
    let AnyPreset::Dimmer(dimmer) = preset else { panic!() };
    assert_eq!(dimmer.name(), "Other Name".to_string());
}

#[test]
fn rename_color_preset() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::color 1 "Test Name""#)).unwrap();

    let preset = engine.show().preset(ColorPresetId(1)).unwrap();
    let AnyPreset::Color(color) = preset else { panic!() };
    assert_eq!(color.name(), "Test Name".to_string());

    engine.exec_cmd(cmd!(r#"rename preset::color 1 "Other Name""#)).unwrap();

    let preset = engine.show().preset(ColorPresetId(1)).unwrap();
    let AnyPreset::Color(color) = preset else { panic!() };
    assert_eq!(color.name(), "Other Name".to_string());
}

#[test]
fn fixture_group_add() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 2"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 3"#)).unwrap();
    assert!(!engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(1)));
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(2)));
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(3)));
    assert!(!engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(4)));
}

#[test]
fn fixture_group_replace_at() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 2"#)).unwrap();
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(2)));
    assert!(!engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(4)));
    engine.exec_cmd(cmd!(r#"fixture_group 1 replace_at 0 4"#)).unwrap();
    assert!(!engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(2)));
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(4)));
}

#[test]
fn fixture_group_remove() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 2"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 3"#)).unwrap();
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(2)));
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(3)));
    engine.exec_cmd(cmd!(r#"fixture_group 1 remove 2"#)).unwrap();
    assert!(!engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(2)));
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(3)));
}

#[test]
fn fixture_group_remove_at() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 2"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 3"#)).unwrap();
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(2)));
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(3)));
    engine.exec_cmd(cmd!(r#"fixture_group 1 remove_at 1"#)).unwrap();
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(2)));
    assert!(!engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(3)));
}

#[test]
fn fixture_group_clear() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create fixture_group 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 2"#)).unwrap();
    engine.exec_cmd(cmd!(r#"fixture_group 1 add 3"#)).unwrap();
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(2)));
    assert!(engine.show().fixture_group(1).unwrap().fixtures().contains(&FixtureId(3)));
    engine.exec_cmd(cmd!(r#"fixture_group 1 clear"#)).unwrap();
    assert!(engine.show().fixture_group(1).unwrap().fixtures().is_empty());
}

#[test]
fn executor_set_button_mode_go() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().button().mode(), ExecutorButtonMode::default());
    engine.exec_cmd(cmd!(r#"executor 1 button mode go"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().button().mode(), ExecutorButtonMode::Go);
}

#[test]
fn executor_set_button_press() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().button().mode(), ExecutorButtonMode::default());
    assert!(!engine.show().executor(1).unwrap().button().currently_pressed());
    engine.exec_cmd(cmd!(r#"executor 1 button press"#)).unwrap();
    assert!(engine.show().executor(1).unwrap().button().currently_pressed());
}

#[test]
fn executor_set_button_release() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().button().mode(), ExecutorButtonMode::default());
    assert!(!engine.show().executor(1).unwrap().button().currently_pressed());
    engine.exec_cmd(cmd!(r#"executor 1 button press"#)).unwrap();
    assert!(engine.show().executor(1).unwrap().button().currently_pressed());
    engine.exec_cmd(cmd!(r#"executor 1 button release"#)).unwrap();
    assert!(!engine.show().executor(1).unwrap().button().currently_pressed());
}

#[test]
fn executor_set_fader_mode_master() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().fader().mode(), ExecutorFaderMode::default());
    engine.exec_cmd(cmd!(r#"executor 1 fader mode master"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().fader().mode(), ExecutorFaderMode::Master);
}

#[test]
fn executor_set_fader_level() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().fader().mode(), ExecutorFaderMode::default());
    engine.exec_cmd(cmd!(r#"executor 1 fader level 0.25"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().fader().level(), 0.25);
}

#[test]
fn executor_sequence() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().sequence_id(), None);
    engine.exec_cmd(cmd!(r#"executor 1 sequence sequence 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().sequence_id(), Some(&SequenceId(1)));
}

#[test]
fn executor_clear() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create executor 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().sequence_id(), None);
    engine.exec_cmd(cmd!(r#"executor 1 sequence sequence 1"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().sequence_id(), Some(&SequenceId(1)));
    engine.exec_cmd(cmd!(r#"executor 1 clear"#)).unwrap();
    assert_eq!(engine.show().executor(1).unwrap().sequence_id(), None);
}

#[test]
fn sequence_add() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 2"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 3"#)).unwrap();
    assert!(!engine.show().sequence(1).unwrap().cues().contains(&CueId(1)));
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(2)));
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(3)));
    assert!(!engine.show().sequence(1).unwrap().cues().contains(&CueId(4)));
}

#[test]
fn sequence_replace_at() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 2"#)).unwrap();
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(2)));
    assert!(!engine.show().sequence(1).unwrap().cues().contains(&CueId(4)));
    engine.exec_cmd(cmd!(r#"sequence 1 replace_at 0 cue 4"#)).unwrap();
    assert!(!engine.show().sequence(1).unwrap().cues().contains(&CueId(2)));
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(4)));
}

#[test]
fn sequence_remove() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 2"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 3"#)).unwrap();
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(2)));
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(3)));
    engine.exec_cmd(cmd!(r#"sequence 1 remove cue 2"#)).unwrap();
    assert!(!engine.show().sequence(1).unwrap().cues().contains(&CueId(2)));
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(3)));
}

#[test]
fn sequence_remove_at() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 2"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 3"#)).unwrap();
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(2)));
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(3)));
    engine.exec_cmd(cmd!(r#"sequence 1 remove_at 1"#)).unwrap();
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(2)));
    assert!(!engine.show().sequence(1).unwrap().cues().contains(&CueId(3)));
}

#[test]
fn sequence_clear() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create sequence 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 2"#)).unwrap();
    engine.exec_cmd(cmd!(r#"sequence 1 add cue 3"#)).unwrap();
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(2)));
    assert!(engine.show().sequence(1).unwrap().cues().contains(&CueId(3)));
    engine.exec_cmd(cmd!(r#"sequence 1 clear"#)).unwrap();
    assert!(engine.show().sequence(1).unwrap().cues().is_empty());
}

#[test]
fn cue_add() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create cue 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"cue 1 add fixture_group 1 preset::dimmer 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"cue 1 add fixture_group 2 preset::dimmer 2"#)).unwrap();
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 1.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(1.into())),
        level_effect: None,
    }));
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 2.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(2.into())),
        level_effect: None,
    }));
}

#[test]
fn cue_replace_at() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create cue 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"cue 1 add fixture_group 1 preset::dimmer 1"#)).unwrap();
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 1.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(1.into())),
        level_effect: None,
    }));
    engine.exec_cmd(cmd!(r#"cue 1 replace_at 0 fixture_group 2 preset::dimmer 2"#)).unwrap();
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 2.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(2.into())),
        level_effect: None,
    }));
}

#[test]
fn cue_remove_at() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create cue 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"cue 1 add fixture_group 1 preset::dimmer 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"cue 1 add fixture_group 2 preset::dimmer 2"#)).unwrap();
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 1.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(1.into())),
        level_effect: None,
    }));
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 2.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(2.into())),
        level_effect: None,
    }));
    engine.exec_cmd(cmd!(r#"cue 1 remove_at 1"#)).unwrap();
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 1.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(1.into())),
        level_effect: None,
    }));
    assert!(!engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 2.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(2.into())),
        level_effect: None,
    }));
}

#[test]
fn cue_clear() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create cue 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"cue 1 add fixture_group 1 preset::dimmer 1"#)).unwrap();
    engine.exec_cmd(cmd!(r#"cue 1 add fixture_group 2 preset::dimmer 2"#)).unwrap();
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 1.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(1.into())),
        level_effect: None,
    }));
    assert!(engine.show().cue(1).unwrap().recipes().contains(&Recipe {
        fixture_group: 2.into(),
        content: RecipeContent::Preset(AnyPresetId::Dimmer(2.into())),
        level_effect: None,
    }));
    engine.exec_cmd(cmd!(r#"cue 1 clear"#)).unwrap();
    assert!(engine.show().cue(1).unwrap().recipes().is_empty());
}

#[test]
fn preset_store_dimmer_universal() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1 "Test Preset""#)).unwrap();

    engine.exec_cmd(cmd!(r#"programmer attribute 1 Dimmer 0.25"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 2 Dimmer 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 3 ColorAdd_R 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"preset::dimmer 1 store universal"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer clear"#)).unwrap();

    assert_eq!(
        engine.show().preset_dimmer(1).unwrap().content(),
        &PresetContent::Universal({
            let mut p = UniversalPreset::new(FeatureGroup::Dimmer);
            p.set_attribute_value(Attribute::Dimmer, AttributeValue::new(0.25));
            p.set_attribute_value(Attribute::Dimmer, AttributeValue::new(0.50));
            p
        })
    );
}

#[test]
fn preset_store_color_universal() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::color 1 "Test Preset""#)).unwrap();

    engine.exec_cmd(cmd!(r#"programmer attribute 1 ColorAdd_R 0.25"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 2 ColorAdd_G 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 3 Dimmer 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"preset::color 1 store universal"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer clear"#)).unwrap();

    assert_eq!(
        engine.show().preset_color(1).unwrap().content(),
        &PresetContent::Universal({
            let mut p = UniversalPreset::new(FeatureGroup::Color);
            p.set_attribute_value(Attribute::ColorAddR, AttributeValue::new(0.25));
            p.set_attribute_value(Attribute::ColorAddG, AttributeValue::new(0.50));
            p
        })
    );
}

#[test]
fn preset_clear_universal() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1 "Test Preset""#)).unwrap();

    engine.exec_cmd(cmd!(r#"programmer attribute 1 Dimmer 0.25"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 2 Dimmer 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"preset::dimmer 1 store universal"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer clear"#)).unwrap();

    assert_eq!(
        engine.show().preset_dimmer(1).unwrap().content(),
        &PresetContent::Universal({
            let mut p = UniversalPreset::new(FeatureGroup::Dimmer);
            p.set_attribute_value(Attribute::Dimmer, AttributeValue::new(0.25));
            p.set_attribute_value(Attribute::Dimmer, AttributeValue::new(0.50));
            p
        })
    );

    engine.exec_cmd(cmd!(r#"preset::dimmer 1 clear"#)).unwrap();

    assert_eq!(
        engine.show().preset_dimmer(1).unwrap().content(),
        &PresetContent::Universal(UniversalPreset::new(FeatureGroup::Dimmer))
    );
}

#[test]
fn preset_store_dimmer_selective() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1 "Test Preset""#)).unwrap();

    engine.exec_cmd(cmd!(r#"programmer attribute 1 Dimmer 0.25"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 2 Dimmer 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 3 ColorAdd_R 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"preset::dimmer 1 store selective"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer clear"#)).unwrap();

    assert_eq!(
        engine.show().preset_dimmer(1).unwrap().content(),
        &PresetContent::Selective({
            let mut p = SelectivePreset::new(FeatureGroup::Dimmer);
            p.set_attribute_value(1.into(), Attribute::Dimmer, AttributeValue::new(0.25));
            p.set_attribute_value(2.into(), Attribute::Dimmer, AttributeValue::new(0.50));
            p
        })
    );
}

#[test]
fn preset_store_color_selective() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::color 1 "Test Preset""#)).unwrap();

    engine.exec_cmd(cmd!(r#"programmer attribute 1 ColorAdd_R 0.25"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 2 ColorAdd_G 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 3 Dimmer 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"preset::color 1 store selective"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer clear"#)).unwrap();

    assert_eq!(
        engine.show().preset_color(1).unwrap().content(),
        &PresetContent::Selective({
            let mut p = SelectivePreset::new(FeatureGroup::Color);
            p.set_attribute_value(1.into(), Attribute::ColorAddR, AttributeValue::new(0.25));
            p.set_attribute_value(2.into(), Attribute::ColorAddG, AttributeValue::new(0.50));
            p
        })
    );
}

#[test]
fn preset_clear_selective() {
    let mut engine = init_engine();
    engine.exec_cmd(cmd!(r#"create preset::dimmer 1 "Test Preset""#)).unwrap();

    engine.exec_cmd(cmd!(r#"programmer attribute 1 Dimmer 0.25"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer attribute 2 Dimmer 0.50"#)).unwrap();
    engine.exec_cmd(cmd!(r#"preset::dimmer 1 store selective"#)).unwrap();
    engine.exec_cmd(cmd!(r#"programmer clear"#)).unwrap();

    assert_eq!(
        engine.show().preset_dimmer(1).unwrap().content(),
        &PresetContent::Selective({
            let mut p = SelectivePreset::new(FeatureGroup::Dimmer);
            p.set_attribute_value(1.into(), Attribute::Dimmer, AttributeValue::new(0.25));
            p.set_attribute_value(2.into(), Attribute::Dimmer, AttributeValue::new(0.50));
            p
        })
    );

    engine.exec_cmd(cmd!(r#"preset::dimmer 1 clear"#)).unwrap();

    assert_eq!(
        engine.show().preset_dimmer(1).unwrap().content(),
        &PresetContent::Selective(SelectivePreset::new(FeatureGroup::Dimmer))
    );
}

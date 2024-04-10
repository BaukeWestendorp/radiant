use backstage::Show;
use dmx::DmxValue;

macro_rules! expect_ok {
    ($test_name:ident, $command:expr, |$show:ident| $($code:tt)*) => {
        #[tokio::test]
        async fn $test_name() {
            let mut $show = Show::from_showfile_str(include_str!("./show.json"))
                .await
                .unwrap();

            $show.execute_command_str($command).unwrap();

            $($code)*
        }
    };
    ($test_name:ident, |$show:ident| $($code:tt)*) => {
        #[tokio::test]
        async fn $test_name() {
            let mut $show = Show::from_showfile_str(include_str!("./show.json"))
                .await
                .unwrap();

            $($code)*
        }
    };
}

macro_rules! expect_error {
    ($test_name:ident, $command:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let mut show = Show::from_showfile_str(include_str!("./show.json"))
                .await
                .unwrap();

            let res = show.execute_command_str($command);
            assert!(res.is_err());
        }
    };
}

expect_ok!(clear, |show| {
    show.execute_command_str("select fixture 1").unwrap();
    assert!(show.is_fixture_selected(1));

    show.execute_command_str("select preset.color 1").unwrap();
    let fixture_changes = show.programmer_changes().get(&1).unwrap();
    assert_eq!(
        fixture_changes.get("ColorAdd_R").unwrap(),
        &DmxValue::new(255)
    );
    assert_eq!(
        fixture_changes.get("ColorAdd_G").unwrap(),
        &DmxValue::new(0)
    );
    assert_eq!(
        fixture_changes.get("ColorAdd_B").unwrap(),
        &DmxValue::new(0)
    );

    show.execute_command_str("clear").unwrap();
    assert!(show.selected_fixtures().is_empty());
    assert!(!show.programmer_changes().is_empty());

    show.execute_command_str("clear").unwrap();
    assert!(show.programmer_changes().is_empty());
});

expect_ok!(select_fixture, "select fixture 1", |show| {
    assert!(show.is_fixture_selected(1));
});

expect_ok!(select_group, "select group 1", |show| {
    assert_eq!(show.selected_fixtures(), show.group(1).unwrap().fixtures);
});

expect_error!(select_sequence, "select sequence 1");

expect_error!(select_cue, "select cue 1.1");

expect_ok!(select_preset_beam, |show| {
    show.execute_command_str("select group 1").unwrap();

    show.execute_command_str("select preset.beam 1").unwrap();
    let attributes = show
        .programmer_changes()
        .get(&show.group(1).unwrap().fixtures[0])
        .unwrap();
    assert_eq!(attributes.get("Effects1").unwrap(), &DmxValue::new(127));
});

expect_ok!(select_preset_color, |show| {
    show.execute_command_str("select group 1").unwrap();

    show.execute_command_str("select preset.color 1").unwrap();
    let attributes = show
        .programmer_changes()
        .get(&show.group(1).unwrap().fixtures[0])
        .unwrap();
    assert_eq!(attributes.get("ColorAdd_R").unwrap(), &DmxValue::new(255));
    assert_eq!(attributes.get("ColorAdd_G").unwrap(), &DmxValue::new(0));
    assert_eq!(attributes.get("ColorAdd_B").unwrap(), &DmxValue::new(0));
});

expect_ok!(select_preset_dimmer, |show| {
    show.execute_command_str("select group 1").unwrap();

    show.execute_command_str("select preset.dimmer 1").unwrap();
    let attributes = show
        .programmer_changes()
        .get(&show.group(1).unwrap().fixtures[0])
        .unwrap();
    assert_eq!(attributes.get("Dimmer").unwrap(), &DmxValue::new(127));
});

expect_ok!(select_preset_focus, |show| {
    show.execute_command_str("select group 1").unwrap();

    show.execute_command_str("select preset.focus 1").unwrap();
    let attributes = show
        .programmer_changes()
        .get(&show.group(1).unwrap().fixtures[0])
        .unwrap();
    assert_eq!(attributes.get("Zoom").unwrap(), &DmxValue::new(127));
});

expect_ok!(select_preset_gobo, |show| {
    show.execute_command_str("select group 1").unwrap();

    show.execute_command_str("select preset.gobo 1").unwrap();
    let attributes = show
        .programmer_changes()
        .get(&show.group(1).unwrap().fixtures[0])
        .unwrap();
    assert_eq!(attributes.get("Gobo1").unwrap(), &DmxValue::new(127));
});

expect_ok!(select_preset_position, |show| {
    show.execute_command_str("select group 1").unwrap();

    show.execute_command_str("select preset.position 1")
        .unwrap();
    let attributes = show
        .programmer_changes()
        .get(&show.group(1).unwrap().fixtures[0])
        .unwrap();
    assert_eq!(attributes.get("Pan").unwrap(), &DmxValue::new(32768));
});

expect_ok!(select_preset_all, |show| {
    show.execute_command_str("select group 1").unwrap();

    show.execute_command_str("select preset.all 1").unwrap();
    let attributes = show
        .programmer_changes()
        .get(&show.group(1).unwrap().fixtures[0])
        .unwrap();
    assert_eq!(attributes.get("Dimmer").unwrap(), &DmxValue::new(127));
    assert_eq!(attributes.get("Gobo1").unwrap(), &DmxValue::new(255));
});

expect_error!(select_executor, "select executor 1");

expect_error!(store_fixture, "store fixture 1");

expect_ok!(store_group, |show| {
    show.execute_command_str("select fixture 1").unwrap();

    show.execute_command_str("store group 100").unwrap();
    assert_eq!(show.group(100).unwrap().fixtures[0], 1);

    assert!(
        show.execute_command_str("store group 100").is_err(),
        "Can't store into existing group."
    );
});

expect_ok!(store_sequence, |show| {
    // Storing nothing into a sequence should be allowed
    show.execute_command_str("store sequence 200").unwrap();

    show.execute_command_str("select group 1").unwrap();
    show.execute_command_str("select preset.color 1").unwrap();

    assert!(show.sequence(100).is_none());

    show.execute_command_str("store sequence 100").unwrap();

    assert_eq!(
        show.sequence(100).unwrap().cues[0]
            .changes
            .get(&1)
            .unwrap()
            .get("ColorAdd_R")
            .unwrap(),
        &DmxValue::new(255)
    );

    assert!(show.sequence(100).unwrap().cues.get(2).is_none());

    show.execute_command_str("store sequence 100").unwrap();

    assert_eq!(
        show.sequence(100).unwrap().cues[1]
            .changes
            .get(&1)
            .unwrap()
            .get("ColorAdd_R")
            .unwrap(),
        &DmxValue::new(255)
    );
});

expect_ok!(store_cue, |show| {
    show.execute_command_str("select group 1").unwrap();
    show.execute_command_str("select preset.color 1").unwrap();
    show.execute_command_str("store sequence 1").unwrap();

    assert!(show.cue(2, 1).is_none());

    show.execute_command_str("store cue 2.1").unwrap();

    assert_eq!(
        show.cue(2, 1)
            .unwrap()
            .changes
            .get(&1)
            .unwrap()
            .get("ColorAdd_R")
            .unwrap(),
        &DmxValue::new(255)
    );

    assert!(show.cue(2, 2).is_none());

    show.execute_command_str("store cue 2.10").unwrap();
    assert_eq!(
        show.cue(2, 2)
            .unwrap()
            .changes
            .get(&1)
            .unwrap()
            .get("ColorAdd_R")
            .unwrap(),
        &DmxValue::new(255)
    );
});

// FIXME: Implement storing presets

expect_ok!(store_executor, |show| {
    show.execute_command_str("select group 1").unwrap();
    show.execute_command_str("select preset.color 1").unwrap();

    show.execute_command_str("store executor 1").unwrap();
    assert_eq!(
        show.cue(1, 2)
            .unwrap()
            .changes
            .get(&1)
            .unwrap()
            .get("ColorAdd_R")
            .unwrap(),
        &DmxValue::new(255)
    );

    assert!(show.executor(10).is_none());

    show.execute_command_str("store executor 10").unwrap();

    assert_eq!(
        show.cue(3, 0)
            .unwrap()
            .changes
            .get(&1)
            .unwrap()
            .get("ColorAdd_R")
            .unwrap(),
        &DmxValue::new(255)
    );
});

expect_ok!(go, |show| {
    // Non looping sequence
    assert_eq!(show.executor(1).unwrap().current_index.get(), None);
    show.execute_command_str("go executor 1").unwrap();
    assert_eq!(show.executor(1).unwrap().current_index.get(), Some(0));
    show.execute_command_str("go executor 1").unwrap();
    assert_eq!(show.executor(1).unwrap().current_index.get(), Some(1));
    show.execute_command_str("go executor 1").unwrap();
    assert_eq!(show.executor(1).unwrap().current_index.get(), None);

    // Looping sequence
    assert_eq!(show.executor(2).unwrap().current_index.get(), Some(1));
    show.execute_command_str("go executor 2").unwrap();
    assert_eq!(show.executor(2).unwrap().current_index.get(), Some(0));
    show.execute_command_str("go executor 2").unwrap();
    assert_eq!(show.executor(2).unwrap().current_index.get(), Some(1));
});

expect_ok!(top, |show| {
    assert_eq!(show.executor(1).unwrap().current_index.get(), None);
    show.execute_command_str("top executor 1").unwrap();
    assert_eq!(show.executor(1).unwrap().current_index.get(), Some(0));

    show.execute_command_str("go executor 1").unwrap();
    assert_eq!(show.executor(1).unwrap().current_index.get(), Some(1));

    show.execute_command_str("top executor 1").unwrap();
    assert_eq!(show.executor(1).unwrap().current_index.get(), Some(0));

    assert!(show.execute_command_str("top cue 1").is_err());
});

use backstage::{
    dmx::DmxChannel,
    show::{AttributeValue, FixtureId, Show},
};

#[test]
fn test_set_get_attribute() {
    let mut show = init_test_show();
    let fixture_id = patch_test_fixture(&mut show);

    let fixture = show.patchlist().fixture(&fixture_id).unwrap().clone();
    show.programmer_mut()
        .set_attribute_value(&fixture, "Dimmer".to_string(), AttributeValue::new(0.5))
        .unwrap();

    let attribute_value = show
        .programmer()
        .get_attribute_value(&fixture_id, "Dimmer")
        .unwrap();
    assert_eq!(attribute_value.value(), 0.5);
}

#[test]
fn test_dmx_output() {
    let mut show = init_test_show();
    let fixture_id = patch_test_fixture(&mut show);

    let fixture = show.patchlist().fixture(&fixture_id).unwrap().clone();
    show.programmer_mut()
        .set_attribute_value(&fixture, "Dimmer".to_string(), AttributeValue::new(0.5))
        .unwrap();

    let dmx_output = show.programmer().get_dmx_output();
    assert_eq!(
        dmx_output.get_value(&DmxChannel::new(0, 1).unwrap()),
        Some(128)
    );
}

fn init_test_show() -> Show {
    std::env::set_var("BACKSTAGE_FIXTURE_CACHE_LOCATION", "tests/fixture_cache");
    let mut show = Show::new();
    smol::block_on(show.initialize(
        std::env::var("TEST_GDTF_SHARE_USER".to_string()).unwrap(),
        std::env::var("TEST_GDTF_SHARE_PASSWORD".to_string()).unwrap(),
    ))
    .unwrap();
    show
}

fn patch_test_fixture(show: &mut Show) -> FixtureId {
    let fixture_id = FixtureId::new(0);
    smol::block_on(show.patchlist_mut().patch_fixture(
        fixture_id,
        "Test Fixture".to_string(),
        1,
        DmxChannel::new(0, 0).unwrap(),
        "Basic".to_string(),
    ))
    .unwrap();
    fixture_id
}

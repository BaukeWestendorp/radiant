use std::str::FromStr;

use gdtf::{DataVersion, DmxBreak, DmxValue, GdtfDescription};

#[test]
fn parse_full_gdtf_description() {
    let description =
        GdtfDescription::from_str(include_str!("../tests/test_description.xml")).unwrap();

    assert_eq!(
        description.data_version,
        DataVersion {
            major: 42,
            minor: 66
        }
    );

    let fixture_type = description.fixture_type;
    assert_eq!(fixture_type.name, "Fixture Type Name");
    assert_eq!(fixture_type.short_name, "Fixture Type Short Name");
    assert_eq!(fixture_type.long_name, "Fixture Type Long Name");
    assert_eq!(fixture_type.manufacturer, "Test Manufacturer");
    assert_eq!(fixture_type.description, "Test Description");
    assert_eq!(
        fixture_type.fixture_type_id,
        "AE92BE76-BDDA-4432-BDAA-06AD46F01BF3"
    );
    assert_eq!(fixture_type.thumbnail, Some("thumbnail".to_string()));
    assert_eq!(fixture_type.thumbnail_offset_x, 90);
    assert_eq!(fixture_type.thumbnail_offset_y, 78);
    assert_eq!(fixture_type.ref_ft, None);
    assert_eq!(fixture_type.can_have_children, false);

    let dmx_mode = fixture_type.dmx_modes.get(0).unwrap();
    assert_eq!(dmx_mode.name, "Basic");
    assert_eq!(dmx_mode.description, "DMX Mode Description");

    let dmx_channel = dmx_mode.dmx_channels.get(0).unwrap();
    assert_eq!(dmx_channel.dmx_break, DmxBreak::Value(42));
    assert_eq!(dmx_channel.offset, Some(vec![1, 2]));
    assert_eq!(
        dmx_channel.highlight,
        Some(DmxValue::from_str("255/1").unwrap())
    );

    let logical_channel = dmx_channel.logical_channels.get(0).unwrap();
    assert_eq!(
        logical_channel.attribute,
        fixture_type.attribute_definitions.attributes[0]
    );

    let channel_function = logical_channel.channel_functions.get(0).unwrap();
    assert_eq!(
        channel_function.attribute,
        Some(
            fixture_type
                .attribute_definitions
                .attributes
                .get(0)
                .unwrap()
                .clone()
        )
    );
    assert_eq!(
        channel_function.dmx_from,
        DmxValue::from_str("0/1").unwrap()
    );
    assert_eq!(
        channel_function.default,
        DmxValue::from_str("255/1").unwrap()
    );
    assert_eq!(channel_function.name, "Red");
}

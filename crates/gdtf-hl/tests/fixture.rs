use gdtf::GdtfFile;
use gdtf_hl::{Attribute, AttributeValue, DmxMode, Fixture, FixtureId};
use std::io::Cursor;

#[test]
fn get_fixture_info() {
    let file_bytes = Cursor::new(include_bytes!("./test_fixture.gdtf"));
    let gdtf = GdtfFile::new(file_bytes).expect("failed to parse gdtf");
    let f = Fixture::new(FixtureId(0), DmxMode::new("Basic"), &gdtf.description.fixture_types[0])
        .expect("failed to create fixture");

    assert_eq!(f.id(), FixtureId(0));

    assert_eq!(f.attribute_info(&Attribute::Dimmer).unwrap().value(), AttributeValue::new(0.0));
    assert_eq!(
        f.attribute_info(&Attribute::Dimmer).unwrap().highlight_value(),
        Some(AttributeValue::new(1.0))
    );
    assert_eq!(f.attribute_info(&Attribute::Pan).unwrap().value(), AttributeValue::new(0.5000076));
    assert_eq!(
        f.attribute_info(&Attribute::Pan).unwrap().default_value(),
        AttributeValue::new(0.5000076)
    );
    assert_eq!(f.attribute_info(&Attribute::Pan).unwrap().highlight_value(), None,);
    assert_eq!(
        f.attribute_info(&Attribute::Shutter(1)).unwrap().value(),
        AttributeValue::new(0.08627451)
    );
}

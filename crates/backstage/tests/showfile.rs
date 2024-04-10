use backstage::Show;

#[tokio::test]
async fn from_empty_showfile() {
    let show = Show::from_showfile_str(include_str!("./show.json")).await;
    assert!(show.is_ok());
}

#[tokio::test]
async fn from_showfile_with_fixture() {
    let show = Show::from_showfile_str(include_str!("./show.json"))
        .await
        .unwrap();

    assert_eq!(
        show.fixture(1).unwrap().description.fixture_type.name,
        "Pixel Tube 16 RGB",
    );
}

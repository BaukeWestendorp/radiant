use std::env;

use backstage::show::{FixtureId, Show};

#[test]
fn deserialize_show() {
    env::set_var(
        "BACKSTAGE_FIXTURE_CACHE_LOCATION",
        env::current_dir().unwrap().join("tests/fixture_cache"),
    );
    let json = r#"{
        "patchlist": {
            "fixtures": [
                {
                    "id": 1,
                    "label": "Test",
                    "revision_id": 1,
                    "channel": [1, 2],
                    "mode": "Mode"
                }
            ]
        }
    }"#;
    let show: Show = serde_json::from_str(json).unwrap();
    assert!(show.patchlist().fixture(&FixtureId::new(1)).is_some())
}

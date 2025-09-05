use satisflow::*;

#[test]
fn save_envelope_serialization_contains_version() {
    let mut t = ProductionTracker::new();
    let json = engine::to_save_json(&t).expect("serialize");
    assert!(json.contains("\"version\": \"1\""));
    // Round trip
    let t2 = engine::from_save_json(&json).expect("deserialize");
    assert_eq!(t.factories.len(), t2.factories.len());
}


use satisflow::*;

#[test]
fn logistics_id_with_detail() {
    let t = ProductionTracker::new();
    let id = t.generate_logistics_id_with_detail(&TransportType::Conveyor, Some("C03"));
    assert!(id.0.starts_with("LG-BUS-"));
    assert!(id.0.ends_with("-C03"));
}


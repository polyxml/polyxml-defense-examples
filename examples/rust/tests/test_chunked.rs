//! Demonstration and verification of consuming topologically chunked modules.
//!
//! When `split_units = true` is configured in `polyxml.toml` or `--split-units`
//! is passed via CLI, PolyXML generates bounded, acyclic chunk submodules
//! (`chunk_00.rs`, `chunk_01.rs`, ...) with a parent `mod.rs` that re-exports all types.
//! Downstream code simply imports `mod.rs` and has access to all types seamlessly.

#[path = "../../../generated/rust_chunked/mod.rs"]
mod uci_chunked;

use std::borrow::Cow;

#[test]
fn test_chunked_reexports_and_functionality() {
    use uci_chunked::*;

    // Types defined in chunk_00 (Enums)
    let class = ClassificationEnum::Unclassified;
    assert_eq!(class.as_str(), "UNCLASSIFIED");

    // Types defined in chunk_01 (Leaf structs)
    let sec = SecurityInformationType {
        classification: class,
        owner_producer: Some(Cow::Borrowed("USA")),
    };
    assert_eq!(sec.owner_producer.as_deref(), Some("USA"));

    // Types defined in chunk_02 (Root struct referencing chunk_00 and chunk_01)
    let entity = EntityMt {
        security_information: sec,
        message_header: HeaderType {
            message_id: Cow::Borrowed("MSG-001"),
            timestamp: Cow::Borrowed("2026-09-30T12:00:00Z"),
            originator_id: Cow::Borrowed("DRONE-01"),
        },
        object_state: Some(ObjectStateEnum::Active),
        message_data: EntityMdt {
            entity_id: EntityIdType {
                uuid: Cow::Borrowed("123e4567-e89b-12d3-a456-426614174000"),
                callsign: Some(Cow::Borrowed("ALPHA-1")),
            },
            creation_timestamp: Cow::Borrowed("2026-09-30T12:00:00Z"),
            entity_status: EntityStatusEnum::Confirmed,
            kinematics: KinematicsType {
                latitude: 37.7749,
                longitude: -122.4194,
                altitude: 1000.0,
                heading: Some(180.0),
                ground_speed: Some(50.0),
                vertical_speed: None,
                airspeed: None,
                pitch: None,
                roll: None,
            },
            source_system: Some(Cow::Borrowed("SENSOR-A")),
            flight_mode: None,
            active_waypoint: None,
            fuel_percentage: Some(85.0),
        },
    };

    assert_eq!(entity.object_state, Some(ObjectStateEnum::Active));
    assert_eq!(
        entity.message_data.source_system.as_deref(),
        Some("SENSOR-A")
    );
}

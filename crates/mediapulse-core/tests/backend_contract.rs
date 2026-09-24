#![cfg(feature = "libmpv")]

use mediapulse_core::backend::{
    BackendKind, LibmpvBackend, PlaybackBackend, PropertyUpdate, PropertyValue,
};

#[test]
fn libmpv_backend_reports_its_kind() {
    let backend = LibmpvBackend::new().expect("libmpv backend");

    assert_eq!(backend.backend_kind(), BackendKind::Libmpv);
}

#[test]
fn libmpv_backend_round_trips_ui_owned_properties() {
    let backend = LibmpvBackend::new().expect("libmpv backend");

    backend
        .set_property(PropertyUpdate::boolean("pause", true))
        .expect("set pause");
    backend
        .set_property(PropertyUpdate::number("volume", 80.0))
        .expect("set volume");
    backend
        .set_property(PropertyUpdate::number("speed", 1.5))
        .expect("set speed");

    assert_eq!(
        backend.get_property("pause").expect("get pause"),
        PropertyValue::Boolean(true)
    );
    assert_eq!(
        backend.get_property("volume").expect("get volume"),
        PropertyValue::Number(80.0)
    );

    let snapshot = backend.snapshot().expect("playback snapshot");
    assert!(snapshot.paused);
    assert!((snapshot.volume - 80.0).abs() < f64::EPSILON);
    assert!((snapshot.speed - 1.5).abs() < f64::EPSILON);
}

#[test]
fn libmpv_backend_reports_invalid_properties_as_typed_errors() {
    let backend = LibmpvBackend::new().expect("libmpv backend");

    let result = backend.get_property("definitely-not-an-mpv-property");

    assert!(result.is_err());
}

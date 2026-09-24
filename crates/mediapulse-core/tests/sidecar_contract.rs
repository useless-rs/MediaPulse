#![cfg(feature = "sidecar")]

use std::sync::Arc;

use mediapulse_core::backend::{PlaybackBackend, SidecarBackend};

fn assert_object_safe_backend<T: PlaybackBackend>() {}

#[test]
fn sidecar_backend_implements_the_shared_playback_contract() {
    assert_object_safe_backend::<SidecarBackend>();
    let _: Option<Arc<dyn PlaybackBackend>> = None;
}

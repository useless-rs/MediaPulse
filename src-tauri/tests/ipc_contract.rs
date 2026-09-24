use mediapulse_core::backend::BackendError;
use mediapulse_lib::IpcError;

#[test]
fn ipc_errors_keep_stable_machine_codes() {
    let error = IpcError::from(BackendError::NotReady);

    assert_eq!(error.code, "notReady");
    assert_eq!(
        serde_json::to_value(error).expect("serialize error")["code"],
        "notReady"
    );
}

use super::*;
use crate::services::instance_manager::signal_kill;

#[test]
fn only_unsolicited_nonzero_exits_are_crashes() {
    for (exit_code, kill_requested, expected) in [
        (None, false, false),
        (None, true, false),
        (Some(0), false, false),
        (Some(0), true, false),
        (Some(1), false, true),
        (Some(1), true, false),
        (Some(-1), false, true),
        (Some(-1), true, false),
        (Some(-1073741819), false, true),
        (Some(-1073741819), true, false),
    ] {
        assert_eq!(
            is_unexpected_exit(exit_code, kill_requested),
            expected,
            "exit_code={exit_code:?}, kill_requested={kill_requested}"
        );
    }
}

#[test]
fn an_exit_observed_before_receiving_kill_is_still_intentional() {
    let id = uuid::Uuid::new_v4().to_string();
    let (tx, _rx) = tokio::sync::oneshot::channel();
    let requested = register_kill_sender(&id, tx);
    assert!(signal_kill(&id));
    unregister_kill_sender(&id);

    // Simulate wait() completing without the kill branch consuming its signal.
    assert!(!is_unexpected_exit(
        Some(1),
        requested.load(Ordering::Acquire)
    ));
}

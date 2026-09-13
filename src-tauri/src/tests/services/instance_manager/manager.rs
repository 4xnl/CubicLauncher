use super::*;

#[test]
fn kill_request_survives_cleanup_before_the_signal_is_received() {
    let id = uuid::Uuid::new_v4().to_string();
    let (tx, mut rx) = oneshot::channel();
    let requested = register_kill_sender(&id, tx);

    assert!(!requested.load(Ordering::Acquire));
    assert!(signal_kill(&id));
    unregister_kill_sender(&id);
    assert!(requested.load(Ordering::Acquire));
    assert_eq!(rx.try_recv(), Ok(()));
    assert!(!signal_kill(&id));
}

#[test]
fn unregistering_does_not_count_as_a_kill_request() {
    let id = uuid::Uuid::new_v4().to_string();
    let (tx, mut rx) = oneshot::channel();
    let requested = register_kill_sender(&id, tx);

    unregister_kill_sender(&id);
    assert!(!requested.load(Ordering::Acquire));
    assert_eq!(rx.try_recv(), Err(oneshot::error::TryRecvError::Closed));
    assert!(!signal_kill(&id));
}

#[test]
fn a_new_execution_does_not_inherit_a_previous_kill_request() {
    let id = uuid::Uuid::new_v4().to_string();
    let (old_tx, _old_rx) = oneshot::channel();
    let old_requested = register_kill_sender(&id, old_tx);
    assert!(signal_kill(&id));

    let (new_tx, _new_rx) = oneshot::channel();
    let new_requested = register_kill_sender(&id, new_tx);
    unregister_kill_sender(&id);
    assert!(old_requested.load(Ordering::Acquire));
    assert!(!new_requested.load(Ordering::Acquire));
}

#[test]
fn signaling_a_closed_receiver_returns_false() {
    let id = uuid::Uuid::new_v4().to_string();
    let (tx, rx) = oneshot::channel();
    register_kill_sender(&id, tx);
    drop(rx);
    assert!(!signal_kill(&id));
}

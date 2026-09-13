use super::*;

#[test]
fn test_atomic_status_off() {
    let s = AtomicStatus::new();
    assert_eq!(s.get(), InstanceStatus::Off);
}

#[test]
fn test_atomic_status_starting() {
    let s = AtomicStatus::new();
    s.set(InstanceStatus::Starting);
    assert_eq!(s.get(), InstanceStatus::Starting);
}

#[test]
fn test_atomic_status_started() {
    let s = AtomicStatus::new();
    s.set(InstanceStatus::Started);
    assert_eq!(s.get(), InstanceStatus::Started);
}

#[test]
fn test_atomic_status_error() {
    let s = AtomicStatus::new();
    s.set(InstanceStatus::Error("something went wrong".into()));
    assert_eq!(
        s.get(),
        InstanceStatus::Error("something went wrong".into())
    );
}

#[test]
fn test_atomic_status_cycle() {
    let s = AtomicStatus::new();
    assert_eq!(s.get(), InstanceStatus::Off);
    s.set(InstanceStatus::Starting);
    assert_eq!(s.get(), InstanceStatus::Starting);
    s.set(InstanceStatus::Off);
    assert_eq!(s.get(), InstanceStatus::Off);
}

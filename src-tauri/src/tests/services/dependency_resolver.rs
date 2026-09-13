use super::*;

#[test]
fn dependency_kind_from_modrinth() {
    assert_eq!(
        DependencyKind::from_modrinth("required"),
        DependencyKind::Required
    );
    assert_eq!(
        DependencyKind::from_modrinth("optional"),
        DependencyKind::Optional
    );
    assert_eq!(
        DependencyKind::from_modrinth("embedded"),
        DependencyKind::Embedded
    );
    assert_eq!(
        DependencyKind::from_modrinth("incompatible"),
        DependencyKind::Incompatible
    );
    assert_eq!(
        DependencyKind::from_modrinth("unknown"),
        DependencyKind::Required
    );
    assert_eq!(
        DependencyKind::from_modrinth("OPTIONAL"),
        DependencyKind::Optional
    );
}

#[test]
fn dependency_kind_from_curseforge_relation() {
    assert_eq!(
        DependencyKind::from_curseforge_relation(1),
        Some(DependencyKind::Embedded)
    );
    assert_eq!(
        DependencyKind::from_curseforge_relation(2),
        Some(DependencyKind::Optional)
    );
    assert_eq!(
        DependencyKind::from_curseforge_relation(3),
        Some(DependencyKind::Required)
    );
    assert_eq!(
        DependencyKind::from_curseforge_relation(5),
        Some(DependencyKind::Incompatible)
    );
    assert_eq!(DependencyKind::from_curseforge_relation(99), None);
}

#[test]
fn record_version_first_insert() {
    let mut visited = HashMap::new();
    let conflict = record_version(
        DependencySource::Modrinth,
        "abc123",
        "v1",
        "root",
        &mut visited,
    );

    assert!(conflict.is_none());
    assert_eq!(
        visited.get(&(DependencySource::Modrinth, "abc123".to_string())),
        Some(&"v1".to_string())
    );
}

#[test]
fn record_version_same_version_is_no_conflict() {
    let mut visited = HashMap::new();
    visited.insert(
        (DependencySource::Modrinth, "abc123".to_string()),
        "v1".to_string(),
    );

    let conflict = record_version(
        DependencySource::Modrinth,
        "abc123",
        "v1",
        "dep-a",
        &mut visited,
    );

    assert!(conflict.is_none());
}

#[test]
fn record_version_different_version_is_conflict() {
    let mut visited = HashMap::new();
    visited.insert(
        (DependencySource::Curseforge, "98765".to_string()),
        "file-1".to_string(),
    );

    let conflict = record_version(
        DependencySource::Curseforge,
        "98765",
        "file-2",
        "dep-b",
        &mut visited,
    );

    assert!(conflict.is_some());
    let conflict = conflict.unwrap();
    assert_eq!(conflict.project_id, "98765");
    assert_eq!(conflict.source, DependencySource::Curseforge);
    assert_eq!(conflict.requested_versions.len(), 2);
    assert_eq!(conflict.requested_versions[0].version_id, "file-1");
    assert_eq!(conflict.requested_versions[1].version_id, "file-2");
    assert_eq!(conflict.requested_versions[1].requested_by, "dep-b");
}

#[test]
fn record_version_tracks_sources_independently() {
    let mut visited = HashMap::new();

    record_version(
        DependencySource::Modrinth,
        "abc123",
        "v1",
        "root",
        &mut visited,
    );
    let conflict = record_version(
        DependencySource::Curseforge,
        "abc123",
        "file-1",
        "root",
        &mut visited,
    );

    assert!(conflict.is_none());
    assert_eq!(visited.len(), 2);
}

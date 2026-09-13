use super::*;

#[test]
fn rule_without_os_allows() {
    let r: Rule = serde_json::from_str(r#"{"action":"allow"}"#).unwrap();
    assert!(r.action_if_matches().is_some());
}

#[test]
fn native_detection_by_path() {
    let lib: Library = serde_json::from_str(
        r#"{
            "name": "org.lwjgl:lwjgl:3.4.1:natives-linux",
            "downloads": {
                "artifact": {
                    "path": "org/lwjgl/lwjgl/3.4.1/lwjgl-3.4.1-natives-linux.jar",
                    "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.4.1/lwjgl-3.4.1-natives-linux.jar",
                    "sha1": "abc123",
                    "size": 12345
                }
            },
            "rules": [{"action":"allow","os":{"name":"linux"}}]
        }"#,
    )
    .unwrap();
    assert!(lib.is_native());
}

#[test]
#[cfg(target_os = "linux")]
fn allow_linux() {
    let r: Rule = serde_json::from_str(r#"{"action":"allow","os":{"name":"linux"}}"#).unwrap();
    assert!(r.evaluate());
}

#[test]
#[cfg(not(target_os = "linux"))]
fn disallow_linux_on_non_linux() {
    let r: Rule = serde_json::from_str(r#"{"action":"allow","os":{"name":"linux"}}"#).unwrap();
    assert!(!r.evaluate());
}

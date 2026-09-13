use super::*;

fn manifest(modern: bool) -> VersionManifest {
    let mut value = serde_json::json!({"id":"1.21.1"});
    if modern {
        value["arguments"] = serde_json::json!({"game":[
            "--username", "${auth_player_name}",
            {"rules":[{"action":"allow","features":{"is_quick_play_multiplayer":true}}],"value":["--quickPlayMultiplayer","${quickPlayMultiplayer}"]},
            {"rules":[{"action":"allow","features":{"is_quick_play_singleplayer":true}}],"value":["--quickPlaySingleplayer","${quickPlaySingleplayer}"]}
        ]});
    } else {
        value["minecraftArguments"] = serde_json::json!("--username ${auth_player_name}");
    }
    VersionManifest::from_bytes(&serde_json::to_vec(&value).unwrap()).unwrap()
}

fn game_args(manifest: &VersionManifest, config: &LaunchConfig) -> Vec<String> {
    let builder = CommandBuilder::new(manifest, Path::new("."), Path::new("."), config);
    let mut args = Vec::new();
    builder.add_game_args(
        &mut args,
        &HashMap::from([("auth_player_name".into(), "Player".into())]),
        manifest,
    );
    builder.cleanup_unresolved(&mut args);
    builder.add_optional_args(&mut args, manifest);
    args
}

#[test]
fn modern_server_launch_has_one_resolved_target_and_no_other_quick_play_flags() {
    let config = LaunchConfig::builder()
        .quick_play(crate::QuickPlay::Multiplayer("play.example:25565".into()))
        .legacy_server("srv.example", 25566)
        .build();
    assert_eq!(
        game_args(&manifest(true), &config),
        [
            "--username",
            "Player",
            "--quickPlayMultiplayer",
            "play.example:25565"
        ]
    );
}

#[test]
fn legacy_server_launch_uses_resolved_host_and_port() {
    let config = LaunchConfig::builder()
        .quick_play(crate::QuickPlay::Multiplayer("play.example:25565".into()))
        .legacy_server("srv.example", 25566)
        .build();
    assert_eq!(
        game_args(&manifest(false), &config),
        [
            "--username",
            "Player",
            "--server",
            "srv.example",
            "--port",
            "25566"
        ]
    );
}

#[test]
fn normal_launch_does_not_join_any_server() {
    assert_eq!(
        game_args(&manifest(true), &LaunchConfig::default()),
        ["--username", "Player"]
    );
    assert_eq!(
        game_args(&manifest(false), &LaunchConfig::default()),
        ["--username", "Player"]
    );
}

#[test]
fn loader_inherits_quick_play_support_from_parent() {
    let child = VersionManifest::from_bytes(br#"{"id":"fabric-loader-0.16.0-1.21.1","inheritsFrom":"1.21.1","arguments":{"game":["--loader-arg"]}}"#).unwrap();
    let resolved = child.resolve(&manifest(true));
    assert!(supports_multiplayer_quick_play(&resolved));
    let config = LaunchConfig::builder()
        .quick_play(crate::QuickPlay::Multiplayer("[::1]:25565".into()))
        .legacy_server("::1", 25565)
        .build();
    let args = game_args(&resolved, &config);
    assert_eq!(
        args.iter()
            .filter(|arg| *arg == "--quickPlayMultiplayer")
            .count(),
        1
    );
    assert!(args.contains(&"[::1]:25565".to_string()));
    assert!(!args.iter().any(|arg| arg.contains("${")));
}

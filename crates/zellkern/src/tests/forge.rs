use super::*;

#[test]
fn simple_maven_coord() {
    let path = maven_to_path("org.ow2.asm:asm:9.9.1");
    assert_eq!(path, PathBuf::from("org/ow2/asm/asm/9.9.1/asm-9.9.1.jar"));
}

#[test]
fn maven_coord_with_classifier() {
    let path = maven_to_path("net.minecraftforge:forge:26.1.2-64.0.8:client");
    assert_eq!(
        path,
        PathBuf::from("net/minecraftforge/forge/26.1.2-64.0.8/forge-26.1.2-64.0.8-client.jar")
    );
}

#[test]
fn maven_coord_with_extension_override() {
    let path = maven_to_path("de.oceanlabs.mcp:mcp_config:1.12.2-20200226.224830@zip");
    assert_eq!(
        path,
        PathBuf::from(
            "de/oceanlabs/mcp/mcp_config/1.12.2-20200226.224830/mcp_config-1.12.2-20200226.224830.zip"
        )
    );
}

#[test]
fn parse_simple_coord() {
    let (g, a, v, c, e) = parse_maven_coord("org.ow2.asm:asm:9.9.1");
    assert_eq!(g, "org.ow2.asm");
    assert_eq!(a, "asm");
    assert_eq!(v, "9.9.1");
    assert_eq!(c, None);
    assert_eq!(e, "jar");
}

#[test]
fn parse_coord_with_classifier() {
    let (g, a, v, c, e) = parse_maven_coord("net.minecraftforge:forge:26.1.2-64.0.8:universal");
    assert_eq!(g, "net.minecraftforge");
    assert_eq!(a, "forge");
    assert_eq!(v, "26.1.2-64.0.8");
    assert_eq!(c, Some("universal".into()));
    assert_eq!(e, "jar");
}

#[test]
fn parse_coord_with_extension() {
    let (_g, _a, _v, c, e) = parse_maven_coord("de.oceanlabs.mcp:mcp_config:1.12.2@zip");
    assert_eq!(e, "zip");
    assert_eq!(c, None);
}

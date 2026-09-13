use super::*;

/// Una URL que comienza con `https://` debe ser aceptada.
#[test]
fn test_open_url_https() {
    assert!(open_url("https://example.com".into()).is_ok());
}

/// Una URL que comienza con `http://` también debe ser aceptada.
#[test]
fn test_open_url_http() {
    assert!(open_url("http://example.com".into()).is_ok());
}

/// Cualquier otro protocolo (ej. `ftp://`) debe ser rechazado por seguridad.
#[test]
fn test_open_url_ftp_rejected() {
    assert!(open_url("ftp://example.com".into()).is_err());
}

/// Una URL vacía no es válida.
#[test]
fn test_open_url_empty() {
    assert!(open_url(String::new()).is_err());
}

/// Una URL sin protocolo debe ser rechazada.
/// `open_url` solo acepta http/https explícitos.
#[test]
fn test_open_url_no_protocol() {
    assert!(open_url("example.com".into()).is_err());
}

pub mod auth;
pub mod core;
pub mod download;
pub mod fs;
pub mod instance;

pub use auth::AuthError;
pub use core::CoreError;
pub use download::DownloadError;
pub use fs::FsError;
pub use instance::InstanceError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Instance(#[from] InstanceError),

    #[error(transparent)]
    CoreError(#[from] CoreError),

    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Download(#[from] DownloadError),

    #[error(transparent)]
    Fs(#[from] FsError),
}

fn json_error(code: &str, params: &[(&str, String)]) -> String {
    if params.is_empty() {
        return format!(r#"{{"code":"{}"}}"#, code);
    }
    let inner: Vec<String> = params
        .iter()
        .map(|(k, v)| {
            format!(
                r#""{}":"{}""#,
                k,
                v.replace('\\', "\\\\").replace('"', "\\\"")
            )
        })
        .collect();
    format!(r#"{{"code":"{}","params":{{{}}}}}"#, code, inner.join(","))
}

impl AppError {
    pub fn to_json(&self) -> String {
        let (code, params) = match self {
            Self::Instance(e) => (e.code(), e.params()),
            Self::CoreError(e) => (e.code(), e.params()),
            Self::Auth(e) => (e.code(), e.params()),
            Self::Download(e) => (e.code(), e.params()),
            Self::Fs(e) => (e.code(), e.params()),
        };
        json_error(code, &params)
    }
}

impl From<AppError> for String {
    fn from(e: AppError) -> String {
        e.to_json()
    }
}

impl From<InstanceError> for String {
    fn from(e: InstanceError) -> String {
        json_error(e.code(), &e.params())
    }
}

impl From<CoreError> for String {
    fn from(e: CoreError) -> String {
        json_error(e.code(), &e.params())
    }
}

impl From<AuthError> for String {
    fn from(e: AuthError) -> String {
        json_error(e.code(), &e.params())
    }
}

impl From<DownloadError> for String {
    fn from(e: DownloadError) -> String {
        json_error(e.code(), &e.params())
    }
}

impl From<FsError> for String {
    fn from(e: FsError) -> String {
        json_error(e.code(), &e.params())
    }
}

#[cfg(test)]
#[path = "../../tests/core/errors.rs"]
mod tests;

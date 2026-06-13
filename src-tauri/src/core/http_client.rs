use std::sync::LazyLock;
use std::time::Duration;

pub const USER_AGENT: &str = "CubicLauncher/27.0.1";

pub static HTTP: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(8)
        .build()
        .expect("Failed to create HTTP client")
});

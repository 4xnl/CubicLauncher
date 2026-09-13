use tauri::WebviewUrl;
use tauri::utils::config::{Config, WindowConfig};

pub(crate) fn secondary_window_config(
    config: &Config,
    label: &str,
    url: WebviewUrl,
) -> Result<WindowConfig, String> {
    let main = config
        .app
        .windows
        .iter()
        .find(|window| window.label == "main")
        .ok_or_else(|| "Missing main window configuration".to_string())?;

    // WebView2 rejects different environment options in the same data directory.
    // Inherit those options, not main's size, visibility or other window settings.
    Ok(WindowConfig {
        label: label.to_string(),
        url,
        additional_browser_args: main.additional_browser_args.clone(),
        browser_extensions_enabled: main.browser_extensions_enabled,
        scroll_bar_style: main.scroll_bar_style.clone(),
        ..Default::default()
    })
}

#[cfg(test)]
#[path = "../tests/core/webview.rs"]
mod tests;

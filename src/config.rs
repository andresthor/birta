use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub port: Option<u16>,
    pub no_open: Option<bool>,
    pub css: Option<PathBuf>,
    pub theme: Option<String>,
    #[serde(default)]
    pub theme_overrides: ThemeOverrides,
}

#[derive(Debug, Default, Deserialize)]
pub struct ThemeOverrides {
    pub syntax: Option<PathBuf>,
    pub body_css: Option<PathBuf>,
}

/// Load config from `~/.config/sheen/config.toml` if it exists.
pub fn load() -> Config {
    config_path()
        .and_then(|path| std::fs::read_to_string(&path).ok())
        .and_then(|contents| match toml::from_str(&contents) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!("sheen: warning: failed to parse config: {e}");
                None
            }
        })
        .unwrap_or_default()
}

fn config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|d| d.join(".config").join("sheen").join("config.toml"))
}

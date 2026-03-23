use std::path::{Path, PathBuf};

use syntect::highlighting::ThemeSet;

use crate::config::Config;

/// A fully resolved theme ready for rendering.
pub struct ResolvedTheme {
    /// Display name.
    pub name: String,
    /// If Some, syntect uses inline styles. If None, CSS class mode (GitHub default).
    pub syntax: Option<SyntaxTheme>,
    /// Additional CSS for body/page/alert variable overrides.
    pub body_css: String,
    /// Whether the dark/light toggle should be shown.
    pub toggle: bool,
    /// Whether this is a light theme (only relevant for fixed-mode themes).
    pub is_light: bool,
}

#[derive(Clone)]
pub struct SyntaxTheme {
    pub theme: syntect::highlighting::Theme,
    pub theme_name: String,
}

/// Return the themes directory: `~/.config/sheen/themes/`.
fn themes_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|d| d.join(".config").join("sheen").join("themes"))
}

/// List installed theme names (directory names under `~/.config/sheen/themes/`).
pub fn list_installed() -> Vec<String> {
    let Some(dir) = themes_dir() else {
        return vec![];
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };
    let mut names: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    names.sort();
    names
}

/// Resolve a theme from CLI/config settings into a `ResolvedTheme`.
pub fn resolve(
    config: &Config,
    cli_theme: Option<&str>,
    cli_syntax: Option<&Path>,
) -> anyhow::Result<ResolvedTheme> {
    let theme_name = cli_theme.map(String::from).or_else(|| config.theme.clone());

    let syntax_override = cli_syntax
        .map(Path::to_path_buf)
        .or_else(|| config.theme_overrides.syntax.clone());
    let body_css_override = config.theme_overrides.body_css.clone();

    // No theme requested — GitHub default (with optional overrides)
    let Some(name) = theme_name else {
        return resolve_default(&syntax_override, &body_css_override);
    };

    // Try as an installed theme name (~/.config/sheen/themes/<name>/)
    if let Some(dir) = themes_dir() {
        let theme_dir = dir.join(&name);
        if theme_dir.is_dir() {
            return resolve_directory(&name, &theme_dir, &syntax_override, &body_css_override);
        }
    }

    // Try as a path to a theme directory
    let path = Path::new(&name);
    if path.is_dir() {
        let dir_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "custom".to_string());
        return resolve_directory(&dir_name, path, &syntax_override, &body_css_override);
    }

    let installed = list_installed();
    if installed.is_empty() {
        anyhow::bail!(
            "theme '{}' not found. No themes installed.\n\
             Install themes to: {}",
            name,
            themes_dir()
                .map(|d| d.display().to_string())
                .unwrap_or_else(|| "~/.config/sheen/themes/".to_string())
        );
    } else {
        anyhow::bail!(
            "theme '{}' not found. Installed themes: {}",
            name,
            installed.join(", ")
        );
    }
}

/// Default GitHub theme with optional syntax/body overrides.
fn resolve_default(
    syntax_override: &Option<PathBuf>,
    body_css_override: &Option<PathBuf>,
) -> anyhow::Result<ResolvedTheme> {
    let syntax = match syntax_override {
        Some(path) => Some(load_tmtheme(path)?),
        None => None,
    };

    let body_css = match body_css_override {
        Some(path) => std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("failed to read body CSS '{}': {e}", path.display()))?,
        None => String::new(),
    };

    let toggle = syntax.is_none() && body_css.is_empty();

    Ok(ResolvedTheme {
        name: "github".to_string(),
        syntax,
        body_css,
        toggle,
        is_light: false,
    })
}

/// Load a theme from a directory containing `syntax.tmTheme` and/or `body.css`.
fn resolve_directory(
    name: &str,
    dir: &Path,
    syntax_override: &Option<PathBuf>,
    body_css_override: &Option<PathBuf>,
) -> anyhow::Result<ResolvedTheme> {
    let syntax_path = syntax_override
        .clone()
        .unwrap_or_else(|| dir.join("syntax.tmTheme"));
    let syntax = if syntax_path.exists() {
        Some(load_tmtheme(&syntax_path)?)
    } else {
        None
    };

    let body_path = body_css_override
        .clone()
        .unwrap_or_else(|| dir.join("body.css"));
    let body_css = if body_path.exists() {
        std::fs::read_to_string(&body_path)?
    } else {
        String::new()
    };

    // Detect light theme from body.css comment convention: /* light */
    let is_light = body_css.contains("light variant");

    Ok(ResolvedTheme {
        name: name.to_string(),
        syntax,
        body_css,
        toggle: false,
        is_light,
    })
}

fn load_tmtheme(path: &Path) -> anyhow::Result<SyntaxTheme> {
    let theme = ThemeSet::get_theme(path)
        .map_err(|e| anyhow::anyhow!("failed to load tmTheme '{}': {e}", path.display()))?;
    let name = path
        .file_stem()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "custom".to_string());
    Ok(SyntaxTheme {
        theme,
        theme_name: name,
    })
}

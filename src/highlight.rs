use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};
use syntect::highlighting::ThemeSet;

use crate::theme::SyntaxTheme;

/// Create a syntax highlighter adapter using CSS classes (theme-agnostic).
///
/// Light/dark theming is handled by `syntax.css` in the browser.
pub fn adapter() -> SyntectAdapter {
    SyntectAdapter::new(None)
}

/// Create a syntax highlighter adapter using inline styles from a loaded theme.
pub fn adapter_with_theme(syntax_theme: &SyntaxTheme) -> SyntectAdapter {
    let mut theme_set = ThemeSet::new();
    theme_set
        .themes
        .insert(syntax_theme.theme_name.clone(), syntax_theme.theme.clone());
    SyntectAdapterBuilder::new()
        .theme(&syntax_theme.theme_name)
        .theme_set(theme_set)
        .build()
}

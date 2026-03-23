use crate::theme::ResolvedTheme;

const VIEWER_HTML: &str = include_str!("../assets/viewer.html");
const GITHUB_CSS: &str = include_str!("../assets/github-markdown.css");
const THEME_OVERRIDES: &str = include_str!("../assets/theme-overrides.css");
const PAGE_CSS: &str = include_str!("../assets/page.css");
const SYNTAX_CSS: &str = include_str!("../assets/syntax.css");
const ALERTS_COLORS_CSS: &str = include_str!("../assets/alerts-colors.css");
const ALERTS_CSS: &str = include_str!("../assets/alerts.css");

pub fn render_page(
    filename: &str,
    content_html: &str,
    custom_css: Option<&str>,
    theme: &ResolvedTheme,
) -> String {
    let custom_style = match custom_css {
        Some(css) => format!("<style>{css}</style>"),
        None => String::new(),
    };

    let has_custom_theme = !theme.body_css.is_empty() || theme.syntax.is_some();

    // When a syntax theme is active, omit syntax.css (inline styles replace it)
    let syntax_css = if theme.syntax.is_some() {
        ""
    } else {
        SYNTAX_CSS
    };

    // When a custom theme is active, omit theme-overrides.css and alerts-colors.css
    // (their high-specificity selectors would override the theme's colors)
    let theme_overrides = if has_custom_theme {
        ""
    } else {
        THEME_OVERRIDES
    };
    let alerts_colors = if has_custom_theme {
        ""
    } else {
        ALERTS_COLORS_CSS
    };

    let theme_body_style = if theme.body_css.is_empty() {
        String::new()
    } else {
        format!("<style>{}</style>", theme.body_css)
    };

    let theme_attr = if has_custom_theme {
        format!("data-sheen-theme=\"{}\"", theme.name)
    } else {
        String::new()
    };

    let theme_mode = if theme.toggle {
        "toggle"
    } else if theme.is_light {
        "fixed-light"
    } else {
        "fixed-dark"
    };

    VIEWER_HTML
        .replace("{{GITHUB_CSS}}", GITHUB_CSS)
        .replace("{{THEME_OVERRIDES}}", theme_overrides)
        .replace("{{PAGE_CSS}}", PAGE_CSS)
        .replace("{{SYNTAX_CSS}}", syntax_css)
        .replace("{{ALERTS_COLORS_CSS}}", alerts_colors)
        .replace("{{ALERTS_CSS}}", ALERTS_CSS)
        .replace("{{THEME_BODY_CSS}}", &theme_body_style)
        .replace("{{CUSTOM_CSS}}", &custom_style)
        .replace("{{THEME_MODE}}", theme_mode)
        .replace("{{FILENAME}}", filename)
        .replace("{{CONTENT}}", content_html)
        .replace("{{THEME_ATTR}}", &theme_attr)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_theme() -> ResolvedTheme {
        ResolvedTheme {
            name: "github".to_string(),
            syntax: None,
            body_css: String::new(),
            toggle: true,
            is_light: false,
        }
    }

    #[test]
    fn render_page_contains_filename() {
        let page = render_page("test.md", "<p>hello</p>", None, &default_theme());
        assert!(
            page.contains("test.md"),
            "rendered page should contain the filename"
        );
    }

    #[test]
    fn render_page_contains_content() {
        let page = render_page("test.md", "<p>hello</p>", None, &default_theme());
        assert!(
            page.contains("<p>hello</p>"),
            "rendered page should contain the content HTML"
        );
    }

    #[test]
    fn render_page_contains_markdown_body_class() {
        let page = render_page("test.md", "", None, &default_theme());
        assert!(
            page.contains("markdown-body"),
            "rendered page should contain the markdown-body class"
        );
    }

    #[test]
    fn render_page_contains_github_css() {
        let page = render_page("test.md", "", None, &default_theme());
        assert!(
            page.contains(".markdown-body"),
            "rendered page should contain github-markdown-css rules"
        );
    }

    #[test]
    fn render_page_includes_custom_css() {
        let page = render_page(
            "test.md",
            "",
            Some("body { color: red; }"),
            &default_theme(),
        );
        assert!(
            page.contains("body { color: red; }"),
            "rendered page should contain the custom CSS"
        );
    }
}

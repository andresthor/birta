use std::io::Read;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    version,
    about = "Preview markdown files in the browser with GitHub-style rendering"
)]
struct Cli {
    /// Path to the markdown file to preview, or "-" for stdin
    file: Option<PathBuf>,

    /// Port to serve on (0 = auto-assign)
    #[arg(short, long)]
    port: Option<u16>,

    /// Don't open the browser automatically
    #[arg(long)]
    no_open: bool,

    /// Custom CSS file to inject after default styles
    #[arg(long)]
    css: Option<PathBuf>,

    /// Theme preset name or path to theme directory
    #[arg(long)]
    theme: Option<String>,

    /// Path to a .tmTheme file for syntax highlighting (overrides preset)
    #[arg(long)]
    syntax_theme: Option<PathBuf>,

    /// List available theme presets and exit
    #[arg(long)]
    list_themes: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.list_themes {
        let themes = sheen::theme::list_installed();
        if themes.is_empty() {
            eprintln!("no themes installed");
            eprintln!(
                "install themes to: {}",
                dirs::home_dir()
                    .map(|d| {
                        d.join(".config")
                            .join("sheen")
                            .join("themes")
                            .display()
                            .to_string()
                    })
                    .unwrap_or_else(|| "~/.config/sheen/themes/".to_string())
            );
        } else {
            for name in themes {
                println!("{name}");
            }
        }
        return Ok(());
    }

    let file = cli
        .file
        .ok_or_else(|| anyhow::anyhow!("missing required argument: FILE"))?;

    let config = sheen::config::load();

    let port = cli.port.or(config.port).unwrap_or(0);
    let no_open = cli.no_open || config.no_open.unwrap_or(false);

    let css_path = cli.css.or(config.css.clone());
    let custom_css = match &css_path {
        Some(path) => {
            if !path.exists() {
                anyhow::bail!("CSS file not found: {}", path.display());
            }
            Some(std::fs::read_to_string(path)?)
        }
        None => None,
    };

    let theme = sheen::theme::resolve(&config, cli.theme.as_deref(), cli.syntax_theme.as_deref())?;

    if file.as_os_str() == "-" {
        let mut markdown = String::new();
        std::io::stdin().read_to_string(&mut markdown)?;
        return sheen::server::run_stdin(&markdown, port, no_open, custom_css.as_deref(), &theme)
            .await;
    }

    if !file.exists() {
        anyhow::bail!("file not found: {}", file.display());
    }

    if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
        if ext != "md" && ext != "markdown" {
            eprintln!(
                "sheen: warning: {} does not have a .md or .markdown extension",
                file.display()
            );
        }
    }

    sheen::server::run(file, port, no_open, custom_css.as_deref(), &theme).await
}

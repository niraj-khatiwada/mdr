mod backend;
mod core;

use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "mdr", version, about = "Lightweight Markdown viewer with live reload")]
struct Cli {
    /// Markdown file to render
    file: PathBuf,

    /// Rendering backend to use
    #[arg(short, long, default_value = "egui", value_parser = parse_backend)]
    backend: String,
}

fn parse_backend(s: &str) -> Result<String, String> {
    match s {
        "egui" | "webview" | "tui" => Ok(s.to_string()),
        _ => Err(format!("unknown backend '{}', expected 'egui', 'webview', or 'tui'", s)),
    }
}

fn main() {
    let cli = Cli::parse();

    if !cli.file.exists() {
        eprintln!("Error: file '{}' not found", cli.file.display());
        process::exit(1);
    }

    let result = match cli.backend.as_str() {
        #[cfg(feature = "egui-backend")]
        "egui" => backend::egui::run(cli.file),

        #[cfg(not(feature = "egui-backend"))]
        "egui" => {
            eprintln!("Error: egui backend not compiled. Rebuild with --features egui-backend");
            process::exit(1);
        }

        #[cfg(feature = "webview-backend")]
        "webview" => backend::webview::run(cli.file),

        #[cfg(not(feature = "webview-backend"))]
        "webview" => {
            eprintln!("Error: webview backend not compiled. Rebuild with --features webview-backend");
            process::exit(1);
        }

        #[cfg(feature = "tui-backend")]
        "tui" => backend::tui::run(cli.file),

        #[cfg(not(feature = "tui-backend"))]
        "tui" => {
            eprintln!("Error: tui backend not compiled. Rebuild with --features tui-backend");
            process::exit(1);
        }

        _ => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

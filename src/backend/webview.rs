use std::path::PathBuf;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

use crate::core::markdown::{parse_markdown, GITHUB_CSS};

pub fn run(file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let markdown_content = std::fs::read_to_string(&file_path)?;
    let html_body = parse_markdown(&markdown_content);
    let full_html = build_html(&html_body);

    let watcher_rx = crate::core::watcher::watch_file(&file_path)?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("mdr - {}", file_path.display()))
        .with_inner_size(tao::dpi::LogicalSize::new(800.0, 900.0))
        .build(&event_loop)?;

    let webview = WebViewBuilder::new()
        .with_html(&full_html)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Check for file changes
        if watcher_rx.try_recv().is_ok() {
            while watcher_rx.try_recv().is_ok() {}
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let new_html = parse_markdown(&content);
                let escaped = new_html.replace('\\', "\\\\").replace('`', "\\`");
                let js = format!("document.body.innerHTML = `{}`;", escaped);
                let _ = webview.evaluate_script(&js);
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

fn build_html(body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>{css}</style>
</head>
<body>
{body}
</body>
</html>"#,
        css = GITHUB_CSS,
        body = body
    )
}

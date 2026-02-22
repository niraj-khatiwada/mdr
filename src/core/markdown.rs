use comrak::{markdown_to_html, Options};

/// Convert markdown content to HTML with all GFM extensions enabled.
pub fn parse_markdown(content: &str) -> String {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.render.unsafe_ = true;

    markdown_to_html(content, &options)
}

/// CSS for GitHub-like markdown rendering with dark/light theme support.
pub const GITHUB_CSS: &str = r#"
@media (prefers-color-scheme: dark) {
    :root { --bg: #0d1117; --fg: #e6edf3; --code-bg: #161b22; --border: #30363d; --link: #58a6ff; --blockquote: #8b949e; }
}
@media (prefers-color-scheme: light) {
    :root { --bg: #ffffff; --fg: #1f2328; --code-bg: #f6f8fa; --border: #d0d7de; --link: #0969da; --blockquote: #656d76; }
}
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 1.6;
    color: var(--fg);
    background: var(--bg);
    max-width: 900px;
    margin: 0 auto;
    padding: 32px 24px;
}
h1, h2, h3, h4, h5, h6 { margin-top: 24px; margin-bottom: 16px; font-weight: 600; line-height: 1.25; }
h1 { font-size: 2em; padding-bottom: 0.3em; border-bottom: 1px solid var(--border); }
h2 { font-size: 1.5em; padding-bottom: 0.3em; border-bottom: 1px solid var(--border); }
code {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
    font-size: 85%;
    background: var(--code-bg);
    padding: 0.2em 0.4em;
    border-radius: 6px;
}
pre {
    background: var(--code-bg);
    padding: 16px;
    border-radius: 6px;
    overflow-x: auto;
    line-height: 1.45;
}
pre code { background: transparent; padding: 0; font-size: 85%; }
table { border-collapse: collapse; width: 100%; margin: 16px 0; }
th, td { border: 1px solid var(--border); padding: 6px 13px; }
th { font-weight: 600; background: var(--code-bg); }
blockquote {
    color: var(--blockquote);
    border-left: 4px solid var(--border);
    padding: 0 16px;
    margin: 16px 0;
}
a { color: var(--link); text-decoration: none; }
a:hover { text-decoration: underline; }
hr { border: none; border-top: 1px solid var(--border); margin: 24px 0; }
img { max-width: 100%; }
ul, ol { padding-left: 2em; }
input[type="checkbox"] { margin-right: 0.5em; }
"#;

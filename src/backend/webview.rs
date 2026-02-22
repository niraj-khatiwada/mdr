use std::path::PathBuf;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

use crate::core::markdown::{parse_markdown, GITHUB_CSS};
use crate::core::toc;

pub fn run(file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = file_path.parent()
        .map(|p| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf()))
        .unwrap_or_default();
    let markdown_content = std::fs::read_to_string(&file_path)?;
    let html_body = parse_markdown(&markdown_content);
    let html_body = resolve_local_images(&html_body, &base_dir);
    let toc_entries = toc::extract_toc(&markdown_content);
    let full_html = build_html(&html_body, &toc_entries);

    let watcher_rx = crate::core::watcher::watch_file(&file_path)?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("mdr - {}", file_path.display()))
        .with_inner_size(tao::dpi::LogicalSize::new(1100.0, 900.0))
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
                let new_html = resolve_local_images(&new_html, &base_dir);
                let new_toc = toc::extract_toc(&content);
                let toc_html = build_toc_html(&new_toc);

                let body_json = serde_json::to_string(&new_html).unwrap_or_default();
                let toc_json = serde_json::to_string(&toc_html).unwrap_or_default();
                let js = format!(
                    "document.querySelector('.content').innerHTML = {}; document.querySelector('.sidebar ul').innerHTML = {};",
                    body_json, toc_json
                );
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

/// Resolve relative image paths to absolute file:// URLs for local file display.
fn resolve_local_images(html: &str, base_dir: &std::path::Path) -> String {
    use regex::Regex;
    let re = Regex::new(r#"<img\s+src="([^"]+)""#).unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let src = &caps[1];
        // Skip URLs (http://, https://, data:, file://)
        if src.starts_with("http://") || src.starts_with("https://")
            || src.starts_with("data:") || src.starts_with("file://")
        {
            return caps[0].to_string();
        }
        // Resolve relative path to absolute file:// URL
        let abs_path = base_dir.join(src);
        if abs_path.exists() {
            format!("<img src=\"file://{}\"", abs_path.display())
        } else {
            caps[0].to_string()
        }
    })
    .to_string()
}

fn build_toc_html(entries: &[toc::TocEntry]) -> String {
    let mut toc = String::new();
    for entry in entries {
        toc.push_str(&format!(
            "<li class=\"toc-h{}\"><a href=\"#{}\">{}</a></li>",
            entry.level, entry.anchor, entry.text
        ));
    }
    toc
}

fn build_html(body: &str, toc_entries: &[toc::TocEntry]) -> String {
    let toc_html = build_toc_html(toc_entries);

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>{css}</style>
</head>
<body>
<nav class="sidebar">
<p class="sidebar-title">Table of Contents</p>
<ul>{toc}</ul>
</nav>
<div class="content">
{body}
</div>
<script>
document.querySelector('.sidebar').addEventListener('click', function(e) {{
    if (e.target.tagName === 'A') {{
        e.preventDefault();
        var id = e.target.getAttribute('href').substring(1);
        var el = document.getElementById(id);
        if (el) {{
            el.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
            document.querySelectorAll('.sidebar a').forEach(a => a.classList.remove('active'));
            e.target.classList.add('active');
        }}
    }}
}});
</script>
<div class="search-bar" id="searchBar" style="display:none;">
    <input type="text" id="searchInput" placeholder="Search..." />
    <span class="search-info" id="searchInfo">0/0</span>
    <button onclick="searchNav(-1)">&#9650;</button>
    <button onclick="searchNav(1)">&#9660;</button>
    <button class="close-btn" onclick="closeSearch()">Esc</button>
</div>
<script>
(function() {{
    var matches = [];
    var currentIdx = -1;

    function clearHighlights() {{
        document.querySelectorAll('mark.search-highlight').forEach(function(m) {{
            var parent = m.parentNode;
            parent.replaceChild(document.createTextNode(m.textContent), m);
            parent.normalize();
        }});
        matches = [];
        currentIdx = -1;
    }}

    function highlightMatches(query) {{
        clearHighlights();
        if (!query) {{ updateInfo(); return; }}
        var walker = document.createTreeWalker(
            document.querySelector('.content'),
            NodeFilter.SHOW_TEXT, null, false
        );
        var textNodes = [];
        while (walker.nextNode()) textNodes.push(walker.currentNode);

        var queryLower = query.toLowerCase();
        for (var i = textNodes.length - 1; i >= 0; i--) {{
            var node = textNodes[i];
            var text = node.textContent;
            var textLower = text.toLowerCase();
            var idx = textLower.lastIndexOf(queryLower);
            while (idx >= 0) {{
                var range = document.createRange();
                range.setStart(node, idx);
                range.setEnd(node, idx + query.length);
                var mark = document.createElement('mark');
                mark.className = 'search-highlight';
                range.surroundContents(mark);
                node = mark.previousSibling || node.parentNode.firstChild;
                idx = idx > 0 ? node.textContent.toLowerCase().lastIndexOf(queryLower, idx - 1) : -1;
            }}
        }}
        matches = document.querySelectorAll('mark.search-highlight');
        if (matches.length > 0) {{ currentIdx = 0; goToCurrent(); }}
        updateInfo();
    }}

    function goToCurrent() {{
        document.querySelectorAll('mark.search-highlight.current').forEach(function(m) {{ m.classList.remove('current'); }});
        if (matches.length > 0 && currentIdx >= 0) {{
            matches[currentIdx].classList.add('current');
            matches[currentIdx].scrollIntoView({{ behavior: 'smooth', block: 'center' }});
        }}
    }}

    function updateInfo() {{
        var info = document.getElementById('searchInfo');
        if (matches.length === 0) {{ info.textContent = '0/0'; }}
        else {{ info.textContent = (currentIdx + 1) + '/' + matches.length; }}
    }}

    window.searchNav = function(dir) {{
        if (matches.length === 0) return;
        currentIdx = (currentIdx + dir + matches.length) % matches.length;
        goToCurrent();
        updateInfo();
    }};

    window.closeSearch = function() {{
        document.getElementById('searchBar').style.display = 'none';
        clearHighlights();
        updateInfo();
    }};

    document.addEventListener('keydown', function(e) {{
        if ((e.ctrlKey || e.metaKey) && e.key === 'f') {{
            e.preventDefault();
            var bar = document.getElementById('searchBar');
            bar.style.display = 'flex';
            var input = document.getElementById('searchInput');
            input.focus();
            input.select();
        }}
        if (e.key === 'Escape') {{
            window.closeSearch();
        }}
        if (e.key === 'Enter' && document.activeElement === document.getElementById('searchInput')) {{
            e.preventDefault();
            if (e.shiftKey) {{ window.searchNav(-1); }}
            else {{ window.searchNav(1); }}
        }}
    }});

    document.getElementById('searchInput').addEventListener('input', function() {{
        highlightMatches(this.value);
    }});
}})();
</script>
</body>
</html>"#,
        css = GITHUB_CSS,
        toc = toc_html,
        body = body
    )
}

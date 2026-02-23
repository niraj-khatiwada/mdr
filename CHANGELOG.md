# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.6] - 2026-02-23

### Added
- Application window icon: both egui and webview backends now display the mdr logo as window icon

## [0.2.4] - 2026-02-23

### Added
- `--list-backends` flag to display available backends at runtime
- Backend names shown in CLI help output

## [0.2.3] - 2026-02-22

### Fixed
- Homebrew tap release workflow: remove broken copy action, fix heredoc syntax

## [0.2.2] - 2026-02-22

### Security
- Path traversal protection for local file access
- Content Security Policy (CSP) headers in webview backend
- Regex caching to prevent ReDoS
- HTML encoding of user-controlled content (`html_encode`)

## [0.2.1] - 2026-02-22

### Fixed
- Windows build: move `[target.'cfg(unix)'.dependencies]` section after optional deps in `Cargo.toml`

## [0.2.0] - 2026-02-22

### Added
- Mouse scroll support in TUI backend
- Mermaid diagram rendering as terminal images in TUI backend
- SVG image support in TUI backend
- Offline Mermaid rendering via embedded `mermaid.js`
- Project logo in README and documentation
- Verbose mode (`-v`) for debug output across all backends

### Fixed
- Image rendering consistency across egui, webview, and TUI backends
- Local image path resolution in TUI backend
- SVG images not displaying in webview backend (#10)
- SVG rendering in egui: rasterize to PNG instead of inline embedding (security fix)
- Mermaid rendering improvements in TUI and webview backends

### Changed
- SVG images rasterized to PNG in egui backend for consistency
- README updated with logo and LLM-era motivation section

## [0.1.1] - 2026-02-22

### Added
- WinGet package publishing job in release workflow
- AUR (Arch User Repository) package publishing job in release workflow
- crates.io publish job in release workflow

## [0.1.0] - 2026-02-22

### Added
- In-document search (`/` to open, `n`/`N` to navigate matches)
- Packaging support: Homebrew, Nix, pre-built binaries
- Release infrastructure: GitHub Actions CI/CD pipeline

## [0.9] - 2026-02-22

### Added
- Initial project bootstrap with egui and webview dual rendering backends
- TUI backend using Ratatui for terminal rendering
- TOC (Table of Contents) sidebar navigation
- Mermaid diagram rendering support
- Image rendering in TUI backend with local image path resolution
- Auto-detection of rendering backend based on environment
- Security hardening for file access and rendering pipeline
- GitHub Actions CI/CD workflow
- 58 unit tests

### Fixed
- Mermaid diagram text rendering
- TOC scroll navigation in egui backend
- Mermaid rendering robustness in egui backend

[0.2.6]: https://github.com/CleverCloud/mdr/compare/v0.2.5...v0.2.6
[0.2.4]: https://github.com/CleverCloud/mdr/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/CleverCloud/mdr/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/CleverCloud/mdr/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/CleverCloud/mdr/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/CleverCloud/mdr/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/CleverCloud/mdr/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/CleverCloud/mdr/compare/0.9...v0.1.0
[0.9]: https://github.com/CleverCloud/mdr/releases/tag/0.9

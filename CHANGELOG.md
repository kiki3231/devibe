# Changelog

All notable changes to devibe will be documented in this file.

## [0.2.0] - 2026-05-24

### Added
- 7 color themes (Dark, Light, Gruvbox, Nord, Catppuccin, Monokai, OneDark) with `t`/`T` keybindings
- Config file support via `.devibe.toml` (theme, days, exclude)
- `--days N` flag for custom daily chart window
- `--export-json <FILE>` and `--export-csv <FILE>` for data export
- Panel 5: Top Authors (by commit count)
- `r` key to refresh data live (re-scan repositories)
- Scroll support in all panels (`j`/`k`, arrows, PgUp/PgDn, Home/End)
- Pre-built binaries for Linux (x86_64, musl), macOS (x86_64, ARM64), Windows (x86_64)
- One-liner install scripts for all platforms (`install.sh`, `install.ps1`)
- Scoop bucket manifest for Windows
- CI/CD pipeline (GitHub Actions): build, test, clippy, fmt on push/PR
- Release pipeline: automated multi-platform builds on git tag
- Repository files: LICENSE (MIT), CONTRIBUTING.md (中文), CHANGELOG.md
- GitHub templates: issue templates (bug/feature/question, all Chinese), PR template
- Dependabot configuration for automated cargo updates

### Changed
- All widgets now accept `Theme` parameter for consistent styling
- Summary bar shows theme name and author count
- Help bar redesigned with theme-aware highlighting
- Picker redesigned with padding, scroll indicators, and clearer navigation hints
- README completely rewritten with badges, install scripts, theme table
- Bilingual README structure preserved (EN / 中文)

### Fixed
- Windows SmartScreen warnings documented with 4 workarounds
- Release artifacts now packaged as `.zip` (Windows) and `.tar.gz` (Linux/macOS)

## [0.1.0] - 2026-05-24

### Added
- Initial release
- 4 TUI panels: Daily commits, Activity heatmap, Language breakdown, Top repos
- `--scan <dir>` flag for recursive repo discovery (depth 3)
- `--repo <dir>` flag for single-repo analysis
- `--summary` flag for plain-text output (CI/CD friendly)
- Interactive directory picker when no repos found nearby
- Keyboard navigation: `1-4` to focus panels, `q` to quit
- Multi-repo aggregate statistics
- Bilingual README (English / 中文)

[0.2.0]: https://github.com/kiki3231/devibe/releases/tag/v0.2.0
[0.1.0]: https://github.com/kiki3231/devibe/releases/tag/v0.1.0

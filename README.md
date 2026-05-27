# Devibe

<p align="center">
  <b>Your Coding Life, Quantified</b><br>
  <i>A beautiful terminal dashboard for your git activity — fully offline, zero config.</i>
</p>

<p align="center">
  <a href="https://github.com/kiki3231/devibe/actions/workflows/CI.yml"><img src="https://github.com/kiki3231/devibe/actions/workflows/CI.yml/badge.svg" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <img src="https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue" alt="Platforms">
</p>

<p align="center">
  <a href="README_CN.md">中文文档</a>
</p>

<p align="center">
  <img src="demo.gif" alt="devibe demo" width="800">
</p>

---

## Why devibe?

**WakaTime** tracks editor time. **GitHub Insights** tracks PRs. **devibe** tracks your commits across every repo on your machine — offline, private, and beautiful.

| | WakaTime | GitHub Insights | **devibe** |
|---|:---:|:---:|:---:|
| Works offline | | | ✓ |
| Private (no cloud) | | | ✓ |
| Multi-repo aggregate | | | ✓ |
| Terminal-native | | | ✓ |
| Free forever | Limited | ✓ | ✓ |
| 7 color themes | | | ✓ |
| JSON/CSV export | ✓ | ✓ | ✓ |

## Features

- **Daily Commit Chart** — bar chart of commits over N days (customizable)
- **Activity Heatmap** — 24h x 7d commit density (GitHub-style)
- **Language Breakdown** — file-count distribution by language
- **Top Repositories** — repos ranked by commit count with progress bars
- **Top Authors** — contributor activity ranking (who's shipping?)
- **`--summary` mode** — plain text output for scripts, CI pipelines, piping
- **`--export-json/--export-csv`** — export all data for external analysis
- **7 color themes** — Dark, Light, Gruvbox, Nord, Catppuccin, Monokai, OneDark
- **Config file** — `.devibe.toml` for theme, date range, and exclusions
- **Keyboard-driven** — `1-5` switch panels, `r` refresh, `t` cycle themes, `jk` scroll
- **Interactive directory picker** — browse filesystem to find git repos (vim-key navigation)
- **Parallel scanning** — multi-repo analysis uses rayon for fast concurrent processing
- **Sub-20ms startup** — all data stays local, no network calls

## Quick Start

```bash
# Clone and build
git clone https://github.com/kiki3231/devibe.git
cd devibe
cargo build --release

# Option 1: Install to PATH
sudo cp target/release/devibe /usr/local/bin/

# Option 2: Shell alias (no sudo, works immediately)
echo "alias devibe='$(pwd)/target/release/devibe'" >> ~/.bashrc
source ~/.bashrc
```

**Requirements:** Rust 1.80+ and libgit2. Or enable vendored libgit2:

```bash
cargo build --release --features vendored
```

### Usage

```bash
# Scan current directory (depth 3)
devibe

# Scan a specific directory
devibe --scan ~/projects

# Analyze a single repo with 30-day window
devibe --repo ~/my-project --days 30

# Export stats to JSON
devibe --scan ~/code --export-json stats.json

# Plain text summary (CI/CD friendly)
devibe --repo . --summary
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `1`-`5` | Switch panel focus (Daily / Heatmap / Languages / Repos / Authors) |
| `j`/`k` / `↑`/`↓` | Scroll active panel |
| `PgUp` / `PgDn` | Page scroll |
| `t` | Cycle to next theme |
| `T` | Cycle to previous theme |
| `r` | Refresh data (re-scan repos) |
| `q` | Quit |

## Themes

<table>
<tr>
<td><code>Dark</code> (default)</td>
<td><code>Light</code></td>
<td><code>Gruvbox</code></td>
<td><code>Nord</code></td>
<td><code>Catppuccin</code></td>
<td><code>Monokai</code></td>
<td><code>OneDark</code></td>
</tr>
</table>

Press `t` to cycle through themes, or set one via config file:

```toml
# ~/.config/devibe/config.toml
theme = "catppuccin"
days = 30
```

## Installation

### From source

```bash
git clone https://github.com/kiki3231/devibe.git
cd devibe
cargo build --release
sudo cp target/release/devibe /usr/local/bin/
```

**Requirements:** Rust 1.80+ and libgit2.

## Configuration

devibe looks for `.devibe.toml` in the current directory (workspace-level) or `~/.config/devibe/config.toml` (global).

```toml
# Theme: dark, light, gruvbox, nord, catppuccin, monokai, onedark
theme = "gruvbox"

# Default days for daily chart
days = 30

# Exclude directories
exclude = ["node_modules", ".terraform"]
```

## Design Philosophy

- **Files are the database** — your git repos are the source of truth; no lock-in
- **Terminal first** — never leave the keyboard, no loading spinners
- **Fully offline** — no account, no telemetry, no network calls, no analytics
- **Zero config** — sensible defaults; point at a directory and go
- **Beautiful by default** — 7 curated color themes, not a settings maze

## Roadmap

- [ ] `e` key to open repo in `$EDITOR`
- [ ] Custom date range filtering (`--since`, `--until`)
- [ ] Gitmoji & Conventional Commit analysis
- [ ] Diff stats per file type
- [ ] CI badge generation (embed stats in README)
- [ ] Remote repo support (via git URL)
- [ ] Shell completions (bash, zsh, fish, powershell)

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) (中文).

Devibe is a Rust project using ratatui, crossterm, git2, chrono, and rayon. Check the [issues](https://github.com/kiki3231/devibe/issues) for tasks tagged `good first issue`.

## Why "devibe"?

**dev** + **vibe** — your development rhythm at a glance. Also a play on "I be coding."

## License

MIT — see [LICENSE](LICENSE).

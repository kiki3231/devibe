# devibe

> **Your coding life, quantified.** — A beautiful terminal dashboard for your git activity.

<p align="center">
  <img src="demo.gif" alt="devibe demo" width="800">
</p>

Turn your git history into a gorgeous TUI dashboard. No cloud, no account, no bloat — just your local repos and a 2MB binary.

## Quick Start

```bash
# Scan all repos under current directory
devibe

# Scan a specific directory (max depth 3)
devibe --scan ~/projects

# Analyze a single repo
devibe --repo ~/my-project

# Text-only summary (great for CI/CD)
devibe --repo . --summary
```

## What You Get

```
┌──────────────────────────────────────────────────────────────────┐
│ Repos: 12  |  Commits: 3,842  |  Lines: +128k / -94k  |  ...   │
├─────────────────────────┬────────────────────────────────────────┤
│  Commits per Day        │  Activity Heatmap (hour × weekday)    │
│                         │      00 03 06 09 12 15 18 21         │
│      ▇                  │  Mon ·  ·  ·  ·  ░  █  █  ▓          │
│    ▇ █                  │  Tue ·  ·  ·  ·  ▓  █  ▓  ░          │
│  ▇ █ █ █                │  Wed ·  ·  ·  ·  █  █  ▓  ░          │
│  █ █ █ █ █   ▇          │  Thu ·  ·  ·  ░  █  ▓  ░  ·          │
│  █ █ █ █ █ █ █ █        │  Fri ·  ·  ·  ░  ▓  ░  ·  ·          │
│  05 06 07 08 09 10 11   │  Sat ·  ·  ·  ·  ·  ·  ·  ·          │
│                         │  Sun ·  ·  ·  ·  ·  ·  ·  ·          │
├─────────────────────────┴────────────────────────────────────────┤
│  Languages (by files)     │  Top Repos (by commits)             │
│  Rust          ██████ 45% │   1. neovim      3,241 commits      │
│  TypeScript    ████ 22%   │   2. my-app      1,847 commits      │
│  Python        ███ 14%    │   3. dotfiles      923 commits      │
│  Go            ██ 8%      │   4. lib           512 commits      │
├──────────────────────────────────────────────────────────────────┤
│ 1(Daily)  2(Heatmap)  3(Languages)  4(Repos)    q:Quit         │
└──────────────────────────────────────────────────────────────────┘
```

## Features

- **4 live panels**: daily commits, hour×weekday heatmap, language breakdown, top repos
- **Multi-repo**: scan a whole directory tree — get aggregate stats across all your projects
- **Zero config**: point it at a directory and go
- **Blazing fast**: 2MB binary, sub-20ms startup, data stays local
- **Git-native**: reads your existing `.git` directories — no new files, no lock-in
- **CI-friendly**: `--summary` flag prints text output for scripts and pipelines
- **Keyboard driven**: `1-4` to focus panels, `q` to quit

## Installation

### Pre-built binary

```bash
# Download the latest release
curl -L https://github.com/yourusername/devibe/releases/latest/download/devibe-linux-x86_64 -o devibe
chmod +x devibe
sudo mv devibe /usr/local/bin/
```

### Build from source

```bash
git clone https://github.com/yourusername/devibe.git
cd devibe
cargo build --release
sudo cp target/release/devibe /usr/local/bin/
```

### Requirements

- **To run**: just the binary — no runtime dependencies
- **To build**: Rust 1.80+, libgit2 (or use `--features vendored`)

## Why devibe?

| | WakaTime | GitHub Insights | devibe |
|---|---|---|---|
| Works offline | ❌ | ❌ | ✅ |
| Private (no cloud) | ❌ | ❌ | ✅ |
| Multi-repo aggregate | ❌ | ❌ | ✅ |
| Terminal-native | ❌ | ❌ | ✅ |
| Free forever | Limited | ✅ | ✅ |
| Binary size | N/A | N/A | **2MB** |

WakaTime tracks your editor time. GitHub Insights tracks your PRs. **devibe** tracks your commits — across every repo on your machine, in a beautiful TUI, with no internet connection needed.

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `1` | Focus daily commits panel |
| `2` | Focus activity heatmap |
| `3` | Focus language breakdown |
| `4` | Focus top repos |
| `q` | Quit |

## Roadmap

- [ ] `r` to refresh data live
- [ ] Custom date range
- [ ] Author breakdown (collaboration stats)
- [ ] Export to JSON/CSV
- [ ] Gitmoji / conventional commit analysis
- [ ] Config file for ignored repos

## License

MIT

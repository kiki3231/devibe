# devibe

> **Your coding life, quantified.** / **量化你的编码人生。** — 一个漂亮的终端开发者仪表盘。

<p align="center">
  <img src="demo.gif" alt="devibe demo" width="800">
</p>

---

## 中文说明

devibe 把你本地的 git 提交历史变成一个绚丽的 TUI 仪表盘。不上传、不登录、不臃肿 —— 只有你的本地仓库和一个 2MB 的二进制文件。

### 快速开始

```bash
# 扫描当前目录下的所有仓库（深度 3）
devibe

# 扫描指定目录
devibe --scan ~/projects

# 分析单个仓库
devibe --repo ~/my-project

# 纯文本摘要（CI/CD 友好）
devibe --repo . --summary
```

### 功能面板

启动后你会看到四个面板：

| 面板 | 内容 | 快捷键 |
|------|------|--------|
| **每日提交** | 近 14 天 commit 频率柱状图 | `1` |
| **活跃热力图** | 24h × 7d 的提交时段分布（绿色越深越活跃） | `2` |
| **语言占比** | 按文件数统计的编程语言分布条 | `3` |
| **仓库排行** | 按 commit 数排序的仓库榜单 | `4` |

底部状态栏显示汇总信息：仓库数、总提交数、代码增删行数、活跃天数。

按 `q` 退出。

### 安装

```bash
# 从源码编译
git clone https://github.com/kiki3231/devibe.git
cd devibe
cargo build --release
sudo cp target/release/devibe /usr/local/bin/
```

- **运行**：只需要二进制文件，无运行时依赖
- **编译**：需要 Rust 1.80+、libgit2（或启用 vendored feature）

### 为什么叫 devibe

**dev** + **vibe** — "开发者的状态/氛围"。看一眼就知道自己最近的编码节奏。同时也是 "dev" + "ibe"（I be coding 的谐音）。

### 设计哲学

- **文件即数据库**：不锁数据，你的 git 仓库就是数据源
- **终端第一**：不离开键盘，不等界面加载
- **完全离线**：不需要账号，不上传任何数据
- **零配置**：指向目录就能跑

### 路线图

- [ ] `r` 键实时刷新
- [ ] 自定义时间范围
- [ ] 作者维度统计（多人协作面板）
- [ ] 导出 JSON/CSV
- [ ] Gitmoji / Conventional Commit 分析
- [ ] 配置文件（排除指定仓库）

---

## English

Turn your git history into a gorgeous TUI dashboard. No cloud, no account, no bloat — just your local repos and a 2MB binary.

### Quick Start

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

### What You Get

```
┌──────────────────────────────────────────────────────────────────┐
│ Repos: 12  |  Commits: 3,842  |  Lines: +128k / -94k  |  ...   │
├─────────────────────────┬────────────────────────────────────────┤
│  Commits per Day        │  Activity Heatmap (hour x weekday)    │
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

### Features

- **4 live panels**: daily commits, hour×weekday heatmap, language breakdown, top repos
- **Multi-repo**: scan a whole directory tree — get aggregate stats across all your projects
- **Zero config**: point it at a directory and go
- **Blazing fast**: 2MB binary, sub-20ms startup, data stays local
- **Git-native**: reads your existing `.git` directories — no new files, no lock-in
- **CI-friendly**: `--summary` flag prints text output for scripts and pipelines
- **Keyboard driven**: `1-4` to focus panels, `q` to quit

### Installation

```bash
# Build from source
git clone https://github.com/kiki3231/devibe.git
cd devibe
cargo build --release
sudo cp target/release/devibe /usr/local/bin/
```

### Requirements

- **To run**: just the binary — no runtime dependencies
- **To build**: Rust 1.80+, libgit2

### Why devibe?

| | WakaTime | GitHub Insights | devibe |
|---|---|---|---|
| Works offline | ❌ | ❌ | ✅ |
| Private (no cloud) | ❌ | ❌ | ✅ |
| Multi-repo aggregate | ❌ | ❌ | ✅ |
| Terminal-native | ❌ | ❌ | ✅ |
| Free forever | Limited | ✅ | ✅ |
| Binary size | N/A | N/A | **2MB** |

WakaTime tracks your editor time. GitHub Insights tracks your PRs. **devibe** tracks your commits — across every repo on your machine, in a beautiful TUI, with no internet connection needed.

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `1` | Focus daily commits panel |
| `2` | Focus activity heatmap |
| `3` | Focus language breakdown |
| `4` | Focus top repos |
| `q` | Quit |

### Roadmap

- [ ] `r` to refresh data live
- [ ] Custom date range
- [ ] Author breakdown (collaboration stats)
- [ ] Export to JSON/CSV
- [ ] Gitmoji / conventional commit analysis
- [ ] Config file for ignored repos

### License

MIT

# Devibe

<p align="center">
  <b>量化你的编码人生</b><br>
  <i>漂亮的终端 Git 活动仪表盘 — 完全离线，零配置。</i>
</p>

<p align="center">
  <a href="https://github.com/kiki3231/devibe/actions/workflows/CI.yml"><img src="https://github.com/kiki3231/devibe/actions/workflows/CI.yml/badge.svg" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <img src="https://img.shields.io/badge/平台-Linux%20%7C%20macOS%20%7C%20Windows-blue" alt="平台">
</p>

<p align="center">
  <a href="README.md">English</a>
</p>

<p align="center">
  <img src="demo.gif" alt="devibe 演示" width="800">
</p>

---

## 为什么用 devibe？

**WakaTime** 追踪编辑器使用时间。**GitHub Insights** 追踪 PR。而 **devibe** 追踪你本机所有仓库的 commit 记录——离线运行，隐私安全，界面精美。

| | WakaTime | GitHub Insights | **devibe** |
|---|:---:|:---:|:---:|
| 离线可用 | | | ✓ |
| 隐私保护（不上传） | | | ✓ |
| 多仓库聚合 | | | ✓ |
| 终端原生 | | | ✓ |
| 永久免费 | 有限制 | ✓ | ✓ |
| 7 套配色主题 | | | ✓ |
| JSON/CSV 导出 | ✓ | ✓ | ✓ |

## 功能特性

- **每日提交图表** — N 天内的 commit 频率柱状图（天数可自定义）
- **活跃热力图** — 24小时 x 7天的提交时段分布（GitHub 风格）
- **语言占比** — 按文件数统计的编程语言分布
- **仓库排行** — 按 commit 数排序的仓库榜单，带动画进度条
- **贡献者排行** — 查看谁在频繁提交（多人项目必备）
- **`--summary` 模式** — 纯文本摘要，适合脚本/CI 流水线/管道处理
- **`--export-json/--export-csv`** — 导出所有数据用于外部分析
- **7 套配色主题** — Dark、Light、Gruvbox、Nord、Catppuccin、Monokai、OneDark
- **配置文件** — `.devibe.toml` 配置主题、时间窗口、排除目录
- **纯键盘操作** — `1-5` 切换面板，`r` 刷新，`t` 切换主题，`jk` 滚动
- **交互式目录选择器** — 无仓库时自动弹出文件浏览器，支持 vim 式导航
- **并行扫描** — 多仓库分析使用 rayon 实现高速并发处理
- **20ms 极速启动** — 所有数据留存本地，零网络请求

## 快速开始

```bash
# 克隆并编译
git clone https://github.com/kiki3231/devibe.git
cd devibe
cargo build --release

# 安装到 PATH（可选）
sudo cp target/release/devibe /usr/local/bin/

# 或直接运行
cargo run --release
```

**依赖：** Rust 1.80+ 和 libgit2。或启用内置 libgit2：

```bash
cargo build --release --features vendored
```

### 使用方式

```bash
# 扫描当前目录（深度 3）
devibe

# 扫描指定目录
devibe --scan ~/projects

# 分析单个仓库，30 天窗口
devibe --repo ~/my-project --days 30

# 导出统计数据为 JSON
devibe --scan ~/code --export-json stats.json

# 纯文本摘要（CI/CD 友好）
devibe --repo . --summary
```

## 快捷键

| 按键 | 操作 |
|------|------|
| `1`-`5` | 切换面板（每日 / 热力图 / 语言 / 仓库 / 贡献者） |
| `j`/`k` / `↑`/`↓` | 滚动当前面板 |
| `PgUp` / `PgDn` | 翻页滚动 |
| `t` | 切换到下一主题 |
| `T` | 切换到上一主题 |
| `r` | 刷新数据（重新扫描仓库） |
| `q` | 退出 |

## 配色主题

<table>
<tr>
<td><code>Dark</code>（默认）</td>
<td><code>Light</code></td>
<td><code>Gruvbox</code></td>
<td><code>Nord</code></td>
<td><code>Catppuccin</code></td>
<td><code>Monokai</code></td>
<td><code>OneDark</code></td>
</tr>
</table>

按 `t` 循环切换主题，或在配置文件中指定：

```toml
# ~/.config/devibe/config.toml
theme = "catppuccin"
days = 30
```

## 安装方式

### 从源码编译

```bash
git clone https://github.com/kiki3231/devibe.git
cd devibe
cargo build --release
sudo cp target/release/devibe /usr/local/bin/
```

**依赖：** Rust 1.80+ 和 libgit2。

## 配置文件

devibe 会查找当前目录下的 `.devibe.toml`（工作区级别）或 `~/.config/devibe/config.toml`（全局）。

```toml
# 主题: dark, light, gruvbox, nord, catppuccin, monokai, onedark
theme = "gruvbox"

# 每日图表的默认天数
days = 30

# 排除目录
exclude = ["node_modules", ".terraform"]
```

## 设计哲学

- **文件即数据库** — 你的 git 仓库就是数据源，不锁定数据，不新增文件
- **终端第一** — 不用离开键盘，没有加载动画
- **完全离线** — 不需要账号，无遥测，无网络请求，无分析追踪
- **零配置** — 一切都有合理默认值，指向目录就能用
- **默认即美观** — 7 套精心设计的配色主题，不是让人头疼的设置迷宫

## 路线图

- [ ] `e` 键在 `$EDITOR` 中打开仓库
- [ ] 自定义日期范围过滤（`--since`、`--until`）
- [ ] Gitmoji / Conventional Commit 分析
- [ ] 按文件类型的差异统计
- [ ] CI 徽章生成（在 README 中嵌入统计）
- [ ] 远程仓库支持（通过 git URL）
- [ ] Shell 自动补全（bash、zsh、fish、powershell）

## 参与贡献

欢迎贡献！详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

devibe 是一个 Rust 项目，主要使用 ratatui、crossterm、git2、chrono 和 rayon。查看 [Issues](https://github.com/kiki3231/devibe/issues) 中标记为 `good first issue` 的任务。

## 为什么叫 devibe？

**dev** + **vibe** — 一眼掌握你的开发节奏。也谐音 "I be coding."

## 开源协议

MIT — 详见 [LICENSE](LICENSE)。

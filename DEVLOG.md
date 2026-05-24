# devibe 开发日志 — Day 2

**日期**: 2026-05-24  
**仓库**: https://github.com/kiki3231/devibe  
**版本**: v0.2.0

---

## 今日完成

### 新增功能

#### 7 套配色主题
- Dark（默认）、Light、Gruvbox、Nord、Catppuccin、Monokai、OneDark
- `t` 键循环切换，`T` 回退
- 全局配置 `~/.config/devibe/config.toml` 中 `theme = "nord"`
- 所有 widget 完整适配（柱状图、热力图、进度条、边框、文本）

#### 配置文件支持
```toml
theme = "gruvbox"
days = 30
exclude = ["node_modules"]
```
- 项目级：当前目录 `.devibe.toml`
- 全局级：`~/.config/devibe/config.toml`

#### 数据导出
- `--export-json stats.json` — 结构化 JSON 导出
- `--export-csv stats.csv` — CSV 表格导出（type, label, value）

#### 面板 5：贡献者排行
- 按 commit 数的作者排名
- 多人项目的协作热力图

#### 实时刷新
- `r` 键重新扫描仓库，更新所有面板数据

#### 滚动支持
- `j`/`k` / `↑`/`↓` 逐行滚动
- `PgUp`/`PgDn` 翻页滚动
- `Home`/`End` 跳到开头/结尾
- 语言和仓库面板显示 `▲▼` 溢出指示器

#### `--days N` 标志
- 自定义每日图表的回溯天数（默认 14）

### 工程化

#### CI/CD（GitHub Actions）
- **CI**：`cargo fmt --check` + `cargo clippy` + `cargo build --release` + `cargo test`
- **Release**：git tag 触发 → 多平台交叉编译 → 打包 → GitHub Release
- 6 个目标平台：Linux x86_64 (gnu/musl)、macOS x86_64/ARM64、Windows x86_64

#### Windows 分发
- 提供 `.zip` 包避免 SmartScreen 直接拦截
- Scoop bucket manifest
- 一键安装脚本 `install.ps1`
- 4种 SmartScreen 解决方案文档化

#### 仓库完善
- `LICENSE`（MIT）
- `CONTRIBUTING.md`（中文）
- `CHANGELOG.md`
- Issue 模板（Bug / 功能请求 / 其他，全中文）
- PR 模板
- Dependabot 自动依赖更新

### 项目统计

| 指标 | v0.1.0 | v0.2.0 |
|------|--------|--------|
| Rust 文件数 | 7 | 9 |
| 代码行数 | 826 | ~1600 |
| 依赖数 | 5 | 9 |
| 平台目标 | 1 | 6 |
| 主题 | 1（硬编码） | 7（可配置） |
| TUI 面板 | 4 | 5 |

## 待改进

- [ ] 录制 demo GIF（终端录制工具 vhs 或 terminalizer）
- [ ] `--since` / `--until` 时间范围过滤
- [ ] Shell 自动补全脚本
- [ ] 仓库内打开编辑器（`e` 键）
- [ ] Gitmoji / Conventional Commit 分析面板
- [ ] 在 README 中嵌入动态统计徽章

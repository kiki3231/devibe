# devibe 开发日志 — Day 1

**日期**: 2026-05-24  
**仓库**: https://github.com/kiki3231/devibe  
**版本**: v0.1.0

---

## 项目定位

devibe = **dev** + **vibe** — 终端开发者仪表盘。读取本地 git 仓库，生成 TUI 数据面板。完全离线，2MB 二进制，零运行时依赖。

## 今日完成

### 架构

```
devibe/
├── Cargo.toml            # Rust 项目配置
├── README.md             # 中英双语 README（带语言切换器）
└── src/
    ├── main.rs           # CLI 入口（--repo / --scan / --summary）
    ├── scanner.rs        # git 仓库发现器（递归扫描，深度 3）
    ├── stats.rs          # 数据统计引擎
    ├── picker.rs         # TUI 目录选择器
    ├── app.rs            # TUI 事件循环 + 状态管理
    ├── ui.rs             # 主布局 + 帮助栏
    └── widgets.rs        # 4 个终端图表 widget
```

**826 行 Rust，零编译警告。**

### 功能面板

| 面板 | 内容 | 快捷键 |
|------|------|--------|
| 每日提交 | 近 14 天 commit 柱状图 | `1` |
| 活跃热力图 | 24h × 7d 提交时段分布 | `2` |
| 语言占比 | 按文件数统计语言条形图 | `3` |
| 仓库排行 | 按 commit 数排序 | `4` |

### 三种模式

| 命令 | 行为 |
|------|------|
| `devibe` | 无参数：当前目录有 git 仓库 → 直接仪表盘；没有 → 弹出目录选择器 |
| `devibe --repo <path>` | 分析单个仓库，跳过选择器 |
| `devibe --scan <dir>` | 递归扫描目录下所有仓库 |
| `devibe --summary` | 纯文本输出，适合 CI/CD |

### 目录选择器

- `j/k/↑/↓` 导航
- `Enter` 进入目录 / 选中 git 仓库
- `Backspace` 返回上级
- `Home` 跳到 `~`
- git 仓库标 `[git]` 并排最前
- 非 TTY 环境自动降级为文字提示

### 技术栈

| 层 | 选型 |
|----|------|
| 语言 | Rust |
| TUI | ratatui + crossterm |
| Git 数据 | git2 (libgit2) |
| 日期 | chrono |
| CLI | clap derive |
| 二进制 | 2.1MB (strip 后) |

### 已修 Bug

1. ratatui 0.29 越界渲染 panic → 添加 `clip()` 函数裁剪所有 widget
2. `--repo .` 显示 "unknown" → `repo_name()` 增加路径 canonicalize
3. 热力图 legend 分数标签错位 → 改为 "Less ← ■ → More"
4. 非 TTY 环境 picker 崩溃 → `IsTerminal` 检测 + 友好提示

## 待改进（明天）

- [ ] 录制 demo GIF 放 README（涨 star 关键）
- [ ] `--days N` 自定义时间窗口
- [ ] 作者维度统计
- [ ] 导出 JSON
- [ ] 刷新功能（`r` 键）
- [ ] 静态链接 musl 编译（跨 Linux 发行版）
- [ ] macOS / Windows 构建
- [ ] TUI 颜色/主题配置

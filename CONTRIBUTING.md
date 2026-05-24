# 贡献指南

感谢你对 devibe 的关注！任何形式的贡献都欢迎：Bug 报告、功能建议、文档改进、代码提交。

## 如何贡献

### 报告 Bug

1. 在 GitHub Issues 中搜索是否已有人报告
2. 如果没有，新建 Issue，选择 Bug 报告模板
3. 请包含：操作系统、终端模拟器、devibe 版本、复现步骤

### 功能建议

1. 在 Issues 中选择功能请求模板
2. 描述你希望的功能以及使用场景
3. 如果有参考工具或截图，欢迎附上

### 提交代码

1. **Fork 仓库**
2. **创建分支** — `git checkout -b feat/my-feature` 或 `fix/description`
3. **编码** — 遵循现有代码风格，`cargo fmt` 和 `cargo clippy` 零警告
4. **测试** — `cargo test` 全部通过
5. **提交** — 使用 Conventional Commits 格式：
   - `feat: 添加主题切换功能`
   - `fix: 修复热力图越界问题`
   - `docs: 更新安装说明`
6. **推送并创建 PR**

## 开发环境

```bash
git clone https://github.com/kiki3231/devibe.git
cd devibe
cargo build
cargo run -- --scan ~/projects
```

## 代码风格

- `cargo fmt` — 自动格式化
- `cargo clippy -- -D warnings` — 零警告
- 模块保持单一职责：`scanner.rs`（发现仓库）、`stats.rs`（数据计算）、`widgets.rs`（图表渲染）
- 新增面板在 `widgets.rs` 中添加，在 `app.rs` 中注册键位

## 提交规范

```
<type>: <简短描述>

类型：feat / fix / docs / style / refactor / perf / test / chore
```

## 行为准则

- 保持友善和专业
- 建设性讨论，对事不对人
- 维护者有权删除不符合社区氛围的内容

再次感谢你的贡献！🎉

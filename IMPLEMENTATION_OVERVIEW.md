# Skill Manager — 全量功能实现概览（PRD v2.1 落地）

> 在已完成的 Tier C（用户标注）基础上，本轮把 PRD v2.1 其余全部功能逐一实现并通过 `cargo check` / `npm run build` / `tauri dev` 验证。
> 开发预览窗口已启动（`skill-manager.exe` 运行中，可直接在桌面查看）。**尚未打包**——按偏好先预览、定了再打包。

## 已实现功能清单

### Tier B — 项目感知（核心差异化）
| 项 | 实现 |
|----|------|
| B1 项目识别与绑定 | `open_project`：projectId 优先级 = `.skillconfig` → `git remote` → 路径指纹哈希 |
| B2 双层级聚合 | 项目内各编辑器 `<.claude/skills>` 等 + 全局 skills 合并，标注来源 |
| B3 自动推断相关性 | 确定性信号：依赖清单(package.json/pyproject.toml/Cargo.toml/go.mod/pom.xml) + 框架标记(next/vite/…) + skill 自声明 tags + 项目已引用；**每个推荐带可解释信号**（如「命中技术栈: FastAPI」）|
| B4 用户覆盖配置 | 启用/停用写入 `.skillconfig` overrides；可加额外技能路径 |
| B5 相关性分层 UI | 三区视图：① 本项目技能 / ② 推荐用于本项目（含信号溯源）/ ③ 全局其他（用不上）|
| B6 配置存储 | 采用方案 a：写 `<项目>/.skillconfig`（跟项目走、git 自带）|

### Tier D — 技能应用商店
| 项 | 实现 |
|----|------|
| D1 可自定义源 | github 仓库 / 本地目录 / 远程索引 URL 三类，存 `~/.skill-manager/marketplace.json`，默认预置离线安全的「本机中心仓库」|
| D2 商店浏览 | 聚合多源（本地直扫 / github 浅克隆缓存 / remote 用 curl 取 JSON）；按源筛选、搜索、标「已装」状态 |
| D3 一键安装到目标 | 安装到指定编辑器全局目录或中心库，复用 A12 安全审计 |

### Tier A-P2
- **A14 在线目录浏览** → remote 索引 URL 源（已含）
- **A15 社区审计集成** → `audit_content` 复用审计正则，商店内「审计」按钮读 SKILL.md 预览风险
- **A16 CLI 接口** → 新增 `skill-cli` 二进制（`[[bin]]`），子命令：`editors / list / search / open / project / enable / disable / activate / install / install-local / marketplace / sources / audit / export / import / feedback`，**已实测可用**
- **A17 导入/导出归档** → `export_archive` / `import_archive` 合并 config+标注+场景+商店源为一个 JSON

### Tier E — 增强
- **E1 复用模板** → `export/import_project_template`（.skillconfig 子集，去 projectId）
- **E2 激活联动** → `activate_project_skills` 把启用项同步(symlink/copy)进对应编辑器全局目录
- **E3 反馈闭环** → `record_feedback` 记录采纳事件，`get_feedback_stats` 统计采纳率（归档面板展示）

## 验证结果
- `cargo check`（含 skill-cli bin）**0 错误 0 警告**
- `npm run build` 前端 **0 错误（48 modules）**
- `skill-cli.exe` 实测：列出 14 个编辑器；对 skill-manager 项目正确识别 TS/React/Vite/Tailwind 技术栈并推荐相关技能
- `tauri dev` 重新编译通过（修复多 bin 导致的 `cargo run` 歧义后窗口正常弹出）

## 关键改动文件
- 后端：`project.rs`（新）、`marketplace.rs`（新）、`cli.rs`（新）、`audit.rs`/`installer.rs`/`models.rs`/`commands.rs`/`lib.rs`/`Cargo.toml` 扩展
- 前端：`ProjectPanel.tsx`（新）、`MarketplacePanel.tsx`（新）、`ArchivePanel.tsx`（新）、`Sidebar.tsx`/`App.tsx`/`types.ts`/`api.ts`/`icons.tsx` 扩展

## 下一步
1. 在预览窗口确认各功能体验（尤其项目感知三区与商店浏览）
2. 确认后执行 `npm run tauri build` 打包（产物 < 30MB，用户双击即用）

# Skill Manager

> 本地 AI 编辑器技能管理器 — 一站式管理 Cursor / Claude Code / Codex CLI / WorkBuddy / Windsurf 等 **14 种** AI 编码工具的 Skills。

![Skill Manager 主界面](docs/screenshot.png)

## ✨ 特性

| 模块 | 功能 |
|------|------|
| **全部技能** | 递归扫描 14 种编辑器的本地 skills，搜索、详情预览、GitHub/本地安装/卸载 |
| **项目感知** | 自动识别项目技术栈，智能推荐相关技能，双层级管理（项目内 + 全局） |
| **技能商店** | 自定义安装源（GitHub / 本地目录 / 远程），浏览、审计、一键安装 |
| **一致性检查** | 跨编辑器重复检测、版本比对、symlink/copy 同步 |
| **场景** | 按 DevOps / Frontend / Backend 等场景分类管理技能组合 |
| **归档与反馈** | 导入导出归档包、反馈采纳率统计、项目模板导入导出 |
| **CLI 工具** | `skill-cli` 命令行完整支持所有核心操作 |

## 📸 界面预览

暗色主题设计，左侧导航六大模块，右侧卡片式展示：

- **Hub-and-Spoke 图标系统** — 编辑器专属图标 + 功能图标
- **等高卡片网格** — 统一 218px 最小高度，底部操作栏自动对齐
- **信号溯源** — 项目感知页面展示推荐依据（依赖清单 / 框架标记 / skill 自声明 tags）

## 🚀 安装

### 方式一：NSIS 安装包（推荐）

下载 `releases/Skill Manager_1.0.0_x64-setup.exe`，双击安装即可。

### 方式二：MSI 安装包

下载 `releases/Skill Manager_1.0.0_x64_en-US.msi`，适用于企业部署。

### 方式三：从源码构建

```bash
# 前置条件
# - Rust stable 1.97+ (x86_64-pc-windows-msvc)
# - Node.js 22+
# - WebView2 (Windows 10+ 自带)

git clone https://github.com/aiyinluya/skill-manager.git
cd skill-manager
npm install
npm run tauri build
```

产物在 `src-tauri/target/release/bundle/` 下：
- `nsis/Skill Manager_1.0.0_x64-setup.exe` (~2.4MB)
- `msi/Skill Manager_1.0.0_x64_en-US.msi` (~3.6MB)
- `skill-manager.exe` (~10.5MB，可直接运行)

## 🏗 技术栈

| 层 | 技术 |
|----|------|
| **桌面框架** | Tauri 2（Rust + 系统 WebView2） |
| **后端** | Rust — 9 个独立模块（scanner/installer/audit/sync/marketplace/project/cli...） |
| **前端** | React 18 + TypeScript + TailwindCSS + Vite |
| **产物体积** | exe ~10.5MB / msi ~3.6MB / setup ~2.4MB（均 <30MB） |
| **用户端依赖** | **零** — 双击即用 |

## 📁 项目结构

```
skill-manager/
├── src-tauri/src/          # Rust 后端
│   ├── main.rs             # 入口 & Tauri setup
│   ├── lib.rs              # 模块注册 & 命令绑定
│   ├── commands.rs         # 全部 Tauri invoke 命令 (35+)
│   ├── scanner.rs          # 递归扫描 14 种编辑器 skills
│   ├── installer.rs        # GitHub/本地安装 & 卸载
│   ├── audit.rs            # 安全审计（命令注入/路径穿越/机密泄露）
│   ├── sync.rs            # 跨编辑器同步 (symlink/copy)
│   ├── scenario.rs        # 场景管理
│   ├── project.rs         # Tier B 项目感知
│   ├── marketplace.rs     # Tier D 技能商店
│   ├── cli.rs             # A16 CLI 二进制 (skill-cli)
│   └── models.rs          # 数据结构定义
├── src/                    # React 前端
│   ├── App.tsx            # 主应用 & 路由
│   ├── api.ts            # 后端 invoke 封装
│   ├── types.ts          # TypeScript 类型
│   └── components/       # UI 组件
│       ├── Sidebar.tsx      # 左侧导航
│       ├── SkillCard.tsx    # 技能卡片
│       ├── ProjectPanel.tsx  # 项目感知面板
│       ├── MarketplacePanel.tsx # 技能商店面板
│       ├── ArchivePanel.tsx    # 归档与反馈面板
│       └── icons.tsx        # Hub-and-Spoke 图标集
├── releases/               # 安装包
├── docs/                   # 文档 & 截图
├── IMPLEMENTATION_OVERVIEW.md  # 实现概览
└── requirements.md         # PRD 需求文档 v2.1
```

## 🔧 支持的编辑器

| 编辑器 | Skills 目录 | 图标标识 |
|--------|------------|---------|
| Cursor | `.cursor/skills` | 🟢 |
| Claude Code | `.claude/commands` | 🟠 |
| Codex CLI | `.codex/skills` | 🔷 |
| WorkBuddy | `.workbuddy/skills` | ⚡ |
| Windsurf | `.windsurf/skills` | 🌊 |
| Augment | `.augment/skills` | 🔺 |
| Trae | `.trae/skills` | 🟣 |
| Kilo Code | `.kilocode/skills` | ⚪ |
| Gemini CLI | `.gemini/skills` | 💎 |
| OpenCode | `.ocode/skills` | 📖 |
| GitHub Copilot | `.github/copilot-instructions.txt` | 🐙 |
| Agents | `.agents/skills` | 🤖 |
| Cline | `.cline/skills` | 🤖 |
| Aider | `.aider/skills` | 🤖 |

## 📄 License

MIT

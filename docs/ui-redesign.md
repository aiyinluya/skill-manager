# Skill Manager — 界面重构设计说明（UI Designer）

> 目标：从「后台管理面板」升级为「现代桌面工具」质感（Raycast / Linear 路线），既美观又易用。
> 本次仅重构前端（React/TS），Rust 后端接口不变。预览用 `tauri dev`（开发模式，不打包）。

## 设计原则
- **暗色精修风**：深层背景 + 多层级面板 + 柔和阴影，中心极淡光晕增加空间感。
- **编辑器品牌色驱动**：每个编辑器自带 `color`，用于侧边栏高亮、卡片左侧描边、徽标底色、同步目标 chip——形成「按编辑器认色」的视觉记忆，解决旧版纯文字堆砌的问题。
- **信息层次清晰**：标题 / 副标题 / 元信息三级文字，关键操作常驻、危险/次要操作 hover 浮出。
- **现代微交互**：卡片 hover 微抬升 + 边框提亮，抽屉滑入、遮罩淡入、Toast 上滑、骨架屏 shimmer。
- **可访问性**：focus-visible 焦点环、44px 级触控区、对比度满足 AA、`prefers-reduced-motion` 降级。

## 布局
- **侧边栏（64 宽）**：品牌区（SVG Logo + 名称 + 总数统计）→ 主导航（全部技能 / 一致性检查[带角标] / 场景）→ 编辑器列表（彩色圆点 + 图标 + 名称 + 数量，选中态用编辑器色做左侧条与底色）。
- **顶栏**：页面标题 + 副标题统计 → 搜索框（带图标、200ms 防抖）→ 排序（名称/大小）→ 刷新 → 同步模式分段控件（软链接/复制）→ 安装技能（主按钮）。
- **内容区**：
  - 全部技能：响应式卡片网格（1→5 列），加载时骨架屏，空态有引导 CTA。
  - 一致性检查：重复技能 + 安全审计，编辑器色徽标 + 风险等级徽章（高/中/低）。
  - 场景：场景卡片 + 新建场景弹窗（技能多选列表）。

## 技能卡片
- 左侧 3px 编辑器色描边；顶部编辑器色徽标 + 名称。
- 已同步技能显示编辑器色「已同步」chip（🔗）。
- 名称（截断）+ 两行摘要 + 触发词 mono chip（最多 3 个，余 `+N`）。
- 底部：文件大小（tabular-nums）+ hover 浮出的「同步 / 卸载」操作。

## 详情：右侧滑出抽屉（替代原居中弹窗 dump JSON）
- 头部：编辑器色渐变背景 + 大徽标 + 名称 + 来源/软链信息 + 关闭。
- 标签页：概览 / 内容 / 文件。
  - 概览：摘要、触发词、元信息网格（大小/来源/有效性/同步方式）、**Frontmatter 改为键值列表**（不再裸 JSON）。
  - 内容：等宽预格式化，可滚动。
  - 文件：文件清单 + 大小。
- 底部：卸载（危险）+ 同步到（编辑器彩色 chip 多选）+ 同步。

## 设计 Token（tailwind.config.js）
- 背景：bg `#0c0d11` / panel `#14161c` / panel2 `#191c23` / panel3 `#21252e` / panel4 `#2a2f3a`
- 边框：border `#272b34` / border2 `#343a46`
- 文本：txt `#e9ebf0` / txt2 `#a7adb9` / txt3 `#6c727f`
- 强调：accent `#6366f1` / accent2 `#818cf8` / accent3 `#a5b4fc`
- 字体：Inter（UI）/ JetBrains Mono（代码、触发词）
- 阴影：soft / pop / glow；动画：shimmer / slide-in / fade-in / toast-in

## 文件结构（新增/重写）
- `tailwind.config.js` — 设计 token
- `src/index.css` — 设计系统（按钮/输入/卡片/chip/导航/分段/骨架/抽屉/动画）
- `src/lib/ui.ts` — cn()、tint()（hex→rgba）、severity 映射
- `src/components/icons.tsx` — lucide 风格描边图标集
- `src/components/Logo.tsx` — 品牌 SVG（与 app 图标一致）
- `src/components/Sidebar.tsx` — 侧边栏
- `src/components/SkillCard.tsx` — 技能卡片
- `src/components/SkillDrawer.tsx` — 详情抽屉
- `src/components/InstallModal.tsx` — 安装弹窗
- `src/components/ConsistencyPanel.tsx` — 一致性检查
- `src/components/ScenariosPanel.tsx` — 场景
- `src/App.tsx` — 编排（布局 / 搜索防抖 / 排序）

## 预览与打包
- 预览：`npm run tauri dev`（开发模式，窗口热更新，**不生成安装包**）。
- 打包（确认后再做）：`npm run tauri build` → `src-tauri/target/release/bundle/{msi,nsis}/`。

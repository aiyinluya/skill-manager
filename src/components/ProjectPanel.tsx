import { useState, useEffect, useCallback } from "react";
import type { ProjectView, ProjectSkill, Skill } from "../types";
import { api } from "../api";
import { cn, tint } from "../lib/ui";
import {
  IconFolderOpen,
  IconActivate,
  IconTemplate,
  IconBadge,
  IconNote,
  IconLink,
  IconCheck,
  IconLayers,
  IconX,
} from "./icons";

interface Props {
  showToast: (m: string) => void;
  onOpenSkill: (s: Skill) => void;
}

export function ProjectPanel({ showToast, onOpenSkill }: Props) {
  const [path, setPath] = useState<string | null>(null);
  const [view, setView] = useState<ProjectView | null>(null);
  const [loading, setLoading] = useState(false);
  const [search, setSearch] = useState("");
  const [busy, setBusy] = useState(false);

  const load = useCallback(
    async (p: string) => {
      setLoading(true);
      try {
        const v = await api.projectSkills(p);
        setView(v);
        setPath(p);
      } catch (e) {
        showToast("读取项目失败: " + String(e));
      } finally {
        setLoading(false);
      }
    },
    [showToast]
  );

  const openFolder = async () => {
    const dir = await api.pickDir();
    if (dir) await load(dir);
  };

  const toggle = async (ps: ProjectSkill) => {
    if (!path) return;
    const key = `${ps.skill.editor_id}:${ps.skill.dir_name}`;
    await api.setProjectSkill(path, key, !ps.enabled);
    load(path);
  };

  const generateConfig = async () => {
    if (!path) return;
    const cfg = await api.getProjectConfig(path);
    await api.setProjectExtraPaths(path, cfg.extra_paths);
    showToast("已生成 .skillconfig");
    load(path);
  };

  const activate = async () => {
    if (!path) return;
    setBusy(true);
    try {
      const r = await api.activateProjectSkills(path, view?.info.project_id);
      showToast(`已激活 ${r.count} 个技能到对应编辑器`);
      load(path);
    } finally {
      setBusy(false);
    }
  };

  const exportTemplate = async () => {
    if (!path) return;
    const json = await api.exportProjectTemplate(path);
    await navigator.clipboard?.writeText(json).catch(() => {});
    showToast("项目模板已复制到剪贴板");
  };

  const importTemplate = async () => {
    if (!path) return;
    const json = window.prompt("粘贴项目模板 JSON：");
    if (!json) return;
    await api.importProjectTemplate(path, json);
    showToast("已导入项目模板");
    load(path);
  };

  const q = search.trim().toLowerCase();
  const filter = (list: ProjectSkill[]) =>
    q
      ? list.filter(
          (x) =>
            x.skill.name.toLowerCase().includes(q) ||
            x.skill.summary.toLowerCase().includes(q)
        )
      : list;

  if (!path || !view) {
    return (
      <div className="h-full grid place-items-center">
        <div className="text-center max-w-md px-6">
          <div className="w-16 h-16 rounded-2xl bg-panel2 border border-border grid place-items-center mx-auto mb-4 text-txt3">
            <IconLayers size={30} />
          </div>
          <div className="text-lg font-semibold text-txt">项目感知</div>
          <div className="text-sm text-txt3 mt-2 leading-relaxed">
            打开任意项目文件夹，工具会自动识别项目、聚合「项目自带 + 全局」两层技能，
            并基于技术栈推断哪些技能适用于本项目、哪些用不上。
          </div>
          <button className="btn btn-primary mx-auto mt-5" onClick={openFolder}>
            <IconFolderOpen size={16} />
            打开项目文件夹
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      {/* 项目头 */}
      <div className="px-5 py-3 border-b border-border bg-panel/60 backdrop-blur sticky top-0 z-10">
        <div className="flex items-center gap-3">
          <button className="btn btn-ghost !px-2.5" onClick={openFolder} title="切换项目">
            <IconFolderOpen size={16} />
          </button>
          <div className="min-w-0">
            <div className="flex items-center gap-2">
              <span className="font-semibold text-txt truncate">{view.info.name}</span>
              <span className="chip font-mono !text-[10px]" title="项目 ID">
                {view.info.project_id}
              </span>
              {view.info.has_config ? (
                <span className="chip text-emerald-300 border-emerald-500/30 bg-emerald-500/10">
                  <IconCheck size={11} /> .skillconfig
                </span>
              ) : (
                <button
                  className="chip hover:border-border2"
                  onClick={generateConfig}
                  title="在项目目录生成 .skillconfig"
                >
                  生成配置
                </button>
              )}
            </div>
            <div className="text-[11px] text-txt3 truncate font-mono">{view.info.path}</div>
          </div>
          <div className="flex-1" />
          <div className="relative">
            <IconNote
              size={14}
              className="absolute left-2.5 top-1/2 -translate-y-1/2 text-txt3"
            />
            <input
              className="input pl-8 w-52"
              placeholder="过滤技能…"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
          <button
            className="btn btn-ghost !px-2.5"
            onClick={() => load(path)}
            title="刷新"
          >
            <IconLink size={16} />
          </button>
        </div>
        {/* 技术栈 */}
        <div className="flex items-center gap-1.5 mt-2.5 flex-wrap">
          <span className="text-[11px] text-txt3 mr-1">检测技术栈：</span>
          {view.stack.length === 0 ? (
            <span className="text-[11px] text-txt3 italic">未检测到明确技术栈</span>
          ) : (
            view.stack.map((s) => (
              <span key={s} className="chip">
                {s}
              </span>
            ))
          )}
        </div>
      </div>

      {/* 三区 */}
      <div className="flex-1 overflow-y-auto p-5 space-y-6">
        {loading ? (
          <div className="text-sm text-txt3">扫描中…</div>
        ) : (
          <>
            <Zone
              title="① 本项目技能"
              desc="位于项目目录内的技能（随项目走，通常提交进 git）"
              tone="project"
              skills={filter(view.project)}
              onToggle={toggle}
              onOpen={onOpenSkill}
            />
            <Zone
              title="② 推荐用于本项目"
              desc="全局技能中命中本项目技术栈的强相关项（可溯源信号）"
              tone="recommended"
              skills={filter(view.recommended)}
              onToggle={toggle}
              onOpen={onOpenSkill}
            />
            <Zone
              title="③ 全局其他（用不上 / 可用）"
              desc="未命中本项目技术栈的全局技能，可手动启用"
              tone="available"
              skills={filter(view.available)}
              onToggle={toggle}
              onOpen={onOpenSkill}
            />
          </>
        )}
      </div>

      {/* 底部操作 */}
      <div className="border-t border-border px-5 py-3 flex items-center gap-2">
        <button className="btn btn-primary" onClick={activate} disabled={busy}>
          <IconActivate size={15} />
          {busy ? "激活中…" : "激活到编辑器"}
        </button>
        <button className="btn btn-ghost" onClick={exportTemplate} title="导出为可复用模板">
          <IconTemplate size={15} /> 导出模板
        </button>
        <button className="btn btn-ghost" onClick={importTemplate} title="导入项目模板">
          <IconTemplate size={15} /> 导入模板
        </button>
        <div className="flex-1" />
        <span className="text-[11px] text-txt3 flex items-center gap-1">
          <IconBadge size={13} /> 启用 {countEnabled(view)} / 共 {totalCount(view)} 项
        </span>
      </div>
    </div>
  );
}

function countEnabled(v: ProjectView) {
  return [v.project, v.recommended, v.available]
    .flat()
    .filter((s) => s.enabled).length;
}
function totalCount(v: ProjectView) {
  return v.project.length + v.recommended.length + v.available.length;
}

function Zone({
  title,
  desc,
  tone,
  skills,
  onToggle,
  onOpen,
}: {
  title: string;
  desc: string;
  tone: "project" | "recommended" | "available";
  skills: ProjectSkill[];
  onToggle: (s: ProjectSkill) => void;
  onOpen: (s: Skill) => void;
}) {
  const accent =
    tone === "project"
      ? "#22c55e"
      : tone === "recommended"
      ? "#3B82F6"
      : "#8b93a7";
  return (
    <section>
      <div className="flex items-baseline gap-2 mb-2.5">
        <h3 className="text-sm font-semibold text-txt" style={{ color: accent }}>
          {title}
        </h3>
        <span className="text-[11px] text-txt3">{desc}</span>
        <span className="text-[11px] text-txt3 ml-auto">{skills.length} 项</span>
      </div>
      {skills.length === 0 ? (
        <div className="text-xs text-txt3 italic border border-dashed border-border rounded-xl px-4 py-3">
          无
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3.5">
          {skills.map((ps) => (
            <ProjectSkillCard
              key={`${ps.skill.editor_id}:${ps.skill.dir_name}`}
              ps={ps}
              accent={accent}
              onToggle={onToggle}
              onOpen={onOpen}
            />
          ))}
        </div>
      )}
    </section>
  );
}

function ProjectSkillCard({
  ps,
  accent,
  onToggle,
  onOpen,
}: {
  ps: ProjectSkill;
  accent: string;
  onToggle: (s: ProjectSkill) => void;
  onOpen: (s: Skill) => void;
}) {
  const color = ps.skill.editor_color || "#8b93a7";
  return (
    <div
      className="card group cursor-pointer flex flex-col min-h-[218px]"
      style={{ borderTopColor: ps.enabled ? accent : "transparent" }}
      onClick={() => onOpen(ps.skill)}
    >
      <span
        className="absolute left-0 top-0 h-full w-[3px] rounded-l-2xl"
        style={{ background: color }}
      />
      <div className="flex items-center gap-2 mb-2">
        <span
          className="w-6 h-6 rounded-lg grid place-items-center text-[13px] shrink-0"
          style={{ background: tint(color, 0.16) }}
        >
          {ps.skill.editor_icon}
        </span>
        <span className="text-[11px] text-txt3 truncate flex-1">{ps.skill.editor_name}</span>
        <button
          onClick={(e) => {
            e.stopPropagation();
            onToggle(ps);
          }}
          className={cn(
            "shrink-0 w-9 h-5 rounded-full p-0.5 transition-colors relative",
            ps.enabled ? "bg-emerald-500/80" : "bg-panel4 border border-border"
          )}
          title={ps.enabled ? "已启用（点击停用）" : "已停用（点击启用）"}
        >
          <span
            className={cn(
              "absolute top-0.5 w-4 h-4 rounded-full bg-white transition-all",
              ps.enabled ? "left-[18px]" : "left-0.5"
            )}
          />
        </button>
      </div>

      <div className="font-semibold text-[14px] text-txt truncate leading-snug">
        {ps.skill.name || ps.skill.dir_name}
      </div>
      <div className="text-[11px] text-txt2 mt-1 line-clamp-2 min-h-[1.75rem] leading-relaxed">
        {ps.skill.summary || <span className="text-txt3 italic">（无摘要）</span>}
      </div>

      {/* 可解释信号 */}
      {ps.signals.length > 0 && (
        <div className="mt-2 space-y-1">
          {ps.signals.map((s, i) => (
            <div
              key={i}
              className="text-[10.5px] text-txt3 flex items-start gap-1"
              title={s}
            >
              <IconBadge size={11} className="shrink-0 mt-0.5" style={{ color: accent }} />
              <span className="line-clamp-1">{s}</span>
            </div>
          ))}
        </div>
      )}

      {ps.skill.triggers.length > 0 && (
        <div className="flex flex-wrap gap-1 mt-2">
          {ps.skill.triggers.slice(0, 2).map((t, i) => (
            <span key={i} className="chip font-mono !text-[10px]">
              {t}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

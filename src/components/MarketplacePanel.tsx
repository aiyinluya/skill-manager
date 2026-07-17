import { useState, useEffect, useCallback } from "react";
import type {
  MarketplaceSource,
  MarketplaceSkill,
  MarketplaceType,
  EditorWithCount,
  AuditIssue,
} from "../types";
import { api } from "../api";
import { cn, sevClass, sevLabel } from "../lib/ui";
import {
  IconStore,
  IconPlus,
  IconRefresh,
  IconTrash,
  IconGlobe,
  IconDrive,
  IconLink,
  IconCheck,
  IconAlert,
  IconExternal,
  IconX,
} from "./icons";

interface Props {
  editors: EditorWithCount[];
  showToast: (m: string) => void;
}

export function MarketplacePanel({ editors, showToast }: Props) {
  const [sources, setSources] = useState<MarketplaceSource[]>([]);
  const [skills, setSkills] = useState<MarketplaceSkill[]>([]);
  const [loading, setLoading] = useState(false);
  const [search, setSearch] = useState("");
  const [filterSource, setFilterSource] = useState<string>("all");
  const [showAdd, setShowAdd] = useState(false);
  const [targets, setTargets] = useState<Record<string, string>>({});
  const [audit, setAudit] = useState<{ name: string; issues: AuditIssue[] } | null>(null);

  const loadSources = useCallback(async () => {
    setSources(await api.marketplaceSources());
  }, []);

  useEffect(() => {
    loadSources();
  }, [loadSources]);

  const browse = useCallback(
    async (refresh: boolean) => {
      setLoading(true);
      try {
        const s = await api.browseMarketplace(refresh);
        setSkills(s);
      } catch (e) {
        showToast("浏览失败: " + String(e));
      } finally {
        setLoading(false);
      }
    },
    [showToast]
  );

  const install = async (ms: MarketplaceSkill) => {
    const target = targets[`${ms.source_id}:${ms.dir_name}`] || "central";
    try {
      const r = await api.installFromMarketplace(ms.source_id, ms.dir_name, target);
      showToast(`已安装 ${r.count} 项到 ${target === "central" ? "中心库" : target}`);
      browse(false);
    } catch (e) {
      showToast("安装失败: " + String(e));
    }
  };

  const doAudit = async (ms: MarketplaceSkill) => {
    if (!ms.skill_dir) {
      showToast("远程索引技能暂不支持预览审计");
      return;
    }
    const content = await api.readTextFile(ms.skill_dir + "/SKILL.md");
    const issues = await api.auditContent(content);
    setAudit({ name: ms.name, issues });
  };

  const q = search.trim().toLowerCase();
  const shown = skills.filter((s) => {
    if (filterSource !== "all" && s.source_id !== filterSource) return false;
    if (q && !(s.name.toLowerCase().includes(q) || s.summary.toLowerCase().includes(q)))
      return false;
    return true;
  });

  return (
    <div className="flex flex-col h-full">
      {/* 头 */}
      <div className="px-5 py-3 border-b border-border bg-panel/60 backdrop-blur sticky top-0 z-10">
        <div className="flex items-center gap-3">
          <div className="min-w-0">
            <h1 className="text-[15px] font-semibold text-txt leading-tight">技能应用商店</h1>
            <div className="text-[11px] text-txt3">
              {sources.length} 个源 · {skills.length} 个可安装技能
            </div>
          </div>
          <div className="flex-1" />
          <div className="relative">
            <IconLink
              size={14}
              className="absolute left-2.5 top-1/2 -translate-y-1/2 text-txt3"
            />
            <input
              className="input pl-8 w-52"
              placeholder="搜索商店技能…"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
          <button className="btn btn-ghost !px-2.5" onClick={() => browse(true)} title="刷新（重新拉取源）">
            <IconRefresh size={16} />
          </button>
          <button className="btn btn-ghost" onClick={() => setShowAdd(true)}>
            <IconPlus size={15} /> 添加源
          </button>
          <button className="btn btn-primary" onClick={() => browse(false)}>
            <IconStore size={15} /> 浏览商店
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-5 space-y-5">
        {/* 源管理 */}
        <section>
          <div className="text-[11px] font-semibold uppercase tracking-wider text-txt3 mb-2">
            商店源（可自定义）
          </div>
          <div className="flex flex-wrap gap-2">
            {sources.map((s) => (
              <div
                key={s.id}
                className="flex items-center gap-2 bg-panel2 border border-border rounded-xl px-3 py-2"
              >
                <SourceIcon type={s.source_type} />
                <div className="min-w-0">
                  <div className="text-sm text-txt font-medium leading-tight">{s.name}</div>
                  <div className="text-[10.5px] text-txt3 truncate font-mono max-w-[220px]">
                    {s.url}
                    {s.subpath && ` / ${s.subpath}`}
                  </div>
                </div>
                <button
                  className="text-txt3 hover:text-red-300 ml-1"
                  onClick={async () => {
                    if (!window.confirm(`移除源「${s.name}」？`)) return;
                    setSources(await api.removeMarketplaceSource(s.id));
                  }}
                  title="移除源"
                >
                  <IconTrash size={14} />
                </button>
              </div>
            ))}
            {sources.length === 0 && (
              <span className="text-xs text-txt3 italic">暂无源，点击「添加源」</span>
            )}
          </div>
        </section>

        {/* 过滤 */}
        {sources.length > 0 && (
          <div className="flex items-center gap-1.5 flex-wrap">
            <span
              className={cn(
                "chip cursor-pointer",
                filterSource === "all" && "border-accent text-txt bg-accent/15"
              )}
              onClick={() => setFilterSource("all")}
            >
              全部
            </span>
            {sources.map((s) => (
              <span
                key={s.id}
                className={cn(
                  "chip cursor-pointer",
                  filterSource === s.id && "border-accent text-txt bg-accent/15"
                )}
                onClick={() => setFilterSource(s.id)}
              >
                {s.name}
              </span>
            ))}
          </div>
        )}

        {/* 技能网格 */}
        {loading ? (
          <div className="text-sm text-txt3">浏览中…</div>
        ) : skills.length === 0 ? (
          <div className="text-sm text-txt3 border border-dashed border-border rounded-xl px-4 py-8 text-center">
            还没有浏览过商店。点击右上角「浏览商店」开始（首次会从各源拉取技能清单）。
          </div>
        ) : shown.length === 0 ? (
          <div className="text-sm text-txt3">无匹配技能。</div>
        ) : (
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3.5">
            {shown.map((ms) => (
              <MarketCard
                key={`${ms.source_id}:${ms.dir_name}`}
                ms={ms}
                editors={editors}
                target={targets[`${ms.source_id}:${ms.dir_name}`] || "central"}
                onTarget={(t) =>
                  setTargets((p) => ({ ...p, [`${ms.source_id}:${ms.dir_name}`]: t }))
                }
                onInstall={() => install(ms)}
                onAudit={() => doAudit(ms)}
              />
            ))}
          </div>
        )}
      </div>

      {/* 添加源弹窗 */}
      {showAdd && (
        <AddSourceModal
          onClose={() => setShowAdd(false)}
          onAdd={async (s) => {
            setSources(
              await api.addMarketplaceSource(s.name, s.url, s.source_type, s.branch, s.subpath)
            );
            setShowAdd(false);
            showToast("已添加源");
          }}
        />
      )}

      {/* 审计弹窗 */}
      {audit && (
        <div
          className="fixed inset-0 z-30 drawer-overlay animate-fade-in flex justify-end"
          onClick={() => setAudit(null)}
        >
          <div
            className="w-full max-w-md h-full bg-panel border-l border-border shadow-pop flex flex-col animate-slide-in"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="px-5 py-4 border-b border-border flex items-center gap-2">
              <IconAlert size={18} className="text-amber-300" />
              <span className="font-semibold text-txt">安全审计 · {audit.name}</span>
              <button className="btn-subtle !p-1.5 ml-auto" onClick={() => setAudit(null)}>
                <IconX size={18} />
              </button>
            </div>
            <div className="flex-1 overflow-y-auto p-5 space-y-2">
              {audit.issues.length === 0 ? (
                <div className="text-sm text-emerald-300">未发现风险模式 ✓</div>
              ) : (
                audit.issues.map((iss, i) => (
                  <div key={i} className={cn("rounded-xl px-3 py-2.5", sevClass(iss.severity))}>
                    <div className="flex items-center gap-2 text-sm font-medium">
                      <span>{sevLabel[iss.severity] ?? iss.severity}</span>
                      <span className="opacity-70">· {iss.issue_type}</span>
                    </div>
                    <div className="text-[11px] opacity-80 mt-0.5">{iss.detail}</div>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function SourceIcon({ type }: { type: MarketplaceType }) {
  if (type === "local")
    return <IconDrive size={16} className="text-txt3 shrink-0" />;
  if (type === "remote")
    return <IconExternal size={16} className="text-txt3 shrink-0" />;
  return <IconGlobe size={16} className="text-txt3 shrink-0" />;
}

function MarketCard({
  ms,
  editors,
  target,
  onTarget,
  onInstall,
  onAudit,
}: {
  ms: MarketplaceSkill;
  editors: EditorWithCount[];
  target: string;
  onTarget: (t: string) => void;
  onInstall: () => void;
  onAudit: () => void;
}) {
  const color =
    editors.find((e) => e.id === target)?.color ?? "#8b93a7";
  return (
    <div className="card flex flex-col min-h-[218px]">
      <div className="flex items-center gap-1.5 mb-2">
        <span className="chip !text-[10px] font-mono truncate flex-1">{ms.source_name}</span>
        {ms.installed && (
          <span className="chip text-emerald-300 border-emerald-500/30 bg-emerald-500/10 shrink-0">
            <IconCheck size={11} /> 已装
          </span>
        )}
      </div>
      <div className="font-semibold text-[14px] text-txt truncate leading-snug">
        {ms.name}
      </div>
      <div className="text-[11px] text-txt2 mt-1 line-clamp-2 min-h-[1.75rem] leading-relaxed">
        {ms.summary || <span className="text-txt3 italic">（无摘要）</span>}
      </div>
      {ms.triggers.length > 0 && (
        <div className="flex flex-wrap gap-1 mt-2">
          {ms.triggers.slice(0, 2).map((t, i) => (
            <span key={i} className="chip font-mono !text-[10px]">
              {t}
            </span>
          ))}
        </div>
      )}
      <div className="flex items-center gap-1.5 mt-auto pt-3 mt-3 border-t border-border">
        <select
          className="input !py-1 !text-xs flex-1 min-w-0"
          value={target}
          onChange={(e) => onTarget(e.target.value)}
          style={{ color }}
        >
          <option value="central">中心库</option>
          {editors.map((e) => (
            <option key={e.id} value={e.id}>
              {e.name}
            </option>
          ))}
        </select>
        <button
          className="btn btn-ghost !py-1 !px-2 !text-xs"
          onClick={onAudit}
          title="安装前安全审计"
        >
          <IconAlert size={13} />
        </button>
        <button className="btn btn-primary !py-1 !px-2.5 !text-xs" onClick={onInstall}>
          <IconPlus size={13} /> 安装
        </button>
      </div>
    </div>
  );
}

function AddSourceModal({
  onClose,
  onAdd,
}: {
  onClose: () => void;
  onAdd: (s: {
    name: string;
    url: string;
    source_type: MarketplaceType;
    branch: string;
    subpath: string;
  }) => void;
}) {
  const [name, setName] = useState("");
  const [url, setUrl] = useState("");
  const [type, setType] = useState<MarketplaceType>("github");
  const [branch, setBranch] = useState("");
  const [subpath, setSubpath] = useState("");

  return (
    <div
      className="fixed inset-0 z-30 grid place-items-center bg-black/40 animate-fade-in p-4"
      onClick={onClose}
    >
      <div
        className="w-full max-w-md bg-panel border border-border rounded-2xl shadow-pop p-5"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center gap-2 mb-4">
          <IconPlus size={18} className="text-accent" />
          <span className="font-semibold text-txt">添加商店源</span>
          <button className="btn-subtle !p-1.5 ml-auto" onClick={onClose}>
            <IconX size={18} />
          </button>
        </div>
        <div className="space-y-3">
          <Field label="名称">
            <input className="input w-full" value={name} onChange={(e) => setName(e.target.value)} placeholder="如：团队 Skills 仓库" />
          </Field>
          <Field label="地址（URL / 本地目录）">
            <input
              className="input w-full font-mono"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              placeholder={type === "local" ? "C:/path/to/skills" : type === "remote" ? "https://index.json" : "user/repo 或 https://github.com/user/repo"}
            />
          </Field>
          <div className="grid grid-cols-2 gap-3">
            <Field label="类型">
              <select className="input w-full" value={type} onChange={(e) => setType(e.target.value as MarketplaceType)}>
                <option value="github">GitHub 仓库</option>
                <option value="local">本地目录</option>
                <option value="remote">远程索引 URL</option>
              </select>
            </Field>
            <Field label="分支（可选）">
              <input className="input w-full" value={branch} onChange={(e) => setBranch(e.target.value)} placeholder="main" />
            </Field>
          </div>
          <Field label="子路径（可选）">
            <input className="input w-full font-mono" value={subpath} onChange={(e) => setSubpath(e.target.value)} placeholder="skills" />
          </Field>
        </div>
        <div className="flex justify-end gap-2 mt-5">
          <button className="btn btn-ghost" onClick={onClose}>
            取消
          </button>
          <button
            className="btn btn-primary"
            disabled={!name || !url}
            onClick={() => onAdd({ name, url, source_type: type, branch, subpath })}
          >
            添加
          </button>
        </div>
      </div>
    </div>
  );
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <label className="block">
      <div className="text-[11px] text-txt3 mb-1">{label}</div>
      {children}
    </label>
  );
}

import { useEffect, useState, useCallback, useMemo } from "react";
import { api } from "./api";
import type {
  EditorWithCount,
  Skill,
  SkillDetail,
  Stats,
  Scenario,
  AuditIssue,
  AppConfig,
  ScenarioSkill,
  Annotation,
} from "./types";
import { EMPTY_ANNOTATION, IMPORTANCE_META } from "./types";
import { Sidebar } from "./components/Sidebar";
import { SkillCard } from "./components/SkillCard";
import { SkillDrawer } from "./components/SkillDrawer";
import { InstallModal } from "./components/InstallModal";
import { ConsistencyPanel } from "./components/ConsistencyPanel";
import { ScenariosPanel } from "./components/ScenariosPanel";
import { ProjectPanel } from "./components/ProjectPanel";
import { MarketplacePanel } from "./components/MarketplacePanel";
import { ArchivePanel } from "./components/ArchivePanel";
import {
  IconSearch,
  IconRefresh,
  IconPlus,
  IconGrid,
} from "./components/icons";

type View = "skills" | "consistency" | "scenarios" | "project" | "marketplace" | "archive";
type Sort = "name" | "size" | "importance";

export default function App() {
  const [view, setView] = useState<View>("skills");
  const [editors, setEditors] = useState<EditorWithCount[]>([]);
  const [stats, setStats] = useState<Stats | null>(null);
  const [skills, setSkills] = useState<Skill[]>([]);
  const [selectedEditor, setSelectedEditor] = useState<string>("all");
  const [searchInput, setSearchInput] = useState("");
  const [search, setSearch] = useState("");
  const [sort, setSort] = useState<Sort>("name");
  const [detail, setDetail] = useState<SkillDetail | null>(null);
  const [showInstall, setShowInstall] = useState(false);
  const [scenarios, setScenarios] = useState<Scenario[]>([]);
  const [audit, setAudit] = useState<AuditIssue[]>([]);
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [toast, setToast] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [annotations, setAnnotations] = useState<Record<string, Annotation>>({});
  const [detailId, setDetailId] = useState<string | null>(null);

  const showToast = (msg: string) => {
    setToast(msg);
    setTimeout(() => setToast(null), 3000);
  };

  const loadEditors = useCallback(async () => {
    setEditors(await api.editors());
  }, []);
  const loadStats = useCallback(async () => {
    setStats(await api.stats());
  }, []);
  const loadSkills = useCallback(async () => {
    setLoading(true);
    try {
      const r = await api.skills(
        selectedEditor === "all" ? undefined : selectedEditor,
        search || undefined
      );
      setSkills(r.skills);
    } finally {
      setLoading(false);
    }
  }, [selectedEditor, search]);

  useEffect(() => {
    loadEditors();
    loadStats();
    api.config().then(setConfig);
    api.annotations().then(setAnnotations);
  }, [loadEditors, loadStats]);

  // 搜索防抖
  useEffect(() => {
    const t = setTimeout(() => setSearch(searchInput), 200);
    return () => clearTimeout(t);
  }, [searchInput]);

  useEffect(() => {
    loadSkills();
  }, [loadSkills]);

  const refreshAll = useCallback(() => {
    loadEditors();
    loadStats();
    loadSkills();
  }, [loadEditors, loadStats, loadSkills]);

  const openDetail = async (s: Skill) => {
    const d = await api.skillDetail(s.editor_id, s.dir_name);
    setDetail(d);
    setDetailId(s.id);
  };

  const doUninstall = async (s: Skill) => {
    if (!window.confirm(`确认卸载 ${s.name} (${s.editor_name}) ?`)) return;
    await api.uninstall(s.editor_id, s.dir_name);
    showToast(`已卸载 ${s.name}`);
    setDetail(null);
    refreshAll();
  };

  const runAudit = async () => {
    setAudit(await api.audit());
    setView("consistency");
  };

  const loadScenarios = async () => setScenarios(await api.scenarios());

  const consistencyBadge = useMemo(() => {
    const d = stats ? Object.keys(stats.duplicates).length : 0;
    return d + audit.length;
  }, [stats, audit]);

  const sortedSkills = useMemo(() => {
    const arr = [...skills];
    arr.sort((a, b) => {
      if (sort === "size") return b.file_size - a.file_size;
      if (sort === "importance") {
        const wa = IMPORTANCE_META[annotations[a.id]?.importance ?? "normal"].weight;
        const wb = IMPORTANCE_META[annotations[b.id]?.importance ?? "normal"].weight;
        if (wb !== wa) return wb - wa;
      }
      return (a.name || a.dir_name).localeCompare(b.name || b.dir_name, "zh");
    });
    return arr;
  }, [skills, sort, annotations]);

  const currentEditor = editors.find((e) => e.id === selectedEditor);
  const title =
    view === "skills"
      ? selectedEditor === "all"
        ? "全部技能"
        : currentEditor?.name ?? "技能"
      : view === "consistency"
      ? "一致性检查"
      : "场景";
  const subtitle =
    view === "skills"
      ? `${sortedSkills.length} 个技能${search ? ` · 匹配“${search}”` : ""}`
      : view === "consistency"
      ? "重复检测与安全审计"
      : "技能组合管理";

  return (
    <div className="flex h-full">
      <Sidebar
        editors={editors}
        stats={stats}
        view={view}
        selectedEditor={selectedEditor}
        consistencyBadge={consistencyBadge}
        onNav={(v) => {
          setView(v);
          if (v === "skills") setSelectedEditor("all");
          if (v === "scenarios") loadScenarios();
        }}
        onSelectEditor={(id) => {
          setSelectedEditor(id);
          setView("skills");
        }}
      />

      <main className="flex-1 flex flex-col min-w-0">
        {/* 顶栏 */}
        <header className="flex items-center gap-3 px-5 py-3 border-b border-border bg-panel/60 backdrop-blur sticky top-0 z-10">
          <div className="min-w-0">
            <h1 className="text-[15px] font-semibold text-txt leading-tight truncate">
              {title}
            </h1>
            <div className="text-[11px] text-txt3">{subtitle}</div>
          </div>

          <div className="flex-1" />

          {view === "skills" && (
            <>
              <div className="relative">
                <IconSearch
                  size={15}
                  className="absolute left-2.5 top-1/2 -translate-y-1/2 text-txt3 pointer-events-none"
                />
                <input
                  className="input pl-8 w-56 lg:w-72"
                  placeholder="搜索名称 / 摘要 / 触发词…"
                  value={searchInput}
                  onChange={(e) => setSearchInput(e.target.value)}
                />
              </div>

              <select
                className="input !py-1.5 w-auto"
                value={sort}
                onChange={(e) => setSort(e.target.value as Sort)}
                title="排序"
              >
                <option value="name">名称</option>
                <option value="size">大小</option>
                <option value="importance">重要性</option>
              </select>
            </>
          )}

          <button
            className="btn btn-ghost !px-2.5"
            onClick={refreshAll}
            title="刷新"
          >
            <IconRefresh size={16} />
          </button>

          {view === "skills" && config && (
            <div className="hidden sm:flex items-center gap-0.5 p-0.5 bg-panel2 border border-border rounded-lg">
              {(["symlink", "copy"] as const).map((m) => (
                <button
                  key={m}
                  onClick={async () => {
                    await api.setSyncMode(m);
                    setConfig((c) => (c ? { ...c, sync_mode: m } : c));
                    showToast(`同步模式: ${m === "symlink" ? "软链接" : "复制"}`);
                  }}
                  className={
                    "seg " +
                    (config.sync_mode === m ? "seg-active" : "")
                  }
                >
                  {m === "symlink" ? "软链接" : "复制"}
                </button>
              ))}
            </div>
          )}

          <button
            className="btn btn-primary"
            onClick={() => setShowInstall(true)}
          >
            <IconPlus size={16} />
            安装技能
          </button>
        </header>

        {/* 内容 */}
        <div className="flex-1 overflow-y-auto p-5">
          {view === "skills" && (
            loading ? (
              <SkeletonGrid />
            ) : sortedSkills.length === 0 ? (
              <EmptySkills onInstall={() => setShowInstall(true)} />
            ) : (
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3.5">
                {sortedSkills.map((s) => (
                  <SkillCard
                    key={s.id}
                    skill={s}
                    annotation={annotations[s.id]}
                    onOpen={openDetail}
                    onUninstall={doUninstall}
                    onSync={(sk) => openDetail(sk)}
                  />
                ))}
              </div>
            )
          )}

          {view === "consistency" && (
            <ConsistencyPanel stats={stats} audit={audit} editors={editors} />
          )}

          {view === "scenarios" && (
            <ScenariosPanel
              scenarios={scenarios}
              skills={skills}
              editors={editors}
              onLoad={loadScenarios}
              showToast={showToast}
            />
          )}

          {view === "project" && (
            <ProjectPanel showToast={showToast} onOpenSkill={openDetail} />
          )}

          {view === "marketplace" && (
            <MarketplacePanel editors={editors} showToast={showToast} />
          )}

          {view === "archive" && <ArchivePanel showToast={showToast} />}
        </div>
      </main>

      {/* 抽屉 / 弹窗 */}
      {detail && (
        <SkillDrawer
          detail={detail}
          editors={editors}
          annotation={detailId ? (annotations[detailId] ?? EMPTY_ANNOTATION) : EMPTY_ANNOTATION}
          onClose={() => setDetail(null)}
          onUninstall={() =>
            doUninstall(
              skills.find(
                (s) => s.id === `${detail.editor_id}:${detail.dir_name}`
              ) ?? ({} as Skill)
            )
          }
          onSynced={() => {
            showToast("已同步");
            refreshAll();
          }}
          onSaved={(a) => {
            if (detailId) setAnnotations((prev) => ({ ...prev, [detailId]: a }));
          }}
          showToast={showToast}
        />
      )}
      {showInstall && (
        <InstallModal
          editors={editors}
          onClose={() => setShowInstall(false)}
          onDone={(msg) => {
            showToast(msg);
            setShowInstall(false);
            refreshAll();
          }}
        />
      )}

      {toast && (
        <div className="fixed bottom-6 left-1/2 -translate-x-1/2 z-40 bg-panel4 border border-border px-4 py-2.5 rounded-xl shadow-pop text-sm text-txt animate-toast-in">
          {toast}
        </div>
      )}
    </div>
  );
}

function SkeletonGrid() {
  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3.5">
      {Array.from({ length: 10 }).map((_, i) => (
        <div key={i} className="card !cursor-default">
          <div className="flex items-center gap-2 mb-3">
            <div className="skeleton w-7 h-7 rounded-lg" />
            <div className="skeleton h-3 w-20 rounded" />
          </div>
          <div className="skeleton h-4 w-3/4 rounded mb-2" />
          <div className="skeleton h-3 w-full rounded mb-1.5" />
          <div className="skeleton h-3 w-5/6 rounded" />
          <div className="flex gap-1.5 mt-3">
            <div className="skeleton h-4 w-10 rounded" />
            <div className="skeleton h-4 w-12 rounded" />
          </div>
        </div>
      ))}
    </div>
  );
}

function EmptySkills({ onInstall }: { onInstall: () => void }) {
  return (
    <div className="h-full grid place-items-center">
      <div className="text-center">
        <div className="w-14 h-14 rounded-2xl bg-panel2 border border-border grid place-items-center mx-auto mb-3 text-txt3">
          <IconGrid size={26} />
        </div>
        <div className="text-txt font-medium">没有匹配的技能</div>
        <div className="text-sm text-txt3 mt-1 mb-4">
          换个关键词，或安装新的技能试试。
        </div>
        <button className="btn btn-primary mx-auto" onClick={onInstall}>
          <IconPlus size={15} />
          安装技能
        </button>
      </div>
    </div>
  );
}

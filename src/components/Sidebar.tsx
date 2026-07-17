import type { EditorWithCount, Stats } from "../types";
import { Logo } from "./Logo";
import {
  IconGrid,
  IconScan,
  IconFilm,
  IconLayers,
  IconStore,
  IconArchive,
} from "./icons";
import { cn, tint } from "../lib/ui";

type View = "skills" | "consistency" | "scenarios" | "project" | "marketplace" | "archive";

interface Props {
  editors: EditorWithCount[];
  stats: Stats | null;
  view: View;
  selectedEditor: string;
  consistencyBadge: number;
  onNav: (v: View) => void;
  onSelectEditor: (id: string) => void;
}

export function Sidebar({
  editors,
  stats,
  view,
  selectedEditor,
  consistencyBadge,
  onNav,
  onSelectEditor,
}: Props) {
  const total = stats?.total_skills ?? 0;
  const activeEditors = stats?.active_editors ?? 0;

  return (
    <aside className="w-64 shrink-0 flex flex-col bg-panel/80 border-r border-border">
      {/* 品牌 */}
      <div className="px-4 pt-4 pb-3">
        <div className="flex items-center gap-2.5">
          <Logo size={30} />
          <div className="leading-tight">
            <div className="text-[15px] font-semibold tracking-tight text-txt">
              Skill Manager
            </div>
            <div className="text-[11px] text-txt3">
              {total} 个技能 · {activeEditors} 个编辑器
            </div>
          </div>
        </div>
      </div>

      {/* 主导航 */}
      <nav className="px-3 space-y-1">
        <button
          className={cn("nav-item", view === "skills" && "nav-item-active")}
          onClick={() => onNav("skills")}
        >
          <IconGrid size={17} className="text-txt3" />
          <span className="flex-1 text-left">全部技能</span>
        </button>
        <button
          className={cn("nav-item", view === "project" && "nav-item-active")}
          onClick={() => onNav("project")}
        >
          <IconLayers size={17} className="text-txt3" />
          <span className="flex-1 text-left">项目感知</span>
        </button>
        <button
          className={cn("nav-item", view === "marketplace" && "nav-item-active")}
          onClick={() => onNav("marketplace")}
        >
          <IconStore size={17} className="text-txt3" />
          <span className="flex-1 text-left">技能商店</span>
        </button>
        <button
          className={cn("nav-item", view === "consistency" && "nav-item-active")}
          onClick={() => onNav("consistency")}
        >
          <IconScan size={17} className="text-txt3" />
          <span className="flex-1 text-left">一致性检查</span>
          {consistencyBadge > 0 && (
            <span className="text-[11px] font-semibold bg-red-500/20 text-red-300 px-1.5 py-0.5 rounded-full">
              {consistencyBadge}
            </span>
          )}
        </button>
        <button
          className={cn("nav-item", view === "scenarios" && "nav-item-active")}
          onClick={() => onNav("scenarios")}
        >
          <IconFilm size={17} className="text-txt3" />
          <span className="flex-1 text-left">场景</span>
        </button>
        <button
          className={cn("nav-item", view === "archive" && "nav-item-active")}
          onClick={() => onNav("archive")}
        >
          <IconArchive size={17} className="text-txt3" />
          <span className="flex-1 text-left">归档与反馈</span>
        </button>
      </nav>

      {/* 编辑器列表 */}
      <div className="px-4 mt-5 mb-1.5 flex items-center justify-between">
        <span className="text-[11px] font-semibold uppercase tracking-wider text-txt3">
          编辑器
        </span>
        <span className="text-[11px] text-txt3">{editors.length}</span>
      </div>
      <div className="flex-1 overflow-y-auto px-3 pb-3 space-y-0.5">
        <button
          className={cn(
            "nav-item",
            selectedEditor === "all" && "nav-item-active"
          )}
          onClick={() => onSelectEditor("all")}
        >
          <span className="w-5 h-5 rounded-md bg-panel4 grid place-items-center text-[11px]">
            All
          </span>
          <span className="flex-1 text-left">全部</span>
          <span className="text-[11px] tabular-nums text-txt3">{total}</span>
        </button>
        {editors.map((e) => {
          const active = selectedEditor === e.id;
          return (
            <button
              key={e.id}
              className={cn("nav-item relative", active && "nav-item-active")}
              onClick={() => onSelectEditor(e.id)}
              style={
                active
                  ? { background: tint(e.color, 0.16), color: "#e9ebf0" }
                  : undefined
              }
            >
              {active && (
                <span
                  className="absolute left-0 top-1/2 -translate-y-1/2 w-[3px] h-5 rounded-r"
                  style={{ background: e.color }}
                />
              )}
              <span
                className="w-5 h-5 rounded-md grid place-items-center text-[13px] shrink-0"
                style={{ background: tint(e.color, 0.18) }}
              >
                {e.icon}
              </span>
              <span
                className={cn(
                  "flex-1 text-left truncate",
                  !e.exists && "opacity-40"
                )}
              >
                {e.name}
              </span>
              <span className="text-[11px] tabular-nums text-txt3">
                {e.skill_count}
              </span>
            </button>
          );
        })}
      </div>
    </aside>
  );
}

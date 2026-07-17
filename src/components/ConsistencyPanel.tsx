import type { Stats, AuditIssue, EditorWithCount } from "../types";
import { sevClass, sevLabel, tint } from "../lib/ui";
import { IconAlert, IconLayers } from "./icons";

interface Props {
  stats: Stats | null;
  audit: AuditIssue[];
  editors: EditorWithCount[];
}

export function ConsistencyPanel({ stats, audit, editors }: Props) {
  const dupEntries = stats ? Object.entries(stats.duplicates) : [];
  const colorOf = (id: string) =>
    editors.find((e) => e.id === id)?.color ?? "#8b93a7";

  return (
    <div className="space-y-7 max-w-3xl">
      <section>
        <div className="flex items-center gap-2 mb-3">
          <IconLayers size={17} className="text-txt3" />
          <h2 className="text-sm font-semibold text-txt">
            重复技能
          </h2>
          <span className="text-xs text-txt3">({dupEntries.length})</span>
        </div>
        {dupEntries.length === 0 ? (
          <Empty text="未检测到跨编辑器重复技能。" />
        ) : (
          <div className="space-y-2.5">
            {dupEntries.map(([name, locs]) => (
              <div
                key={name}
                className="bg-panel2 border border-border rounded-xl p-3.5"
              >
                <div className="font-medium text-txt">{name}</div>
                <div className="flex flex-wrap items-center gap-1.5 mt-2">
                  {locs.map((l) => (
                    <span
                      key={l.editor_id}
                      className="inline-flex items-center gap-1.5 rounded-lg px-2 py-1 text-xs border border-border"
                      style={{
                        background: tint(colorOf(l.editor_id), 0.14),
                      }}
                    >
                      <span
                        className="w-2 h-2 rounded-full"
                        style={{ background: colorOf(l.editor_id) }}
                      />
                      {l.editor_name}
                    </span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        )}
      </section>

      <section>
        <div className="flex items-center gap-2 mb-3">
          <IconAlert size={17} className="text-txt3" />
          <h2 className="text-sm font-semibold text-txt">安全审计</h2>
          <span className="text-xs text-txt3">({audit.length})</span>
        </div>
        {audit.length === 0 ? (
          <Empty text="未发现风险模式。" good />
        ) : (
          <div className="space-y-2.5">
            {audit.map((a, i) => (
              <div
                key={i}
                className="bg-panel2 border border-border rounded-xl p-3.5 flex items-start gap-3"
              >
                <span
                  className={
                    "text-[11px] font-semibold px-2 py-0.5 rounded-md shrink-0 " +
                    sevClass(a.severity)
                  }
                >
                  {sevLabel[a.severity] ?? a.severity}
                </span>
                <div className="min-w-0">
                  <div className="text-sm text-txt">
                    <span className="font-medium">{a.skill_name}</span>
                    <span className="text-txt3"> · {a.issue_type}</span>
                  </div>
                  <div className="text-xs text-txt2 mt-1 leading-relaxed">
                    {a.detail}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </section>
    </div>
  );
}

function Empty({ text, good }: { text: string; good?: boolean }) {
  return (
    <div
      className={
        "rounded-xl border border-dashed px-4 py-6 text-center text-sm " +
        (good
          ? "border-emerald-500/30 text-emerald-300/80 bg-emerald-500/5"
          : "border-border text-txt3")
      }
    >
      {text}
    </div>
  );
}

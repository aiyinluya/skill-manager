import { useState, useEffect } from "react";
import type { SkillDetail, EditorWithCount, Annotation, Importance } from "../types";
import { IMPORTANCE_META } from "../types";
import { api } from "../api";
import { cn, tint } from "../lib/ui";
import {
  IconX,
  IconTrash,
  IconLink,
  IconFile,
  IconLayers,
  IconCheck,
  IconNote,
} from "./icons";

type Tab = "overview" | "content" | "files";

interface Props {
  detail: SkillDetail;
  editors: EditorWithCount[];
  annotation: Annotation;
  onClose: () => void;
  onUninstall: () => void;
  onSynced: () => void;
  onSaved: (a: Annotation) => void;
  showToast: (m: string) => void;
}

function fmtValue(v: unknown): string {
  if (typeof v === "string" || typeof v === "number" || typeof v === "boolean")
    return String(v);
  if (Array.isArray(v)) return v.join(", ");
  if (v && typeof v === "object") return JSON.stringify(v);
  return String(v);
}

export function SkillDrawer({
  detail,
  editors,
  annotation,
  onClose,
  onUninstall,
  onSynced,
  onSaved,
  showToast,
}: Props) {
  const [tab, setTab] = useState<Tab>("overview");
  const [targets, setTargets] = useState<string[]>([]);
  const [busy, setBusy] = useState(false);
  const [notes, setNotes] = useState(annotation.notes);
  const [importance, setImportance] = useState<Importance>(annotation.importance);
  useEffect(() => {
    setNotes(annotation.notes);
    setImportance(annotation.importance);
  }, [annotation]);

  const saveAnn = async () => {
    const id = `${detail.editor_id}:${detail.dir_name}`;
    const a = await api.saveAnnotation(id, notes.trim(), importance);
    onSaved(a);
    showToast("标注已保存");
  };

  const color =
    editors.find((e) => e.id === detail.editor_id)?.color ?? "#8b93a7";

  const others = editors.filter(
    (e) => e.id !== detail.editor_id && e.exists
  );

  const toggle = (id: string) =>
    setTargets((t) =>
      t.includes(id) ? t.filter((x) => x !== id) : [...t, id]
    );

  const doSync = async () => {
    if (targets.length === 0) {
      showToast("请选择目标编辑器");
      return;
    }
    setBusy(true);
    try {
      await api.copySkill(detail.editor_id, detail.dir_name, targets);
      showToast(`已同步到 ${targets.length} 个编辑器`);
      onSynced();
      setTargets([]);
    } finally {
      setBusy(false);
    }
  };

  return (
    <div
      className="fixed inset-0 z-30 drawer-overlay animate-fade-in flex justify-end"
      onClick={onClose}
    >
      <div
        className="w-full max-w-xl h-full bg-panel border-l border-border shadow-pop flex flex-col animate-slide-in"
        onClick={(e) => e.stopPropagation()}
      >
        {/* 头部 */}
        <div
          className="px-5 py-4 border-b border-border flex items-start gap-3"
          style={{
            background: `linear-gradient(180deg, ${tint(color, 0.12)}, transparent)`,
          }}
        >
          <span
            className="w-10 h-10 rounded-xl grid place-items-center text-xl shrink-0"
            style={{ background: tint(color, 0.18) }}
          >
            {detail.editor_icon}
          </span>
          <div className="flex-1 min-w-0">
            <div className="text-lg font-semibold text-txt truncate">
              {detail.name}
            </div>
            <div className="text-xs text-txt3 mt-0.5 flex items-center gap-1.5 flex-wrap">
              <span>{detail.editor_name}</span>
              <span className="opacity-50">·</span>
              <span className="font-mono">{detail.dir_name}</span>
              {detail.is_symlink && detail.symlink_target && (
                <>
                  <span className="opacity-50">·</span>
                  <span className="text-txt2 inline-flex items-center gap-1">
                    <IconLink size={12} />
                    {detail.symlink_target}
                  </span>
                </>
              )}
            </div>
          </div>
          <button className="btn-subtle !p-1.5" onClick={onClose} title="关闭">
            <IconX size={18} />
          </button>
        </div>

        {/* 标签页 */}
        <div className="px-5 flex gap-1 border-b border-border">
          {([
            ["overview", "概览"],
            ["content", "内容"],
            ["files", "文件"],
          ] as [Tab, string][]).map(([k, label]) => (
            <button
              key={k}
              onClick={() => setTab(k)}
              className={cn(
                "relative px-3 py-2.5 text-sm transition-colors",
                tab === k ? "text-txt font-medium" : "text-txt3 hover:text-txt2"
              )}
            >
              {label}
              {tab === k && (
                <span className="absolute left-2 right-2 -bottom-px h-0.5 rounded bg-accent" />
              )}
            </button>
          ))}
        </div>

        {/* 内容 */}
        <div className="flex-1 overflow-y-auto px-5 py-4">
          {tab === "overview" && (
            <div className="space-y-5">
              {detail.summary && (
                <p className="text-sm text-txt2 leading-relaxed">
                  {detail.summary}
                </p>
              )}

              {detail.triggers.length > 0 && (
                <div>
                  <div className="text-[11px] font-semibold uppercase tracking-wider text-txt3 mb-1.5">
                    触发词
                  </div>
                  <div className="flex flex-wrap gap-1.5">
                    {detail.triggers.map((t, i) => (
                      <span key={i} className="chip font-mono">
                        {t}
                      </span>
                    ))}
                  </div>
                </div>
              )}

              <div className="grid grid-cols-2 gap-3">
                <Meta label="大小" value={detail.file_size_formatted} />
                <Meta label="来源编辑器" value={detail.editor_name} />
                <Meta
                  label="有效性"
                  value={detail.is_valid ? "有效" : "无效"}
                  tone={detail.is_valid ? "ok" : "bad"}
                />
                <Meta
                  label="同步方式"
                  value={detail.is_symlink ? "软链接" : "独立副本"}
                />
              </div>

              {Object.keys(detail.frontmatter).length > 0 && (
                <div>
                  <div className="text-[11px] font-semibold uppercase tracking-wider text-txt3 mb-1.5">
                    Frontmatter
                  </div>
                  <div className="bg-panel2 border border-border rounded-xl divide-y divide-border overflow-hidden">
                    {Object.entries(detail.frontmatter).map(([k, v]) => (
                      <div
                        key={k}
                        className="flex gap-3 px-3 py-2 text-sm"
                      >
                        <span className="text-txt3 font-mono text-xs w-32 shrink-0 pt-0.5">
                          {k}
                        </span>
                        <span className="text-txt2 break-words flex-1">
                          {fmtValue(v)}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* 我的标注（C1 备注 + C2 重要性） */}
              <div>
                <div className="text-[11px] font-semibold uppercase tracking-wider text-txt3 mb-1.5 flex items-center gap-1.5">
                  <IconNote size={13} /> 我的标注
                </div>
                <textarea
                  className="input w-full h-20 resize-none"
                  placeholder="给这个技能加个备注，方便一眼看到用途…"
                  value={notes}
                  onChange={(e) => setNotes(e.target.value)}
                />
                <div className="flex items-center gap-2 mt-2.5">
                  <span className="text-xs text-txt3 shrink-0">重要性</span>
                  <div className="flex gap-1">
                    {(["normal", "important", "critical"] as Importance[]).map((lv) => {
                      const m = IMPORTANCE_META[lv];
                      const on = importance === lv;
                      return (
                        <button
                          key={lv}
                          onClick={() => setImportance(lv)}
                          className={cn(
                            "inline-flex items-center gap-1 rounded-lg px-2.5 py-1 text-xs border transition-colors",
                            on ? "text-txt" : "border-border text-txt2 hover:border-border2"
                          )}
                          style={
                            on
                              ? { borderColor: m.color, background: m.color + "22", color: m.color }
                              : undefined
                          }
                        >
                          <span
                            className="w-1.5 h-1.5 rounded-full"
                            style={{ background: m.color }}
                          />
                          {m.label}
                        </button>
                      );
                    })}
                  </div>
                  <div className="flex-1" />
                  <button className="btn btn-primary !py-1.5 !px-3 !text-xs" onClick={saveAnn}>
                    保存标注
                  </button>
                </div>
              </div>
            </div>
          )}

          {tab === "content" && (
            <pre className="bg-panel2 border border-border rounded-xl p-4 text-xs leading-relaxed text-txt2 font-mono whitespace-pre-wrap max-h-full overflow-auto">
              {detail.content || "（无内容）"}
            </pre>
          )}

          {tab === "files" && (
            <div className="bg-panel2 border border-border rounded-xl divide-y divide-border overflow-hidden">
              {detail.all_files.map((f, i) => (
                <div key={i} className="flex items-center gap-2 px-3 py-2">
                  <IconFile size={15} className="text-txt3 shrink-0" />
                  <span className="text-sm text-txt2 truncate flex-1 font-mono">
                    {f.path}
                  </span>
                  <span className="text-xs text-txt3 tabular-nums shrink-0">
                    {f.size_formatted}
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* 底部操作 */}
        <div className="border-t border-border px-5 py-3.5">
          <div className="flex items-center gap-2 mb-3">
            <IconLayers size={15} className="text-txt3 shrink-0" />
            <span className="text-xs text-txt2">同步到：</span>
            <div className="flex flex-wrap gap-1.5 flex-1">
              {others.length === 0 && (
                <span className="text-xs text-txt3">无其他可用编辑器</span>
              )}
              {others.map((e) => {
                const on = targets.includes(e.id);
                return (
                  <button
                    key={e.id}
                    onClick={() => toggle(e.id)}
                    className={cn(
                      "inline-flex items-center gap-1 rounded-lg px-2 py-1 text-xs border transition-colors",
                      on
                        ? "border-accent text-txt bg-accent/15"
                        : "border-border text-txt2 hover:border-border2"
                    )}
                    style={
                      on
                        ? { borderColor: e.color, background: tint(e.color, 0.16) }
                        : undefined
                    }
                  >
                    <span
                      className="w-3.5 h-3.5 rounded grid place-items-center text-[9px]"
                      style={{ background: tint(e.color, 0.2) }}
                    >
                      {e.icon}
                    </span>
                    {e.name}
                    {on && <IconCheck size={12} className="text-accent2" />}
                  </button>
                );
              })}
            </div>
          </div>
          <div className="flex items-center gap-2.5">
            <button
              className="btn btn-danger"
              onClick={onUninstall}
            >
              <IconTrash size={15} />
              卸载
            </button>
            <div className="flex-1" />
            <button
              className="btn btn-primary"
              onClick={doSync}
              disabled={busy}
            >
              <IconLink size={15} />
              {busy ? "同步中…" : "同步"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

function Meta({
  label,
  value,
  tone,
}: {
  label: string;
  value: string;
  tone?: "ok" | "bad";
}) {
  return (
    <div className="bg-panel2 border border-border rounded-xl px-3 py-2.5">
      <div className="text-[11px] text-txt3 mb-0.5">{label}</div>
      <div
        className={cn(
          "text-sm font-medium",
          tone === "ok" && "text-emerald-300",
          tone === "bad" && "text-red-300",
          !tone && "text-txt"
        )}
      >
        {value}
      </div>
    </div>
  );
}

import type { Skill, Annotation } from "../types";
import { IMPORTANCE_META } from "../types";
import { tint } from "../lib/ui";
import { IconLink as Link, IconTrash as Trash, IconSync, IconNote } from "../components/icons";

interface Props {
  skill: Skill;
  annotation?: Annotation;
  onOpen: (s: Skill) => void;
  onUninstall: (s: Skill) => void;
  onSync: (s: Skill) => void;
}

export function SkillCard({ skill, annotation, onOpen, onUninstall, onSync }: Props) {
  const color = skill.editor_color || "#8b93a7";
  const importance = annotation?.importance ?? "normal";
  const imp = IMPORTANCE_META[importance];
  const notes = annotation?.notes?.trim();

  return (
    <div
      className="card group flex flex-col min-h-[218px]"
      style={{ borderTopColor: importance !== "normal" ? imp.color : tint(color, 0.55) }}
      onClick={() => onOpen(skill)}
    >
      {/* 顶部品牌色描边 */}
      <span
        className="absolute left-0 top-0 h-full w-[3px] rounded-l-2xl"
        style={{ background: color }}
      />

      <div className="flex items-center gap-2 mb-2.5">
        <span
          className="w-7 h-7 rounded-lg grid place-items-center text-[15px] shrink-0"
          style={{ background: tint(color, 0.16) }}
        >
          {skill.editor_icon}
        </span>
        <span className="text-xs text-txt3 truncate flex-1">
          {skill.editor_name}
        </span>
        {importance !== "normal" && (
          <span
            className="inline-flex items-center gap-1 text-[10px] font-medium shrink-0"
            style={{ color: imp.color }}
            title={`重要性：${imp.label}`}
          >
            <span className="w-1.5 h-1.5 rounded-full" style={{ background: imp.color }} />
            {imp.label}
          </span>
        )}
        {skill.is_symlink && (
          <span
            className="chip"
            style={{
              background: tint(color, 0.14),
              borderColor: tint(color, 0.35),
              color,
            }}
            title="软链接同步"
          >
            <Link size={11} />
            已同步
          </span>
        )}
      </div>

      <div className="font-semibold text-[15px] text-txt truncate leading-snug">
        {skill.name || skill.dir_name}
      </div>

      <div className="text-xs text-txt2 mt-1.5 line-clamp-2 min-h-[2rem] leading-relaxed">
        {skill.summary || (
          <span className="text-txt3 italic">（无摘要）</span>
        )}
      </div>

      {skill.triggers.length > 0 && (
        <div className="flex flex-wrap gap-1 mt-2.5">
          {skill.triggers.slice(0, 3).map((t, i) => (
            <span key={i} className="chip font-mono">
              {t}
            </span>
          ))}
          {skill.triggers.length > 3 && (
            <span className="chip text-txt3">
              +{skill.triggers.length - 3}
            </span>
          )}
        </div>
      )}

      {notes && (
        <div className="flex items-start gap-1.5 mt-2.5 text-txt3">
          <IconNote size={13} className="shrink-0 mt-0.5" />
          <span className="text-[11px] italic line-clamp-1 leading-snug">{notes}</span>
        </div>
      )}

      <div className="flex items-center justify-between mt-auto pt-3 mt-3 border-t border-border">
        <span className="text-[11px] text-txt3 tabular-nums">
          {skill.file_size_formatted}
        </span>
        <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
          <button
            className="btn btn-ghost !py-1 !px-2 !text-xs"
            onClick={(e) => {
              e.stopPropagation();
              onSync(skill);
            }}
            title="同步到其他编辑器"
          >
            <IconSync size={13} />
            同步
          </button>
          <button
            className="btn !py-1 !px-2 !text-xs text-red-300 hover:bg-red-500/15 hover:text-red-200"
            onClick={(e) => {
              e.stopPropagation();
              onUninstall(skill);
            }}
            title="卸载"
          >
            <Trash size={13} />
          </button>
        </div>
      </div>
    </div>
  );
}

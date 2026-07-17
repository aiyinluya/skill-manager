import { useState } from "react";
import type {
  Scenario,
  Skill,
  EditorWithCount,
  ScenarioSkill,
} from "../types";
import { api } from "../api";
import { tint } from "../lib/ui";
import {
  IconPlus,
  IconTrash,
  IconLink,
  IconX,
  IconFilm,
} from "./icons";

interface Props {
  scenarios: Scenario[];
  skills: Skill[];
  editors: EditorWithCount[];
  onLoad: () => void;
  showToast: (m: string) => void;
}

export function ScenariosPanel({
  scenarios,
  skills,
  editors,
  onLoad,
  showToast,
}: Props) {
  const [showCreate, setShowCreate] = useState(false);
  const colorOf = (id: string) =>
    editors.find((e) => e.id === id)?.color ?? "#8b93a7";

  const apply = async (id: string) => {
    if (!window.confirm("确认应用该场景到所有编辑器？")) return;
    try {
      await api.applyScenario(id);
      showToast("场景已应用");
    } catch (e: any) {
      showToast(`应用失败: ${e?.toString() ?? ""}`);
    }
  };
  const remove = async (id: string) => {
    await api.deleteScenario(id);
    onLoad();
  };

  return (
    <div className="space-y-4 max-w-3xl">
      <div className="flex items-center justify-between">
        <div className="text-sm text-txt2">
          保存一组技能组合，一键同步到所有编辑器。
        </div>
        <button
          className="btn btn-primary"
          onClick={() => setShowCreate(true)}
        >
          <IconPlus size={15} />
          新建场景
        </button>
      </div>

      {scenarios.length === 0 ? (
        <div className="rounded-xl border border-dashed border-border px-4 py-10 text-center">
          <IconFilm size={28} className="text-txt3 mx-auto mb-2" />
          <div className="text-sm text-txt3">还没有场景，点击右上角创建。</div>
        </div>
      ) : (
        <div className="space-y-2.5">
          {scenarios.map((sc) => (
            <div
              key={sc.id}
              className="bg-panel2 border border-border rounded-xl p-3.5 flex items-center gap-3"
            >
              <div className="flex-1 min-w-0">
                <div className="font-medium text-txt">{sc.name}</div>
                <div className="text-xs text-txt2 mt-1 flex items-center gap-2 flex-wrap">
                  <span>{sc.description || "（无描述）"}</span>
                  <span className="text-txt3">·</span>
                  <span className="inline-flex items-center gap-1">
                    <IconLink size={12} className="text-txt3" />
                    {sc.skills.length} 个技能
                  </span>
                </div>
              </div>
              <button className="btn btn-ghost" onClick={() => apply(sc.id)}>
                应用
              </button>
              <button
                className="btn !px-2 !py-1.5 text-red-300 hover:bg-red-500/15 hover:text-red-200"
                onClick={() => remove(sc.id)}
                title="删除"
              >
                <IconTrash size={15} />
              </button>
            </div>
          ))}
        </div>
      )}

      {showCreate && (
        <CreateScenarioModal
          skills={skills}
          editors={editors}
          colorOf={colorOf}
          onClose={() => setShowCreate(false)}
          onDone={async (name, desc, sel) => {
            await api.createScenario(name, desc, sel);
            setShowCreate(false);
            onLoad();
            showToast("场景已创建");
          }}
        />
      )}
    </div>
  );
}

function CreateScenarioModal({
  skills,
  colorOf,
  onClose,
  onDone,
}: {
  skills: Skill[];
  editors: EditorWithCount[];
  colorOf: (id: string) => string;
  onClose: () => void;
  onDone: (name: string, desc: string, sel: ScenarioSkill[]) => void;
}) {
  const [name, setName] = useState("");
  const [desc, setDesc] = useState("");
  const [picked, setPicked] = useState<Record<string, boolean>>({});

  const selected: ScenarioSkill[] = Object.entries(picked)
    .filter(([, v]) => v)
    .map(([id]) => {
      const [editorId, dirName] = id.split("::");
      return { editor_id: editorId, dir_name: dirName };
    });

  return (
    <div
      className="fixed inset-0 z-30 drawer-overlay animate-fade-in flex items-center justify-center p-6"
      onClick={onClose}
    >
      <div
        className="w-full max-w-lg bg-panel border border-border rounded-2xl shadow-pop overflow-hidden animate-slide-in flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between px-5 py-4 border-b border-border">
          <div className="font-semibold text-txt">新建场景</div>
          <button className="btn-subtle !p-1.5" onClick={onClose}>
            <IconX size={18} />
          </button>
        </div>
        <div className="p-5 space-y-3.5">
          <div>
            <label className="text-xs text-txt2">场景名称</label>
            <input
              className="input w-full mt-1.5"
              placeholder="例如：前端开发标配"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <div>
            <label className="text-xs text-txt2">描述（可选）</label>
            <input
              className="input w-full mt-1.5"
              placeholder="一句话说明用途"
              value={desc}
              onChange={(e) => setDesc(e.target.value)}
            />
          </div>
          <div className="text-xs text-txt3">
            选择包含的技能（{selected.length} 已选）
          </div>
          <div className="bg-panel2 border border-border rounded-xl p-2.5 max-h-64 overflow-y-auto space-y-1">
            {skills.map((s) => {
              const id = `${s.editor_id}::${s.dir_name}`;
              return (
                <label
                  key={id}
                  className="flex items-center gap-2.5 px-2 py-1.5 rounded-lg hover:bg-panel3 cursor-pointer transition-colors"
                >
                  <input
                    type="checkbox"
                    className="accent-[#6366f1] w-4 h-4"
                    checked={!!picked[id]}
                    onChange={(e) =>
                      setPicked((p) => ({ ...p, [id]: e.target.checked }))
                    }
                  />
                  <span
                    className="w-5 h-5 rounded-md grid place-items-center text-[12px] shrink-0"
                    style={{ background: tint(colorOf(s.editor_id), 0.18) }}
                  >
                    {s.editor_icon}
                  </span>
                  <span className="text-sm text-txt truncate flex-1">
                    {s.name || s.dir_name}
                  </span>
                  <span className="text-xs text-txt3 shrink-0">
                    {s.editor_name}
                  </span>
                </label>
              );
            })}
          </div>
        </div>
        <div className="border-t border-border px-5 py-3.5 flex justify-end gap-2.5">
          <button className="btn btn-ghost" onClick={onClose}>
            取消
          </button>
          <button
            className="btn btn-primary"
            onClick={() => {
              if (!name.trim()) return onClose();
              onDone(name.trim(), desc.trim(), selected);
            }}
          >
            创建
          </button>
        </div>
      </div>
    </div>
  );
}

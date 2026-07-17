import { useState } from "react";
import type { EditorWithCount } from "../types";
import { api } from "../api";
import {
  IconX,
  IconDownload,
  IconFolder,
  IconBook,
} from "./icons";

interface Props {
  editors: EditorWithCount[];
  onClose: () => void;
  onDone: (msg: string) => void;
}

export function InstallModal({ editors, onClose, onDone }: Props) {
  const [tab, setTab] = useState<"github" | "local">("github");
  const [url, setUrl] = useState("");
  const [localPath, setLocalPath] = useState("");
  const [target, setTarget] = useState("central");
  const [busy, setBusy] = useState(false);

  const activeEditors = editors.filter((e) => e.exists);

  const doInstall = async () => {
    setBusy(true);
    try {
      if (tab === "github") {
        if (!url.trim()) return onDone("请输入 GitHub 地址");
        const r = await api.installGithub(url.trim(), target);
        onDone(`已安装 ${r.count} 个技能`);
      } else {
        if (!localPath) return onDone("请选择本地目录");
        const r = await api.installLocal(localPath, target);
        onDone(`已安装 ${r.count} 个技能`);
      }
    } catch (e: any) {
      onDone(`安装失败: ${e?.toString() ?? "未知错误"}`);
    } finally {
      setBusy(false);
    }
  };

  return (
    <div
      className="fixed inset-0 z-30 drawer-overlay animate-fade-in flex items-center justify-center p-6"
      onClick={onClose}
    >
      <div
        className="w-full max-w-md bg-panel border border-border rounded-2xl shadow-pop overflow-hidden animate-slide-in"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between px-5 py-4 border-b border-border">
          <div className="font-semibold text-txt">安装技能</div>
          <button className="btn-subtle !p-1.5" onClick={onClose}>
            <IconX size={18} />
          </button>
        </div>

        <div className="px-5 py-4 space-y-4">
          {/* 来源切换 */}
          <div className="grid grid-cols-2 gap-2 p-1 bg-panel2 border border-border rounded-xl">
            {([
              ["github", "从 GitHub", IconDownload],
              ["local", "本地目录", IconFolder],
            ] as const).map(([k, label, Ic]) => (
              <button
                key={k}
                onClick={() => setTab(k)}
                className={
                  "flex items-center justify-center gap-2 rounded-lg py-2 text-sm font-medium transition-colors " +
                  (tab === k
                    ? "bg-panel4 text-txt shadow-sm"
                    : "text-txt3 hover:text-txt2")
                }
              >
                <Ic size={15} />
                {label}
              </button>
            ))}
          </div>

          {tab === "github" ? (
            <div>
              <label className="text-xs text-txt2">仓库地址</label>
              <input
                className="input w-full mt-1.5 font-mono"
                placeholder="user/repo 或 https://github.com/user/repo"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
              />
              <p className="text-[11px] text-txt3 mt-1.5 flex items-center gap-1">
                <IconBook size={12} /> 自动解析并安装仓库内的全部技能
              </p>
            </div>
          ) : (
            <div>
              <label className="text-xs text-txt2">本地目录</label>
              <div className="flex gap-2 mt-1.5">
                <input
                  className="input flex-1 font-mono !text-xs"
                  placeholder="未选择"
                  value={localPath}
                  readOnly
                />
                <button
                  className="btn btn-ghost shrink-0"
                  onClick={async () => {
                    const p = await api.pickDir();
                    if (p) setLocalPath(p);
                  }}
                >
                  选择…
                </button>
              </div>
            </div>
          )}

          <div>
            <label className="text-xs text-txt2">安装到</label>
            <select
              className="input w-full mt-1.5"
              value={target}
              onChange={(e) => setTarget(e.target.value)}
            >
              <option value="central">Central 库（统一管理）</option>
              {activeEditors.map((e) => (
                <option key={e.id} value={e.id}>
                  {e.name}
                </option>
              ))}
            </select>
          </div>
        </div>

        <div className="border-t border-border px-5 py-3.5 flex justify-end gap-2.5">
          <button className="btn btn-ghost" onClick={onClose}>
            取消
          </button>
          <button
            className="btn btn-primary"
            disabled={busy}
            onClick={doInstall}
          >
            {busy ? "安装中…" : "安装"}
          </button>
        </div>
      </div>
    </div>
  );
}

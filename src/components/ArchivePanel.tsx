import { useState, useEffect } from "react";
import type { FeedbackStats } from "../types";
import { api } from "../api";
import {
  IconArchive,
  IconDownload,
  IconUpload,
  IconChart,
  IconCheck,
} from "./icons";

interface Props {
  showToast: (m: string) => void;
}

export function ArchivePanel({ showToast }: Props) {
  const [archive, setArchive] = useState("");
  const [stats, setStats] = useState<FeedbackStats | null>(null);

  const loadStats = () => api.feedbackStats().then(setStats);
  useEffect(() => {
    loadStats();
  }, []);

  const exportArchive = async () => {
    const json = await api.exportArchive();
    setArchive(json);
    await navigator.clipboard?.writeText(json).catch(() => {});
    showToast("归档已导出并复制到剪贴板");
  };

  const importArchive = async () => {
    if (!archive.trim()) {
      showToast("请先粘贴归档 JSON");
      return;
    }
    try {
      const msg = await api.importArchive(archive);
      showToast(msg);
      loadStats();
    } catch (e) {
      showToast("导入失败: " + String(e));
    }
  };

  const adoption =
    stats && stats.enabled + stats.disabled > 0
      ? Math.round((stats.enabled / (stats.enabled + stats.disabled)) * 100)
      : 0;

  return (
    <div className="flex flex-col h-full">
      <div className="px-5 py-3 border-b border-border bg-panel/60 backdrop-blur">
        <h1 className="text-[15px] font-semibold text-txt leading-tight">归档与反馈</h1>
        <div className="text-[11px] text-txt3">配置 / 标注 / 场景 / 商店源 备份复用，及推荐采纳反馈</div>
      </div>

      <div className="flex-1 overflow-y-auto p-5 space-y-6 max-w-3xl">
        {/* 导入导出 */}
        <section>
          <div className="text-[11px] font-semibold uppercase tracking-wider text-txt3 mb-2">
            配置归档（A17）
          </div>
          <div className="flex gap-2 mb-3">
            <button className="btn btn-primary" onClick={exportArchive}>
              <IconDownload size={15} /> 导出归档
            </button>
            <button className="btn btn-ghost" onClick={importArchive}>
              <IconUpload size={15} /> 导入归档
            </button>
          </div>
          <textarea
            className="input w-full h-56 font-mono text-[11px]"
            value={archive}
            onChange={(e) => setArchive(e.target.value)}
            placeholder="点击「导出归档」生成 JSON，或粘贴归档 JSON 后点击「导入归档」…"
          />
        </section>

        {/* 反馈闭环 */}
        <section>
          <div className="text-[11px] font-semibold uppercase tracking-wider text-txt3 mb-2 flex items-center gap-1.5">
            <IconChart size={13} /> 推荐采纳反馈（E3）
          </div>
          <div className="grid grid-cols-3 gap-3">
            <Stat label="采纳事件总数" value={stats?.total_events ?? 0} />
            <Stat label="启用次数" value={stats?.enabled ?? 0} tone="ok" />
            <Stat label="停用次数" value={stats?.disabled ?? 0} tone="bad" />
          </div>
          <div className="mt-3 bg-panel2 border border-border rounded-xl px-4 py-3 flex items-center gap-3">
            <div className="text-sm text-txt3">推荐采纳率</div>
            <div className="flex-1 h-2 rounded-full bg-panel4 overflow-hidden">
              <div
                className="h-full bg-emerald-500/70"
                style={{ width: `${adoption}%` }}
              />
            </div>
            <div className="text-sm font-semibold text-txt tabular-nums">{adoption}%</div>
          </div>
          <div className="text-[11px] text-txt3 mt-2 flex items-center gap-1">
            <IconCheck size={12} className="text-emerald-300" />
            在项目感知中启用/停用推荐技能会自动记录，用于衡量推荐质量。
          </div>
        </section>
      </div>
    </div>
  );
}

function Stat({ label, value, tone }: { label: string; value: number; tone?: "ok" | "bad" }) {
  return (
    <div className="bg-panel2 border border-border rounded-xl px-3 py-2.5">
      <div className="text-[11px] text-txt3 mb-0.5">{label}</div>
      <div
        className={
          "text-xl font-semibold tabular-nums " +
          (tone === "ok" ? "text-emerald-300" : tone === "bad" ? "text-red-300" : "text-txt")
        }
      >
        {value}
      </div>
    </div>
  );
}

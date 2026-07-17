export function cn(...parts: Array<string | false | null | undefined>): string {
  return parts.filter(Boolean).join(" ");
}

/** 将 #rgb / #rrggbb 转为带透明度的 rgba 字符串（用于编辑器官方色做底色） */
export function tint(hex: string, alpha: number): string {
  if (!hex) return `rgba(148,163,184,${alpha})`;
  const h = hex.replace("#", "").trim();
  const full =
    h.length === 3
      ? h
          .split("")
          .map((c) => c + c)
          .join("")
      : h;
  if (full.length !== 6) return `rgba(148,163,184,${alpha})`;
  const r = parseInt(full.slice(0, 2), 16);
  const g = parseInt(full.slice(2, 4), 16);
  const b = parseInt(full.slice(4, 6), 16);
  if ([r, g, b].some((n) => Number.isNaN(n)))
    return `rgba(148,163,184,${alpha})`;
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

export type Severity = "high" | "medium" | "low" | string;

export const severityStyle: Record<string, string> = {
  high: "bg-red-500/15 text-red-300 border border-red-500/30",
  medium: "bg-amber-500/15 text-amber-300 border border-amber-500/30",
  low: "bg-zinc-500/15 text-zinc-300 border border-zinc-500/30",
};
export function sevClass(s: string): string {
  return severityStyle[s] ?? severityStyle.low;
}

export const sevLabel: Record<string, string> = {
  high: "高危",
  medium: "中危",
  low: "低危",
};

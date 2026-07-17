export interface Editor {
  id: string;
  name: string;
  icon: string;
  skill_dir: string;
  color: string;
}

export interface Skill {
  id: string;
  name: string;
  dir_name: string;
  summary: string;
  triggers: string[];
  editor_id: string;
  editor_name: string;
  editor_icon: string;
  editor_color: string;
  skill_dir: string;
  file_path: string | null;
  file_size: number;
  file_size_formatted: string;
  related_files: string[];
  frontmatter: Record<string, unknown>;
  content: string;
  is_valid: boolean;
  installed_at: string | null;
  is_symlink: boolean;
}

export interface FileInfo {
  name: string;
  path: string;
  size: number;
  size_formatted: string;
  modified: string;
}

export interface SkillDetail {
  editor_id: string;
  editor_name: string;
  editor_icon: string;
  dir_name: string;
  skill_dir: string;
  exists: boolean;
  name: string;
  summary: string;
  triggers: string[];
  frontmatter: Record<string, unknown>;
  content: string;
  file_size: number;
  file_size_formatted: string;
  related_files: string[];
  all_files: FileInfo[];
  is_valid: boolean;
  is_symlink: boolean;
  symlink_target: string | null;
}

export interface ScenarioSkill {
  editor_id: string;
  dir_name: string;
}

export interface Scenario {
  id: string;
  name: string;
  description: string;
  skills: ScenarioSkill[];
  created_at: string;
}

export interface DuplicateLocation {
  editor_id: string;
  editor_name: string;
}

export interface AuditIssue {
  skill_id: string;
  skill_name: string;
  severity: string;
  issue_type: string;
  detail: string;
}

export interface EditorWithCount {
  id: string;
  name: string;
  icon: string;
  skill_dir: string;
  color: string;
  exists: boolean;
  skill_count: number;
}

export interface EditorStat {
  name: string;
  icon: string;
  color: string;
  count: number;
}

export interface Stats {
  total_skills: number;
  total_editors: number;
  active_editors: number;
  per_editor: Record<string, EditorStat>;
  duplicates: Record<string, DuplicateLocation[]>;
}

export interface SkillList {
  total: number;
  skills: Skill[];
}

export interface InstallItem {
  name: string;
  editor: string;
  status: string;
}

export interface InstallResult {
  success: boolean;
  installed: InstallItem[];
  count: number;
}

export interface UninstallResult {
  success: boolean;
  removed: string;
  was_symlink: boolean;
}

export interface SyncItem {
  editor: string;
  status: string;
}

export interface SyncResult {
  success: boolean;
  synced: SyncItem[];
  count: number;
}

export interface ApplyItem {
  skill: string;
  editor: string;
  status: string;
}

export interface ApplyResult {
  success: boolean;
  results: ApplyItem[];
}

export interface AppConfig {
  editors: Editor[];
  sync_mode: string;
  theme: string;
  central_skills_dir: string;
}

export type Importance = "normal" | "important" | "critical";

export interface Annotation {
  notes: string;
  importance: Importance;
}

export const EMPTY_ANNOTATION: Annotation = { notes: "", importance: "normal" };

export const IMPORTANCE_META: Record<
  Importance,
  { label: string; color: string; weight: number }
> = {
  normal: { label: "普通", color: "#8b93a7", weight: 1 },
  important: { label: "重要", color: "#3B82F6", weight: 2 },
  critical: { label: "关键", color: "#EF4444", weight: 3 },
};

// ============ Tier B — 项目感知 ============

export interface ProjectInfo {
  path: string;
  name: string;
  project_id: string;
  has_config: boolean;
  config_path: string;
}

export interface ProjectSkill {
  skill: Skill;
  layer: "project" | "recommended" | "available";
  source: "project" | "global";
  signals: string[];
  enabled: boolean;
  default_enabled: boolean;
}

export interface ProjectConfig {
  version: number;
  project_id: string | null;
  extra_paths: string[];
  overrides: Record<string, string>;
}

export interface ProjectView {
  info: ProjectInfo;
  stack: string[];
  project: ProjectSkill[];
  recommended: ProjectSkill[];
  available: ProjectSkill[];
  overrides: Record<string, string>;
}

export interface ActivateResult {
  success: boolean;
  activated: string[];
  count: number;
}

// ============ Tier D — 技能应用商店 ============

export type MarketplaceType = "github" | "local" | "remote";

export interface MarketplaceSource {
  id: string;
  name: string;
  url: string;
  source_type: MarketplaceType;
  branch: string;
  subpath: string;
}

export interface MarketplaceSkill {
  source_id: string;
  source_name: string;
  name: string;
  dir_name: string;
  summary: string;
  triggers: string[];
  skill_dir: string;
  installed: boolean;
  update_available: boolean;
  extra: unknown;
}

export interface FeedbackStats {
  total_events: number;
  enabled: number;
  disabled: number;
  per_project: Record<string, number>;
}

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type {
  EditorWithCount,
  SkillList,
  SkillDetail,
  Stats,
  InstallResult,
  UninstallResult,
  SyncResult,
  Scenario,
  ScenarioSkill,
  AuditIssue,
  AppConfig,
  Annotation,
  Importance,
  ProjectInfo,
  ProjectView,
  ProjectConfig,
  ActivateResult,
  MarketplaceSource,
  MarketplaceSkill,
  FeedbackStats,
} from "./types";

export const api = {
  editors: () => invoke<EditorWithCount[]>("get_editors"),
  skills: (editorId?: string, search?: string) =>
    invoke<SkillList>("get_skills", { editorId, search }),
  skillDetail: (editorId: string, dirName: string) =>
    invoke<SkillDetail>("get_skill_detail_cmd", { editorId, dirName }),
  stats: () => invoke<Stats>("get_stats"),
  installGithub: (repoUrl: string, targetEditorId?: string) =>
    invoke<InstallResult>("install_github", { repoUrl, targetEditorId }),
  installLocal: (localPath: string, targetEditorId?: string) =>
    invoke<InstallResult>("install_local", { localPath, targetEditorId }),
  uninstall: (editorId: string, dirName: string) =>
    invoke<UninstallResult>("uninstall", { editorId, dirName }),
  syncSkill: (skillName: string, targetEditorIds: string[]) =>
    invoke<SyncResult>("sync_skill", { skillName, targetEditorIds }),
  copySkill: (editorId: string, dirName: string, targetEditorIds: string[]) =>
    invoke<SyncResult>("copy_skill", { editorId, dirName, targetEditorIds }),
  scenarios: () => invoke<Scenario[]>("get_scenarios"),
  createScenario: (name: string, description: string, skills: ScenarioSkill[]) =>
    invoke<Scenario>("create_scenario", { name, description, skills }),
  deleteScenario: (id: string) => invoke<void>("delete_scenario", { id }),
  applyScenario: (id: string) => invoke<SyncResult>("apply_scenario", { id }),
  audit: () => invoke<AuditIssue[]>("audit_skills_cmd"),
  config: () => invoke<AppConfig>("get_config"),
  setSyncMode: (mode: string) => invoke<void>("set_sync_mode", { mode }),
  annotations: () => invoke<Record<string, Annotation>>("get_annotations"),
  saveAnnotation: (skillId: string, notes: string, importance: Importance) =>
    invoke<Annotation>("set_annotation", { skillId, notes, importance }),
  deleteAnnotation: (skillId: string) =>
    invoke<void>("delete_annotation", { skillId }),

  // ===== Tier B 项目感知 =====
  openProject: (path: string) => invoke<ProjectInfo>("open_project_cmd", { path }),
  projectSkills: (path: string) => invoke<ProjectView>("get_project_skills_cmd", { path }),
  setProjectSkill: (path: string, skillKey: string, enabled: boolean) =>
    invoke<ProjectConfig>("set_project_skill_cmd", { path, skillKey, enabled }),
  getProjectConfig: (path: string) => invoke<ProjectConfig>("get_project_config_cmd", { path }),
  setProjectExtraPaths: (path: string, extraPaths: string[]) =>
    invoke<ProjectConfig>("set_project_extra_paths_cmd", { path, extraPaths }),
  activateProjectSkills: (path: string, projectId?: string) =>
    invoke<ActivateResult>("activate_project_skills_cmd", { path, projectId }),
  exportProjectTemplate: (path: string) =>
    invoke<string>("export_project_template_cmd", { path }),
  importProjectTemplate: (path: string, templateJson: string) =>
    invoke<ProjectConfig>("import_project_template_cmd", { path, templateJson }),
  feedbackStats: () => invoke<FeedbackStats>("get_feedback_stats_cmd"),

  // ===== Tier D 技能应用商店 =====
  marketplaceSources: () => invoke<MarketplaceSource[]>("get_marketplace_sources_cmd"),
  addMarketplaceSource: (
    name: string,
    url: string,
    sourceType: string,
    branch: string,
    subpath: string
  ) =>
    invoke<MarketplaceSource[]>("add_marketplace_source_cmd", {
      name,
      url,
      sourceType,
      branch,
      subpath,
    }),
  updateMarketplaceSource: (
    id: string,
    name: string,
    url: string,
    sourceType: string,
    branch: string,
    subpath: string
  ) =>
    invoke<MarketplaceSource[]>("update_marketplace_source_cmd", {
      id,
      name,
      url,
      sourceType,
      branch,
      subpath,
    }),
  removeMarketplaceSource: (id: string) =>
    invoke<MarketplaceSource[]>("remove_marketplace_source_cmd", { id }),
  browseMarketplace: (refresh: boolean) =>
    invoke<MarketplaceSkill[]>("browse_marketplace_cmd", { refresh }),
  installFromMarketplace: (
    sourceId: string,
    dirName: string,
    targetEditorId?: string
  ) =>
    invoke<InstallResult>("install_from_marketplace_cmd", {
      sourceId,
      dirName,
      targetEditorId,
    }),
  auditContent: (content: string) =>
    invoke<AuditIssue[]>("audit_content_cmd", { content }),
  readTextFile: (path: string) => invoke<string>("read_text_file_cmd", { path }),

  // ===== A17 导入/导出归档 =====
  exportArchive: () => invoke<string>("export_archive_cmd"),
  importArchive: (json: string) => invoke<string>("import_archive_cmd", { json }),

  pickDir: () => open({ directory: true, multiple: false }) as Promise<string | null>,
};

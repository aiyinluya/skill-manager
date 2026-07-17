//! Skill Manager 命令行界面（A16）
//! 供 coding agent / 终端直接驱动：列出、搜索、项目感知、安装、审计、归档。
//! 所有数据复用与桌面端相同的 ~/.skill-manager 存储与全局编辑器目录。

use std::io::{self, Read};
use std::process::exit;

use skill_manager_lib::commands;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    let sub = args[1].as_str();
    let result: Result<String, String> = match sub {
        "editors" => Ok(to_json(&commands::get_editors())),
        "list" => {
            let r = commands::get_skills(None, None);
            Ok(to_json(&r.skills))
        }
        "search" => {
            let q = args.get(2).cloned().unwrap_or_default();
            let r = commands::get_skills(None, Some(q));
            Ok(to_json(&r.skills))
        }
        "open" => {
            let p = need(&args, 2, "path");
            Ok(to_json(&commands::open_project_cmd(p)))
        }
        "project" => {
            let p = need(&args, 2, "path");
            Ok(to_json(&commands::get_project_skills_cmd(p)))
        }
        "enable" => {
            let p = need(&args, 2, "path");
            let key = need(&args, 3, "skillKey");
            Ok(to_json(&commands::set_project_skill_cmd(p, key, true)))
        }
        "disable" => {
            let p = need(&args, 2, "path");
            let key = need(&args, 3, "skillKey");
            Ok(to_json(&commands::set_project_skill_cmd(p, key, false)))
        }
        "activate" => {
            let p = need(&args, 2, "path");
            Ok(to_json(&commands::activate_project_skills_cmd(p, None)))
        }
        "install" => {
            let url = need(&args, 2, "repoUrl");
            let target = args.get(3).cloned();
            commands::install_github(url, target).map(|r| to_json(&r))
        }
        "install-local" => {
            let path = need(&args, 2, "localPath");
            let target = args.get(3).cloned();
            commands::install_local(path, target).map(|r| to_json(&r))
        }
        "marketplace" => {
            let refresh = args.iter().any(|a| a == "--refresh");
            Ok(to_json(&commands::browse_marketplace_cmd(refresh)))
        }
        "sources" => Ok(to_json(&commands::get_marketplace_sources_cmd())),
        "audit" => {
            let path = need(&args, 2, "file");
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            Ok(to_json(&commands::audit_content_cmd(content)))
        }
        "export" => Ok(commands::export_archive_cmd()),
        "import" => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| format!("读取标准输入失败: {}", e))?;
            commands::import_archive_cmd(buf)
        }
        "feedback" => Ok(to_json(&commands::get_feedback_stats_cmd())),
        _ => {
            print_usage();
            exit(1);
        }
    };

    match result {
        Ok(out) => println!("{}", out),
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    }
    Ok(())
}

fn to_json<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string_pretty(v).unwrap_or_else(|_| "null".into())
}

fn need(args: &[String], idx: usize, name: &str) -> String {
    args.get(idx)
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("error: 缺少参数 <{}>", name);
            exit(1);
        })
}

fn print_usage() {
    println!(
        r#"Skill Manager CLI (A16) — 供 coding agent 调用

用法:
  skill-cli editors                      列出已识别的编辑器及其技能数
  skill-cli list                         列出全部全局技能
  skill-cli search <关键词>              按名称/摘要/触发词搜索
  skill-cli open <项目路径>              识别项目并返回 projectId
  skill-cli project <项目路径>           返回项目三区技能视图（含相关性信号）
  skill-cli enable  <项目路径> <skillKey>  在项目内启用某技能
  skill-cli disable <项目路径> <skillKey>  在项目内停用某技能
  skill-cli activate <项目路径>          把启用的项目技能激活到对应编辑器
  skill-cli install <repoUrl> [editor]   从 GitHub 安装到指定编辑器（默认中心库）
  skill-cli install-local <路径> [editor] 从本地目录安装
  skill-cli sources                      列出已配置的商店源
  skill-cli marketplace [--refresh]      浏览全部商店源的可安装技能
  skill-cli audit <SKILL.md 路径>        对一段技能内容做安全审计
  skill-cli export                       导出配置归档 JSON（到 stdout）
  skill-cli import                       从 stdin 导入配置归档 JSON
  skill-cli feedback                     查看推荐采纳反馈统计

skillKey 格式: <editorId>:<dirName>，如 claude:my-skill"#
    );
}

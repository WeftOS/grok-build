//! Tapestry R17: a populated `~/.claude`, `~/.claude.json`, project `.claude/`, `.mcp.json` and
//! `CLAUDE.md` contribute nothing, even with every env var, `[compat]` TOML key and remote setting
//! set to on. These tests describe the default build, so they do not apply when `vendor-compat` is
//! built in.
#![cfg(not(feature = "vendor-compat"))]

use std::fs;
use std::path::Path;

use crate::compat::{
    CompatConfig, CompatConfigToml, CompatEnv, COMPAT_CELLS, INSTRUCTION_FILENAMES, VENDOR_COMPAT,
    resolve_compat_config, resolve_compat_hooks, resolve_compat_sessions,
};
use crate::mcp_servers::server_list::DefinitionResolver;
use crate::mcp_servers::{McpServerSources, json_config, load_mcp_servers};
use crate::{ClaudeImport, McpPreferencesFile, RemoteSettings};

const MCP_ENTRY: &str = r#"{"mcpServers": {"vendor-test-server": {"command": "vendor-test"}}}"#;

fn write(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

/// A temp HOME and project populated with everything Claude Code and Cursor would have supplied.
fn populated() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let project = tmp.path().join("project");
    write(&home.join(".claude/skills/x/SKILL.md"), "---\nname: x\n---\nbody");
    write(&home.join(".claude/agents/a.md"), "---\nname: a\n---\nbody");
    write(&home.join(".claude/rules/r.md"), "rule");
    write(
        &home.join(".claude/settings.json"),
        r#"{"permissions": {"allow": ["Bash(ls:*)"]}, "env": {"VENDOR_TEST": "1"}}"#,
    );
    write(
        &home.join(".claude.json"),
        &format!(
            r#"{{"mcpServers": {{"vendor-test-user": {{"command": "u"}}}}, "projects": {{"{}": {{"mcpServers": {{"vendor-test-project": {{"command": "p"}}}}}}}}}}"#,
            project.display()
        ),
    );
    write(&home.join(".cursor/mcp.json"), MCP_ENTRY);
    write(&project.join(".claude/skills/y/SKILL.md"), "---\nname: y\n---\nbody");
    write(&project.join(".claude/agents/b.md"), "---\nname: b\n---\nbody");
    write(&project.join(".claude/settings.json"), r#"{"permissions": {"deny": ["Bash(rm:*)"]}}"#);
    write(&project.join(".claude/CLAUDE.md"), "claude project instructions");
    write(&project.join(".cursor/mcp.json"), MCP_ENTRY);
    write(&project.join(".mcp.json"), MCP_ENTRY);
    write(&project.join("CLAUDE.md"), "claude instructions");
    (tmp, home, project)
}

fn every_source_on() -> (CompatConfigToml, CompatEnv, RemoteSettings) {
    let toml: CompatConfigToml = toml::from_str(
        r#"
        [cursor]
        skills = true
        rules = true
        agents = true
        mcps = true
        hooks = true
        sessions = true
        [claude]
        skills = true
        rules = true
        agents = true
        mcps = true
        hooks = true
        sessions = true
        [codex]
        skills = true
        rules = true
        agents = true
        mcps = true
        hooks = true
        sessions = true
        "#,
    )
    .unwrap();
    let env = CompatEnv::read(|_| Some(true));
    let remote: RemoteSettings = serde_json::from_str(
        r#"{
        "cursor_skills_enabled": true, "cursor_rules_enabled": true, "cursor_agents_enabled": true,
        "cursor_mcps_enabled": true, "cursor_hooks_enabled": true, "cursor_sessions_enabled": true,
        "claude_skills_enabled": true, "claude_rules_enabled": true, "claude_agents_enabled": true,
        "claude_mcps_enabled": true, "claude_hooks_enabled": true, "claude_sessions_enabled": true,
        "codex_sessions_enabled": true}"#,
    )
    .unwrap();
    (toml, env, remote)
}

#[test]
fn the_build_has_vendor_compat_off() {
    assert!(!VENDOR_COMPAT);
}

#[test]
fn no_env_toml_or_remote_setting_turns_a_cell_on() {
    let (toml, env, remote) = every_source_on();
    let resolved = resolve_compat_config(&toml, &env, Some(&remote));
    for cell in COMPAT_CELLS {
        assert!(
            !resolved.value(cell),
            "{}.{} resolved on",
            Into::<&str>::into(cell.vendor()),
            Into::<&str>::into(cell.surface())
        );
    }
    assert_eq!(resolved, CompatConfig::default());
    let effective: toml::Value = toml::from_str(
        "[compat.claude]\nsessions = true\nhooks = true\n[compat.codex]\nsessions = true\n[compat.cursor]\nsessions = true\nhooks = true\n",
    )
    .unwrap();
    let sessions = resolve_compat_sessions(Some(&effective), &env, Some(&remote));
    assert!(!sessions.claude && !sessions.codex && !sessions.cursor);
    let hooks = resolve_compat_hooks(Some(&effective), &env, Some(&remote));
    assert!(!hooks.claude && !hooks.cursor);
}

#[test]
fn resolved_cells_expose_no_vendor_directory_or_instruction_name() {
    let (toml, env, remote) = every_source_on();
    let resolved = resolve_compat_config(&toml, &env, Some(&remote));
    assert_eq!(resolved.skill_config_dirs(), vec![".grok", ".agents"]);
    assert_eq!(resolved.rules_dirs(), vec![".grok/rules"]);
    assert!(resolved.agents_home_dirs().is_empty());
    let names = resolved.agent_filenames();
    assert!(
        names.iter().all(|n| !n.to_lowercase().contains("claude")),
        "{names:?}"
    );
    assert!(INSTRUCTION_FILENAMES.contains(&"AGENTS.md"));
    assert!(
        INSTRUCTION_FILENAMES
            .iter()
            .all(|n| !n.to_lowercase().contains("claude"))
    );
}

#[test]
fn mcp_files_are_never_read_even_with_every_cell_forced_on() {
    let (_tmp, home, project) = populated();
    let sources = McpServerSources {
        cwd: project.clone(),
        project_configs: Vec::new(),
        mcp_json_files: vec![project.join(".mcp.json")],
        compat: CompatConfig::all_on(),
        claude_import: ClaudeImport::NotImported,
    };

    // The user's own grok-home servers may load; none may come from a vendor file
    assert!(
        load_mcp_servers(&sources)
            .iter()
            .all(|server| !format!("{server:?}").contains("vendor-test")),
    );
    assert!(
        json_config::load_mcp_json_servers_as_configs(
            &sources.mcp_json_files,
            ClaudeImport::NotImported
        )
        .is_empty()
    );
    assert!(
        json_config::load_mcp_json_servers_as_configs_unfiltered(&sources.mcp_json_files)
            .is_empty()
    );
    assert!(
        json_config::load_mcp_json_servers(&sources.mcp_json_files, ClaudeImport::NotImported)
            .is_empty()
    );
    assert!(
        json_config::load_claude_json_mcp_servers_as_configs(
            &project,
            &CompatConfig::all_on(),
            ClaudeImport::NotImported
        )
        .is_empty()
    );

    let preferences = McpPreferencesFile::default();
    assert!(
        json_config::claude_json_resolutions(
            &sources,
            &DefinitionResolver::expanding_env(&preferences),
            Some(&home)
        )
        .is_empty()
    );
    assert!(
        json_config::load_claude_json_mcp_servers_for_attribution(
            &project,
            Some(&home),
            &preferences
        )
        .is_empty()
    );
    assert!(
        json_config::load_cursor_mcp_servers_as_configs(&project, &CompatConfig::default())
            .is_empty()
    );
}

//! Tapestry R17 follow-up: with the default build, user plugins registered under `~/.claude`
//! and skills under the shared `~/.agents` and project `.agents` directories contribute nothing,
//! while project `.grok` content and an explicit grok home dir still load. (`grok_home()` is cached
//! per process, so the grok-home half is checked through the explicit `global_dir` argument.)
#![cfg(not(feature = "vendor-compat"))]

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use serial_test::serial;
use xai_grok_config::compat::CompatConfig;

use crate::plugins::discovery::{DiscoveryConfig, discover_plugins};
use crate::plugins::trust::TrustStore;
use crate::prompt::skills::{SkillsConfig, collect_skill_config_dirs, list_skills};

struct EnvVarGuard {
    key: &'static str,
    prev: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: impl AsRef<OsStr>) -> Self {
        let prev = std::env::var_os(key);
        unsafe { std::env::set_var(key, value) };
        Self { key, prev }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match self.prev.take() {
            Some(v) => unsafe { std::env::set_var(self.key, v) },
            None => unsafe { std::env::remove_var(self.key) },
        }
    }
}

fn write_skill(dir: &Path, name: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join("SKILL.md"),
        format!("---\nname: {name}\ndescription: test skill {name}\n---\n\nbody\n"),
    )
    .unwrap();
}

#[test]
#[serial]
fn claude_installed_plugins_are_not_discovered() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let project = tmp.path().join("project");
    fs::create_dir_all(&project).unwrap();
    let plugin = home.join(".claude/plugins/cache/mp/vendor-plug/1.0.0");
    fs::create_dir_all(plugin.join("skills")).unwrap();
    fs::write(plugin.join("plugin.json"), r#"{"name": "vendor-plug"}"#).unwrap();
    let json = serde_json::json!({
        "plugins": { "vendor-plug@mp": [{ "installPath": plugin.to_string_lossy() }] }
    });
    fs::write(
        home.join(".claude/plugins/installed_plugins.json"),
        serde_json::to_string(&json).unwrap(),
    )
    .unwrap();

    // A native project plugin under `.grok/plugins` still loads.
    let native = project.join(".grok/plugins/native-plug");
    fs::create_dir_all(&native).unwrap();
    fs::write(native.join("plugin.json"), r#"{"name": "native-plug"}"#).unwrap();

    let _h = EnvVarGuard::set("HOME", &home);
    let _u = EnvVarGuard::set("USERPROFILE", &home);

    let trust = TrustStore::load_from(tmp.path().join("trust"));
    let found = discover_plugins(Some(&project), &DiscoveryConfig::default(), &trust, true);
    let names: Vec<&str> = found.iter().map(|p| p.manifest.name.as_str()).collect();
    assert!(
        !names.contains(&"vendor-plug"),
        "claude plugin leaked: {names:?}"
    );
    assert!(
        names.contains(&"native-plug"),
        "native plugin missing: {names:?}"
    );
}

#[tokio::test]
#[serial]
async fn agents_dirs_are_not_read_but_grok_dirs_are() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let grok_home = tmp.path().join("grok-home");
    let project = tmp.path().join("project");
    fs::create_dir_all(&project).unwrap();
    git2::Repository::init(&project).unwrap();

    write_skill(&home.join(".agents/skills/home-agents-x"), "home-agents-x");
    write_skill(
        &project.join(".agents/skills/proj-agents-y"),
        "proj-agents-y",
    );
    write_skill(&grok_home.join("skills/grok-home-z"), "grok-home-z");
    write_skill(&project.join(".grok/skills/proj-grok-w"), "proj-grok-w");

    let _h = EnvVarGuard::set("HOME", &home);
    let _u = EnvVarGuard::set("USERPROFILE", &home);

    let skills = list_skills(
        Some(project.to_str().unwrap()),
        &SkillsConfig::default(),
        CompatConfig::default(),
        true,
    )
    .await;
    let names: Vec<&str> = skills.iter().map(|s| s.name.as_str()).collect();
    assert!(
        !names.contains(&"home-agents-x"),
        "~/.agents leaked: {names:?}"
    );
    assert!(
        !names.contains(&"proj-agents-y"),
        ".agents leaked: {names:?}"
    );
    assert!(
        names.contains(&"proj-grok-w"),
        "project .grok skill missing: {names:?}"
    );

    let dirs = collect_skill_config_dirs(
        Some(&project),
        None,
        &grok_home,
        &[],
        CompatConfig::default(),
    );
    assert!(dirs.contains(&grok_home), "grok home dir missing: {dirs:?}");
    assert!(dirs.contains(&project.join(".grok")), "{dirs:?}");
    assert!(
        dirs.iter()
            .all(|d| d.file_name() != Some(OsStr::new(".agents"))),
        ".agents dir listed: {dirs:?}"
    );
}

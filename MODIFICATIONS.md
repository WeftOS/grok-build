# Modifications

Tapestry is a modified version of [xai-org/grok-build](https://github.com/xai-org/grok-build),
licensed under the Apache License, Version 2.0. The baseline is upstream commit
`2bdd1d6a6369de0e8c68132ea4539e9abd9e14a8` (short `2bdd1d6a`).

This file is the prominent change notice required by Apache-2.0 section 4(b).
Every file changed, added or deleted relative to the baseline is listed below.
Files under MPL-2.0 carry an `(MPL-2.0)` marker on their line; changes to them
must also be published as source (see `docs/tapestry/mpl-files.md`). Original
licence and notice files are kept unchanged (see `docs/tapestry/notices-inventory.md`).

`scripts/check-modifications.sh` fails when a changed path is missing here.
Add each path in backticks on a list line, in the form
``- `path` (status): note``.

## Changes

- `docs/plans/grok-build-deep-fork-plan-2026-11-02.md` (added): the deep-fork plan.
- `docs/upstream-snapshot.md` (added): record of the upstream snapshot and baseline.
- `MODIFICATIONS.md` (added): this file.
- `SECURITY.md` (modified): replaced with a Tapestry vulnerability-reporting policy.
- `scripts/check-modifications.sh` (added): checks that this file lists every change.
- `docs/tapestry/notices-inventory.md` (added): inventory of licence and notice files.
- `docs/tapestry/mpl-files.md` (added): inventory of MPL-2.0 files and dependencies.
- `rust-toolchain.toml` (modified): channel 1.94.0 to 1.95, to match WeftOS. `cargo check -p xai-grok-pager-bin` passes unchanged on 1.95 (x86_64 Linux and aarch64 macOS).
- `Cargo.toml` (modified): the two pinned git dependencies now come from WeftOS mirrors at the same revisions: async-openai from `WeftOS/async-openai` (was `our-forks/async-openai`) and nucleo from `WeftOS/nucleo` (was `helix-editor/nucleo`). clippy: workspace lint `collapsible_match` set to allow, because the Rust 1.95 rewrite moves the inner `if` into the arm guard and lets values fall through to later arms.
- `Cargo.lock` (modified): the matching source URLs for those two git dependencies; no version or revision changed.

- `crates/codegen/xai-grok-agent/Cargo.toml` (modified): R17: new feature `vendor-compat` forwarding to `xai-grok-config/vendor-compat` (off by default).
- `crates/codegen/xai-grok-agent/src/discovery.rs` (modified): R17: `.claude/agents` (project and user) and the legacy `~/.grok/agents` (the installed Grok CLI home, when `GROK_HOME` points elsewhere) not scanned unless `VENDOR_COMPAT`; tests gated and added.
- `crates/codegen/xai-grok-agent/src/plugins/discovery.rs` (modified): R17b: `~/.claude/plugins/installed_plugins.json` (step 5c) not read unless `VENDOR_COMPAT`. R17: `~/.claude/plugins` and `.claude/plugins` not scanned unless `VENDOR_COMPAT`; tests gated and added.
- `crates/codegen/xai-grok-agent/src/plugins/marketplace.rs` (modified): R17: `.claude/settings.json` and `known_marketplaces.json` marketplaces resolve to nothing unless `VENDOR_COMPAT`.
- `crates/codegen/xai-grok-agent/src/prompt/agents_md.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-agent/src/prompt/skills.rs` (modified): R17b: `~/.agents` not scanned unless `VENDOR_COMPAT`. R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.

- `crates/codegen/xai-grok-config/Cargo.toml` (modified): R17: new feature `vendor-compat` (off by default).
- `crates/codegen/xai-grok-config/src/compat.rs` (modified): R17b: `skill_config_dirs()` lists `.agents` only with `VENDOR_COMPAT`. R17: new `VENDOR_COMPAT` const (cargo feature `vendor-compat`, off by default); every vendor cell defaults off and `resolve_compat_cells` ignores env, `[compat]` TOML and remote settings when it is off; `CLAUDE.md`/`CLAUDE.local.md` dropped from `INSTRUCTION_FILENAMES`; test helper `CompatConfig::all_on()`.
- `crates/codegen/xai-grok-config/src/compat_tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`. clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-config/src/mcp_servers.rs` (modified): R17: vendor block-key attribution reads no vendor file unless `VENDOR_COMPAT`; registers the isolation test.
- `crates/codegen/xai-grok-config/src/mcp_servers/json_config.rs` (modified): R17: `.mcp.json`, `~/.claude.json` and Cursor MCP loaders return nothing unless `VENDOR_COMPAT`.
- `crates/codegen/xai-grok-config/src/mcp_servers/json_config_tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-config/src/mcp_servers/server_list_tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`. clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-config/src/mcp_servers/session_merge_tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`. clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-config/src/mcp_servers_tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-external-agent-migration/src/source_cla.rs` (modified): R17: the Claude import scan returns an empty plan unless `VENDOR_COMPAT`.
- `crates/codegen/xai-grok-hooks/Cargo.toml` (modified): R17: new feature `vendor-compat` forwarding to `xai-grok-config/vendor-compat` (off by default).
- `crates/codegen/xai-grok-hooks/src/discovery.rs` (modified): R17: Claude and Cursor hook files are sources only when `VENDOR_COMPAT`; vendor test gated, isolation test added.
- `crates/codegen/xai-grok-pager/docs/user-guide/04-slash-commands.md` (modified): R17d: notes that `/import-claude` exists only in a `vendor-compat` build.
- `crates/codegen/xai-grok-pager/src/app/dispatch/import_claude.rs` (modified): R17d: the import action does nothing and writes no marker unless `VENDOR_COMPAT`.
- `crates/codegen/xai-grok-pager/src/slash/commands/mod.rs` (modified): R17d: `/import-claude` is registered only when `VENDOR_COMPAT`; test added.
- `crates/codegen/xai-grok-pager/src/app/effects/mod.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-permission-rules/Cargo.toml` (modified): R17: new feature `vendor-compat` forwarding to `xai-grok-config/vendor-compat` (off by default).
- `crates/codegen/xai-grok-permission-rules/src/claude_settings.rs` (modified): R17: no `.claude/settings*.json` path is discovered unless `VENDOR_COMPAT`.
- `crates/codegen/xai-grok-permission-rules/src/project_config.rs` (modified): R17: no `.mcp.json` candidate path unless `VENDOR_COMPAT`.
- `crates/codegen/xai-grok-permission-rules/src/resolution_tests.rs` (modified): R17: Claude-settings tests gated on feature `vendor-compat`; new default-build isolation test.
- `crates/codegen/xai-grok-shell/src/agent/folder_trust.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/agent/mvp_agent/tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/agent/subagent/isolated_spawn_e2e.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/claude_import_tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/inspect/compat.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/session/acp_session_tests/interjection_actor_tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/session/agent_mcp_tests.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/session/agent_rebuild.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/session/managed_mcp.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/util/config/mcp.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/util/config/mcp_reenable.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-subagent-resolution/src/definition.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-tools/Cargo.toml` (modified): R17: new feature `vendor-compat` forwarding to `xai-grok-config/vendor-compat` (off by default).
- `crates/codegen/xai-grok-tools/src/implementations/grok_build/read_file/mod.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`. clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-tools/src/implementations/skills/discovery.rs` (modified): R17b: dynamic skill discovery does not scan `.agents` unless `VENDOR_COMPAT`. R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`.
- `crates/codegen/xai-grok-tools/src/types/agents_md_tracker.rs` (modified): R17: tests that build vendor-enabled configs use `CompatConfig::all_on()` instead of `default()` (now off); tests asserting vendor discovery gated on feature `vendor-compat`. clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-agent/src/lib.rs` (modified): R17b: registers the test module `vendor_isolation_tests`.
- `crates/codegen/xai-grok-agent/src/vendor_isolation_tests.rs` (added): R17b: tests that `~/.claude` installed plugins and `~/.agents` / project `.agents` skills load nothing while `GROK_HOME` and project `.grok` still load.
- `crates/codegen/xai-grok-tools/src/reminders/skill_discovery.rs` (modified): R17b: `SKILL_CONFIG_DIRS` is only `.grok` unless feature `vendor-compat`.
- `crates/codegen/xai-grok-shell/src/extensions/skills.rs` (modified): R17b: skill source listing omits `.agents` and `.claude` dirs unless `VENDOR_COMPAT`.
- `crates/codegen/xai-grok-config/src/vendor_isolation_tests.rs` (added): R17: new tests that a populated HOME and project yield nothing from Claude/Cursor files, even with env, TOML and remote forced on. clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-agent/src/prompt/user_message.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-markdown/src/parse.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager-pty-harness/benches/paste_latency.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager-pty-harness/benches/pty_bench.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/app/dispatch/rewind.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/app/session_startup.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/recent_dirs.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/scrollback/render.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/scrollback/text_selection.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/slash/mru.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/tool_usage.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/views/dashboard/state.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/views/history_search.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/views/settings_modal/state.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/src/views/welcome/menu.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/tests/settings_e2e.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-sandbox/src/command/observe.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell-base/src/util/subprocess.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/agent/roster.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/extensions/prompt_history.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/extensions/suggest/history_provider.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/extensions/suggest/mod.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/session/acp_session_impl/prompt_offload_tests.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/session/acp_session_impl/prompt_queue.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/session/acp_session_tests/tool_layer_images_bridge_tests.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/session/compaction.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/session/storage/jsonl/mod.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-shell/src/session/unified_list/mod.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-tools/src/implementations/read_file/metadata.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-tty-utils/src/lib.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-tty-utils/src/runtime_tests.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/common/xai-tracing/src/http_client.rs` (modified): clippy: Rust 1.95 lints (mechanical).
- `crates/codegen/xai-grok-pager/docs/internal/25-enterprise.md` (added): Tapestry placeholder for the unpublished upstream enterprise pinning table; fill in when deploying into a managed environment.
- `crates/codegen/xai-grok-pager/docs/internal/22-environment-variables.md` (added): Tapestry placeholder for the unpublished upstream environment-variable table; fill in when deploying into an environment.

# Notices inventory

Every licence, notice and third-party-attribution file in the tree at the
baseline `2bdd1d6a` (upstream `xai-org/grok-build`). All are kept as shipped;
none is changed by Tapestry. Regenerate with:

    find . -path ./.git -prune -o -type f \( -iname '*LICEN*' -o -iname '*NOTICE*' -o -iname 'COPYING*' \) -print

## Licence and notice files

| Path | Covers |
|---|---|
| `LICENSE` | Apache-2.0, Copyright 2023-2026 SpaceXAI. The licence for the whole repository except where a file or directory says otherwise. |
| `THIRD-PARTY-NOTICES` | Generated notice for every Rust dependency (Part I per-package entries, Part II licence texts). Lists the MPL-2.0 packages (see `mpl-files.md`). |
| `third_party/NOTICE` | Index of vendored crates under `third_party/`; points at each crate's licence. |
| `third_party/README.md` | Describes the vendored crates (not a licence file; kept for context). |
| `third_party/mermaid-to-svg/LICENSE` | MIT, Copyright 2025-2026 Denver Technologies, Inc. Upstream warpdotdev/mermaid-to-svg. |
| `third_party/mermaid-to-svg/THIRD_PARTY_NOTICES` | Ancestry of mermaid-to-svg (mermaid.js, dagre.js and related, MIT texts). |
| `third_party/dagre_rust/LICENCE` | Apache-2.0 text for the vendored `dagre_rust` crate. |
| `third_party/graphlib_rust/LICENCE` | Apache-2.0 text for the vendored `graphlib_rust` crate. |
| `third_party/ordered_hashmap/LICENCE` | Apache-2.0 text for the vendored `ordered_hashmap` crate. |
| `crates/codegen/xai-ratatui-textarea/NOTICE` | Statement that the crate is first-party xAI code (Apache-2.0); dependency licences are in the root `THIRD-PARTY-NOTICES`. |
| `crates/codegen/xai-ratatui-inline/NOTICE` | Code derived from Ratatui (MIT / Apache-2.0), Copyright The Ratatui Developers and the tui-rs lineage; rest is xAI Apache-2.0. |
| `crates/codegen/xai-grok-tools/THIRD_PARTY_NOTICES.md` | Ported source (openai/codex, sst/opencode; licence texts and the Apache-2.0 section 4(b) change notice) and bundled tool binaries. |
| `crates/codegen/xai-grok-mermaid/assets/Roboto-LICENSE.txt` | Apache-2.0 text for the bundled Roboto font. |

Filename matches that are source code, not notices (ignore): files named
`*notice*.rs` under `xai-grok-pager` and `xai-grok-sampling-types` (model-notice
UI and types).

## Ported-code attribution (in-source)

Source files that carry a "ported from" attribution, covered by the crate
notice `crates/codegen/xai-grok-tools/THIRD_PARTY_NOTICES.md`:

| Path | Origin |
|---|---|
| `crates/codegen/xai-grok-tools/src/implementations/codex/mod.rs` | Module header: tools ported from openai/codex (Apache-2.0, Copyright 2025 OpenAI). |
| `.../implementations/codex/list_dir/mod.rs` | `codex-rs/core/src/tools/handlers/list_dir.rs` |
| `.../implementations/codex/grep_files/mod.rs` | `codex-rs/core/src/tools/handlers/grep_files.rs` |
| `.../implementations/codex/apply_patch/apply.rs` | `codex-rs/apply-patch/src/lib.rs` |
| `.../implementations/codex/apply_patch/parser.rs` | `codex-rs/apply-patch/src/parser.rs` |
| `.../implementations/codex/apply_patch/tool.rs` | Tool description derived from codex `apply_patch_tool_instructions.md` |
| `.../implementations/codex/read_file/indentation.rs` | Indentation-mode reader ported from codex |
| `.../implementations/opencode/mod.rs` | Module header: tools ported from sst/opencode (`bash`, `edit`, `glob`, `grep`, `read`, `skill`, `todowrite`, `write`). |
| `crates/codegen/xai-ratatui-inline/src/terminal.rs` | "Derived from ratatui's Terminal implementation" (MIT / Apache-2.0). |

The per-file list above comes from a header grep; other opencode tool files
carry the module-level attribution only. Treat the whole `codex/` and
`opencode/` directories as ported code.

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
- `Cargo.toml` (modified): the two pinned git dependencies now come from WeftOS mirrors at the same revisions: async-openai from `WeftOS/async-openai` (was `our-forks/async-openai`) and nucleo from `WeftOS/nucleo` (was `helix-editor/nucleo`).
- `Cargo.lock` (modified): the matching source URLs for those two git dependencies; no version or revision changed.

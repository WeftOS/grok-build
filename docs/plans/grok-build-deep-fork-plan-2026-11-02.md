# WeftOS deep fork of grok-build — plan (2026-11-02)

Status: snapshot created 2026-11-02. Rebrand and code changes have not started.

Snapshot: GitHub `main` at `2bdd1d6a6369de0e8c68132ea4539e9abd9e14a8`.
Upstream monorepo id in the unchanged root file `SOURCE_REV`:
`559751fdcec02d413e4c57c8832ab275e4f44980`. Record:
`docs/upstream-snapshot.md`. This repo is not a GitHub network fork and
has no upstream remote.

## Decision
Hard-fork xai-org/grok-build (Apache-2.0, Rust, ~2M LOC, ~110 crates) into a WeftOS-owned tree.
No upstream tracking branch. Upstream releases are treated as a parts bin: on each release we
"sift and graft" — diff by SOURCE_REV, pick wanted changes, port them in by hand.

## Why (Mathew, 2026-11-02)
- The grok-build TUI/harness is very effective for Mathew; keep that interface.
- WeftOS becomes the substrate: agents/bots surface automatically in the harness (discovered from the mesh).
- We provide our own desktops/computers (ruOS fleet, ThreadCone, xpc) instead of the Grok Build backend.
- Hedge: Grok Build may be discontinued; the fork must run with zero xAI backend dependency.

## Design goal implied
- Every xAI cloud call gets a WeftOS-owned replacement or is removed (auth, cloud-config, sessions, computer-hub, update).
- Model access is pluggable; xAI optional.
- Map crates/common/xai-computer-hub-* onto ruOS desktops.

## Phase 0 — Fork hygiene
- Fork at a recorded SOURCE_REV; keep LICENSE, NOTICE, THIRD-PARTY-NOTICES; add our own NOTICE of changes.
- Rebrand binary/crates (xai-* -> weft-*), drop xAI marks/logos, own update channel (replace xai-grok-update).
- Strip: xai-mixpanel, xai-grok-telemetry/otel endpoints, announcements, feedback, cloud-config pointing at xAI.
- Audit xai-grok-auth/login: keep xAI API as one optional provider, check API terms; default to WeftOS-routed providers.

## Phase 1 — WeftOS-native runtime
- Model layer (xai-grok-sampler/models): provider router driven by WeftOS (xpc local models, OpenRouter, xAI).
- Session events -> signed WeftOS frames on the mesh (replaces telemetry); session journal as accountable record.
- Workspace daemon -> WeftOS node/service (ThreadCone hub, xpc heavy peer).
- Sandbox + permission-rules -> fed by WeftOS policy/segmentation per bot.
- MCP/ACP/hooks: WeftOS MCP built in; Grok Bot connector at threadcone.weavelogic.ai/grokbot.

## Phase 2 — Harness on the mesh
- Subagents/tasks become bid + scored WeftOS tasks (Cardano scoring plan: signed bids, proper scoring, house edge, bailouts).
- Edge offload via WOP1/ComputeCap from the networking plan; receipts + credit ledger.
- Multi-harness hub: same router serves Claude/Codex/Grok sessions.

## Sift-and-graft process (per upstream release)
1. Clone new upstream, diff old..new SOURCE_REV per crate.
2. Triage table: take / adapt / ignore (security fixes always reviewed).
3. Port as normal commits tagged `graft: upstream <rev>`.
4. Record in docs/upstream-grafts.md.

## Research needed
- Crate dependency map; which crates are xAI-service-coupled vs generic.
- Build reqs (DotSlash, hermetic protoc) on ThreadCone (x86 macOS) and xpc.
- xAI API terms for third-party clients; trademark constraints on naming.
- Proto/gRPC contracts the agent assumes (crates/build/xai-proto-build).

## Open questions for Mathew
- Name of the fork/binary; repo home (weave-logic-ai org?).
- Keep xAI API as a provider or go local/OpenRouter only?
- Which phase-1 seam first (suggest: model router + events-to-mesh).

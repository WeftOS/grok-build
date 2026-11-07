# Enterprise feature pinning

Tapestry placeholder. Upstream `xai-org/grok-build` references
`docs/internal/25-enterprise.md`, but that file was never published in the
public snapshot (`2bdd1d6a`). This table lists every key in `FEATURES` so the
operator-doc check can run. Fill in the pinning/policy notes when Tapestry is
deployed into a managed environment.

| Feature key | Config path | Default enabled |
| --- | --- | --- |
| `session_search` | `features.session_search` | true |
| `lsp_tools` | `features.lsp_tools` | false |
| `web_fetch` | `features.web_fetch` | false |
| `session_recap` | `features.session_recap` | true |
| `ask_user_question` | `features.ask_user_question` | true |
| `voice_mode` | `features.voice_mode` | true |
| `write_file` | `features.write_file` | true |
| `feedback` | `features.feedback` | true |
| `feedback_trace_card` | `features.feedback_trace_card` | false |
| `turn_summary` | `features.turn_summary` | true |
| `cancel_rewind` | `features.cancel_rewind` | true |
| `compaction_verbatim_input` | `features.compaction_verbatim_input` | true |
| `two_pass_compaction` | `features.two_pass_compaction` | true |
| `backend_tools` | `features.backend_tools` | true |
| `auto_wake` | `features.auto_wake` | true |
| `subagent_worktree_snapshot` | `features.subagent_worktree_snapshot` | false |
| `subagent_model_inheritance` | `features.subagent_model_inheritance` | false |
| `active_agent_messages` | `features.active_agent_messages` | false |
| `dock` | `features.dock` | false |
| `terminal_theme` | `features.terminal_theme` | false |
| `file_acceleration` | `features.file_acceleration` | false |

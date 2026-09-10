<!-- aep-version-route: start -->
<!-- aep-cli-entrypoint: 5.0 -->
## AEP workflow selection

AEP default: v5
AEP v5: opt-in preview

Use the workflow explicitly selected by the user; otherwise use the project default above. Keep that selection for the task and carry its version, guidance source, stores, and worktree into handoffs.

For v4, use the installed `aep-*` skills and legacy stores. For v5, run `aep skills`, select `aep skills show <name>` and needed references, and use the native stores listed in `.aep/config.toml`. Missing capabilities are reported in the selected workflow. Installing the binary or reading guidance leaves the data owner unchanged; applying a reviewed migration selects v5. Only the selected workflow writes its state.

Existing AEP v4 workflow instructions elsewhere in this project apply only when v4 is selected. General project constraints remain applicable in either workflow. Review `project-rules/README.md` for native project rules when using v5. Read the catalog release and digest for the actual embedded guidance version.
<!-- aep-version-route: end -->

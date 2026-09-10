# AEP preview feature map

| User capability | Reach and drive | Observable result | Coverage boundary |
| --- | --- | --- | --- |
| Discover native guidance | `aep --skill --json`, then `aep --skill project`, `aep --skill design --ref prototype`, `aep --skill validate --ref self-verification` | Eight native entries with version/digest; complete content available outside source checkout | Selection by a cold agent is observed separately |
| Start alongside v4 | `aep init` in a Git fixture with v4 AGENTS and legacy files | Explicit v4 default; legacy source and installed skill hashes unchanged | Root instruction and real host consumer cases also covered by Rust fixtures |
| Preview and apply migration | `migrate plan --output <file>`, inspect it, then `migrate apply --plan <file>` and `migrate verify` | Plan leaves source files present; native records exist; routing selects v5; repeated apply preserves state | Synthetic source, no production migration claim |
| Inspect migrated work | `status`, `query`, `context`, `timeline` | Imported story and its context are accessible; reads leave the fixture files unchanged | Context includes explicit links; additional sources remain agent-selected |
| Continue native work | Create a new pending story with `story new`, then query it | New story lives in native ledger while original legacy bytes remain unchanged | No claim that a pending story has been implemented or verified |

The helper's `summary.json` lists which surfaces were exercised and retains command responses. Rust lifecycle fixtures separately exercise checks, review policy, stale evidence, gates and local delivery. A failing product path stays a defect; revise this map only when the intended interface changes or the map itself is wrong.

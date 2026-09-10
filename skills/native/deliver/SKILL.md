---
name: deliver
description: Prepare a PR or perform authorized integration using current evidence, then preserve lessons and close owned resources.
---

# Deliver

Use `aep deliver plan --story <id>` to inspect the exact head and unmet requirements. Bring verification and required review current before recording integration.

For a PR request, use `aep deliver pr --story <id> --base <branch>`. For authorized GitHub merge, use `aep deliver merge --story <id>`; for explicitly local integration, use its `--local` mode from the intended integration branch. The command records observed provider/Git results.

Reconcile ambiguous external results with `aep deliver status --story <id>` before retrying. After a lost merge response, inspect the provider, then use `aep deliver reconcile --story <id>` to record confirmed pending integration. PR retries look up the recorded branch before creating another PR.

Review combined integration and run required checks. Publish applicable deltas with `aep spec publish --change <id>` and close the change when linked work is integrated. A different or unavailable integration tree blocks promotion; inspect it and use `aep story reopen <id>` for current validation while retaining historical delivery. `aep release promote <id> --environment <target> --by <actor>` requires current gates and environment evidence.

Read [Closure](references/closure.md) before cleaning resources or completing handoff. Use `aep --skill reflect` for useful observations. Report integration identity, actual verification scope, retained evidence and remaining resources or gaps.

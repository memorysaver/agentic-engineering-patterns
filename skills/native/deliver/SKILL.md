---
name: deliver
description: Prepare a PR or perform authorized integration using current evidence, then preserve lessons and close owned resources.
---

# Deliver

Recover the requested endpoint from the user’s request, prior authorization and project workflow. Read [Closure](references/closure.md) (`aep --skill deliver --ref closure`) to apply scope and completion rules before choosing the next action. For end-to-end work, carry implementation through authorized delivery and closure; treat a ready candidate as an intermediate result.

Use `aep deliver plan --story <id>` to inspect the exact head and unmet requirements. The plan separates current verification readiness from integration state; an integrated story continues to publication/closure. Dry runs check local action prerequisites; provider state is rechecked when executing. Delivery actions and closure still need observed results. Bring verification and required review current before recording integration.

For an authorized PR, use `aep deliver pr --story <id> --base <branch>`. For authorized GitHub merge, use `aep deliver merge --story <id>`; for explicitly local integration, use its `--local` mode from the intended integration branch. The command records observed provider/Git results.

Reconcile ambiguous external results with `aep deliver status --story <id>` before retrying. After a lost merge response, inspect the provider, then use `aep deliver reconcile --story <id>` to record confirmed pending integration. PR retries look up the recorded branch before creating another PR.

Review combined integration and run required checks. When shared context changes after integration, rerun verification and required review on the retained candidate, then retry publication; publication binds current evidence separately from the immutable merge receipt. Publish applicable deltas with `aep spec publish --change <id>` and close the change when linked work is integrated. A different or unavailable integration tree blocks promotion; inspect it and use `aep story reopen <id>` for current validation while retaining historical delivery. `aep release promote <id> --environment <target> --by <actor>` requires current gates and environment evidence.

Apply the closure procedure before completing handoff. Use `aep --skill reflect` for useful observations. Report integration identity, actual verification scope, retained evidence and remaining resources or gaps.

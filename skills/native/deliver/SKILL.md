---
name: deliver
description: Prepare a PR or perform an authorized integration using the current implementation, checks, and review evidence.
---

# Deliver

Use `aep deliver plan --story <id>` to inspect the exact implementation head and unmet verification requirements. Bring checks and review up to date before publishing an integration claim.

For a PR request, use `aep deliver pr --story <id> --base <branch>`. For an authorized GitHub merge, use `aep deliver merge --story <id>`; for an explicitly local integration, use its `--local` mode from the intended integration branch. The command performs the requested operation and records observed provider/Git results.

Reconcile an ambiguous external result with `aep deliver status --story <id>` before retrying. Review the combined integration and run its required checks. Publish applicable specification deltas with `aep spec publish --change <id>` and close the change when all linked work is integrated. Release/deployment evidence remains a separate project gate.

Record useful observations with the reflect skill. Report the PR or integration identity and actual verification scope.

After a lost merge response, use `aep deliver status --story <id>` to inspect the provider, then `aep deliver reconcile --story <id>` to record a confirmed pending integration. A PR creation retry looks up the recorded branch before creating another PR.

A different or unavailable provider integration tree blocks promotion. Inspect it and use `aep story reopen <id>` for current validation while retaining its historical delivery. Release promotion is separate: `aep release promote <id> --environment <target> --by <actor>` requires current gates and environment evidence for every member.

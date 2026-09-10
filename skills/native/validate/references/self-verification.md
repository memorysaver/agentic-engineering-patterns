# Observable self verification

Start from accepted intent and the actual diff or artifact. Identify which claims changed and which established checks still apply. For each important claim, connect its acceptance source to an operation, expected result, observed result, revision/environment, and retained artifact. A short evidence note can express this; it is not a new CLI receipt schema.

Use the project's verification skill and Feature Map. If the operation method is missing, read `aep skills show project --ref verification-setup` and establish the smallest useful harness within task scope. Run preflight first so missing credentials, dependencies or services are distinguishable from product defects.

| Surface | Useful observation | Limit of narrower evidence |
| --- | --- | --- |
| CLI | Actual binary, exit/stdout/stderr, before/after files; PTY for interaction | Compilation alone leaves behavior untested |
| API/service | Public request, response and persistence/permission effects | Handler tests leave deployed routing/auth uncertain |
| UI/desktop | Operate controls, observe feedback and resulting state | Static screenshots leave interactions uncertain |
| Library | Public API consumer compile/run harness | Private helper tests leave integration uncertain |
| Documents/research | Primary sources, links, executable instructions and key claims | Fluent prose leaves factual support uncertain |

Choose probes that could expose a mistaken completion claim, using relevant failure or boundary cases grounded in the change. Fix observed defects, rerun affected behavior, and reuse still-valid evidence. Broaden checks when new information or unresolved concerns justify the cost; a fixed score, topology or number of rounds is unnecessary.

`verify run` executes configured commands and records their result/environment/head/fingerprint. A successful process exit establishes what that command actually checked. Arbitrary screenshots, prototype files and linked external documents are not automatically digested or assessed for acceptance coverage by the gate. Preserve their identity and location in durable notes/artifacts, and state the manual coverage/enforcement boundary.

Review comments can find gaps, but another model's judgment cannot replace execution. Record actual reviewer attribution when independent review is chosen or required. Keep concrete findings actionable with affected behavior, evidence and impact; close them with the appropriate revision/check evidence.

Report failures, untested surfaces, environment limitations and stale evidence plainly. Local tests cannot establish deployment behavior. Retain useful evidence outside temporary resources before cleanup, keeping sensitive data under the project's handling rules.

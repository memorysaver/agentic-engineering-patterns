---
name: bug-fix
kind: runbook
description: "Fix a reported bug in AEP or a project: reproduce it, find the root cause, add a test that fails first, make the smallest fix, verify the committed candidate with the configured checks, and record a lesson when the same mistake repeats. Use for any bug whose fix changes code, tests or procedures."
steps:
  - id: reproduce
    do: "Reproduce the bug with a command against the current head, and keep the command output that shows the failure."
    evidence: [command-output]
    principles: [prove-it]
  - id: root-cause
    do: "Find the code or record that causes the failure, and state why the symptom appears from it, not only where it appears."
    evidence: [file-content]
  - id: test-first
    do: "Add or extend a test that fails for the reported behavior before the fix, and keep its failing output."
    evidence: [command-output]
  - id: fix
    do: "Make the smallest change that removes the root cause, and commit it as the candidate."
    evidence: [git-ref]
  - id: verify
    do: "Run the configured checks on the committed candidate and keep their receipts; a result that is not from the candidate does not count."
    evidence: [check-id]
    principles: [prove-it, verifiable-units]
  - id: learn
    do: "If the same mistake was corrected before, record it as a lesson or rule with the evidence; otherwise state that no repeat was found."
    evidence: [record-id]
    principles: [encode-lessons]
---

# Bug fix

This runbook fixes one reported bug. Copy its steps into `progress.yaml` in the attempt's worktree (outside tracked source), with the attempt identifier, and tick each step with its evidence. `aep runbook check progress.yaml` validates the file; it runs no step.

For each step, read the cited principle first (`aep --skill principles --ref ID --attempt ATTEMPT`) so that the citation is recorded in the attempt's read log.

A step you cannot perform is marked skipped with a reason. Configured verification and delivery requirements still apply to skipped steps.

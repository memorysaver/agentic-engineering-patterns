---
name: verify-aep
description: Exercise an AEP preview binary through source-preserving migration and native inspection in a disposable project, retaining runtime evidence for release review.
---

# Verify the AEP executable

Read [the feature map](references/feature-map.md) for the supported user paths and known coverage limits. This repository's build/check rules remain in `project-rules/rust.md`; this procedure adds an installed-product probe, not a downstream maintenance story loop.

Read this indexed file directly in the AEP source checkout, which has no downstream `.aep/config.toml`. CLI local-skill discovery applies in initialized projects; source maintenance does not require initializing this repository.

Build the candidate with `cargo build --locked --release -p aep-cli`, or select the extracted executable from a release archive. Confirm its exact version with `<binary> --version`. The runtime probe needs Python 3 and Git; those are verification tooling, not dependencies of the installed AEP binary.

Run from this repository, substituting a new evidence directory and an absolute executable path:

```bash
python3 scripts/verify-native-preview.py --binary /absolute/path/to/aep --output /tmp/aep-preview-proof-unique
```

The helper drives the public CLI in a fresh disposable Git project, records each JSON response and legacy file hashes, verifies migration and a new native story, then removes only its temporary fixture. It creates no external provider requests, services or background workers. The output directory must be new so an earlier proof stays intact.

Read `summary.json` and relevant command transcripts. Check the binary digest/version, per-surface observations, and cleanup result. On failure the helper retains the transcript and reports the failing assertion or command; diagnose that result before rerunning into another output directory. A source migration failure and a product behavior failure require different fixes.

This proof establishes the exercised synthetic local paths. Cross-platform CI, actual downstream policy, interactive artifact coverage, GitHub provider behavior, and full implementation/delivery are separate checks. Report skipped or unavailable surfaces with their reason.

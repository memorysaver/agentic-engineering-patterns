# Status from evidence

Use `aep status`, `aep query`, `aep context <id>`, and `aep timeline` to find the relevant records. For a narrower view, use `aep query --kind story --status pending` or `aep query --kind story --status blocked`, then inspect selected IDs. Add `--json` when structured fields are needed; the CLI reads native records without an additional YAML parser. Inspect actual checks, attempts, integration identities and environment gates for the claims being summarized.

A useful update states the current outcome, what changed, supporting evidence, remaining work, and decisions that really require the reader. Distinguish accepted intent, candidate implementation, verified behavior, integration, and deployment. A recorded “done” label alone is weaker than the associated integration/check evidence.

For blockers, name the unresolved dependency, finding, access gap or decision and its owner when known. Report “waiting since” only with a source for that time. Event recording timestamps cannot reconstruct unknown historical merge dates.

Keep the explanation in the user's terms; link canonical IDs/artifacts so another agent can resume. A text summary usually suffices; presentation work follows the requested deliverable.

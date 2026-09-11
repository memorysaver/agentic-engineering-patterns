# Release rules

A native release couples the Rust binary and embedded `skills/native/` content under the Cargo workspace version. Record a matching `CHANGELOG.md` entry. Update the default config/entrypoint version, native documentation, and migration instructions together. Validate the installed binary and archive checksums.

The optional legacy plugin remains pinned to v4.1.0 in `.claude-plugin/marketplace.json`. A native version bump does not relabel the old plugin. Legacy plugin changes still require its own matching metadata, template/migration updates, package validation, and changelog.

The initial downstream candidate is `5.0.0-preview.1`. Tags with a prerelease suffix publish as GitHub prereleases with `latest=false`, preserving the legacy stable release. A prepared candidate, open PR, local archive, and public release are separate states; report the actual one. See `docs/decisions/aep-v5-preview-adoption.md`.

`.github/workflows/native.yml` validates native changes and creates release archives for a `v5.*` tag whose version matches Cargo.toml. The archive contains the standalone binary and license; its checksum is separate. Native build, installation, and release must not depend on dashboard jobs or JavaScript tools. Dashboard implementation and runtime bundling are deferred from the current release scope. Creating an implementation PR does not publish a release. Tag/publish only when the user's request includes release authorization. Downstream adoption and migration are separate operations, performed against the target project's actual rules and evidence.

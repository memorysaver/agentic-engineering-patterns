# Release rules

A native release couples the Rust binary and embedded `skills/native/` content under the Cargo workspace version. Record a matching `CHANGELOG.md` entry. Update the default config/entrypoint version, native documentation, and migration instructions together. Validate the installed binary and archive checksums.

The optional legacy plugin remains pinned to v4.1.0 in `.claude-plugin/marketplace.json`. A native version bump does not relabel the old plugin. Legacy plugin changes still require its own matching metadata, template/migration updates, package validation, and changelog.

`.github/workflows/native.yml` validates native changes and creates release archives for a `v5.*` tag whose version matches Cargo.toml. Creating an implementation PR does not publish a release. Tag/publish only when the user's request includes release authorization. Downstream adoption and migration are separate operations, performed against the target project's actual rules and evidence.

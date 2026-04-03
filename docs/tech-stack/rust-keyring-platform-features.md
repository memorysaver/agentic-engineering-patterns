# Rust `keyring` Crate — Platform Features Required

**Discovered:** 2026-04-03 (Looplia Layer 0.5 dogfood testing)

## Problem

`keyring = "3.6"` without platform features has **no credential backend**. The crate silently returns `NoEntry` for all reads, making it appear like no key is stored. This is not an error — it's the default behavior when no platform backend is compiled in.

## Fix

Always specify platform-specific features:

```toml
# macOS
keyring = { version = "3.6", features = ["apple-native"] }

# Linux
keyring = { version = "3.6", features = ["linux-native"] }
```

## Recommendation

Rust project scaffolding (`/scaffold`) should include platform-specific dependency notes for common crates that require feature flags to function. The `keyring` crate is a common example — other crates with similar platform-gated behavior should be documented here as they're discovered.

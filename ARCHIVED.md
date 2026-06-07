# DraconDev monorepo — archived components

## `dracon-ai-sdk/` (moved 2026-07-06)

The standalone SDK crate at `DraconDev/dracon-ai-sdk` has been **moved** into
the platform monorepo at `DraconDev/dracon-ai-platform/crates/ai-api-sdk/`.

**Why:** The SDK is a thin HTTP client for the `ai-api` server. It belongs
with the server, not as a sibling. The lib (`DraconDev/dracon-ai-lib`) is
the AI engine itself and ships no SDK.

**Migration:** If you depended on `dracon-ai-sdk = { git = "..." }` in any
Cargo.toml, change it to:

```toml
ai-api-sdk = { git = "https://github.com/DraconDev/dracon-ai-platform.git", tag = "<next>" }
```

The package name changed from `dracon-ai-sdk` → `ai-api-sdk`. The type
`DraconAi` and the `ChatMessage` API are identical.

**Status:** This directory is kept for git history. Do not add new code here.

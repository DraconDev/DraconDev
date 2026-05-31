# DraconDev — Private Repos Product Roadmap

Every local private repo categorized as: **Commercial Product**, **Internal Tooling**, or **Abandoned**.

---

## 🔴 Commercial Products (Revenue-Ready)

| Repo | Product Name | One-Line Positioning |
|:-----|:-------------|:-------------------|
| `dracon-code` | Dracon Code | Blueprint-driven autonomous engineering runtime. Not a chatbot — a crew of AI agents you control. Rust core, Tauri GUI. |
| `ai-auto-repo-rot-scanner-todo-agent` | Repo Rot Scanner | AI-powered repository rot scanner with auto-discovery and classification. Finds dead, stale, and abandoned repos in your organization. |
| `ai-auto-writer` | AI Auto Writer | AI-powered book generation with pluggable storage. Write long-form content at scale. |
| `rust-ai-web-auto` | Ghost Orchestrator | Enterprise AI-driven browser automation engine using raw Chrome DevTools Protocol. Headless, auditable, scriptable. |
| `SamAI` | SamAI | AI-powered browser extension. Summarize any page, chat with websites, extract content, fill forms. |
| `avid` | AVID | Lightweight CLI for processing videos with silence detection, audio enhancement, scene detection. |
| `pully-fully-pull-based-fleet-reconciler` | Pully | Pull-based server fleet reconciler. Write recipes in git, Pully enforces them on every server. |

## 🟡 Internal Tooling (Operational, Not for Sale)

| Repo | Purpose |
|:-----|:--------|
| `dracon-platform` | Self-contained platform for hosted services, billing, and account management. Core infrastructure. |
| `dracon-ai-lib` | Rust library for unified AI provider access with automatic failover, circuit breaker, and rate limiting. Used internally by all AI products. |
| `dracon-libs` | Vertical Rust libraries — git, terminal, system, media, memory. Shared foundation. |
| `dracon-terminal-engine` | Terminal app framework. App, widgets, compositor, 15 built-in themes. Shared TUI infrastructure. |
| `dracon-utilities` | CLI binaries for dracon system services. Install to `~/.local/bin/`. System-level tooling. |
| `dracon-demons` | Internal service-to-service daemons for the Dracon platform. |
| `browser-extensions-shared` | Monorepo of Chrome extensions built with WXT, React, TypeScript. Shared library for volume, fullscreen, API debugger extensions. |
| `video-factory` | Web platform for video processing with FFmpeg, S3 storage, and YouTube publishing. Backend for AVID. |
| `cli-file-manager` | Directory listing with instant context. Internal tooling. |
| `test-auto-create` | Test repo. Internal experiment. |
| `pi-auto-review` | Event-driven project review for Pi coding agent — scans for problems, patterns, and opportunities. Pi-specific, not general product. |
| `opencode-auto-continue` | OpenCode plugin for session management. Open source, niche audience. |
| `opencode-auto-force-resume` | OpenCode plugin for stall recovery. Open source, superseded by auto-continue. |
| `opencode-auto-review-completed-todos` | OpenCode plugin for todo review. Open source, small audience. |
| `video-uploader` | Rust library + CLI for YouTube Data API v3 uploads. Used internally. |

## 🟢 Possible Products (Uncertain — Needs Decision)

| Repo | Notes |
|:-----|:------|
| `dracon-voice-notifications` | Desktop announcer with voice. Interesting niche but unclear market. |
| `wal-backup` | SQLite database API with automatic S3 backup. Self-contained. Could be a product. |
| `youtube-video-uploader` | Rust CLI for YouTube uploads. Overlaps with video-factory. |

## ⚫ Abandoned

| Repo | Reason |
|:-----|:-------|
| `Junk-Runner-bevy` | Game project (roguelike). Doesn't fit tools/infra GitHub profile. |
| `one-mil-girls` | Tauri/SvelteKit template experiment. Abandoned. |
| `kittentts-showcase` | TTS showcase, 0 stars, not a product. |

---

## Decision Matrix — What to Commercialize?

Based on B2B targeting (small engineering teams, 2–5 engineers):

| Repo | B2B Problem Solved | Commercial? |
|:-----|:-------------------|:------------|
| `dracon-code` | Engineering teams can't scale code reviews, refactors, or greenfield work with LLM chat | ✅ Flagship |
| `pully` | DevOps teams want GitOps-style server management without Ansible/CF complexity | ✅ Strong B2B |
| `rust-ai-web-auto` | QA/testing teams need auditable browser automation without fragile Selenium | ✅ B2B-adjacent |
| `ai-auto-repo-rot-scanner-todo-agent` | Engineering leads need visibility into repo health across org | ✅ B2B |
| `dracon-voice-notifications` | Desktop notifications for CI/CD pipelines | ❓ Unclear |
| `wal-backup` | Small teams need dead-simple SQLite backups to S3 | ❓ Possible |
| `avid` | Content teams need automated video editing | ❌ B2C |
| `ai-auto-writer` | Writers/authors need book generation | ❌ B2C |

**Recommendation:** Start with `dracon-code` (flagship), `pully` (fleet reconciler), and `ai-auto-repo-rot-scanner-todo-agent` (repo rot scanner) as the commercial trio. Focus on small engineering teams — DevOps/platform engineers and tech leads.

---

*Last updated: 2026-05-31*
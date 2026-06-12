# DraconDev — Product & Project Inventory

**Last updated:** 2026-06-02
**Total projects:** 31 local + 19 browser extensions = 50 total
**Total code:** 1,737 src files, 589,677 lines (local) + ~800 extension files

---

## Executive Summary

| Category | Count | Total Lines | Revenue Potential |
|:---------|------:|------------:|:------------------|
| **Infrastructure** (platform, daemons, libs) | 5 | 202,613 | High (B2B SaaS) |
| **Developer Tools** (Rust CLI/frameworks) | 7 | 93,241 | Medium (open source + consulting) |
| **AI Products** (browser + automation) | 4 | 60,258 | High (freemium/SaaS) |
| **Browser Extensions** (Chrome) | 19 | ~800 | Medium (freemium) |
| **Video Tools** (YouTube pipeline) | 4 | 28,264 | Low (utility) |
| **Games & Creative** | 2 | 22,333 | Low (portfolio) |
| **Utilities & Tools** | 6 | 39,507 | Medium (open source) |
| **Infrastructure (internal)** | 4+ | ~100,000+ | Part of platform |

---

## TIER 1 — FLAGSHIP PRODUCTS (sell these)

### dracon-platform
- **What:** Self-contained platform — auth, billing, email, TTS, STT, dashboard, file storage
- **Files:** 284 | **Lines:** 52,622 | **Lang:** Rust + TypeScript + JavaScript
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Dev teams, SaaS builders
- **Revenue model:** B2B SaaS — host it for customers, or sell as self-hosted license
- **Status:** Production infrastructure for your own product stack
- **On GitHub:** No (private)

### dracon-code
- **What:** Flagship product (details private)
- **Files:** 131 | **Lines:** 48,975 | **Lang:** Rust
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Your customers
- **Revenue model:** Core product — sell directly
- **On GitHub:** No (private)

### SamAI
- **What:** AI browser companion — summarize, chat, fill forms, BYOK
- **Files:** 101 (local) + 144 (extension) | **Lines:** 12,086
- **Maturity:** Beta (tests ✓, CI ✓) — inventory claim: already on Chrome Web Store; verify before public claims.
- **Audience:** Everyone who uses a browser
- **Revenue model:** Freemium — free BYOK tier + paid hosted AI tier
- **On GitHub:** No (closed source product)

### dracon-demons
- **What:** Service-to-service daemons — binary RPC over Unix sockets, stateful compute
- **Files:** 92 | **Lines:** 53,590 | **Lang:** Rust + Python
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Internal (powers dracon-platform)
- **Revenue model:** Part of platform — not sold separately
- **On GitHub:** No (private)

---

## TIER 2 — CREDIBILITY TOOLS (open source candidates)

### pully-fully-pull-based-fleet-reconciler
- **What:** Pull-based server fleet reconciler — write recipes in git, Pully enforces them
- **Files:** 44 | **Lines:** 36,660 | **Lang:** Rust
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** DevOps teams managing 5-100 VPS
- **Revenue model:** Open source core + paid managed offering
- **Competitive with:** Ansible, Kubernetes (for small fleets), Coolify
- **Key features:** Circuit breaker, rollback, health checks, auto-provisioning (Caddy, Litestream, Nix, Git, Warden), 17K-line rules engine
- **On GitHub:** No — **STRONG open source candidate**
- **Why publish:** Solves a real problem, no good alternative for small fleets, would get traction in DevOps community

### dracon-terminal-engine
- **What:** Terminal application framework for Rust — compositor, widgets, z-indexed layers, themes
- **Files:** 327 | **Lines:** 144,390 | **Lang:** Rust
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Rust developers building TUIs
- **Revenue model:** Open source credibility; optional paid support only if packaged as a productized offer.
- **Competitive with:** ratatui (but this is a framework ON TOP of ratatui)
- **On GitHub:** Yes (1★) — **needs more visibility**
- **Why publish:** Most impressive Rust project by size. Frameworks get more stars than apps.

### rust-ai-web-auto
- **What:** Enterprise AI-driven browser automation engine using raw Chrome DevTools Protocol
- **Files:** 53 | **Lines:** 12,180 | **Lang:** Rust + JavaScript
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Dev teams, automation agencies
- **Revenue model:** Open source core + paid enterprise features
- **Competitive with:** Playwright, Puppeteer (but Rust-based, AI-driven)
- **On GitHub:** No — **STRONG open source candidate**
- **Why publish:** Playwright/Puppeteer market is huge. A Rust alternative with AI would get attention.

### dracon-libs
- **What:** Multi-language Rust libraries — git, terminal, system, files, TTS, STT
- **Files:** 61 | **Lines:** 12,025 | **Lang:** Rust
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Rust developers
- **Revenue model:** Open source (ecosystem play)
- **On GitHub:** Yes (1★) — **needs more visibility**
- **Why publish:** Library collections get traction. Each sub-library is useful independently.

### tiles-tui-file-manager
- **What:** Dual-pane TUI file manager — vim-style nav, git integration, SSH, system monitor
- **Files:** 58 | **Lines:** 22,092 | **Lang:** Rust + Python
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Terminal users, developers
- **Revenue model:** Open source (credibility)
- **On GitHub:** Yes (2★)
- **Why publish:** TUI file managers are always in demand. Already has stars.

### cli-file-manager
- **What:** Contextual file manager — git status, build status, TODO count, languages, ports, docker
- **Files:** 42 | **Lines:** 9,905 | **Lang:** Rust
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Developers
- **Revenue model:** Open source (credibility)
- **On GitHub:** No
- **Why publish:** Unique concept — "ls on steroids." Would get attention.

### obs-wayland-hotkey
- **What:** OBS hotkey daemon for Wayland/X11
- **Files:** 7 | **Lines:** 2,200 | **Lang:** Rust
- **Maturity:** Alpha (CI ✓)
- **Audience:** Streamers on Linux
- **Revenue model:** Open source (credibility)
- **On GitHub:** Yes (8★) — highest starred repo
- **Why publish:** Already published, already has stars. Keep it.

### ai-auto-repo-rot-scanner-todo-agent
- **What:** AI-powered repository rot scanner — detects outdated deps, security vuls, drift
- **Files:** 61 | **Lines:** ~15,000 | **Lang:** Rust
- **Maturity:** Alpha (tests ✓)
- **Audience:** Dev teams maintaining many repos
- **Revenue model:** Open source + SaaS dashboard
- **On GitHub:** No
- **Why publish:** Unique tool. Repo maintenance is a real problem.

### ai-auto-writer
- **What:** AI-powered book generation with pluggable storage
- **Files:** 100 | **Lines:** 31,800 | **Lang:** Rust + Python
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Writers, content creators
- **Revenue model:** Open source + hosted service
- **On GitHub:** No
- **Why publish:** Novel concept. Rust + AI is a compelling combo.

---

## TIER 3 — BROWSER EXTENSIONS (revenue via Chrome Web Store)

### Primary (sell these)

| Extension | Files | What it does | Revenue model |
|:----------|------:|:-------------|:--------------|
| **SamAI** | 144 | AI browser companion — summarize, chat, fill forms | Freemium (BYOK free + paid tier) |
| **vidpro-extension** | 111 | YouTube Studio optimization — titles, tags, descriptions | Freemium |
| **api-debugger** | 91 | Capture & debug HTTP requests | Freemium |
| **bugkit** | 58 | Capture replayable bug evidence bundles | One-time purchase |
| **auto-form-filler** | 48 | AI form filling, BYOK | Freemium |

### Secondary (utility)

| Extension | Files | What it does |
|:----------|------:|:-------------|
| **ai-ats** | 32 | AI candidate screening |
| **ai-job-finder** | 24 | Auto-apply for jobs |
| **dark-mode-themes** | 35 | Dark mode with natural colors |
| **custom-search** | 29 | Block content farms from Google |
| **youtube-dislike** | 69 | Comment sentiment analysis |

### Novelty (not products)

death-note-typing-practice, cinematic-pages, cursor-style, volume-and-video-pro, live-reload-pro, full-page-screenshot, custom-history, google-tasks-pro, calmweb

---

## TIER 4 — VIDEO TOOLS (YouTube pipeline)

| Project | Files | Lines | What it does |
|:--------|------:|------:|:-------------|
| **video-uploader** | 30 | 9,282 | Rust CLI for YouTube uploads via Data API v3 |
| **youtube-video-uploader** | 30 | 9,001 | Same (duplicate?) |
| **avid** | 18 | 4,549 | Auto video processor — silence detection, audio enhancement |
| **video-factory** | 27 | ~5,000 | Web platform for video processing with FFmpeg + S3 |

**These are internal creator workflow tools. They are useful proof and possible product seeds, but not public products until packaged.**

---

## TIER 5 — GAMES & CREATIVE

### Junk-Runner-bevy
- **What:** Survival roguelike with neural terminal aesthetic — manage resources, make deals, stay alive
- **Files:** 97 | **Lines:** 18,897 | **Lang:** Rust (Bevy engine + Tauri)
- **Maturity:** Prototype
- **Audience:** Gamers, roguelike fans
- **Revenue model:** Open source (portfolio piece) or Steam release
- **On GitHub:** No
- **Note:** Bevy is a hot Rust game engine. A roguelike in Bevy + Tauri is technically impressive.

### one-mil-girls
- **What:** Visual novel — 100 Girlfriends-inspired dating sim
- **Files:** 47 | **Lines:** 3,436 | **Lang:** Rust (Tauri) + SvelteKit + TypeScript
- **Maturity:** Prototype
- **Audience:** Visual novel fans
- **Revenue model:** Niche — Steam itch.io release
- **On GitHub:** No
- **Note:** Tauri + SvelteKit is a compelling tech stack showcase.

---

## TIER 6 — UTILITIES & TOOLS

### wal-backup
- **What:** Self-contained SQLite database API with automatic S3 backup
- **Files:** 74 | **Lines:** 26,316 | **Lang:** Rust
- **Maturity:** Beta
- **Audience:** Developers who need simple persistent storage
- **Revenue model:** Open source + hosted service
- **On GitHub:** Yes
- **Why publish:** Unique concept — SQLite as a service with S3 backup. Solves a real problem.

### dracon-ai-lib
- **What:** Unified AI provider access — automatic failover, circuit breaking, lane-based routing
- **Files:** 19 | **Lines:** 4,411 | **Lang:** Rust
- **Maturity:** Alpha
- **Audience:** Rust developers building AI apps
- **Revenue model:** Open source (ecosystem play)
- **On GitHub:** Yes
- **Why publish:** Useful library. AI provider failover is a real problem.

### dracon-voice-notifications (Kiki)
- **What:** Desktop announcer — speaks notifications aloud with personality
- **Files:** 14 | **Lines:** 5,865 | **Lang:** Rust
- **Maturity:** Alpha (on crates.io)
- **Audience:** Desktop users who want audio notifications
- **Revenue model:** Open source (fun project, gets attention)
- **On GitHub:** Yes
- **Why publish:** Novel concept. "Sassy desktop announcer" is memorable.

### respec-spec-reconciler
- **What:** Spec-driven reconciliation for Pi — read SPEC.md, work through requirements, loop
- **Files:** 15 | **Lines:** 2,368 | **Lang:** TypeScript
- **Maturity:** Beta (tests ✓, CI ✓)
- **Audience:** Pi users
- **Revenue model:** Internal tool
- **On GitHub:** No

### git-seal
- **What:** Git filter — encrypts sensitive files on commit, decrypts on checkout
- **Files:** 1 | **Lines:** 139 | **Lang:** Go
- **Maturity:** Prototype
- **Audience:** Developers with secrets in repos
- **Revenue model:** Open source (utility)
- **On GitHub:** Yes (2★)
- **Why publish:** Already published. Small but useful.

### kittentts-showcase
- **What:** TTS voice showcase
- **Files:** 7 | **Lines:** 382 | **Lang:** ?
- **Maturity:** Prototype
- **Audience:** Demo/showcase
- **Revenue model:** None (portfolio piece)
- **On GitHub:** No

### test-auto-create
- **What:** Test project
- **Files:** 0 | **Lines:** 0
- **Maturity:** N/A
- **Revenue model:** None
- **On GitHub:** No
- **Note:** Can be excluded from inventory — it's an empty test directory.

---

## TIER 7 — INFRASTRUCTURE (public installable utilities)

| Project | Files | Lines | What it does |
|:--------|------:|------:|:-------------|
| **dracon-utilities** | 67 | 35,406 | CLI binaries — sync, system-guard, warden |
| **dracon-system** | part of utilities | 3,401 (main.rs) | System monitoring, SSH, notifications |
| **dracon-sync** | part of utilities | 8,000+ (sync+policy+report) | Git auto-commit, multi-mirror daemon |
| **dracon-warden** | part of utilities | 2,120+1,534 | Security — encryption, team keys, secret scanning |

**These are public installable utilities.** Feature the parent repo while keeping `dracon-sync`, `dracon-system`, and `dracon-warden` as distinct component stories, not as a raw dump.

---

## GitHub Profile Strategy

### Pin (3-4 repos)
1. **dracon-terminal-engine** — 144K lines of Rust, most impressive project
2. **pully-fully** (if published) — real DevOps tool, would get traction
3. **tiles-tui-file-manager** — already has stars, shows shipping ability
4. **obs-wayland-hotkey** — concrete utility people understand quickly; already has stars

### Link in README
- **SamAI** — link to verified Chrome Web Store page when confirmed (not source code)
- **dracon-utilities** — public installable utilities; feature the three component jobs, but do not spend a pin slot on the vague "utilities" label
- **YouTube channel** — content marketing
- **dracon.uk** — website

### Keep private
- dracon-platform, dracon-code, dracon-demons — core business
- All browser extensions (except SamAI) — don't show hand

### Publish to GitHub (if open-sourcing)
- **pully-fully** — strongest open source candidate
- **rust-ai-web-auto** — Playwright alternative, would get attention
- **ai-auto-repo-rot-scanner** — unique tool
- **ai-auto-writer** — novel concept

---

## Monetization Summary

| Product | Model | Price point | Timeline |
|:--------|:------|:------------|:---------|
| SamAI (free tier) | Free BYOK | $0 | Verify store page, then publish pricing |
| SamAI (paid tier) | Subscription | $5-10/mo | Build backend |
| vidpro-extension | Freemium | $5-10/mo | Package + publish |
| api-debugger | Freemium | $3-5/mo | Package + publish |
| dracon-platform | B2B SaaS | $50-200/mo | Build pricing page |
| pully-fully | Open core + managed | Free + $20/mo managed | Publish + build managed |
| rust-ai-web-auto | Open core + enterprise | Free + custom pricing | Publish + enterprise sales |
| YouTube | Ad revenue + sponsorships | Variable | Grow channel |
| Productized support | Fixed-scope support/license add-on | Productized pricing | Only if tied to an existing asset |

---

## Total Codebase Value

| Metric | Value |
|:-------|------:|
| Total projects | 50 (31 local + 19 extensions) |
| Total src files | ~2,500 |
| Total lines of code | ~660,000 |
| Rust projects | 18 |
| TypeScript/JS projects | 20+ |
| Python projects | 5 |
| Go projects | 1 |
| Beta maturity | 18 projects |
| Already on GitHub | 11 repos |
| Already on Chrome Web Store | 1 claimed (SamAI; verify before public use) |
| On crates.io | 1 (Kiki) |

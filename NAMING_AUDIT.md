# DraconDev Project Naming Audit

Deep analysis of every project — each examined via README, source code, Cargo.toml, and actual behavior.

---

## OPAQUE — Name gives zero clue what it is

### azumi → KEEP (always pair with subtitle)
**What it is:** Full-stack Rust web framework. Server-rendered HTML with client-side interactivity. Zero custom JS. 41 widgets. Compile-time CSS validation. ~16kb runtime. The "HTMX but instant" answer.

**Why opaque:** "Azumi" is a Japanese surname/manga character. Zero hint it is a web framework.

**Options considered:**
- `azumi-web` — Brand + domain, searchable / ✗ Crate rename on crates.io is disruptive
- `keep azumi` — Brand established on crates.io, README subtitle solves discoverability / ✗ GitHub search alone won't find it

**→ KEEP** — Always pair with subtitle "azumi — Rust web framework". Renaming a published crate on crates.io hurts more than it helps.

---

### tiles → tiles-fm
**What it is:** Terminal dual-pane file manager. Built-in text editor (syntect), git integration, SSH browsing, system monitor. Vim-style. Smart terminal spawning. 60FPS.

**Why opaque:** Could be a game, a tile renderer, a grid layout library, a window manager...

**Options considered:**
- `tiles-fm` — Brand + domain. `fm` is universal terminal file manager abbreviation (like lf, nnn) / ✗ Minor crate rename
- `tiles-file-manager` — Fully explicit / ✗ Long. crates.io name is already this and it's too long

**→ tiles-fm** — Retains brand, instantly tells you the domain. The crates.io name `tiles-tui-file-manager` is already too long.

---

### terma → terma-tui
**What it is:** Low-level terminal compositor/rendering engine. Z-indexed layers, TrueColor, SGR mouse, Kitty keyboard protocol. Drop-in Ratatui support. The rendering layer UNDER dracon-terminal-engine.

**Why opaque:** Could be a terminal emulator, multiplexer, theme pack...

**Options considered:**
- `terma-tui` — Brand + domain. Pairs with tiles-fm pattern / ✗ None significant
- `terma-compositor` — More specific about rendering role / ✗ "Compositor" is jargon for non-TUI devs

**→ terma-tui** — Consistent with tiles-fm pattern. Short. Domain-clear.

---

### SamAI → samai-web-assistant
**What it is:** Browser extension: summarize any page, chat with websites, extract content, fill forms. Multi-provider AI (Mistral, NVIDIA, OpenRouter). Also has a CLI component. Lives inside browser-extensions monorepo.

**Why opaque:** "Sam" could be a person, SAM file format, AWS SAM... The "AI" suffix helps but not enough.

**Options considered:**
- `samai-web-assistant` — Brand + domain. "Web assistant" captures browser + AI / ✗ Longer but justified
- `sam-assistant` — Shorter, drops brand awkwardness / ✗ Loses the SamAI brand

**→ samai-web-assistant** — Brand + domain. Being inside the monorepo means the repo name matters less, but the extension name should be clearer.

---

### Junk-Runner-bevy → junk-runner
**What it is:** Procedural space trading roguelike. Dark economy — no heroes, no redemption, just credits. Crew management, haggling, bribes, contraband. Neural terminal aesthetic. Full name is "Junk Runner // Neural Terminal".

**Why opaque:** "Junk Runner" is actually a good game name. The problem is `-bevy` suffix — that's the engine, not the game. No game names itself after its engine.

**Options considered:**
- `junk-runner` — Clean game name. Bevy is implementation detail / ✗ Cargo crate would need rename
- `junk-runner-roguelike` — Genre tag for discoverability / ✗ Unnecessary

**→ junk-runner** — Drop the engine suffix. The game IS "Junk Runner". The "Neural Terminal" subtitle goes in the tagline.

---

## VAGUE — Name hints at domain but doesn't say what it DOES

### dracon-demons → dracon-services
**What it is:** 5 platform microservice daemons: auth-daemon, billing-daemon, email-daemon, ai-daemon, bucket-daemon. Connected via tarpc over Unix sockets. README calls them "Internal service-to-service daemons for the Dracon platform."

**Why vague:** "Demons" is a cute Unix spelling of daemons, but tells you nothing about WHICH services. Are they system daemons? Game daemons? Service daemons for what platform?

**Options considered:**
- `dracon-services` — Standard, clear, immediately understood / ✗ Generic — could be confused with dracon-utilities (also services)
- `dracon-platform-services` — Explicit about being the platform layer / ✗ Long. "platform" is already in dracon-platform
- `dracon-microservices` — Accurate about architecture / ✗ "Microservices" is jargon, not identity

**→ dracon-services** — Clean, standard. Pairs with `dracon-system-services` for the local daemons. The cute "demons" spelling can be a subtitle in the README.

---

### dracon-utilities → dracon-system-services
**What it is:** 3 LOCAL system daemons: dracon-sync (invisible git auto-commit + 3-mirror sync), dracon-system/dracon-system-guard (disk space watcher + process renicer), dracon-warden (secret encryption in git repos). All systemd user services.

**Why vague:** "Utilities" is a junk drawer word. These are NOT arbitrary scripts — they are 3 specific local system daemons. And they are LOCAL (your machine), not platform (the Dracon service layer).

**Options considered:**
- `dracon-system-services` — Clear scope: local system daemons. Distinguishes from dracon-services (platform) / ✗ Long-ish
- `dracon-local-services` — Explicit about local scope / ✗ "Local" is ambiguous (localhost? local network?)
- `dracon-guard` — Evocative — all three services are protective (sync guards your work, system guards your machine, warden guards your secrets) / ✗ Abstract — loses the "services" framing

**→ dracon-system-services** — Clean pairing with dracon-services. One is platform services (auth, billing, email), the other is system services (sync, guard, warden). The scope difference is immediate.

---

### dracon-platform → dracon-portal
**What it is:** User-facing web apps: public-site-app (landing pages), products-app (pricing/licensing), dashboard-app (authenticated user dashboard), ai-api-gateway (external AI API proxy), ai-hub (provider rankings/plans). Uses azumi + Axum + Caddy. README says "This repo owns what users see."

**Why vague:** "Platform" = everything and nothing. Is it infrastructure? API? SDK? It's specifically the web UI layer that users interact with.

**Options considered:**
- `dracon-portal` — Captures the multi-app gateway nature. Public site + products + dashboard + AI hub = portal / ✗ "Portal" has corporate intranet vibes
- `dracon-web-apps` — Literal, clear / ✗ Dry, generic
- `dracon-dashboard` — The primary app IS the dashboard / ✗ Misses the public site and products apps
- `dracon-site` — Short / ✗ Too simple — implies just one site

**→ dracon-portal** — It IS a portal: multiple apps behind one gateway. "What users see" = portal. Better than dashboard (which is only one section).

---

### dracon-code → dracon-agent
**What it is:** Autonomous engineering runtime. Blueprint-driven: you write `plan/blueprint.toml`, it executes slices, runs gate verifiers (fmt/clippy/test), checkpoints progress, recovers automatically, escalates with outcome report. Headless CLI. "No chat-driven step execution. The blueprint is the only source of truth."

**Why vague:** "Code" = editor? IDE? LSP? Code review? Code generator? It is specifically an autonomous agent that executes engineering plans from blueprints.

**Options considered:**
- `dracon-agent` — It IS an agent. Blueprint-driven, autonomous. The word "agent" in 2026 means exactly this / ✗ "Agent" is overloaded (AI agent, secret agent...)
- `dracon-runner` — Part of its identity (`dracon run`) / ✗ "Run" is just one command, not the identity
- `dracon-runtime` — README says "execution runtime" / ✗ Too generic — runtime of what?

**→ dracon-agent** — It acts autonomously on blueprints. The README literally says "autonomous execution runtime." That is an agent. The blueprint model is exactly what "agent" means in the current landscape.

---

### volume-and-video-pro → vidpro
**What it is:** Chrome extension: per-tab volume and video playback controls. Already named "vidpro-extension" (v1.13.634) inside the browser-extensions monorepo.

**Why vague:** Two things crammed together (volume AND video). "Pro" is a meaningless suffix. The REAL product name is already "VidPro" inside the monorepo.

**Options considered:**
- `vidpro` — Its actual product name already. Short, memorable / ✗ Drops "volume" but volume IS part of video controls
- `browser-media-controls` — Fully descriptive / ✗ Generic, no brand identity

**→ vidpro** — Use the real product name. VidPro is already what it's called inside the monorepo. No reason to have a different repo-level name.

---

### video-uploader → youtube-uploader
**What it is:** Rust library + CLI for uploading videos to YouTube via Data API v3. Resumable chunked uploads (308 resume), encrypted credential storage, multi-channel workspaces, batch processing via CSV manifest. Published on crates.io as video-uploader v0.2.

**Why vague:** Upload to WHERE? Vimeo? S3? Custom server? It's YouTube-specific. The name hides the most important detail.

**Options considered:**
- `youtube-uploader` — Exact. No ambiguity about target / ✗ Crate rename on crates.io needed
- `yt-uploader` — Short, uses common YT abbreviation / ✗ Less professional

**→ youtube-uploader** — Be specific. If you add Vimeo later, rename then. Generic names for specific tools help nobody. Crate is v0.2 — early enough to rename.

---

## STALE NAME — Project already renamed itself

### dracon-rust-ui → dracon-app-engine
**What it is:** ALREADY RENAMED internally to "Dracon App Engine". AI-first app engine. ECS-based UI with Vello GPU rendering. 6 shell archetypes (launcher, workspace, detail, dashboard, editor, media). Bounded theme system (10 themes). 44 showcases. 406 tests. App catalog with manifests.

**Why stale:** The project already calls itself "Dracon App Engine" in its own README. The repo name `dracon-rust-ui` is from an earlier era when it was just a UI toolkit. Now it's a full app engine with shells, capabilities, and an AI styling contract.

**→ dracon-app-engine** — The project already renamed itself. Just sync the repo name to match. This is a no-brainer.

---

## UNWIELDY — Two concepts crammed into one name

### dracon-spark-and-director → dracon-fleet
**What it is:** Pull-based fleet management for VPS nodes. Spark = node agent (runs on each VPS, pulls desired state from git, reports health). Director = control plane (manages desired state, assigns nodes, deploys services, syncs DNS to Cloudflare). Contracts = shared types.

**Why unwieldy:** Two nouns joined by "and". Repos shouldn't list their internal components in their name. The system IS a fleet manager — spark and director are just its parts.

**Options considered:**
- `dracon-fleet` — What the system IS. Clean. Spark & director are components OF the fleet / ✗ None significant
- `dracon-fleet-manager` — Unambiguous / ✗ Redundant — fleet already implies management

**→ dracon-fleet** — Clean, evocative. The system manages a fleet of VPS nodes. Spark and director stay as component names inside the repo.

---

## OVERLONG — Name includes trigger condition, not identity

### opencode-auto-review-completed-todos → opencode-auto-review
**What it is:** OpenCode plugin: listens for todo.updated events. When all todos are completed/cancelled, sends a review message. Debounced. Pairs with opencode-todo-reminder.

**Why overlong:** 36 characters. "completed-todos" is the TRIGGER condition, not the name. What it does is: auto-review.

**→ opencode-auto-review** — The trigger condition belongs in the description, not the name.

---

## WRONG SUFFIX — Implementation detail in the name

### browser-extensions-shared → browser-extensions
**What it is:** Monorepo of 17 Chrome extensions: SamAI, api-debugger, calmweb, custom-history, auto-form-filler, ai-ats, bugkit, web-automator, full-page-screenshot, dark-mode-themes, custom-search, youtube-dislike, death-note-typing-practice, cursor-style, live-reload-pro, vidpro-extension, volume-and-video-pro. WXT + React + TypeScript.

**Why wrong:** "-shared" implies shared code/utilities between extensions. It's NOT a shared-utilities repo — it's the EXTENSIONS MONOREPO. The extensions ARE the product, not the shared code.

**→ browser-extensions** — Drop "-shared". The shared code is an implementation detail. The repo IS the extensions.

---

## OLDER SUBSET — Keep repo, remove from README promotion

### opencode-auto-force-resume → KEEP REPO, remove from README
**What it is:** OpenCode plugin v6.0.6. Detects stalled sessions, aborts, sends continue. Stall recovery + todo context + review on completion + nudger + auto-compaction + terminal timer + progress bar.

**Why not in README:** opencode-auto-continue (v7.21.0) does everything this does PLUS 4-layer compaction, question auto-answer, plan-aware continue, hallucination loop detection, tool-text recovery, prompt guard, custom prompts, session monitor. Force-resume is the earlier, simpler version. Still useful for simpler setups — just shouldn't be promoted as a peer.

**→ Keep the repo, remove from README.** Auto-continue is the current flagship. No rename needed.

---

## NEEDS README — Can't name what you can't describe

### video-factory → likely video-pipeline
**What it is:** Video processing pipeline service. Worker does: FFmpeg processing, S3 uploads, Whisper transcription, thumbnail generation, title generation. API server: auth, DB (PostgreSQL), S3 integration, SSE for progress, YouTube integration. No README, no git remote.

**→ video-pipeline** — It IS a pipeline: ingest → process → transcribe → thumbnail → upload → notify. "Factory" implies creation; "pipeline" implies processing stages. But write a README first.

---

## SUMMARY TABLE

| Current | Recommended | Category |
|:--------|:-----------|:---------|
| `azumi` | **keep** (always subtitle) | OPAQUE |
| `tiles` | **`tiles-fm`** | OPAQUE |
| `terma` | **`terma-tui`** | OPAQUE |
| `SamAI` | **`samai-web-assistant`** | OPAQUE |
| `Junk-Runner-bevy` | **`junk-runner`** | OPAQUE |
| `dracon-demons` | **`dracon-services`** | VAGUE |
| `dracon-utilities` | **`dracon-system-services`** | VAGUE |
| `dracon-platform` | **`dracon-portal`** | VAGUE |
| `dracon-code` | **`dracon-agent`** | VAGUE |
| `volume-and-video-pro` | **`vidpro`** | VAGUE |
| `video-uploader` | **`youtube-uploader`** | VAGUE |
| `dracon-rust-ui` | **`dracon-app-engine`** | STALE |
| `dracon-spark-and-director` | **`dracon-fleet`** | UNWIELDY |
| `opencode-auto-review-completed-todos` | **`opencode-auto-review`** | OVERLONG |
| `browser-extensions-shared` | **`browser-extensions`** | WRONG SUFFIX |
| `opencode-auto-force-resume` | **keep repo, remove from README** | OLDER SUBSET |
| `video-factory` | **`video-pipeline`** (needs README) | NEEDS README |

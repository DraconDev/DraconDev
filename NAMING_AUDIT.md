# DraconDev Project Naming Audit

Every project examined via README, source, Cargo.toml, and actual behavior.
Names judged by one test: **would you click on it in a GitHub search?**

Your best names already prove the pattern:
- `obs-wayland-hotkey` → "OBS hotkeys on Wayland? Yes please"
- `git-ai-committer` → "AI commits for me? Yes please"

**3-5 words. Name the problem + hook. Make them want it.**

---

## Final Recommendations

| Current | → Proposed | Words | Why |
|:--------|:----------|:------|:----|
| `azumi` | **`zero-js-web`** | 3 | "Zero JS" is the irresistible hook. You get SSR + interactivity without writing JavaScript. Sounds impossible → they click. |
| `tiles` | **`terminal-file-workspace`** | 3 | "File" = findable in search. "Workspace" = tempting (it's more than a file manager — editor, git, SSH, monitor). |
| `terma` | **`layered-terminal-engine`** | 3 | "Layered" is the hook. Z-indexed rendering in a terminal? TUI devs click. |
| `SamAI` | **`ai-web-agent`** | 3 | "AI" = how, "web" = where, "agent" = what. It acts on the web for you. |
| `Junk-Runner-bevy` | **`junk-runner`** | 2 | Games are the rare 2-word case. "Junk Runner" IS the game. Drop the engine suffix. |
| `dracon-demons` | **`backend-service-daemons`** | 3 | Ready-made backend: auth, billing, email, AI routing, storage. "Daemons" = long-running services. "Backend service" = what they are. |
| `dracon-utilities` | **`system-guard-daemons`** | 3 | "Guard" is the hook. Sync guards your work, disk guard guards your machine, warden guards your secrets. |
| `dracon-platform` | **`platform-web-apps`** | 3 | It's the web apps of the platform. Public site + products + dashboard + AI hub. Clear. |
| `dracon-code` | **`auto-ai-agent`** | 3 | 🔥 "AI agent that runs itself?" — `auto` = autonomous, `ai` = the hook, `agent` = the what. Not pigeonholed as coding-only. |
| `dracon-rust-ui` | **`ai-app-engine`** | 3 | 🔥 "AI builds my app??" — the project already renamed itself to "Dracon App Engine" internally. "AI" is the hook. |
| `dracon-spark-and-director` | **`vps-fleet-manager`** | 3 | You have VPS nodes? You click. "Fleet" is evocative. Spark & director stay as internal component names. |
| `volume-and-video-pro` | **`tab-volume-video`** | 3 | "Tab" is the differentiator. Per-tab volume + video controls = why this exists. |
| `video-uploader` | **`youtube-uploader`** | 2 | 2-word rare case. YouTube is the target. Be specific. |
| `opencode-auto-review-completed-todos` | **`opencode-todo-review`** | 3 | "Completed-todos" was the trigger condition, not the name. What it does: review when todos are done. |
| `browser-extensions-shared` | **`browser-extensions`** | 2 | 2-word rare case. It IS the extensions. "-shared" was an implementation detail. |
| `video-factory` | **`automated-video-pipeline`** | 3 | "Automated" is the hook. Drop video in, processed product comes out. |
| `opencode-auto-force-resume` | **keep repo, remove from README** | — | Older/simpler version (v6) of auto-continue (v7). Still useful, just not the flagship. |

---

## Detailed Reasoning

### azumi → zero-js-web
**What it is:** Server-rendered HTML + client interactivity, all Rust. 41 widgets. Compile-time CSS validation. ~16kb runtime. The "HTMX but instant" answer.

**Why not the other options:**
- `azumi` — Brand only works if you already know it. Crates.io name stays `azumi` but the REPO should be findable.
- `compile-time-web` — Accurate but sounds like a build tool, not a framework.
- `zero-js-web` — "Zero JavaScript? In a web framework? How??" → they click. That's the irresistible hook.

**Note:** The crate on crates.io stays `azumi`. The repo name becomes `zero-js-web`. README opens with "zero-js-web (formerly azumi)" or "zero-js-web — the framework previously known as azumi."

---

### tiles → terminal-file-workspace
**What it is:** Dual-pane file manager + text editor + git integration + SSH browsing + system monitor. Vim-style. 60FPS.

**Why not the other options:**
- `tiles` — Opaque. Could be a game, a tile renderer, a window manager.
- `tiles-fm` — "fm" is jargon. Also loses the "more than files" hook.
- `terminal-workspace` — Tempting but not FINDABLE. You search "file manager" and miss it.
- `terminal-file-workspace` — "File" = findable. "Workspace" = tempting. Best of both.

---

### terma → layered-terminal-engine
**What it is:** Low-level terminal rendering engine. Z-indexed layers, TrueColor, SGR mouse, Kitty keyboard. Drop-in Ratatui support. The rendering layer underneath dracon-terminal-engine.

**Why not the other options:**
- `terma` — Opaque.
- `terma-tui` — "tui" is jargon.
- `terminal-compositor` — "Compositor" is Wayland jargon.
- `layered-terminal-engine` — "Layered" is the hook. Z-indexed rendering in a terminal? TUI devs are instantly curious.

---

### SamAI → ai-web-agent
**What it is:** Browser extension: summarize any page, chat with websites, extract content, fill forms. Multi-provider AI (Mistral, NVIDIA, OpenRouter).

**Why not the other options:**
- `samai-web-assistant` — "Assistant" is diluted in 2026. Everything is an assistant.
- `smart-web-agent` — "Smart" is weaker than "AI" as a search term.
- `ai-web-agent` — Direct. AI = how, web = where, agent = what. It acts on the web for you.

---

### dracon-code → auto-ai-agent
**What it is:** Blueprint-driven autonomous execution runtime. Write `plan/blueprint.toml`, it executes slices, runs verifiers, checkpoints, recovers, escalates. Headless CLI. NOT just code — the verifiers happen to be `fmt/clippy/test` today, but the architecture is objective-in, result-out. Deployments, content, infrastructure — anything.

**Why this works:** `auto` = it runs itself (the hook — you walk away). `ai` = how it achieves things (the #1 search term in 2026). `agent` = what it is (acts on your behalf). Not pigeonholed as coding-only.

**Why not the other options:**
- `autonomous-code-agent` — too narrow. Pigeonholes as coding-only. But it can execute ANY objective.
- `autonomous-objective-agent` — "objective" is corporate jargon. Nobody clicks on that.
- `autonomous-ai-agent` — same meaning but "autonomous" is 10 letters vs "auto" is 4.
- `self-driving-agent` — great metaphor but overloaded with automotive.
- `blueprint-driven-agent` — the differentiator, but "blueprint" isn't a search term.

---

### dracon-rust-ui → ai-app-engine
**What it is:** Already renamed itself to "Dracon App Engine" internally. ECS-based UI + Vello GPU. 6 shell archetypes. 10 themes. 44 showcases. AI styling contract.

**Why this works:** "AI app engine" → "AI builds my app??" The AI-first design IS the differentiator. The project already renamed itself — just sync the repo name.

---

### dracon-spark-and-director → vps-fleet-manager
**What it is:** Pull-based fleet management. Spark = node agent (runs on each VPS). Director = control plane. Contracts = shared types.

**Why this works:** You have VPS nodes? You click. "Fleet" is evocative without being abstract. "Manager" is clear.

**Why not `dracon-fleet`:** Too vague. Fleet of what? Ships? Kubernetes? VPS nodes specifically.

---

### dracon-utilities → system-guard-daemons
**What it is:** 3 local daemons: dracon-sync (auto git commit + 3-mirror sync), dracon-system (disk guard + process renicer), dracon-warden (secret encryption in repos).

**Why this works:** "Guard" is the hook. These three daemons all GUARD something — your work (sync), your machine (disk/process), your secrets (warden). You see "system-guard-daemons" and think "what are they guarding?" → click.

**Why not `dracon-system-services`:** Boring. "Services" could be anything. "Guard daemons" tells you they PROTECT.

---

### dracon-demons → backend-service-daemons
**What it is:** 5 platform daemons: auth, billing, email, AI routing, object storage. tarpc + Unix sockets.

**Why this works:** "Backend service daemons" = ready-made backend. If you need auth + billing + email, you click. "Daemons" = long-running services (the Unix spelling is fine when qualified).

**Pairing with system-guard-daemons:** One is backend (platform-facing), one is system (machine-facing). Clear distinction.

---

## The Naming Pattern

Your best names follow: **[domain]-[hook]-[qualifier]**

| Word | Purpose | Examples |
|:-----|:--------|:---------|
| Domain | WHERE it operates | terminal, vps, youtube, tab, web, browser |
| Hook | WHY you want it | zero-js, autonomous, ai, guard, layered, fleet |
| Qualifier | WHAT it is | engine, agent, daemons, manager, workspace |

The hook is the word that makes someone click. Everything else is searchable context.

---

## What Stays the Same

These names already follow the pattern:

| Repo | Words | Why it works |
|:-----|:------|:------------|
| `obs-wayland-hotkey` | 3 | Domain + problem + solution |
| `git-ai-committer` | 3 | Domain + hook + what |
| `chrome-auto-fullscreen` | 3 | Domain + hook + what |
| `css-peek-pro` | 3 | Domain + hook + qualifier |
| `api-debugger` | 2 | Rare 2-word direct case |
| `ai-vid-editor` | 3 | Hook + domain + what |
| `git-seal` | 2 | Domain + hook (seal=encrypt) |
| `opencode-auto-continue` | 3 | Domain + hook + what |
| `wal-backup` | 2 | Domain + what |
| `ai-auto-writer` | 3 | Hook + hook + what |
| `dracon-terminal-engine` | 3 | Brand + domain + what |
| `dracon-libs` | 2 | Brand + what |

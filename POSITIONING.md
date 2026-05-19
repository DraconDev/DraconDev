# Full Repository Positioning Analysis

All repos audited: 29 on GitHub + local dirs. Goal: name everything well, position clearly.

---

## PUBLIC REPOS — Currently Featured

### ✅ obs-wayland-hotkey (7★)
**What it is:** Rust daemon, global hotkeys for OBS Studio on Wayland + X11. Auto-reconnect. F13–F24 as stream deck buttons. Single binary.

**Verdict:** Name is perfect. Keep as-is. Flagship.
**Should change:** Nothing.

### ✅ git-ai-committer (6★)
**What it is:** VS Code extension. Monitors coding pauses, AI-generates commit messages, auto-commits. No interrupting your flow.

**Verdict:** Name is perfect. Keep as-is. Flagship.
**Should change:** Nothing.

### ⚠️ chrome-auto-fullscreen (2★) — GitHub only, NOT locally
**What it is:** Chrome extension. Auto-fullscreen any page on load, navigation, or click-hold. SPA detection (YouTube, Odysee). Exit by moving cursor to top edge.

**Verdict:** Name is good. Clone locally. Keep public.
**Should change:** `gh repo clone DraconDev/chrome-auto-fullscreen`

### ✅ git-seal (2★)
**What it is:** Transparent encryption filter for Git. Commit files so they're encrypted at rest, readable by collaborators with the right key.

**Verdict:** Name is perfect. Keep public.
**Should change:** Nothing.

### ⚠️ azumi (1★) — MISMATCH: GitHub = azumi-live-ssr-framework
**What it is:** Live SSR for Rust, builds on Axum. Server-rendered HTML with instant interactivity, compile-time validation, zero custom JavaScript. ~10KB gzipped. 41 widgets.

**Verdict:** GitHub name is too long. Local name `azumi` is too opaque. Needs a better name.
**Should change:** Rename GitHub repo to match local (`azumi`). Keep `azumi` but pair with better subtitle in README.

### ⚠️ tiles (1★) — Local name is opaque
**What it is:** Terminal file manager in Rust. Dual panes, text editor, git integration, SSH browsing, system monitor. Vim-style. 60FPS. Crate name: `tiles-tui-file-manager`.

**Verdict:** Local name `tiles` is opaque. GitHub name matches local. `tiles-tui-file-manager` on crates.io is clearer.
**Should change:** Rename to `terminal-file-workspace` (local) + rename GitHub repo + update crates.io.

### ⚠️ ai-vid-editor (1★) — MISMATCH: GitHub = ai-gui-auto-video-editor
**What it is:** CLI + GUI tool for automated video editing. AGAVE is the product name in the README. Silence cutting, audio enhancement, scene detection, auto-reframe. Crate name: `ai-gui-auto-video-editor`.

**Verdict:** Two names in use: `ai-vid-editor` (local) and `ai-gui-auto-video-editor` (GitHub). The product calls itself `AGAVE` in the README. Need one canonical name.
**Should change:** Align all three (local dir, GitHub repo, crate name) on one name. `agave` is the product name — strong candidate.

### ⚠️ dracon-libs (1★) — GitHub has TWO repos: public `dracon-libs` + private `dracon-libs-l3`
**What it is:** Vertical Rust libraries — git, terminal, system, media, memory. Workspace of reusable crates.

**Verdict:** Name is generic but acceptable for a library collection. Confusion with `dracon-libs-l3`.
**Should change:** Delete `dracon-libs-l3` (private, duplicate). Keep `dracon-libs` as-is.

### ⚠️ dracon-terminal-engine (1★) — Name is bloated
**What it is:** Terminal application framework for Rust. App, widgets, compositor, themes, 15 built-in themes.

**Verdict:** `dracon-terminal-engine` is brand-prefix + domain + what. Acceptable but could be punchier.
**Should change:** Consider shortening to `terminal-engine` or `dracon-tui-engine`. But `dracon-` prefix has brand value. Keep for now.

### ⚠️ terma — NOT PUSHED to GitHub
**What it is:** Next-generation terminal compositor engine for Rust. Z-indexed layers, TrueColor, SGR mouse, Kitty keyboard, drop-in Ratatui support.

**Verdict:** Strong project. Public it. Name `terma` is too opaque.
**Should change:** Rename to `layered-terminal-engine` locally and on GitHub. Push to GitHub.

### ⚠️ opencode-auto-continue (1★)
**What it is:** OpenCode plugin. Stall recovery, todo nudging, review-on-completion, AI-driven session analysis. One plugin replaces three.

**Verdict:** Name is descriptive and accurate. Keep as-is.
**Should change:** Nothing.

### ⚠️ opencode-auto-review-completed-todos (0★) — Name is too long
**What it is:** OpenCode plugin. Auto-detect when all session todos are completed and trigger a review.

**Verdict:** Name is too long. The `opencode-` prefix + `opencode-auto-review-completed-todos` is unwieldy.
**Should change:** Rename to `opencode-todo-review`. Align local dir, GitHub repo, and package.json.

### 🔴 opencode-auto-force-resume (0★) — NOT featured in README
**What it is:** OpenCode plugin. Detects and recovers stalled AI sessions. Older version of auto-continue (v6 vs v7).

**Verdict:** Repo stays public. Don't feature in README. It's superseded by auto-continue.
**Should change:** Nothing.

### ⚠️ volume-and-video-pro (0★) — MISMATCH: name ≠ description
**What it is:** Browser extension. Volume boost (up to 1000%), playback speed, bass boost, voice boost, mono audio, per-site settings. GitHub README says "Volume Booster Pro."

**Verdict:** The GitHub README calls it "Volume Booster Pro." The GitHub name says "volume-and-video-pro." The local name is the same. These three don't agree. And the name doesn't match the description (it's volume/audio focused, not video).
**Should change:** Rename to `tab-volume-booster` or `tab-audio-control`. Update local dir, GitHub repo, and update README.

### ⚠️ api-debugger (1★) — GitHub only, NOT locally
**What it is:** Chrome extension. Captures every HTTP request, replays, modifies, imports from Postman/Insomnia/OpenAPI. No login, no cloud.

**Verdict:** Name is great. Clone locally.
**Should change:** `gh repo clone DraconDev/api-debugger`

---

## PUBLIC REPOS — Never Cloned

These are public on GitHub but have no local directory:

| Repo | Stars | Should clone? |
|:-----|:------|:-------------|
| `calmweb` | 0 | ❌ No description, no clear purpose |
| `kittentts-showcase` | 0 | ❌ One-off showcase, not a product |

---

## PRIVATE REPOS — Flagship Candidates

These are private, 0 stars, but are genuinely strong. Should be publicized.

### auto-ai-agent (was dracon-code) — TOP PRIORITY
**What it is:** Autonomous engineering runtime. Blueprint-driven: write `plan/blueprint.toml`, it executes slices, runs verifiers (fmt/clippy/test), checkpoints, recovers, escalates. Headless CLI + dark GUI (Tauri + SolidJS).

**Why it's flagship:** It's the brain of the ecosystem. "You set the objective. It runs. You supervise exceptions." Not just code — any objective.

**Positioning:** AI coding agent, but not pigeonholed to coding. Blueprint-driven autonomous execution.

**Proposed name:** `auto-ai-agent`

**Should change:** 1) Rename repo `dracon-code` → `auto-ai-agent`. 2) Make public. 3) Feature in README as #3 after obs-hotskey + git-ai-committer.

### ai-app-engine (was dracon-rust-ui)
**What it is:** ECS-based GPU UI engine for Rust. Vello rendering. 10 themes. 44 showcases. 6 shell archetypes. AI styling contract (describe a UI, it generates it).

**Why it's strong:** Vello GPU rendering in Rust is rare. The AI styling angle is compelling.

**Proposed name:** `ai-app-engine`

**Should change:** 1) Rename repo `dracon-rust-ui` → `ai-app-engine`. 2) Make public. 3) Feature in README under Rust section.

### ai-web-agent (was SamAI)
**What it is:** Browser extension. Summarize any page, chat with websites, extract content, fill forms. Multi-provider AI (Mistral, NVIDIA, OpenRouter).

**Proposed name:** `ai-web-agent`

**Should change:** 1) Rename repo `SamAI` → `ai-web-agent`. 2) Make public. 3) Feature in README under Browser section.

---

## PRIVATE REPOS — Useful but Niche

| Repo | What it is | Should publicize? | Proposed name |
|:-----|:-----------|:------------------|:-------------|
| `system-guard-daemons` (was dracon-utilities) | dracon-sync (auto git commit + 3-mirror), dracon-system (disk guard + renicer), dracon-warden (secret encryption) | ⚠️ Maybe | `system-guard-daemons` |
| `backend-service-daemons` (was dracon-demons) | Auth, billing, email, AI routing, object storage daemons | ⚠️ Maybe | `backend-service-daemons` |
| `vps-fleet-manager` (was dracon-spark-and-director) | Pull-based VPS fleet management | ⚠️ Maybe | `vps-fleet-manager` |
| `browser-extensions` (was browser-extensions-shared) | Monorepo of 17 Chrome extensions | ⚠️ Maybe | `browser-extensions` |
| `wal-backup` | SQLite WAL replication as a library, not a sidecar | ⚠️ Maybe | `wal-backup` |
| `youtube-uploader` | Rust lib + CLI, multi-channel workspaces, upload profiles | ⚠️ Maybe | `youtube-uploader` |

**Decision:** Keep private for now. Publicize after they accumulate stars or get a README polish.

---

## PRIVATE REPOS — Marginal / Abandoned

| Repo | Issue |
|:-----|:------|
| `ai-auto-writer` | AI book generation. No README, no description. Not compelling at 0 stars. |
| `ai-auto-repo-rot-scanner-todo-agent` | Niche tool, no README. Keep private. |
| `video-factory` | No README, no git. Not ready. |
| `dracon-voice-notifications` | No README, no description. |
| `pi-auto-review` | Pi-specific. No remote. Niche. |
| `Junk-Runner-bevy` | Game. Doesn't fit a tools/infra README. Keep private. |
| `dracon-platform` | Web surfaces. Needs polish before public. |

---

## GRAVEYARD — Delete from GitHub

These are test repos, legacy cruft, duplicates, and abandoned projects. Mass delete:

### Test / Junk
```
test-repo          test-github-private     my-repo
test-conflict      live-reload-pro          custom-history
```

### Legacy `demon-*` era
```
demon-workplace    demon-ext              demon-suite
demon-wp          demon-leg              demon-git-stand
demon-deploy      legacy-fat-demon        demon-legacy
```

### Legacy `dracon-*` duplicates
```
dracon-core        dracon-security         dracon-git
dracon-home        dracon-home-repo         dracon-files
dracon-services    dracon-private-services  dracon-nixos-auto-box
```

### Legacy libs duplicates
```
dracon-libs-legacy
```

### One-off demos / forks
```
nixpkgs            Learn-React             burger-builder
learn-hooks        PriceCompareSite         momo
wallet-guard       junk-runner-tauri        Remi
remi-app           ace                     memory-palace-fm
dracon-diles       rust-extensions          citadel
Nexus              dracon-spark-recipes     dracon-spark
dracon-director    dracon-apis              dracon-releaser
nexus-new-tab      dracon-agent             dracon-ai-directory
dracon-platformer  ai-ats                   web-automator
auto-form-filler   dracon-billing-daemon   dracon-auth-daemon
dracon-email-daemon dracon-ai-daemon        dracon-bucket-daemon
dracon-bucket-service dracon-billing-service dracon-auth-service
dracon-email-service dracon-ai-gateway-service dracon-litestream-daemon
```

### Internal metadata (not real repos)
```
.dracon
vidpro-extension    dracon-ai-gateway-service
```

---

## Current GitHub ↔ Local Name Mismatches (must fix)

| Local Dir | GitHub Repo | Status |
|:----------|:-----------|:-------|
| `azumi` | `azumi-live-ssr-framework` | Mismatch. Align on `azumi`. |
| `ai-vid-editor` | `ai-gui-auto-video-editor` | Mismatch. Align on `agave` (product name) or keep longer form. |
| `dracon-libs` | `dracon-libs-l3` (private) | Duplicate. Delete `l3`. |
| `terma` | (not on GitHub) | Not pushed. Rename to `layered-terminal-engine` then push. |
| `opencode-auto-review-completed-todos` | same | Rename to `opencode-todo-review`. |

---

## GitHub-only Public Repos (not cloned locally)

| Repo | Stars | Action |
|:-----|:------|:-------|
| `api-debugger` | 1★ | Clone locally |
| `chrome-auto-fullscreen` | 2★ | Clone locally |
| `calmweb` | 0★ | Investigate or delete |
| `kittentts-showcase` | 0★ | Investigate or delete |

---

## Summary of Actions

### Immediate (do today)
| Action | Repos |
|:-------|:------|
| Clone from GitHub | `api-debugger`, `chrome-auto-fullscreen` |
| Rename GitHub repos | `azumi-live-ssr-framework` → `azumi`, `ai-gui-auto-video-editor` → `agave` (or `ai-video-editor`) |
| Rename + push | `terma` → `layered-terminal-engine` then push |
| Rename local dirs + repos | `opencode-auto-review-completed-todos` → `opencode-todo-review` |
| Delete duplicate private repos | `dracon-libs-l3` |
| Update README links | All renamed repos |

### Short term (after rename)
| Action | Repos |
|:-------|:------|
| Make public + feature | `auto-ai-agent`, `ai-app-engine`, `ai-web-agent` |
| Clean up public repos | `volume-and-video-pro` → `tab-volume-booster` |

### Long term / Later
| Action | Repos |
|:-------|:------|
| Delete graveyard | All ~40 test/legacy/duplicate repos |
| Publicize niche tools | `system-guard-daemons`, `backend-service-daemons`, `vps-fleet-manager`, etc. |
| Polish before public | `video-factory` (needs README), `ai-auto-writer` (needs README) |

---

## Recommended README After All Changes

When the 3 flagship privates go public, the README becomes:

```
# DraconDev
Performance-first tools. Compile-time guarantees.

obs-wayland-hotkey · git-ai-committer · auto-ai-agent · git-seal
chrome-auto-fullscreen · zero-js-web · agave · terminal-file-workspace
layered-terminal-engine · terminal-engine · dracon-libs · css-peek-pro
opencode-auto-continue · opencode-todo-review · api-debugger · tab-volume-booster
ai-app-engine · ai-web-agent
```
# GitHub Profile README — Scratch Pad

## Pinned Repos (6)

| # | Repo | Lines | Tests | Why it's strong | Hook | Status |
|:-:|:-----|------:|------:|:----------------|:-----|:-------|
| 1 | dracon-terminal-engine | 143K | 3,658 | TUI framework, 43 widgets, on crates.io | "Rust's terminal framework — 43 widgets, 20 themes, one import" | ✓ On GitHub |
| 2 | pully-fully | 27.6K | 1,393 | Fleet reconciler, 9K-line rules engine | "GitOps for small fleets — the gap between Ansible and Kubernetes" | ✗ Needs publishing |
| 3 | dracon-sync | 21K | 341 | Auto-commit, multi-mirror daemon (mechanical messages, not AI) | "Invisible git sync for AI-powered development" | ✗ Needs publishing |
| 4 | dracon-warden | 9K | 171 | Git encryption, secret scanning, team keys | "Your .env files are encrypted in git, plaintext in your working tree" | ✗ Needs publishing |
| 5 | folder-auto-banner | 8K | 108 | Contextual directory dashboard | "Run `f` — see git status, TODOs, ports, and build status instantly" | ✓ On GitHub, 1★ |
| 6 | obs-wayland-hotkey | 2K | 36 | OBS hotkeys on Wayland | "OBS hotkeys that actually work on Wayland" | ✓ On GitHub, 8★ |

**Reorder rationale:** terminal-engine first (impressive scale), pully-fully second (best quality + unique positioning), sync third (AI-relevant timing), warden fourth (security mindset), folder-banner fifth (memorable pitch), obs sixth (already has stars).

**Not pinned (but notable):**
- dracon-system (6K) — disk monitoring, more basic
- ai-auto-writer (31K) — service, not open source
- rust-ai-web-auto (11K) — browser automation
- tiles-tui-file-manager — TUI file manager (2★ on GitHub)
- dracon-ai-lib (4K) — AI provider library

---

## Hero Section

**Option A: Minimal**
```
Rust developer building infrastructure tools.
Working on terminal engines, fleet reconcilers, and AI-powered dev tools.
```

**Option B: With personality**
```
I build systems that run themselves.
Terminal engines, fleet reconcilers, git automation — anything that saves humans from repetitive infrastructure work.
```

**Option C: Direct**
```
Building Rust tools for developers who'd rather write code than manage servers.
```

---

## Links

- 🌐 Website: [dracon.uk](https://dracon.uk)
- 🎥 YouTube: [DraconDev](https://youtube.com/@DraconDev)
- 📦 SamAI: [Chrome Web Store](https://chromewebstore.google.com/detail/samai) *(closed source)*

---

## Pinned Repos (detailed notes)

### 1. dracon-terminal-engine (143K lines, 3,658 tests)
- **What:** Terminal application framework for Rust — app lifecycle, widgets, event handling, styles
- **Lines:** 143,089 | **Files:** 325 | **Tests:** 3,658 | **Clippy:** Clean
- **On crates.io:** Yes
- **Architecture:** App/Ctx entry point → Compositor (Plane layers) → 43 widgets → Command-driven (AI can enumerate/trigger actions)
- **Quality:** ⭐⭐⭐⭐⭐ — massive test suite, clippy clean, proper RAII, feature-gated modules
- **Angle:** "Rust's terminal framework — build TUIs with proper architecture"
- **README:** ⭐⭐⭐⭐ — badges, architecture, code examples. Missing: screenshots/GIFs
- **Competitors:** ratatui (lower-level), crossterm (raw terminal)
- **Hook:** "143K lines of Rust. 43 widgets. 20 themes. One import."

### 2. pully-fully (27.6K lines)
- **What:** Pull-based server fleet reconciler — write desired state in git, nodes reconcile autonomously
- **Lines:** 27,528 | **Files:** 44 | **Tests:** 1,393 `#[test]` annotations across 25 files
- **Architecture:** Control repo (git) → Pully (per-node agent) + Fully (fleet manager) → 9K-line rules engine
- **Quality:** ⭐⭐⭐⭐ — 27.6K lines, comprehensive docs (11 files), security model
- **Angle:** "GitOps for small fleets — the gap between Ansible and Kubernetes"
- **README:** ⭐⭐⭐⭐⭐ — architecture diagram, competitor comparison, bootstrap guide
- **Competitors:** Ansible (push-based), Kubernetes (overkill), Coolify
- **Hook:** "27.6K lines. 9K-line rules engine. No control plane. Git is your database."

### 3. dracon-sync (21K lines)
- **What:** Invisible git sync daemon — auto-commit, multi-mirror (GitHub/GitLab/Codeberg), mechanical blast-radius messages
- **Lines:** 21,504 | **Files:** 29
- **Architecture:** Daemon (systemd) → watch_roots → deterministic sync → multi-mirror push → mechanical blast-radius messages
- **Quality:** ⭐⭐⭐⭐ — self-healing, push failure decision tree, startup cleanup
- **Angle:** "Invisible git sync for AI-powered development"
- **README:** ⭐⭐⭐⭐⭐ — competitor table, push failure decision tree, daemon reliability
- **Competitors:** git-auto-sync, gitea-mirror, git-bridge, swarf
- **Hook:** "Your AI agent makes 50 commits/hour. This keeps them all synced to 3 providers."
- **Note:** README claims AI Scribe but code explicitly removed AI messages ("hallucinate context"), uses mechanical blast-radius instead

### 4. dracon-warden (9K lines, 171 tests)
- **What:** Git filter + repo hardening — encrypts secrets at rest with age encryption
- **Lines:** 9,156 | **Files:** 27 | **Tests:** 171 (16 security test files)
- **Architecture:** Clean/smudge filter pipeline → age encryption (x25519) → secret scanner → team keys
- **Quality:** ⭐⭐⭐⭐⭐ — ReDoS prevention, atomic writes, leak prevention tests, proptest regressions
- **Angle:** "Your .env files are encrypted in git, plaintext in your working tree"
- **README:** ⭐⭐⭐⭐ — mental model section, key hierarchy, recovery tools
- **Competitors:** git-crypt, git-secret, SOPS, BlackBox
- **Hook:** "171 security tests. ReDoS prevention. Atomic writes. Git encryption done right."

### 5. folder-auto-banner (8K lines, 108 tests)
- **What:** Contextual directory dashboard — git status, TODO count, ports, Docker, build status, code metrics
- **Lines:** 8,005 | **Files:** 28 | **Tests:** 108 | **Benchmarks:** Criterion
- **Architecture:** CLI + Daemon (Unix socket) → inotify caching → modules (git/todo/ports/docker/build/code_metrics)
- **Quality:** ⭐⭐⭐⭐ — benchmarks, daemon architecture, release profile optimized
- **Angle:** "ls on steroids — see your project context at a glance"
- **README:** ⭐⭐⭐⭐ — competitor table (vs lsd/eza), config reference, env vars
- **Competitors:** lsd, eza, exa
- **Hook:** "Run `f` in any project. See git status, TODO count, open ports, and build status instantly."

### 6. obs-wayland-hotkey (2K lines, 8★)
- **What:** Lightweight Rust daemon for OBS global hotkeys on Wayland/X11
- **Lines:** 2,200 | **Files:** 7 | **Stars:** 8 (highest)
- **Architecture:** evdev input → OBS WebSocket → systemd service → TOML config
- **Quality:** ⭐⭐⭐ — small, focused, 36 tests for 2K lines
- **Angle:** "OBS hotkeys that actually work on Wayland"
- **README:** ⭐⭐⭐⭐ — badges, install guide, hotkey customization
- **Hook:** "8 stars. The only way to get OBS hotkeys working on Wayland."

---

## What's Missing (to add later)

- [ ] Revenue/products section (when ready)
- [ ] Testimonials/clients (when available)
- [ ] GitHub Sponsors badge
- [ ] More stars (need to publish + promote)

---

## Questions to Resolve

1. **dracon-utilities — monorepo or separate repos?**
   - Separate repos = more pins, more discoverability, more history work
   - Monorepo = cleaner parent repo, preserves history, fewer pins
   - Recommendation: Keep `dracon-utilities` as a simple parent monorepo and make `dracon-sync`, `dracon-system`, `dracon-warden` distinct through component READMEs and profile subpath links

2. **Hero section — which tone?**
   - Minimal vs personality vs direct

3. **Should we show YouTube videos?**
   - If channel has content, embed a latest video
   - If not, just link it

4. **SamAI — show or just link?**
   - Currently: link only (closed source)
   - Could add a screenshot/demo

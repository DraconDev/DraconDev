# GitHub Profile README — Proposal v1

> **File:** `README_PROPOSAL.md` (review this, then we create the real `dracon/.github` README)
> **Style:** Clean showcase with personality — not stat widgets, not walls of text

---

<!-- ============================================================ -->
<!-- HERO                                                          -->
<!-- ============================================================ -->

<div align="center">

# DraconDev

### I build systems that run themselves.

Terminal engines, fleet reconcilers, git automation — anything that saves humans from repetitive infrastructure work.

[🌐 dracon.uk](https://dracon.uk) · [🎥 YouTube](https://youtube.com/@DraconDev) · [📦 SamAI](https://chromewebstore.google.com/detail/samai)

</div>

---

<!-- ============================================================ -->
<!-- WHAT I BUILD                                                  -->
<!-- ============================================================ -->

## What I Build

| | Category | What |
|:--:|:---------|:-----|
| 🖥️ | **TUI Frameworks** | Terminal application runtimes with widgets, themes, and compositor |
| ⚓ | **Fleet Management** | GitOps reconcilers for small server fleets |
| 🔄 | **Git Automation** | Auto-commit daemons, multi-mirror sync, AI-powered workflows |
| 🔒 | **Security Tools** | Git encryption, secret scanning, repo hardening |
| 🛠️ | **Developer Tools** | Contextual file browsers, system monitors, Wayland utilities |
| 🤖 | **AI Products** | Browser companions, form fillers, Chrome extensions |

---

<!-- ============================================================ -->
<!-- FEATURED PROJECTS                                             -->
<!-- ============================================================ -->

## Featured Projects

### 🖥️ dracon-terminal-engine
> **Rust's terminal framework — 43 widgets, 20 themes, one import.**

A complete runtime for building terminal applications. Owns the terminal, input, rendering, and event loop. Command-driven architecture — AI can enumerate and trigger any action.

```rust
use dracon_terminal_engine::framework::prelude::*;

App::new().unwrap()
    .title("My App")
    .fps(30)
    .set_theme(Theme::cyberpunk())
    .on_tick(|ctx, _tick| { /* called every 250ms */ })
    .run(|ctx| { /* build your UI here */ });
```

**143K lines · 3,658 tests · On crates.io**

---

### ⚓ pully-fully
> **GitOps for small fleets — the gap between Ansible and Kubernetes.**

Pull-based server fleet reconciler. Write desired state in git, each node pulls and reconciles autonomously. No control plane. No database. Git is your source of truth.

Built for 5-100 VPS where Kubernetes is overkill and Ansible can't self-heal.

**27.5K lines · 1,393 tests · Circuit breaker, rollback, health checks**

---

### 🔄 dracon-sync
> **Invisible git sync for AI-powered development.**

Auto-commit daemon that watches your repos, commits every change, and pushes to GitHub, GitLab, and Codeberg simultaneously. Self-healing with automatic lock repair and stuck-push recovery.

**21K lines · Systemd service · Multi-mirror push**

---

### 🔒 dracon-warden
> **Your .env files are encrypted in git, plaintext in your working tree.**

Git filter + repo hardening. Age-based encryption with team key support. Clean/smudge pipeline — encrypts on commit, decrypts on checkout. Secret scanning for AWS, GCP, Azure, GitHub keys.

**9K lines · 171 security tests · ReDoS prevention · Atomic writes**

---

### 🛠️ folder-auto-banner
> **`ls` on steroids — see your project context at a glance.**

Contextual directory dashboard. Git status, TODO count, open ports, Docker status, build status, code metrics. Daemon caching for instant repeated access.

```bash
f          # see everything
f --tree   # recursive view
f config   # edit settings
```

**8K lines · 108 tests · Benchmarked with Criterion**

---

### 🖥️ obs-wayland-hotkey
> **OBS hotkeys that actually work on Wayland.**

Lightweight Rust daemon for controlling OBS Studio with global hotkeys on Wayland/X11. evdev input + OBS WebSocket.

**2K lines · 36 tests · On crates.io · ⭐ 8 stars**

---

<!-- ============================================================ -->
<!-- MORE PROJECTS                                                 -->
<!-- ============================================================ -->

## Also Built

| Project | What | Lines |
|:--------|:-----|------:|
| [dracon-system](https://github.com/dracon/dracon-system) | System monitoring — disk cleanup, process renice, trend prediction | 6K |
| [dracon-ai-lib](https://github.com/dracon/dracon-ai-lib) | Unified AI provider access with automatic failover | 4K |
| [dracon-voice-notifications](https://github.com/dracon/dracon-voice-notifications) | Desktop announcer — speaks notifications aloud | 6K |
| [tiles-tui-file-manager](https://github.com/dracon/tiles-tui-file-manager) | Dual-pane TUI file manager with vim-style nav | 22K |

---

<!-- ============================================================ -->
<!-- BY THE NUMBERS                                                -->
<!-- ============================================================ -->

## By the Numbers

| | |
|:--:|:--|
| 🦀 | **170K+ lines of Rust** across 6 open-source projects |
| 🧪 | **5,000+ tests** — every project has comprehensive test coverage |
| 📦 | **2 on crates.io** — terminal-engine and obs-wayland-hotkey |
| 🌐 | **3 providers** — syncing to GitHub, GitLab, and Codeberg |
| 🔒 | **171 security tests** — warden prevents secret leaks |
| ⭐ | **8 stars** — obs-wayland-hotkey, highest-starred repo |

---

<!-- ============================================================ -->
<!-- CURRENT FOCUS                                                 -->
<!-- ============================================================ -->

## Currently

- 🔨 Publishing more utilities from `dracon-utilities` monorepo
- 📝 Growing the YouTube channel with Rust/DevOps content
- 🚀 Building [SamAI](https://chromewebstore.google.com/detail/samai) — AI browser companion

---

<!-- ============================================================ -->
<!-- LINKS                                                         -->
<!-- ============================================================ -->

<div align="center">

### Find Me

[🌐 Website](https://dracon.uk) · [🎥 YouTube](https://youtube.com/@DraconDev) · [📦 SamAI](https://chromewebstore.google.com/detail/samai)

</div>

---

<!-- ============================================================ -->
<!-- NOTES FOR REVIEW                                              -->
<!-- ============================================================ -->

## Review Notes

### What this proposal does:
1. **Hero** — personality-driven, not generic "Rust developer"
2. **What I Build** — 6 categories with icons, shows breadth
3. **Featured Projects** — 6 detailed cards with hooks, code examples, metrics
4. **Also Built** — 4 more projects in a table (maximizes discoverability)
5. **By the Numbers** — credibility signal without stat widgets
6. **Currently** — shows active development
7. **Links** — clean footer

### What's NOT included:
- No GitHub stats cards (they look generic)
- No contribution graphs (your code quality is the stat)
- No animated elements (they break often)
- No walls of text (project cards do the talking)

### Style:
- Clean markdown, no HTML gymnastics
- Icons for visual scanning
- 1-2 line pitches, not paragraphs
- Code example for terminal-engine (shows the framework in action)

### Questions to resolve:
1. **Hero tone** — Is "I build systems that run themselves" right?
2. **Project order** — Should pully-fully be #2 or #3?
3. **"Also Built"** — Include links or just names?
4. **"Currently"** — What are you actually working on?
5. **Footer** — Just links, or add a fun line?

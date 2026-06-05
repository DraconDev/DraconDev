# GitHub Profile README — Scratch Pad

## Pinned Repos (6)

| # | Repo | Lines | Why it's strong | Status |
|:-:|:-----|------:|:----------------|:-------|
| 1 | dracon-terminal-engine | 143K | Massive Rust project, TUI framework, on crates.io | ✓ On GitHub |
| 2 | dracon-sync | 21K | AI auto-commit daemon, multi-mirror, competitor comparison table | ✗ Needs publishing |
| 3 | folder-auto-banner | 8K | "ls on steroids" — git status, TODO count, ports, daemon caching | ✓ On GitHub, 1★ |
| 4 | obs-wayland-hotkey | 2K | OBS hotkeys on Wayland, solves real problem | ✓ On GitHub, 8★ |
| 5 | dracon-warden | 9K | Git encryption + secret scanning, security tool | ✗ Needs publishing |
| 6 | pully-fully | 37K | Fleet reconciler for 5-100 VPSes | ✗ Needs publishing |

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

### 1. dracon-terminal-engine
- **What:** Terminal application framework for Rust — app lifecycle, widgets, event handling, styles
- **Lines:** 143,089 | **Files:** 325 | **Tests:** 112
- **On crates.io:** Yes
- **Angle:** "Rust's terminal framework — build TUIs with proper architecture"
- **README:** Good (badges, architecture, code examples)

### 2. dracon-sync
- **What:** Invisible git sync daemon for AI-powered development
- **Lines:** 21,504 | **Files:** 29
- **Features:** Auto-commit, multi-mirror (GitHub/GitLab/Codeberg), AI commit messages, self-healing git
- **Angle:** "The only tool that combines auto-commit + multi-mirror + AI messages"
- **README:** Excellent (competitor comparison table, install guide, architecture)
- **Competitors:** git-auto-sync, gitea-mirror, git-bridge, swarf

### 3. folder-auto-banner
- **What:** Directory listing with instant context (git status, TODO count, ports, build status)
- **Lines:** 8,005 | **Files:** 28 | **Tests:** 93
- **Angle:** "ls on steroids — see your project context at a glance"
- **README:** Good (competitor comparison vs lsd/eza, install guide)
- **Competitors:** lsd, eza, exa

### 4. obs-wayland-hotkey
- **What:** Lightweight Rust daemon for controlling OBS Studio with global hotkeys on Wayland/X11
- **Lines:** 2,200 | **Files:** 7 | **Stars:** 8
- **Angle:** "OBS hotkeys that actually work on Wayland"
- **README:** Good (badges, install guide)

### 5. dracon-warden
- **What:** Git filter + repo hardening — encrypts secrets at rest in git
- **Lines:** 9,156 | **Files:** 27
- **Features:** Age-based encryption, secret scanning (AWS/GCP/Azure/GitHub), clean/smudge filter pipeline
- **Angle:** "Your .env files are encrypted in git, plaintext in your working tree"
- **README:** Good (architecture, install guide)

### 6. pully-fully
- **What:** Pull-based server fleet reconciler — write recipes in git, Pully enforces them
- **Lines:** 36,660 | **Files:** 44
- **Target:** 5-100 VPSes where Kubernetes is overkill, Ansible can't self-heal
- **Angle:** "GitOps for small fleets — the gap between Ansible and Kubernetes"
- **README:** Excellent (architecture diagram, competitor comparison, install guide)
- **Competitors:** Ansible, Kubernetes, Coolify

---

## What's Missing (to add later)

- [ ] Revenue/products section (when ready)
- [ ] Testimonials/clients (when available)
- [ ] GitHub Sponsors badge
- [ ] More stars (need to publish + promote)

---

## Questions to Resolve

1. **dracon-utilities — monorepo or separate repos?**
   - Separate repos = more pins, more discoverability
   - Monorepo = cleaner, but only 1 pin
   - Recommendation: Separate repos for sync/warden/system

2. **Hero section — which tone?**
   - Minimal vs personality vs direct

3. **Should we show YouTube videos?**
   - If channel has content, embed a latest video
   - If not, just link it

4. **SamAI — show or just link?**
   - Currently: link only (closed source)
   - Could add a screenshot/demo

# DraconDev — Positioning Audit

**Date:** 2026-06-02
**Purpose:** Final recommendation for every project — what to do with it.

---

## Decision Key

| Action | Meaning |
|:-------|:--------|
| **PIN** | Show on GitHub profile pinned repos |
| **LINK** | Reference in README or bio (not pinned) |
| **PUBLISH** | Make public on GitHub as open source |
| **SELL** | Monetize via Chrome Web Store or website |
| **PRIVATE** | Keep closed source, don't show |
| **DELETE** | Remove from filesystem |

---

## Local Projects

### dracon-terminal-engine
- **Action:** PIN
- **Reason:** 144K lines of Rust, most impressive project by size. Terminal frameworks get attention. Already on GitHub (1★) — needs visibility, not code changes.
- **Effort:** None — already published.

### pully-fully-pull-based-fleet-reconciler
- **Action:** PUBLISH + PIN
- **Reason:** Real DevOps tool that solves a real problem (Kubernetes overkill for 5-100 VPS). 37K lines, production-quality (tests ✓, CI ✓). Would get traction in DevOps community. Strongest open source candidate.
- **Effort:** Clean up README, add license, publish.

### tiles-tui-file-manager
- **Action:** PIN
- **Reason:** Already on GitHub (2★), 22K lines, shows shipping ability. TUI file managers are always in demand.
- **Effort:** None — already published.

### obs-wayland-hotkey
- **Action:** PIN
- **Reason:** Highest starred repo (8★), solves a real problem for streamers on Linux. Small but impactful.
- **Effort:** None — already published.

### rust-ai-web-auto
- **Action:** PUBLISH (when ready)
- **Reason:** Enterprise browser automation in Rust. Playwright/Puppeteer alternative. Would get attention in both Rust and automation communities. Not ready yet.
- **Effort:** Needs cleanup before publishing.

### ai-auto-repo-rot-scanner-todo-agent
- **Action:** PUBLISH (when ready)
- **Reason:** Unique tool — repo maintenance is a real problem. AI-powered scanner would get attention.
- **Effort:** Needs cleanup before publishing.

### ai-auto-writer
- **Action:** PUBLISH (when ready)
- **Reason:** Novel concept — Rust + AI book generation. Compelling combo. Not ready yet.
- **Effort:** Needs cleanup before publishing.

### dracon-libs
- **Action:** LINK (in README)
- **Reason:** Foundation library collection. Useful but not impressive on its own. Link as "ecosystem" reference.
- **Effort:** Already on GitHub (1★).

### cli-file-manager
- **Action:** PUBLISH (optional)
- **Reason:** Unique concept — "ls on steroids." Could get attention. Not critical.
- **Effort:** Needs cleanup.

### video-uploader / youtube-video-uploader
- **Action:** PRIVATE
- **Reason:** Internal YouTube pipeline tools. Not products, not impressive enough for profile.
- **Effort:** None.

### video-factory
- **Action:** PRIVATE
- **Reason:** Internal video processing platform. Not ready for public.
- **Effort:** None.

### avid
- **Action:** PRIVATE
- **Reason:** Internal video processor. Part of YouTube pipeline.
- **Effort:** None.

### dracon-platform
- **Action:** PRIVATE
- **Reason:** Core infrastructure — auth, billing, email, TTS. Competitive advantage. Never publish.
- **Effort:** None.

### dracon-code
- **Action:** PRIVATE
- **Reason:** Flagship product. Never publish source code.
- **Effort:** None.

### dracon-demons
- **Action:** PRIVATE
- **Reason:** Internal service daemons. Powers dracon-platform.
- **Effort:** None.

### dracon-utilities
- **Action:** PRIVATE
- **Reason:** Internal CLI tools (sync, system-guard, warden). Competitive advantage.
- **Effort:** None.

### dracon-ai-lib
- **Action:** PUBLISH (optional)
- **Reason:** AI provider failover library. Useful to Rust AI developers. Already on GitHub.
- **Effort:** None — already published.

### dracon-voice-notifications (Kiki)
- **Action:** PUBLISH
- **Reason:** Fun project, memorable name, already on crates.io. Gets attention.
- **Effort:** None — already published.

### git-seal
- **Action:** PUBLISH
- **Reason:** Already on GitHub (2★). Small but useful.
- **Effort:** None — already published.

### respec-spec-reconciler
- **Action:** PRIVATE
- **Reason:** Internal Pi tool. Not relevant to public.
- **Effort:** None.

### opencode-auto-continue
- **Action:** PRIVATE
- **Reason:** OpenCode plugin. Tiny audience.
- **Effort:** None.

### opencode-auto-force-resume
- **Action:** PRIVATE
- **Reason:** OpenCode plugin. Tiny audience.
- **Effort:** None.

### opencode-auto-review-completed-todos
- **Action:** PRIVATE
- **Reason:** OpenCode plugin. Tiny audience.
- **Effort:** None.

### Junk-Runner-bevy
- **Action:** PRIVATE (or PUBLISH as portfolio)
- **Reason:** Game project. Bevy + Tauri is impressive but not relevant to business. Could publish as portfolio piece.
- **Effort:** Low if publishing.

### one-mil-girls
- **Action:** PRIVATE
- **Reason:** Visual novel. Niche, not relevant to business.
- **Effort:** None.

### wal-backup
- **Action:** PUBLISH (optional)
- **Reason:** SQLite + S3 backup API. Useful tool. Already on GitHub.
- **Effort:** None — already published.

### kittentts-showcase
- **Action:** DELETE
- **Reason:** Empty showcase. No value.
- **Effort:** rm -rf.

### test-auto-create
- **Action:** DELETE
- **Reason:** Empty test directory.
- **Effort:** rm -rf.

---

## Browser Extensions

### SamAI
- **Action:** SELL + LINK
- **Reason:** Flagship product. On Chrome Web Store. Link in README, don't pin (closed source).
- **Effort:** Add paid tier.

### vidpro-extension
- **Action:** SELL
- **Reason:** YouTube tool for YouTubers. Natural fit with YouTube channel.
- **Effort:** Package for Chrome Web Store.

### api-debugger
- **Action:** SELL
- **Reason:** Dev tool. Already on GitHub (1★). Low effort to publish on store.
- **Effort:** Package for Chrome Web Store.

### bugkit
- **Action:** SELL (optional)
- **Reason:** Bug evidence capture. QA teams pay for this.
- **Effort:** Package for Chrome Web Store.

### auto-form-filler
- **Action:** SELL (optional)
- **Reason:** AI form filling. Pairs with SamAI.
- **Effort:** Package for Chrome Web Store.

### ai-ats
- **Action:** SELL (optional)
- **Reason:** AI candidate screening. Recruiters pay for this.
- **Effort:** Package for Chrome Web Store.

### ai-job-finder
- **Action:** SELL (optional)
- **Reason:** Auto-apply for jobs. Job seekers pay for this.
- **Effort:** Package for Chrome Web Store.

### dark-mode-themes
- **Action:** PRIVATE
- **Reason:** Crowded market. Not differentiated enough.
- **Effort:** None.

### custom-search
- **Action:** PRIVATE
- **Reason:** Niche. Google dominates.
- **Effort:** None.

### youtube-dislike
- **Action:** PRIVATE
- **Reason:** Novelty. Not a product.
- **Effort:** None.

### calmweb
- **Action:** PRIVATE
- **Reason:** No description. Unclear value.
- **Effort:** None.

### cursor-style
- **Action:** PRIVATE
- **Reason:** Novelty. Not a product.
- **Effort:** None.

### death-note-typing-practice
- **Action:** PRIVATE
- **Reason:** Game. Not relevant.
- **Effort:** None.

### cinematic-pages-cooler-presentations
- **Action:** PRIVATE
- **Reason:** Novelty. Not a product.
- **Effort:** None.

### volume-and-video-pro
- **Action:** PRIVATE
- **Reason:** Niche. Crowded market.
- **Effort:** None.

### live-reload-pro
- **Action:** PRIVATE
- **Reason:** Developer tool. Tiny audience.
- **Effort:** None.

### full-page-screenshot
- **Action:** PRIVATE
- **Reason:** Crowded market. Many alternatives.
- **Effort:** None.

### custom-history
- **Action:** PRIVATE
- **Reason:** Niche. Privacy concerns.
- **Effort:** None.

### google-tasks-pro-tasks-kanban
- **Action:** PRIVATE
- **Reason:** Niche. Google ecosystem dependency.
- **Effort:** None.

---

## Summary

| Action | Count | Projects |
|:-------|------:|:---------|
| **PIN** | 4 | terminal-engine, pully, tiles, obs-wayland-hotkey |
| **LINK** | 3 | SamAI, dracon-libs, YouTube |
| **PUBLISH** | 6 | pully, rust-ai-web-auto, repo-rot-scanner, ai-auto-writer, Kiki, wal-backup |
| **SELL** | 7 | SamAI, vidpro, api-debugger, bugkit, auto-form-filler, ai-ats, ai-job-finder |
| **PRIVATE** | 24 | platform, code, demons, utilities, all other extensions, etc. |
| **DELETE** | 2 | kittentts-showcase, test-auto-create |

---

## Priority Actions

1. **Clean up README** — short, link SamAI + YouTube + dracon.uk
2. **Publish pully-fully** — strongest open source candidate
3. **Package vidpro-extension** — natural YouTube play
4. **Package api-debugger** — already on GitHub, low effort
5. **Delete** kittentts-showcase and test-auto-create

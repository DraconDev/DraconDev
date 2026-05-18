# README Decision Doc

## Current README (all links verified ✅)

All 15 links in the current README are public. No private repos leaked.

---

## Decision Framework

**Include in README if:**
1. Repo is **PUBLIC** on GitHub
2. Name is **self-explanatory** (3-5 words or clear keyword)
3. Has **>0 stars** OR is a **flagship** (strategic, not just random)

**Exclude from README if:**
1. Repo is **PRIVATE** — nobody can click it
2. Name is **opaque** without context — hurts discoverability more than it helps
3. Has **0 stars** and is **not flagship** — nobody's looking for it yet

**Add to README later when:**
1. Repo is made public
2. Name is renamed to be self-explanatory
3. Stars accumulate (≥1 makes it worth mentioning)

---

## All 16 Public Repos

| Repo | Stars | Name Quality | Include? | Notes |
|:-----|:------|:------------|:---------|:------|
| obs-wayland-hotkey | 7 | ✅ Perfect | **YES — Featured** | Top project. Wayland + OBS is a real pain point. |
| git-ai-committer | 6 | ✅ Perfect | **YES — Featured** | Top project. Auto-commits while you code. |
| git-seal | 2 | ✅ Good | **YES — Dev Tools** | Self-explanatory. Encryption for Git. |
| chrome-auto-fullscreen | 2 | ✅ Perfect | **YES — Browser** | Self-explanatory. |
| azumi | 1 | ⚠️ Opaque | **YES — Featured** | Brand established on crates.io. Pair with subtitle. |
| tiles | 1 | ⚠️ Opaque | **YES — Featured** | Pair with subtitle. Crate name is longer anyway. |
| ai-vid-editor | 1 | ✅ Perfect | **YES — Featured** | Self-explanatory. |
| terma | 1 | ⚠️ Opaque | **YES — Rust/Terminal** | Needs subtitle. |
| dracon-terminal-engine | 1 | ⚠️ OK | **YES — Rust/Terminal** | Brand prefix helps. |
| dracon-libs | 1 | ⚠️ OK | **YES — Rust/Terminal** | "libs" is clear enough for Rust devs. |
| css-peek-pro | 1 | ✅ Good | **YES — Dev Tools** | Self-explanatory with "CSS". |
| opencode-auto-continue | 1 | ✅ Good | **YES — Dev Tools** | Clear if you know OpenCode. |
| api-debugger | 1 | ✅ Good | **YES — Browser** | Self-explanatory. |
| opencode-auto-review-completed-todos | 0 | ⚠️ Overlong | **YES — Dev Tools** | Wait for rename to `opencode-todo-review`. |
| volume-and-video-pro | 0 | ❌ Vague | **YES — Browser** | Rename to `tab-volume-video` planned. |
| dracon-terminal-engine | 1 | — | *(duplicate)* | |

### Recommendation: All 15 current links stay. One duplicate removed.

---

## All 13 Private Repos

| Repo | Proposed Name | Stars | Worth Publicizing? | Decision |
|:-----|:-------------|:------|:------------------|:---------|
| auto-ai-agent | auto-ai-agent | 0 | ✅ YES — flagship | Top project after obs/gai. Make it the 3rd featured. |
| ai-app-engine | ai-app-engine | 0 | ✅ YES | Already calls itself this internally. |
| ai-web-agent | ai-web-agent | 0 | ✅ YES | The SamAI extension deserves a public face. |
| backend-service-daemons | backend-service-daemons | 0 | ⚠️ Maybe | Good but 0 stars. Wait for signal. |
| system-guard-daemons | system-guard-daemons | 0 | ⚠️ Maybe | Useful but niche. |
| platform-web-apps | platform-web-apps | 0 | ❌ No | Needs users first. Keep private. |
| vps-fleet-manager | vps-fleet-manager | 0 | ❌ No | Too niche for README at 0 stars. |
| browser-extensions | browser-extensions | 0 | ❌ No | Good for organization but no discoverability value yet. |
| youtube-uploader | youtube-uploader | 0 | ❌ No | Too niche. Maybe later. |
| automated-video-pipeline | automated-video-pipeline | 0 | ❌ No | No README even. Needs work first. |
| wal-backup | wal-backup | 0 | ❌ No | Niche. Maybe later. |
| ai-auto-writer | ai-auto-writer | 0 | ❌ No | Niche. Maybe later. |
| junk-runner | junk-runner | 0 | ❌ No | Games don't belong on a tools README. |

### Priority to public: `auto-ai-agent` → `ai-app-engine` → `ai-web-agent`

---

## Renames to Do Before Making Public

These names are locked in the naming audit. Rename before making public:

| Old (current GitHub) | New | Status |
|:--------------------|:----|:-------|
| dracon-code | auto-ai-agent | Rename then public |
| dracon-rust-ui | ai-app-engine | Rename then public |
| SamAI | ai-web-agent | Rename then public |
| opencode-auto-review-completed-todos | opencode-todo-review | Rename regardless |
| volume-and-video-pro | tab-volume-video | Rename regardless |

---

## README Sections — What Goes Where

### Featured (top 6)
The projects with stars OR strategic flagship value. Max 6 — keeps them visible.

### Rust & Terminal
Rust-specific tools. TUI frameworks, terminal engines, shared crates.

### Dev Tools
VS Code extensions, OpenCode plugins, Git tools.

### Browser
Chrome extensions. Audio, video, automation.

### Meta
Only `DraconDev` itself (the showcase). Don't add a Meta section with nothing in it.

---

## Summary

| Action | Count |
|:-------|:------|
| **Current links** | 15 (all public ✅) |
| **Private repos worth publicizing soon** | 3 (auto-ai-agent, ai-app-engine, ai-web-agent) |
| **Renames to do regardless** | 4 (before any public release) |
| **Keep private for now** | 10 |
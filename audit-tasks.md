# Dracon Ecosystem Audit — Task List

**Generated:** 2026-05-29  
**Total Repos:** 100 (22 public, 78 private)  
**Status:** Active Audit

---

## 🚨 CRITICAL — Name Mismatches

These repos have local names that don't match their GitHub names. This causes confusion and broken references.

### 🔴 HIGH PRIORITY

- [ ] **azumi** → Rename to `azumi-live-ssr-framework`
  - Local: `azumi`
  - GitHub: `azumi-live-ssr-framework`
  - Impact: 920 score, BETA, Engine role
  - Action: `mv azumi azumi-live-ssr-framework` + update all imports

- [ ] **ai-vid-editor** → Rename to `ai-gui-auto-video-editor`
  - Local: `ai-vid-editor`
  - GitHub: `ai-gui-auto-video-editor`
  - Impact: 890 score, BETA, Creator role
  - Action: `mv ai-vid-editor ai-gui-auto-video-editor` + update README refs

- [ ] **tiles** → Give it a proper descriptive name
  - Local: `tiles` (opaque name)
  - GitHub: `tiles-tui-file-manager`
  - Impact: 800 score, BETA, Interface role
  - Action: Either sync to `tiles-tui-file-manager` or rename to `tiles` on GitHub

---

## ⚠️ HIGH PRIORITY — Naming Issues

### Repos Needing Rename

- [ ] **opencode-auto-review-completed-todos** → `opencode-todo-review`
  - Current name is too long
  - Action: Rename local dir + GitHub repo + package.json
  - Impact: 750 score, BETA, Tool role

- [ ] **dracon-terminal-engine** → Consider `terminal-engine` or `dracon-tui-engine`
  - Current name is "bloated"
  - Brand prefix `dracon-` has value — keep if not renamed
  - Impact: 1★, needs better positioning

### Repos Not Featured in README

- [ ] **opencode-auto-force-resume** → Add to README
  - Currently not featured in README
  - Impact: Tool role, stall recovery plugin
  - Action: Add to README feature list or document removal reason

---

## 📋 REPOS NOT CLONED LOCALLY

These exist on GitHub but not in the DraconDev monorepo:

- [ ] **chrome-auto-fullscreen** — GitHub only
  - 2★, Chrome extension for auto-fullscreen
  - Action: `gh repo clone DraconDev/chrome-auto-fullscreen`
  - Status: Keep public

- [ ] **api-debugger** — GitHub only
  - 1★, HTTP request debugger Chrome extension
  - Action: `gh repo clone DraconDev/api-debugger`
  - Status: Keep public

- [ ] **volume-and-video-pro** — GitHub only
  - Name ≠ description mismatch
  - Action: Investigate, clone locally, or deprecate

- [ ] **calmweb** — GitHub only, no description
  - Action: Investigate purpose and clone or archive

- [ ] **auto-cleaner** — GitHub only, no description
  - Action: Investigate purpose and clone or archive

- [ ] **css-peek-pro** — GitHub only
  - VSCode extension for CSS navigation
  - Action: Investigate, clone locally, or deprecate

- [ ] **kittentts-showcase** — GitHub only
  - Action: Investigate, clone locally, or archive

---

## ✅ NAMING AUDIT — APPROVED FLAGS

These names are perfect. Do not change:

- [ ] **obs-wayland-hotkey** ✅ (7★) — Flaghship, perfect name
- [ ] **git-seal** ✅ (2★) — Flaghship, perfect name
- [ ] **opencode-auto-continue** ✅ — Descriptive, accurate, keep as-is
- [ ] **dracon-code** ✅ (995 score) — Brand + function, perfect
- [ ] **dracon-utilities** ✅ (960 score) — Brand + function, perfect
- [ ] **Junk-Runner** ✅ (980 score) — Unique, memorable, keep
- [ ] **Azumi** (as GitHub name) ✅ — Server-rendered framework

---

## 📁 REPO SYNC CHECKLIST

Verify all public repos are cloned locally and vice versa:

### Public Repos to Verify Locally
- [ ] dracon-terminal-engine — cloned? ✅/❌
- [ ] opencode-auto-review-completed-todos — cloned? ✅/❌ (needs rename)
- [ ] DraconDev — this repo ✅
- [ ] pi-auto-review — cloned? ✅/❌
- [ ] ai-gui-auto-video-editor — cloned? ✅/❌ (local is ai-vid-editor)
- [ ] respec-spec-reconciler — cloned? ✅/❌
- [ ] dracon-libs — cloned? ✅/❌
- [ ] azumi-live-ssr-framework — cloned? ✅/❌ (local is azumi)
- [ ] tiles-tui-file-manager — cloned? ✅/❌ (local is tiles)
- [ ] youtube-video-uploader — cloned? ✅/❌
- [ ] obs-wayland-hotkey — cloned? ✅/❌
- [ ] terma — cloned? ✅/❌
- [ ] git-ai-committer — cloned? ✅/❌
- [ ] opencode-auto-continue — cloned? ✅/❌
- [ ] opencode-auto-force-resume — cloned? ✅/❌

### Public Repos Only on GitHub (not cloned)
- [ ] auto-cleaner
- [ ] css-peek-pro
- [ ] kittentts-showcase
- [ ] api-debugger
- [ ] volume-and-video-pro
- [ ] calmweb
- [ ] chrome-auto-fullscreen

---

## 🛡️ LICENSE COMPLIANCE

### Source Policy: AGPLv3 + CLA

All Dracon projects use: `🟡 AGPLv3 + CLA`

- [ ] Verify all 100 repos have CLA workflow in `.github/workflows/cla.yml`
- [ ] Verify all public repos have `AGPLv3` LICENSE file
- [ ] Verify all new repos inherit the license stack
- [ ] Check `dracon-libs-l3` vs `dracon-libs` — why two repos?

---

## 🔧 INFRASTRUCTURE CHECKLIST

### Git Multi-Host Setup
- [ ] GitHub ✅ (git@github.com:DraconDev/DraconDev.git)
- [ ] GitLab ✅ (git@gitlab.com:dracondev/DraconDev.git)
- [ ] Codeberg ✅ (git@codeberg.org:dracondev/DraconDev.git)
- [ ] Verify all remotes are in sync
- [ ] Check for divergent branches

### Age Keys
- [ ] Verify keys in `.dracon/data/keys/` are current
- [ ] Check key rotation schedule
- [ ] Verify `.dracon/data/keys/owner_age*.pub` files are valid

### NixOS Integration
- [ ] Verify `owner_nixos.pub` key in `.dracon/data/keys/`
- [ ] Check NixOS configuration files exist

---

## 📊 SCOREBOARD VERIFICATION

Verify each scored project exists and matches description:

### Tier 1: 900+ Score (Critical Projects)
- [ ] **dracon-code** (995) — BETA, Brain — Verify exists + working
- [ ] **Junk-Runner** (980) — BETA, Simulation — Verify exists + working
- [ ] **dracon-utilities** (960) — BETA, Heartbeat — Verify exists + working
- [ ] **dracon-spark-and-director** (940) — BETA, Orchestration — Verify exists + working
- [ ] **Axiom UI** (940) — ALPHA, Interface — Verify exists + working
- [ ] **Azumi** (920) — BETA, Engine — Verify exists + working
- [ ] **ai-auto-writer** (910) — SERVICE, Sauce — Verify exists + working
- [ ] **AI Video Editor** (890) — BETA, Creator — Verify exists + working

### Tier 2: 800-899 Score
- [ ] **Terma** (880) — BETA, Engine
- [ ] **dracon-demons** (870) — BETA, Simulation
- [ ] **dracon-files** (870) — ALPHA, Interface
- [ ] **SamAI** (860) — ALPHA, Extension
- [ ] **Warehouse** (840) — STABLE, Vault
- [ ] **browser-extensions-shared** (820) — BETA, Extension
- [ ] **dracon-libs** (820) — STABLE, DNA

### Tier 3: Below 800 Score
- [ ] **tiles** (800) — BETA, Interface
- [ ] **Autopub** (740) — STABLE, Delivery
- [ ] **video-uploader** (750) — BETA, Tool
- [ ] **opencode-auto-continue** (750) — BETA, Tool

---

## 🔍 DUPLICATE/INCONSISTENT REPOS

These need investigation:

- [ ] **dracon-libs** vs **dracon-libs-l3**
  - Public `dracon-libs` exists
  - Private `dracon-libs-l3` exists
  - Why two? What's the difference?

- [ ] **respec** vs **respec-spec-reconciler**
  - Both exist on GitHub
  - Are they the same? Different versions?

- [ ] **spec-reconciler** vs **respec** vs **respec-spec-reconciler**
  - Three similar repos
  - Consolidate or clarify purpose

- [ ] **Junk-Runner** vs **Junk-Runner-bevy** vs **junk-runner-tauri**
  - Three variants
  - Which is the canonical version?

---

## 📝 DOCUMENTATION AUDIT

### Files to Review
- [ ] `AUDIT_SCOREBOARD.md` — Update with current state
- [ ] `NAMING_AUDIT.md` — Verify all naming decisions are applied
- [ ] `POSITIONING.md` — Update with any changes
- [ ] `README_DECISIONS.md` — Verify all decisions are followed
- [ ] `MIGRATION.md` — Verify migration paths are correct
- [ ] `SOVEREIGN_*.md` files — Verify they're up to date

### README Verification
- [ ] `_README.md` is current (last modified May 19)
- [ ] All featured repos in README match actual local/GitHub repos
- [ ] No orphaned references to deleted repos

---

## 🚀 DEPLOYMENT READINESS

### Projects by Stage
- [ ] **STABLE (3):** Warehouse, Autopub, dracon-libs
  - Production-ready
  - Verify CI/CD pipelines

- [ ] **BETA (11):** dracon-code, Junk-Runner, dracon-utilities, dracon-spark-and-director, Azumi, AI Video Editor, Terma, dracon-demons, tiles, video-uploader, opencode-auto-continue
  - Feature-complete, testing
  - Verify test coverage

- [ ] **ALPHA (4):** Axiom UI, dracon-files, SamAI, browser-extensions-shared
  - Early development
  - Verify basic functionality

- [ ] **SERVICE (1):** ai-auto-writer
  - Running service
  - Verify monitoring/health checks

---

## 🧪 TECHNICAL DEBT

### Legacy Repos to Address
- [ ] `dracon-libs-legacy` — Archive or migrate
- [ ] `hello-start` — Archive or migrate
- [ ] `Learn-React` — Archive or migrate
- [ ] `burger-builder` — Archive or migrate
- [ ] `learn-hooks` — Archive or migrate
- [ ] `PriceCompareSite` — Archive or migrate
- [ ] `momo` — Archive or migrate
- [ ] `test-*` repos — Archive or consolidate
  - test-auto-create
  - test-repo
  - test-conflict
  - test-github-private

### Obsolete/Dead Projects
- [ ] `my-repo` — No description, unclear purpose
- [ ] `custom-history` — No description, unclear purpose
- [ ] `Remi` / `remi-app` — Duplicates? Investigate

---

## 🎯 ACTION ITEMS BY PRIORITY

### Immediately (Today)
1. [ ] Fix `azumi` → `azumi-live-ssr-framework` rename
2. [ ] Fix `ai-vid-editor` → `ai-gui-auto-video-editor` rename
3. [ ] Rename `opencode-auto-review-completed-todos` → `opencode-todo-review`
4. [ ] Clone missing public repos locally

### This Week
5. [ ] Investigate duplicate repos (dracon-libs variants, respec variants, Junk-Runner variants)
6. [ ] Update AUDIT_SCOREBOARD.md with latest status
7. [ ] Verify all CLA workflows are in place
8. [ ] Archive dead/legacy repos

### This Month
9. [ ] Complete scoreboard verification for all 19+ scored projects
10. [ ] Update all documentation files
11. [ ] Verify Git multi-host sync
12. [ ] Deploy monitoring for SERVICE projects

---

## 📅 AUDIT SCHEDULE

- [ ] **Weekly:** Quick scan for new repos, check for stale activity
- [ ] **Monthly:** Full scoreboard review, update priorities
- [ ] **Quarterly:** Major ecosystem audit, consolidate/archive
- [ ] **On-demand:** After any major repo rename or deletion

---

## 📞 CONTACTS & OWNERSHIP

| Project | Owner | Notes |
|---------|-------|-------|
| dracon-code | ? | 995 score, highest priority |
| Junk-Runner | ? | 980 score, simulation flagship |
| dracon-utilities | ? | 960 score, system services |
| dracon-spark-and-director | ? | 940 score, infrastructure |
| Azumi | ? | 920 score, SSR framework |
| Terma | ? | 880 score, terminal compositor |
| dracon-demons | ? | 870 score, daemon services |

*Note: Add owner column as ownership is clarified*

---

## ✅ COMPLETION CHECKLIST

- [ ] All name mismatches resolved
- [ ] All public repos cloned locally
- [ ] CLA workflow on all repos
- [ ] Scoreboard matches actual repo state
- [ ] Documentation updated
- [ ] Legacy repos archived
- [ ] Git remotes in sync
- [ ] License compliance verified

---

**Next Review:** After all CRITICAL items completed
**Audit Owner:** DraconDev Ecosystem
**Last Updated:** 2026-05-29
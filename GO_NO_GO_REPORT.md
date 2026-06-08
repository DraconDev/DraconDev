# Publish Readiness Audit Report

**Date:** 2026-06-08
**Auditor:** DraconDev coding agent
**Objective:** Verify all repos are ready to publish and README links are correct

---

## Executive Summary

**Audit verdict:** NOT all repos are ready to publish.

**Ready to publish:** 0 repos
**Need fixes before publish:** 2 repos (pully-fully, kiki)
**On hold:** 1 repo (rust-ai-web-auto)
**Blocked by monorepo:** 3 repos (sync, warden, system)
**Already public:** 4 repos

**README_DRAFT.md:** Corrected to show only working links (12 lines, 4 public repos)

---

## README_DRAFT.md Status

**Current state:** Corrected to show only working links.

**Shows:**
- 4 already public repos (terminal-engine, tiles, folder-auto-banner, obs-wayland-hotkey)
- Links to dracon.uk, YouTube, Sponsor

**Removed:**
- pully-fully (not ready - README has broken links)
- kiki (not ready - no CI workflow)
- rust-ai-web-auto (on hold)
- dracon-sync/warden/system (private monorepo)
- SamAI (Chrome Web Store link redirects to generic store homepage)

**Line count:** 12 lines (very compact, scannable)

**Link verification:** All 9 links in README_DRAFT.md return 200 status.

---

## Repository Audit Results

### ✅ Already Public (4 repos)

| Repo | Status | License | Notes |
|:-----|:-------|:--------|:------|
| `dracon-terminal-engine` | ✅ Public | AGPL-3.0 | 139K lines, 3,658 tests |
| `tiles-tui-file-manager` | ✅ Public | AGPL-3.0 | 22K lines, dual-pane file manager |
| `folder-auto-banner` | ✅ Public | MIT | 8K lines, 108 tests |
| `obs-wayland-hotkey` | ✅ Public | ⚠️ License mismatch | 2.2K lines, 36 tests |

### ❌ NOT Ready to Publish (1 repo)

| Repo | Status | License | README | Source | Tests | TODO/FIXME | Build | Verdict |
|:-----|:-------|:--------|:-------|:-------|:------|:-----------|:------|:--------|
| `pully-fully-pull-based-fleet-reconciler` | ❌ NOT ready | AGPL-3.0 | 367 lines | 58 files | 9 test files | 0 | Not checked | **NO-GO** |

**pully-fully blockers:**
- ❌ README has 7 broken internal links:
  - `WEIGHTED_RANKING.md`
  - `docs/HOW_IT_WORKS.md`
  - `docs/DESIGN.md`
  - `docs/BOOTSTRAP.md`
  - `docs/SECURITY.md`
  - `docs/COMPARISON.md`
  - `COMMERCIAL-LICENSE.md`
- ❌ README has 2 broken external links:
  - `github.com/pully-works/pully`
  - `pully-fleet-template`
- **Verdict:** NOT ready to publish until README broken links are fixed

### ❌ NOT Ready to Publish (1 repo)

| Repo | Status | License | README | Source | Tests | TODO/FIXME | Build | Verdict |
|:-----|:-------|:--------|:-------|:-------|:------|:-----------|:------|:--------|
| `kiki-sassy-desktop-announcer` | ❌ NOT ready | MIT | 268 lines | 14 files | 3 Rust test files + 2 shell test scripts | 3 (low) | ⚠️ Environment-dependent | **NO-GO** |

**kiki blockers:**
- ❌ No GitHub workflows (no CI evidence)
- ⚠️ `cargo check --locked` is environment-dependent (audit env missing alsa.pc / alsa-sys)
- 3 TODOs (low priority, future improvements)
- **Verdict:** NOT ready to publish until CI workflow is added

### ⏸️ On Hold (1 repo)

| Repo | Status | License | README | Source | Tests | TODO/FIXME | Verdict |
|:-----|:-------|:--------|:-------|:-------|:------|:-----------|:--------|
| `rust-ai-web-auto` | ⏸️ On hold | AGPL-3.0 | 95 lines | 83 files | 29 test files | 0 | **HOLD** |

**rust-ai-web-auto notes:**
- User explicitly said: "we are deliberating how to include the ai lib for it"
- 1253 commits in last 30 days
- 0 TODO/FIXME/XXX
- 1 WIP mention (in audit-report.md, outdated)
- **Verdict:** On hold per user decision

### ⚠️ Blocked by Monorepo (3 repos)

| Repo | Status | License | README | Source | Tests | TODO/FIXME | Verdict |
|:-----|:-------|:--------|:-------|:-------|:------|:-----------|:--------|
| `dracon-sync` | ⚠️ In monorepo | AGPL-3.0 (inherited) | 351 lines | 32 files | 4 test files | 0 | **BLOCKED** |
| `dracon-warden` | ⚠️ In monorepo | AGPL-3.0 (inherited) | 306 lines | 29 files | 24 test files | 1 (test) | **BLOCKED** |
| `dracon-system` | ⚠️ In monorepo | AGPL-3.0 (inherited) | 311 lines | 12 files | 4 test files | 0 | **BLOCKED** |

**Monorepo situation:**
- All 3 are in `DraconDev/dracon-utilities` (PRIVATE monorepo)
- They are NOT standalone repos
- README links to `DraconDev/dracon-sync`, `DraconDev/dracon-warden`, `DraconDev/dracon-system` will 404
- The monorepo is configured as a workspace with members: `dracon-sync`, `dracon-system`, `dracon-warden`
- Root monorepo has AGPL-3.0 license
- Subcrate `Cargo.toml` files declare AGPL licenses
- **Verdict:** Need to either make the monorepo public or split into standalone repos

---

## README Link Audit

### Current README links (corrected)

| Link | Status | Notes |
|:-----|:-------|:------|
| `dracon-terminal-engine` | ✅ Works | Public |
| `tiles-tui-file-manager` | ✅ Works | Public |
| `folder-auto-banner` | ✅ Works | Public |
| `obs-wayland-hotkey` | ✅ Works | Public |
| `dracon.uk` | ✅ Works | External |
| `YouTube` | ✅ Works | External |
| `Sponsor` | ✅ Works | External |

### Removed links (will be added back when ready)

| Link | Status | Notes |
|:-----|:-------|:------|
| `pully-fully-pull-based-fleet-reconciler` | ❌ NOT ready | README has broken links |
| `kiki-sassy-desktop-announcer` | ❌ NOT ready | No CI workflow |
| `rust-ai-web-auto` | ⏸️ On hold | Per user decision |
| `dracon-sync` | ⚠️ Blocked by monorepo | In private monorepo |
| `dracon-warden` | ⚠️ Blocked by monorepo | In private monorepo |
| `dracon-system` | ⚠️ Blocked by monorepo | In private monorepo |
| `SamAI` | ⚠️ Link redirects to generic store homepage | Not a verified product page |

---

## License Audit (Corrected)

| Repo | License | Source | Status |
|:-----|:--------|:-------|:-------|
| `dracon-terminal-engine` | AGPL-3.0 | GitHub API | ✅ |
| `tiles-tui-file-manager` | AGPL-3.0 | GitHub API | ✅ |
| `folder-auto-banner` | MIT | GitHub API | ✅ |
| `obs-wayland-hotkey` | ⚠️ AGPL-3.0 on GitHub, MIT in Cargo.toml/README/crates.io | GitHub API + Cargo.toml + README + crates.io | ⚠️ Mismatch |
| `pully-fully-pull-based-fleet-reconciler` | AGPL-3.0 | GitHub API | ✅ |
| `dracon-sync` | AGPL-3.0 (inherited) | Monorepo root | ✅ |
| `dracon-warden` | AGPL-3.0 (inherited) | Monorepo root | ✅ |
| `dracon-system` | AGPL-3.0 (inherited) | Monorepo root | ✅ |
| `rust-ai-web-auto` | AGPL-3.0 | GitHub API | ✅ |
| `kiki-sassy-desktop-announcer` | MIT | GitHub API | ✅ |

**Verdict:** License audit corrected. `obs-wayland-hotkey` has a license mismatch that needs investigation.

---

## Code Quality Audit

### TODO/FIXME/XXX

| Repo | Count | Severity | Notes |
|:-----|:------|:---------|:------|
| `pully-fully` | 0 | None | Clean |
| `kiki` | 3 | Low | Future improvements (ERR-4, spawn_local, TriggerType) |
| `dracon-sync` | 0 | None | Clean |
| `dracon-warden` | 1 | None | False positive in tests (fake secret key) |
| `dracon-system` | 0 | None | Clean |

### WIP Mentions

| Repo | Count | Notes |
|:-----|:------|:------|
| `pully-fully` | 1 | Says "No active work in progress" |
| `rust-ai-web-auto` | 1 | Outdated audit-report.md |

**Verdict:** No significant blockers. The TODOs in kiki are low-priority future improvements.

---

## Monorepo Analysis

### Current situation
- `DraconDev/dracon-utilities` is **PRIVATE**
- Contains 3 sub-repos as workspace members:
  - `dracon-sync`
  - `dracon-system`
  - `dracon-warden`
- These are NOT standalone repos
- README links to standalone repos will 404
- Root monorepo has AGPL-3.0 license
- Subcrate `Cargo.toml` files declare AGPL licenses

### Options

**Option A: Make monorepo public**
- Pros: No splitting required, preserves history
- Cons: All 3 sub-repos are in one repo, links need updating to `/dracon-utilities/dracon-sync`
- Links in README would need to point to:
  - `https://github.com/DraconDev/dracon-utilities/tree/main/dracon-sync`
  - `https://github.com/DraconDev/dracon-utilities/tree/main/dracon-warden`
  - `https://github.com/DraconDev/dracon-utilities/tree/main/dracon-system`

**Option B: Split into standalone repos**
- Pros: Clean, matches README links
- Cons: More work, need to preserve history
- Links in README already point to standalone repos

**Option C: Remove from README**
- Pros: No broken links
- Cons: Hides 3 significant repos

**Recommendation:** Option A (make monorepo public) is simplest if you want to keep the monorepo structure. Option B (split) is cleanest but more work.

---

## Publish Plan

### Phase 1: Fix README links (1 repo)
1. ❌ Fix `pully-fully` README broken links (7 internal + 2 external)

### Phase 2: Add CI workflow (1 repo)
2. ❌ Add CI workflow to `kiki`

### Phase 3: Publish ready repos (0 repos currently)
3. ⏸️ Publish `pully-fully-pull-based-fleet-reconciler` after Phase 1
4. ⏸️ Publish `kiki-sassy-desktop-announcer` after Phase 2

### Phase 4: On hold (1 repo)
5. ⏸️ Hold `rust-ai-web-auto` until AI lib decision is made

### Phase 5: Monorepo decision (3 repos)
6. ⚠️ Decide on monorepo approach:
   - Option A: Make `dracon-utilities` public and update README links
   - Option B: Split into 3 standalone repos
   - Option C: Remove from README

---

## README Recommendation

Given the user's preference for **tangible things only**:

**Current README_DRAFT.md (corrected) shows only working links:**
- 4 already public repos
- Links to dracon.uk, YouTube, Sponsor

**This is the right approach:**
- No broken links
- No "working on" fluff
- Shows only tangible, shipped work

**When to add more repos:**
- Add `pully-fully` after fixing its README broken links
- Add `kiki` after adding CI workflow
- Add monorepo sub-repos after deciding on monorepo approach
- Add `rust-ai-web-auto` when AI lib decision is made

---

## Files Referenced
- `README_DRAFT.md` — Corrected README (12 lines, only working links)
- `REPO_FINAL_LIST.md` — Final repo list
- `REPO_ANALYSIS.md` — Repo analysis
- `GITHUB_PROFILE_RESEARCH.md` — Profile research
- `EXPERT_VALIDATION.md` — Expert validation

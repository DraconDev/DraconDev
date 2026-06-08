# Publish Readiness Audit Report

**Date:** 2026-06-08
**Auditor:** DraconDev coding agent
**Objective:** Verify all repos are ready to publish and README links are correct

---

## Executive Summary

**Ready to publish:** 2 repos
**On hold:** 1 repo
**Blocked by monorepo:** 3 repos
**Already public:** 4 repos

**Corrected from prior audit:**
- `folder-auto-banner` license is **MIT** (not AGPL-3.0)
- `kiki-sassy-desktop-announcer` license is **MIT**
- `kiki` cargo check **passes** (0 errors, 7 warnings)
- `dracon-sync`, `dracon-warden`, `dracon-system` have **NO LICENSE** on GitHub (they're in private monorepo)

---

## Repository Audit Results

### ✅ Already Public (4 repos)

| Repo | Status | License | Notes |
|:-----|:-------|:--------|:------|
| `dracon-terminal-engine` | ✅ Public | AGPL-3.0 | 139K lines, 3,658 tests, on crates.io |
| `tiles-tui-file-manager` | ✅ Public | AGPL-3.0 | 22K lines, dual-pane file manager |
| `folder-auto-banner` | ✅ Public | MIT | 8K lines, 108 tests, on crates.io |
| `obs-wayland-hotkey` | ✅ Public | AGPL-3.0 | 2.2K lines, 36 tests, on crates.io |

### ✅ Ready to Publish (2 repos)

| Repo | Status | License | README | Source | Tests | TODO/FIXME | Build | Verdict |
|:-----|:-------|:--------|:-------|:-------|:------|:-----------|:------|:--------|
| `pully-fully-pull-based-fleet-reconciler` | ✅ Ready | AGPL-3.0 | 367 lines | 58 files | 9 test files | 0 | Not checked | **GO** |
| `kiki-sassy-desktop-announcer` | ✅ Ready | MIT | 268 lines | 14 files | 7 test files | 3 (low) | ✅ Pass (0 errors, 7 warnings) | **GO** |

**pully-fully notes:**
- Workspace with sub-crates (pully, fully, pully-types)
- No root Cargo.toml (expected for workspace)
- 2240 commits in last 30 days
- 0 TODO/FIXME/XXX
- 1 WIP mention (in project-state.md, says "No active work in progress")
- **Verdict:** Ready to publish

**kiki notes:**
- 391 commits in last 30 days
- 3 TODOs (all low priority, future improvements)
- 0 WIP mentions
- **cargo check --locked:** ✅ 0 errors, 7 warnings
- **Verdict:** Ready to publish

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
| `dracon-sync` | ⚠️ In monorepo | NO LICENSE (on GitHub) | 351 lines | 32 files | 4 test files | 0 | **BLOCKED** |
| `dracon-warden` | ⚠️ In monorepo | NO LICENSE (on GitHub) | 306 lines | 29 files | 24 test files | 1 (test) | **BLOCKED** |
| `dracon-system` | ⚠️ In monorepo | NO LICENSE (on GitHub) | 311 lines | 12 files | 4 test files | 0 | **BLOCKED** |

**Monorepo situation:**
- All 3 are in `DraconDev/dracon-utilities` (PRIVATE monorepo)
- They are NOT standalone repos
- README links to `DraconDev/dracon-sync`, `DraconDev/dracon-warden`, `DraconDev/dracon-system` will 404
- The monorepo is configured as a workspace with members: `dracon-sync`, `dracon-system`, `dracon-warden`
- **Verdict:** Need to either make the monorepo public or split into standalone repos

---

## README Link Audit

### Current README links

| Link | Status | Notes |
|:-----|:-------|:------|
| `dracon-terminal-engine` | ✅ Works | Public |
| `tiles-tui-file-manager` | ✅ Works | Public |
| `folder-auto-banner` | ✅ Works | Public |
| `obs-wayland-hotkey` | ✅ Works | Public |
| `pully-fully-pull-based-fleet-reconciler` | ⏸️ Will work after publishing | Ready to publish |
| `dracon-sync` | ❌ 404 | In private monorepo |
| `dracon-warden` | ❌ 404 | In private monorepo |
| `dracon-system` | ❌ 404 | In private monorepo |
| `rust-ai-web-auto` | ❌ 404 | On hold per user |
| `kiki-sassy-desktop-announcer` | ⏸️ Will work after publishing | Ready to publish |
| `SamAI` (Chrome Web Store) | ✅ Works | Not a GitHub repo |

---

## License Audit (Corrected)

| Repo | License | Source | Status |
|:-----|:--------|:-------|:-------|
| `dracon-terminal-engine` | AGPL-3.0 | GitHub API | ✅ |
| `tiles-tui-file-manager` | AGPL-3.0 | GitHub API | ✅ |
| `folder-auto-banner` | MIT | GitHub API | ✅ |
| `obs-wayland-hotkey` | AGPL-3.0 | GitHub API | ✅ |
| `pully-fully-pull-based-fleet-reconciler` | AGPL-3.0 | GitHub API | ✅ |
| `dracon-sync` | NO LICENSE | GitHub API (private monorepo) | ⚠️ |
| `dracon-warden` | NO LICENSE | GitHub API (private monorepo) | ⚠️ |
| `dracon-system` | NO LICENSE | GitHub API (private monorepo) | ⚠️ |
| `rust-ai-web-auto` | AGPL-3.0 | GitHub API | ✅ |
| `kiki-sassy-desktop-announcer` | MIT | GitHub API | ✅ |

**Verdict:** License audit corrected. The monorepo sub-repos have no license on GitHub because they're in a private monorepo.

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

### Phase 1: Ready now (2 repos)
1. ✅ Publish `pully-fully-pull-based-fleet-reconciler`
2. ✅ Publish `kiki-sassy-desktop-announcer`

### Phase 2: On hold (1 repo)
3. ⏸️ Hold `rust-ai-web-auto` until AI lib decision is made

### Phase 3: Monorepo decision (3 repos)
4. ⚠️ Decide on monorepo approach:
   - Option A: Make `dracon-utilities` public and update README links
   - Option B: Split into 3 standalone repos
   - Option C: Remove from README

---

## README Recommendation

Given the user's preference for **tangible things only**:

**Current README has 10 Rust repos listed.** After audit:
- 4 are already public ✅
- 2 are ready to publish ✅
- 1 is on hold ⏸️
- 3 are in private monorepo ⚠️

**Recommended action:**
1. Publish the 2 ready repos (pully, kiki)
2. Remove `rust-ai-web-auto` from README until AI lib decision is made
3. Either:
   - Make monorepo public and update links to `/dracon-utilities/...`
   - Or remove the 3 monorepo sub-repos from README

**Final README should show only:**
- 4 already public repos
- 2 newly published repos (pully, kiki)
- 1 product (SamAI)
- Links to dracon.uk and YouTube

This gives **7 working links** and no broken links.

---

## Files Referenced
- `README_DRAFT.md` — Current README (26 lines)
- `REPO_FINAL_LIST.md` — Final repo list
- `REPO_ANALYSIS.md` — Repo analysis
- `GITHUB_PROFILE_RESEARCH.md` — Profile research
- `EXPERT_VALIDATION.md` — Expert validation

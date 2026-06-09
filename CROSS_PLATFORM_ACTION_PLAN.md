# Cross-Platform Action Plan

**Date:** 2026-06-09
**Purpose:** Turn the effectiveness-scoring findings into concrete updates across the DraconDev funnel. This is not just a README change; the README is one asset in a broader cross-platform strategy.

Exact copy/templates for the remaining external updates are in `EXTERNAL_PLATFORM_UPDATE_SNIPPETS.md`.

## Decision Summary

The README should **not** become the place where every funnel update lives. The effectiveness scoring showed that the strongest strategies use each platform for a specific job:

| Asset | Job in the funnel | Current decision |
|:------|:------------------|:-----------------|
| `README.md` / `README_DRAFT.md` | Public GitHub profile proof: hero line, 4 pinned repos, stats, 3 CTAs | Already aligned; keep unchanged unless new evidence appears |
| `PROFILE_STRATEGY.md` | Internal strategy map: what to do, where, why, and in what order | Updated with effectiveness scoring and action plan |
| `EFFECTIVENESS_SCORING.md` | Evidence base: factor scores, why they work, and synthesized strategy | Created as the scoring artifact |
| GitHub profile bio | Name-drop the killer work above the README | External platform update; requires GitHub profile access |
| GitHub Sponsors page bio | Convert warm visitors with concrete stats | External platform update; requires GitHub Sponsors access |
| `dracon.uk` footer | Route product/tip traffic without diluting the README | External platform update; requires site repo access |
| YouTube channel nav | 5-link ecosystem map | External platform update; requires YouTube Studio access |
| YouTube descriptions | Reusable 5+1 template for each video | External platform update; requires YouTube Studio access |

## In-Repo Updates

### 1. README / `README_DRAFT.md`
**Status:** ✅ Aligned.
**Decision:** Keep the current 15-line structure:
- Hero line: `Hey, I make tools that run themselves.`
- One-line positioning: `I build infrastructure in Rust — terminal engines, fleet reconcilers, git daemons. I teach what I build on YouTube, and I sell what I make on dracon.uk.`
- Stats line: `239K+ lines of Rust · 5,600+ tests · 4 on crates.io`
- 4 pinned repos under `TUI & Terminal`
- 3-link footer: `dracon.uk · YouTube · Sponsor`

**Why this is correct:** The scoring showed that README presence is useful but not the main conversion lever. The current README already follows the strongest pattern: concrete work, concrete stats, no WIP, no clutter.

### 2. `PROFILE_STRATEGY.md`
**Status:** ✅ Updated.
**Decision:** Add an effectiveness-scoring section that:
- Explains factor-based scoring
- Summarizes the 4×16 matrix
- Lists the top 5 factors
- Synthesizes the combined strategy
- Explains why the top 5 factors work
- Cross-references `EFFECTIVENESS_SCORING.md`

### 3. `EFFECTIVENESS_SCORING.md`
**Status:** ✅ Created.
**Decision:** Keep as the durable scoring artifact. It is the source of truth for why the strategy is built the way it is.

### 4. Other docs
**Status:** ✅ Cross-referenced.
**Decision:** `YOUTUBE_AND_MONETIZATION_RESEARCH.md` and `GITHUB_PROFILE_RESEARCH.md` remain the research sources. No further in-repo edits are required unless the strategy changes.

## External Platform Updates

### 1. GitHub profile bio
**Status:** ⏳ Needs external access.
**Asset:** GitHub profile bio field
**Recommended copy:** `Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests, 24 crates on crates.io.`
**Why:** This is the mitsuhiko pattern: name-drop the killer work.
**Verification needed:** Screenshot or exported profile text showing the new bio.

### 2. GitHub Sponsors page bio
**Status:** ⏳ Needs external access.
**Asset:** `github.com/sponsors/DraconDev`
**Recommended copy:** `239K+ lines of Rust, 5,600+ tests, 24 crates on crates.io, 12 in-scope repos per the audit.`
**Why:** This is the dtolnay pattern: concrete stats make the page credible.
**Verification needed:** Screenshot or exported page text showing the new bio.

### 3. `dracon.uk` footer
**Status:** ⏳ Needs site repo access.
**Asset:** Site footer or global nav
**Recommended copy:** Add Ko-fi link only on the site, not the README.
**Why:** Ko-fi is the digital-product/tip layer; the README is the OSS profile layer.
**Verification needed:** Rendered site screenshot or deployed URL showing the footer link.

### 4. YouTube channel nav
**Status:** ⏳ Needs YouTube Studio access.
**Asset:** YouTube channel links
**Recommended links:** `dracon.uk`, GitHub, Sponsor, Ko-fi, Discord (if active)
**Why:** This is the Theo pattern: a 5-link ecosystem map.
**Verification needed:** Channel page screenshot or YouTube Studio export showing the links.

### 5. YouTube descriptions
**Status:** ⏳ Needs YouTube Studio access.
**Asset:** Video description template
**Recommended structure:** Hook → sponsor (if any) → body → topic index → social links → evergreen CTA
**Why:** This is the Fireship/Theo pattern: sponsor at top, social links at bottom, recurring CTA.
**Verification needed:** Published video description screenshot or exported description text.

### 6. Content cadence / format
**Status:** ⏳ Needs execution plan.
**Asset:** Publishing workflow
**Recommended cadence:** Weekly shorts or weekly live streams
**Recommended format:** Short-form hooks + long-form deep-dives
**Why:** This is the antfu/Fireship pattern: consistency plus retention.
**Verification needed:** Published video schedule or content calendar.

## Constraints Preserved

- Keep the 4-repo pin list.
- Keep the 239K+ lines / 5,600+ tests stat unless a verified stronger stat replaces it.
- Keep Ko-fi off the README.
- Keep the 3-link README footer density.
- Do not add WIP, working-on, or experimental sections.
- Do not introduce placeholders or undocumented assumptions.

## Verification Checklist

Before marking the goal complete, verify:
1. `README.md` and `README_DRAFT.md` are identical.
2. All README links return HTTP 200.
3. `PROFILE_STRATEGY.md` references the action plan and the scoring artifact.
4. `EFFECTIVENESS_SCORING.md` exists and is referenced by `PROFILE_STRATEGY.md`.
5. Any external platform update is backed by a screenshot, exported text, or deployment URL.
6. Any missing external update is explicitly marked as blocked by access, not deferred silently.

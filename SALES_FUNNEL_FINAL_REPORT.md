# Sales Funnel Profile Research Final Report

Date: 2026-06-11

## Outcome

Completed research on additional public GitHub profiles to answer whether other developers use GitHub profiles as sales funnels.

The short answer is **yes**. The strongest examples are course creators, training platforms, and company founders who make the destination explicit in the profile bio, profile README, or visible organization link.

## Files added

- `SALES_FUNNEL_PROFILE_RESEARCH.md` — full research report with:
  - candidate list,
  - classification summary,
  - strong funnel examples,
  - soft funnel / portfolio-first / no funnel examples,
  - DraconDev implications.
- `SALES_FUNNEL_RECOMMENDATIONS.md` — concise recommendations:
  - top 3 patterns to copy,
  - top 3 patterns to avoid,
  - concrete README/profile changes DraconDev should consider later.
- `sales_funnel_profile_scores.json` — structured scoring sheet for every researched profile.

## Evidence directory

Captured evidence is under:

- `/tmp/sales_funnel_profiles/`

For each researched profile, the evidence directory contains at least:

- `profile.html` — raw GitHub profile page HTML,
- `endpoint.html` — raw external destination page HTML,
- `endpoint_status.txt` — endpoint URL, final HTTP status, effective URL, and title,
- `github_api.json` — GitHub API profile data.

Some profiles also contain `profile_readme.md` when a profile README exists.

## Profiles researched

Total: **18**

Classification counts:

- **Strong sales funnel:** 9
- **Soft funnel:** 6
- **Portfolio-first:** 2
- **No funnel:** 1

Strong sales funnel examples:

1. `wesbos` — courses funnel to `wesbos.com/courses`
2. `kentcdodds` — product/course funnel to `kentcdodds.com`
3. `getify` — course funnel to Frontend Masters
4. `bradtraversy` — course/store funnel to Traversy Media
5. `unclebob` — training funnel to Clean Coders
6. `rauchg` — company/product funnel to Vercel
7. `1Marc` — platform/course funnel to Frontend Masters
8. `amix` — company/product funnel to Doist
9. `btholt` — platform/course funnel to Frontend Masters

Soft funnel examples:

- `cassidoo` — newsletter, Patreon, blog, apps
- `colbyfayock` — newsletter and educational content
- `mbeaudru` — free cheatsheet / community resource
- `troyhunt` — personal security brand / site
- `benawad` — personal site / Voidpet
- `feross` — company/product mention, but endpoint was bot-blocked in this run

Portfolio-first examples:

- `addyosmani` — strong personal/professional brand, weak sales funnel
- `joshwcomeau` — strong personal brand, weak explicit funnel

No funnel example:

- `jason` — no clear profile-level destination or funnel

## Verification evidence

- Final audit script: `/tmp/sales_funnel_profiles/final_audit.sh`
- Final audit output:

```json
{
  "profiles_researched": 18,
  "classification_counts": {
    "Strong sales funnel": 9,
    "Soft funnel": 6,
    "Portfolio-first": 2,
    "No funnel": 1
  },
  "strong_funnel_count": 9,
  "strong_endpoint_statuses": {
    "wesbos": "200",
    "kentcdodds": "200",
    "getify": "200",
    "bradtraversy": "200",
    "unclebob": "200",
    "rauchg": "200",
    "1Marc": "200",
    "amix": "200",
    "btholt": "200"
  },
  "constrained_files_unchanged": true,
  "no_secret_patterns": true,
  "required_files_present": true
}
```

- The audit confirmed:
  - at least 12 profiles were researched,
  - at least 3 strong funnel examples were found,
  - every strong funnel example has a captured profile artifact,
  - every strong funnel example has an external destination URL,
  - every strong funnel endpoint returned HTTP 200 in this run,
  - constrained DraconDev files were unchanged during this research goal,
  - no secret-like patterns were present in the research outputs.

## Requirement-to-evidence map

| Requirement | Evidence |
|-------------|----------|
| Research more profiles beyond prior peer set | `SALES_FUNNEL_PROFILE_RESEARCH.md` lists 18 profiles; `sales_funnel_profile_scores.json` contains 18 entries. |
| Classify each profile into funnel buckets | `SALES_FUNNEL_PROFILE_RESEARCH.md` classification summary and per-profile sections; `sales_funnel_profile_scores.json` has `classification` for each entry. |
| Document strong funnel path, destination, profile evidence, why it is a funnel, and DraconDev advice | Strong funnel sections in `SALES_FUNNEL_PROFILE_RESEARCH.md`; JSON fields `funnel_path`, `monetization_destination`, `profile_evidence`, `why_funnel`, `draconded_advice`. |
| Capture evidence for each researched profile | `/tmp/sales_funnel_profiles/<login>/profile.html`, `endpoint.html`, `endpoint_status.txt`, `github_api.json`; some also have `profile_readme.md`. |
| Save scoring sheet | `sales_funnel_profile_scores.json`. |
| Save final recommendations | `SALES_FUNNEL_RECOMMENDATIONS.md`. |
| Preserve constrained files | Final audit confirmed `README.md`, `README_DRAFT.md`, `README_SUGGESTED_FORM.md`, `SUGGESTED_FORM_BLOCKERS.md`, and `SUGGESTED_FORM_USAGE.md` unchanged vs. audit snapshots. |
| No secret leakage | Final audit regex check found no `ghp_`, `github_pat_`, or private-key patterns in research outputs. |
| Answer the user's question directly | `SALES_FUNNEL_PROFILE_RESEARCH.md` short answer: yes, and the strongest examples are course creators, training platforms, and company founders. |

## Main recommendation for DraconDev

The best-fit pattern is a hybrid of **Wes Bos + Kent C. Dodds + Brad Traversy**:

- short bio,
- explicit product/course destination,
- clear CTA,
- no wall of text,
- no mixing product destinations with code links.

If DraconDev wants to sell tools, the destination should be `dracon.uk` or a specific product page. If DraconDev wants to sell learning content, the destination should be a course or YouTube CTA. The profile should remain short, explicit, and destination-led.

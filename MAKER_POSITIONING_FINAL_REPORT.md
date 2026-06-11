# Maker/Product-Seller Reframe Final Report

Date: 2026-06-11

## Outcome

Reframed DraconDev profile positioning around the user's stated intent:

> **DraconDev makes real tools, presents them clearly, and directs the right people to buy or use them.**

The updated artifacts no longer over-index on educator/course-funnel examples. They now emphasize product/company funnels, shipped-tool proof, product-destination dropdowns, and code as supporting evidence.

## Files added

- `MAKER_PROFILE_POSITIONING.md` — maker/product-seller positioning note:
  - core positioning sentence,
  - what DraconDev is not emphasizing,
  - patterns to copy/avoid,
  - README product-vs-code guidance,
  - recommended tone and success criterion.

## Files updated

- `SALES_FUNNEL_RECOMMENDATIONS.md`
  - now framed as a maker/product-seller revision.
  - top patterns to copy are product/company funnel, shipped-tool proof, and product-destination dropdowns.
  - educator/course-first funnels are explicitly marked as structural inspiration only, not wording to copy.
- `SALES_FUNNEL_PROFILE_RESEARCH.md`
  - added `Maker/product-seller lens` section.
  - reclassifies the most relevant examples for DraconDev:
    - `rauchg` → Vercel,
    - `amix` → Doist,
    - `feross` → Socket.
  - updates DraconDev implications to say products first, code second.
- `README_SUGGESTED_FORM.md`
  - rewritten as `Maker/Product-Seller Form`.
  - product destinations now appear before code.
  - code section is labeled `Inspect the code` and described as proof.
  - form explicitly avoids educator/course language.
- `SUGGESTED_FORM_BLOCKERS.md`
  - added blocker notes for `tool-or-product-1-url` and `tool-or-product-2-url`.
  - added product/tool decision rule.
- `SUGGESTED_FORM_USAGE.md`
  - adoption guidance now says to keep maker/product-seller positioning: products first, code as proof.

## Files intentionally unchanged

The live README files were not changed:

- `README.md`
- `README_DRAFT.md`

This matches the user's request to keep the positioning in mind and update the strategy/form artifacts, not adopt a new live README yet.

## Diffs saved

- `/tmp/maker_goal/SALES_FUNNEL_RECOMMENDATIONS.diff`
- `/tmp/maker_goal/SALES_FUNNEL_PROFILE_RESEARCH.diff`
- `/tmp/maker_goal/README_SUGGESTED_FORM.diff`
- `/tmp/maker_goal/SUGGESTED_FORM_BLOCKERS.diff`
- `/tmp/maker_goal/SUGGESTED_FORM_USAGE.diff`

## Verification evidence

Final audit script:

- `/tmp/maker_goal/final_audit.sh`

Final audit output:

```json
{
  "live_readme_unchanged": true,
  "live_draft_unchanged": true,
  "maker_positioning_present": true,
  "educator_first_absent_from_form": true,
  "no_secret_patterns": true,
  "link_check_results": [
    {
      "url": "https://addyosmani.com/",
      "status": "200"
    },
    {
      "url": "https://benawad.com/",
      "status": "200"
    },
    {
      "url": "https://cassidoo.co/",
      "status": "200"
    },
    {
      "url": "https://cleancoders.com/",
      "status": "200"
    },
    {
      "url": "https://codetv.dev/series/learn-with-jason/s8",
      "status": "200"
    },
    {
      "url": "https://doist.com/",
      "status": "200"
    },
    {
      "url": "https://dracon.uk",
      "status": "200"
    },
    {
      "url": "https://egghead.io/q/resources-by-colby-fayock",
      "status": "200"
    },
    {
      "url": "https://frontendmasters.com/teachers/brian-holt/",
      "status": "200"
    },
    {
      "url": "https://frontendmasters.com/teachers/kyle-simpson/",
      "status": "200"
    },
    {
      "url": "https://frontendmasters.com/teachers/marc-grabanski/",
      "status": "200"
    },
    {
      "url": "https://github.com/sponsors/DraconDev",
      "status": "200"
    },
    {
      "url": "https://kentcdodds.com/",
      "status": "200"
    },
    {
      "url": "https://lib.rs/search?q=dracon",
      "status": "200"
    },
    {
      "url": "https://mbeaudru.github.io/modern-js-cheatsheet/",
      "status": "200"
    },
    {
      "url": "https://vercel.com/",
      "status": "200"
    },
    {
      "url": "https://wesbos.com/courses",
      "status": "200"
    },
    {
      "url": "https://www.joshwcomeau.com/",
      "status": "200"
    },
    {
      "url": "https://www.socket.dev/",
      "status": "allowed-bot-block"
    },
    {
      "url": "https://www.traversymedia.com/store",
      "status": "200"
    },
    {
      "url": "https://www.troyhunt.com/",
      "status": "200"
    },
    {
      "url": "https://youtube.com/@DraconDev",
      "status": "200"
    }
  ]
}
```

## Requirement-to-evidence map

| Requirement | Evidence |
|-------------|----------|
| Preserve user positioning: maker/product seller, not educator | `MAKER_PROFILE_POSITIONING.md`; `SALES_FUNNEL_RECOMMENDATIONS.md`; `README_SUGGESTED_FORM.md` |
| Revisit sales-funnel research through maker/product-seller lens | `SALES_FUNNEL_PROFILE_RESEARCH.md` section `Maker/product-seller lens` |
| Update sales-funnel recommendations so they no longer over-index on educator/course examples | `SALES_FUNNEL_RECOMMENDATIONS.md` top patterns now prioritize product/company funnel, shipped-tool proof, and product-destination dropdowns |
| Update suggested README form to emphasize shipped tools, product destinations, proof points, buy/use CTAs, and code as evidence | `README_SUGGESTED_FORM.md` rewritten with `Use it / buy it` before `Inspect the code` |
| Keep live README and DRAFT unchanged | Final audit `live_readme_unchanged: true`, `live_draft_unchanged: true`; SHA snapshots under `/tmp/maker_goal/before/` |
| Save diffs showing changes | `/tmp/maker_goal/*.diff` |
| No secret leakage | Final audit `no_secret_patterns: true` |
| No broken links introduced | Final audit link checks; all checked URLs 200 except known `socket.dev` bot-blocked endpoint |
| Do not reframe DraconDev as educator/course creator | Final audit `educator_first_absent_from_form: true`; form body contains no "I teach" or "learn from me" |

## Main recommendation

The profile should answer these three questions in under ten seconds:

1. What does DraconDev make?
2. Where do I buy or use it?
3. Where can I inspect the code if I care?

The strongest structure is:

1. Maker/product-seller hero.
2. Short proof line.
3. Product destinations (`dracon.uk`, Chrome Web Store, verified games).
4. Code as supporting proof.
5. Footer with `dracon.uk`, YouTube, Sponsor.

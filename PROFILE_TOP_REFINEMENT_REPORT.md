# Profile Top Refinement Report

Date: 2026-06-11

## Change made

Updated `README.md` and `README_DRAFT.md` so the profile top is less paragraph-heavy and more list-driven, while keeping product destinations separate from code.

The new top now uses:

1. **Use it / buy it** — finished tools, not source code.
   - `Chrome extensions` detail toggle.
   - `Games` detail toggle.
2. **Inspect the code** — concise list of public Rust/tooling repos.
   - `More code worth knowing about` detail toggle for secondary/private/future code.

## Why this is less boring / more list-like

Before:

- The Code section used longer prose bullets such as “a polished TUI app that ships the engine in a real product.”
- Product sections were placed after Code.
- The intro said “I teach what I build,” which leaned educator-first even though the profile positioning is maker/product-seller.

After:

- Product destinations appear first in collapsible toggles.
- Code is a short list of repo name + compact value phrase.
- The Code section is ~31.7% shorter than before:
  - before: 1,670 characters
  - after: 1,141 characters
- The page now says “Use or buy the finished tools” and removes “I teach” language.
- Details remain available for secondary code without crowding the first screen.

## Structure preserved

- Product destinations remain separate from source code.
- Chrome extensions stay in a detail toggle and point to the Chrome Web Store, not source.
- Games stay in a detail toggle and do not link to private source.
- Code links remain GitHub source repos only.
- `dracon-utilities` remains out of the first-screen code list because it is private and not fmt-clean.

## Files changed

- `README.md`
- `README_DRAFT.md`

## Verification evidence

- `cmp -s README.md README_DRAFT.md` returned `0`.
- Rendered `README.md` with `markdown-it-py`; output saved to `/tmp/profile_top_refine_goal/README_rendered.html`.
- Link check saved to `/tmp/profile_top_refine_goal/readme_link_check.json`: 12 links, all HTTP 200.
- Structure check saved to `/tmp/profile_top_refine_goal/structure_check.json`:
  - `readmes_identical: true`
  - `has_use_it_buy_it: true`
  - `has_chrome_extensions_details: true`
  - `has_games_details: true`
  - `has_more_code_details: true`
  - `code_is_list_driven: true`
  - `code_shorter_than_before: true`
  - `no_teach_language: true`
  - `product_details_no_github_source: true`
  - `code_no_chromewebstore: true`
  - `code_no_itch: true`
  - `junk_runner_unlinked: true`
- Final audit saved to `/tmp/profile_top_refine_goal/final_audit.sh`; last run printed `PASS profile top refinement audit`.

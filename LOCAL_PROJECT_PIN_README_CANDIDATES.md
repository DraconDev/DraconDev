# Local Project Future Pin/README Candidates

**Date:** 2026-06-12  
**Purpose:** Identify local projects worth adding later to GitHub pins or README/profile copy without weakening the current six-pin strategy.

## Current pin baseline

Current pins should stay:

1. `dracon-sync`
2. `dracon-system`
3. `dracon-warden`
4. `tiles-tui-file-manager`
5. `folder-auto-banner`
6. `obs-wayland-hotkey`

Current README has more freedom and can mention broader usable tools, product destinations, and secondary build-with foundations.

## Best future README additions

These are decent README additions before they are pin-worthy:

| Project | Add where | Evidence |
|:--------|:----------|:---------|
| `pully-fully-pull-based-fleet-reconciler` | Future usable tools / product candidate | Pull-based fleet reconciler for 5-100 VPS; strong DevOps product story once public; ~361 inspected files / ~32.3K LOC. |
| `rust-ai-web-auto` | Future usable tools / product candidate | Rust browser automation with optional AI; strong Playwright/Puppeteer-adjacent story once public; ~190 inspected files / ~29.8K LOC. |
| `ai-auto-repo-rot-scanner-todo-agent` | Future usable tools / product candidate | AI-powered repo health scanner; useful for maintainers and teams if packaged cleanly; ~641 inspected files / ~46.5K LOC. |
| `one-mil-girls` | Future games section | Visual novel with SvelteKit/Tauri-style web stack; only feature after public play/build destination is verified; ~1,970 inspected files / ~248.3K LOC. |
| `kiki-sassy-desktop-announcer` | Future usable tools | Memorable desktop announcer; crates.io + clear install path; ~67 inspected files / ~134.3K LOC. |
| `dracon-ai-lib` | Build-with foundations | Rust AI library for BYOK consumers; useful to developers building tools; ~110 inspected files / ~16.1K LOC. |
| `avid` / `youtube-video-uploader` | Usable tools | Creator workflow utility; already related to YouTube pipeline. |

## Best future pin candidates

These could replace or supplement pins later, but only after public packaging and validation:

| Future pin candidate | Why it could fit | Blocker before pinning |
|:---------------------|:-----------------|:-----------------------|
| `kiki-sassy-desktop-announcer` | Concrete, memorable, installable desktop tool; crates.io badge; clear user-facing value. | Needs clean public repo polish and current audit docs hidden or summarized. |
| `ai-auto-repo-rot-scanner-todo-agent` | Strong devtool story; repo health/security is easy to explain. | Needs public release, clean README, and dependency/source policy resolved. |
| `one-mil-girls` | Product destination if public playable demo exists; games are high-signal when playable. | Needs public play/build destination and no private content risk. |
| `pully-fully-pull-based-fleet-reconciler` | Strong DevOps product; open-core/managed path. | Needs public repo and clear install/demo story. |
| `rust-ai-web-auto` | Strong technical/product story; Rust AI browser automation. | Needs public repo and clear non-scary automation positioning. |

## Do not promote yet

Keep private or secondary until public/verified:

- `dracon-platform`
- `dracon-code`
- `dracon-demons`
- `browser-extensions-shared`
- private browser extensions except verified SamAI
- private games without public play pages
- `git-seal` as a primary item, because it is older `dracon-warden` lineage

## Recommendation

Do **not** change the current six pins now. Add future README mentions first when projects are public and polished. Promote to pins only when a project is:

1. public,
2. easy to explain in one line,
3. usable or product-destination verified,
4. validation-clean,
5. stronger than the current weakest pin candidate.

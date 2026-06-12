# Local Project Future Pin/README Candidates

**Date:** 2026-06-12  
**Purpose:** Keep future local projects in the right public-feature layer without weakening the current six-pin strategy.

## Hard categorization rules

- **Pins:** only concrete usable things. No vague parent buckets, no build-with libraries, no private products, no games/extensions unless they are verified public product destinations and still fit the pin story.
- **README usable-tools section:** usable tools and public product candidates that solve a job directly.
- **README details sections:** Chrome extensions and games go in details, not in the main pin list.
- **README build-with foundations:** libraries/frameworks/foundations belong only in the README, not pins.
- **Private products:** mention cautiously only when public/verified or when clearly labeled private.

## Current pin baseline

Current pins should stay:

1. `dracon-sync`
2. `dracon-system`
3. `dracon-warden`
4. `tiles-tui-file-manager`
5. `folder-auto-banner`
6. `obs-wayland-hotkey`

Reason: all six are concrete usable items. Six pins is still limited, so most local projects should remain README/profile copy, not pins.

## Future README additions by section

### Future usable tools / product candidates

These are decent README additions when public and polished:

| Project | Add where | Evidence |
|:--------|:----------|:---------|
| `pully-fully-pull-based-fleet-reconciler` | Future usable tools / product candidate | Pull-based fleet reconciler for 5-100 VPS; strong DevOps product story once public; ~361 inspected files / ~32.3K LOC. |
| `rust-ai-web-auto` | Future usable tools / product candidate | Rust browser automation with optional AI; strong Playwright/Puppeteer-adjacent story once public; ~190 inspected files / ~29.8K LOC. |
| `ai-auto-repo-rot-scanner-todo-agent` | Future usable tools / product candidate | AI-powered repo health scanner; useful for maintainers and teams if packaged cleanly; ~641 inspected files / ~46.5K LOC. |
| `kiki-sassy-desktop-announcer` | Future usable tools | Memorable desktop announcer; crates.io + clear install path; ~67 inspected files / ~134.3K LOC. |
| `avid` / `youtube-video-uploader` | Usable tools | Creator workflow utility; already related to YouTube pipeline. |

### Future build-with foundations

These can appear in the README build-with section, but not pins:

| Project | Add where | Reason |
|:--------|:----------|:-------|
| `dracon-ai-lib` | Build-with foundations | Rust AI library for BYOK consumers; useful to developers building tools; ~110 inspected files / ~16.1K LOC. |
| `dracon-terminal-engine` | Build-with foundations | Strong terminal/TUI foundation, but not direct newcomer value as a pin. |
| `azumi-live-ssr-framework` | Build-with foundations | SSR framework; useful if building similar tools, but not a first-screen product. |
| `ai-gui-auto-video-editor` | Build-with foundations | Product demo/foundation, not first-screen library positioning. |

### Future game details

Games do **not** go in pins. They go in the README games details section only after public play/build destination is verified.

| Project | Add where | Reason |
|:--------|:----------|:-------|
| `one-mil-girls` | Game details only, not pins | Visual novel/game product; ~1,970 inspected files / ~248.3K LOC; only feature after public play/build destination is verified. |
| `Junk-Runner-bevy` | Game details only, not pins | Survival/trading roguelike; only feature after public play/build destination is verified. |

### Future extension details

Browser extensions do **not** go in pins. They go in the README Chrome extension details section only when verified public install pages exist.

| Project | Add where | Reason |
|:--------|:----------|:-------|
| SamAI | Extension details/product link | Link verified Chrome Web Store page when confirmed; do not pin source. |
| `vidpro-extension` | Extension details | YouTube Studio/product tool; package/publish first. |
| `api-debugger` | Extension details | Dev-tool extension; publish to Chrome Web Store first. |
| `bugkit` | Extension details | Bug evidence product; publish/verify first. |
| Other private extensions | Do not feature yet | No verified public install pages. |

## Best future pin candidates

Only promote to pins if a project becomes public, polished, validation-clean, and clearly more usable than the current weakest pin candidate:

| Future pin candidate | Why it could fit | Blocker before pinning |
|:---------------------|:-----------------|:-----------------------|
| `kiki-sassy-desktop-announcer` | Concrete, memorable, installable desktop tool; crates.io badge; clear user-facing value. | Needs clean public repo polish and current audit docs hidden or summarized. |
| `ai-auto-repo-rot-scanner-todo-agent` | Strong devtool story; repo health/security is easy to explain. | Needs public release, clean README, and dependency/source policy resolved. |
| `pully-fully-pull-based-fleet-reconciler` | Strong DevOps product; open-core/managed path. | Needs public repo and clear install/demo story. |
| `rust-ai-web-auto` | Strong technical/product story; Rust AI browser automation. | Needs public repo and clear non-scary automation positioning. |

Do **not** treat these as future pins:

- `one-mil-girls` — game; details section only.
- `dracon-ai-lib` — library/build-with foundation; README only.
- `dracon-terminal-engine` — build-with foundation; README only.
- private browser extensions — extension details only after verified install pages.
- `dracon-code` — private/core product; only mention if made public and polished enough.

## Private/core products

Keep private unless explicitly made public:

- `dracon-platform`
- `dracon-code`
- `dracon-demons`
- `browser-extensions-shared`
- private browser extensions except verified SamAI
- private games without public play pages
- `git-seal` as a primary item, because it is older `dracon-warden` lineage

`dracon-code` is a product candidate to consider if it is made good enough to show publicly, but it is not a pin or README feature while private.

## Recommendation

Do **not** change the current six pins now. Add future README mentions first when projects are public and polished. Promote to pins only when a project is:

1. public,
2. easy to explain in one line,
3. usable,
4. validation-clean,
5. stronger than the current weakest pin candidate.

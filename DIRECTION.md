# DraconDev — Direction

**Last updated:** 2026-06-11  
**Purpose:** Stop the audit spiral and turn the current state into a clear operating direction.

---

## Current direction

DraconDev should present itself primarily as a maker of usable things people can install, open, or play, with Chrome extensions and games featured when there is a verified public destination. Build-with foundations are secondary and should not crowd the product-facing story.

The profile should lead with things people can understand quickly:

1. **Usable Chrome extensions and games** — installable or playable product destinations.
2. **Usable tools** — tools that solve a job directly.
3. **Build-with foundations** — libraries/frameworks for people building similar tools.
4. **Private/unverified work** — mentioned honestly, not promoted as public products.

The operating principle is:

> When in doubt, let the stuff do the talking: shipped tools, install pages, and public code.

---

## Current featured direction

### Featured usable tools

These are the current best profile features because they are concrete, understandable, and more grab-and-run than pure foundations:

1. **`folder-auto-banner`**
   - Contextual directory dashboard.
   - “`ls` with git, TODO, ports, and build context.”
   - Has a thumbnail and a clear before/after demo story.
   - Strong first feature.

2. **`tiles-tui-file-manager`**
   - TUI file manager.
   - Ships the terminal engine.
   - Strong usable tool.

3. **`obs-wayland-hotkey`**
   - OBS hotkey daemon.
   - Concrete Linux/OBS utility.
   - Public repo, but local copy is currently missing for validation.

4. **`youtube-video-uploader`**
   - Rust YouTube API client + CLI.
   - Useful for creator workflows.

5. **`dracon-utilities`**
   - Public monorepo of three installable Rust CLI utilities.
   - Feature the components clearly: `dracon-sync`, `dracon-system`, and `dracon-warden`.
   - Local validation passed: `cargo fmt`, `cargo clippy`, `cargo test --workspace`, `cargo deny check`, and `./scripts/verify-spec.sh`.

6. **`git-seal`**
   - Older Git filter lineage.
   - Keep in inventory/history only; do not use as a primary README or GitHub pin item because `dracon-warden` is the stronger current successor.

### Build-with foundations

These are valuable but less grab-and-run:

- **`dracon-terminal-engine`**
  - TUI engine; 43 widgets.
  - README-worthy, but more “build with” than “install and use.”

- **`azumi-live-ssr-framework`**
  - Live SSR framework for Rust on Axum.
  - Good foundation, but not a first-screen grab-and-run product.

- **`ai-gui-auto-video-editor`**
  - AGAVE product demo.
  - Useful as a demo/product signal, but not first-screen library positioning.

---

## Product destination direction

Chrome extensions and games are the main product destinations to feature, but only when there is a verified public install/play/store page. Keep them in details/collapsible sections until they are ready to be surfaced cleanly.

### Chrome extensions

- Feature only verified public install/store pages.
- Do not link private source repos as product destinations.
- Current verified public install link: Auto Fullscreen on Chrome Web Store.
- Other extensions remain local/private inventory until public install pages are verified.

### Games

- Feature only verified public play/install destinations.
- Current state: Junk Runner is private; no public play page verified.
- Do not promote private games as public products.

---

## Private/unverified blockers

These are strong but should not be featured as public products yet:

1. **`browser-extensions-shared`**
   - Private monorepo.
   - Many local extension projects, but most lack verified public install pages.
   - Do not invent store status from local files.

2. **`Junk-Runner-bevy`**
   - Private game.
   - No verified public play page.

3. **`avid`**
   - Private/local product.
   - No verified public destination.

4. **`one-mil-girls`**
   - Private/local game/product.
   - No verified public play page.

5. **`ai-auto-writer`**
   - Private/local AI product.
   - No verified public destination.

---

## Validation state

Evidence from the latest state audit:

- `README.md` and `README_DRAFT.md` intentionally differ while testing a cleaner product/pin/foundation structure.
- README public links pass.
- README renders correctly.
- `dracon-utilities` local validation passed: `cargo fmt`, `cargo clippy`, `cargo test --workspace`, `cargo deny check`, and `./scripts/verify-spec.sh`.
- `Junk-Runner-bevy/web npm run build` passed.
- Local `obs-wayland-hotkey` copy is missing, so local validation for that repo is blocked.
- Expensive broader validation beyond the local workspace commands was not run because the local workspace is large; this should be a later validation goal, not part of routine profile direction.

---

## What not to do next

Do **not** keep producing broad audits unless they directly cause a decision.

Avoid:

- More “full state” audits with no action.
- Adding private repos as public product links.
- Rewriting the profile into a long audit report.
- Promoting extensions/games without verified install/play pages.
- Treating build-with foundations as grab-and-run tools.
- Treating grab-and-run tools as generic code libraries.

---

## Next decisions

### Decision 1: utilities feature rollout

Question:

> Do we want to feature `dracon-utilities` as a public installable-utilities product with `dracon-sync`, `dracon-warden`, and `dracon-system` as distinct component stories?

Current recommendation:

- Yes, feature it as a usable-tools product, not as a vague infrastructure bucket.
- Lead with the three concrete jobs.
- Keep validation and install prerequisites clear.
- Do not claim paid revenue or broad casual-user readiness.

### Decision 2: warden vs git-seal

Current recommendation:

- De-emphasize `git-seal` in primary profile/README copy because it is older `dracon-warden` lineage.
- Feature `dracon-warden` as the stronger current Git hardening story inside `dracon-utilities`.
- Do not spend a GitHub pin slot on `git-seal`.

### Decision 3: obs-wayland-hotkey validation

Current blocker:

- Public repo exists.
- Local copy is missing.

Next useful step:

- Restore/clone `obs-wayland-hotkey` locally and run practical validation:
  - `cargo fmt --check`
  - `cargo test`
  - `cargo clippy -- -D warnings`

### Decision 5: GitHub pin rollout

Current recommendation:

- Final pin set: `dracon-sync`, `dracon-system`, `dracon-warden`, `tiles-tui-file-manager`, `folder-auto-banner`, and `obs-wayland-hotkey`.
- Do not pin `dracon-utilities` as a vague parent bucket.
- Do not pin games, browser extensions, libraries, build-with foundations, private products, or `dracon-terminal-engine`.
- Pin rollout is blocked until `dracon-sync`, `dracon-system`, and `dracon-warden` are split/published as separate public repositories.

Next useful step:

- If those component repos are split/published, manually update GitHub profile pins from the profile page.
- If not, keep `dracon-utilities` as README/profile copy only and do not pin the parent bucket.

### Decision 4: product destinations

Next useful step:

- Verify public install/play pages for extensions and games.
- Only then update profile product details.

---

## Recommended near-term roadmap

1. **Lock the profile direction**
   - Usable Chrome extensions and games first.
   - Usable tools next.
   - Build-with foundations second.
   - Product destinations only when verified.
   - Private/unverified work noted honestly.

2. **Feature `dracon-utilities` as public installable utilities**
   - Lead with `dracon-sync`, `dracon-system`, and `dracon-warden` as distinct jobs.
   - Keep the repo link public and validated.
   - Do not claim revenue or casual-user polish.

3. **Treat `dracon-warden` as the stronger current successor to `git-seal`**
   - It is the stronger successor to `git-seal`.
   - Should be linked through `dracon-utilities` because the monorepo is public and validated.
   - Do not feature `git-seal` as a primary README/pin item.

4. **Unblock GitHub pins for the utilities components**
   - Split/publish `dracon-sync`, `dracon-system`, and `dracon-warden` as separate public repos if direct pinning is desired.
   - Until then, link to component subdirectories from README/profile copy and do not pin `dracon-utilities` as the parent bucket.

5. **Restore and validate `obs-wayland-hotkey`**
   - Needed before claiming strong validation beyond public repo status.

6. **Verify extension/game destinations**
   - Store/play pages only.
   - No source-code links as product destinations.

---

## One-line direction

DraconDev should lead with **usable things people can install, open, or play**, feature **Chrome extensions and games when verified**, keep **build-with foundations in a second tier**, and only promote **private work when it becomes public, clean, and verifiably usable**.

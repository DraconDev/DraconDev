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

5. **`git-seal`**
   - Git filter for transparent file encryption.
   - Public and usable.
   - `dracon-warden` is the stronger private successor.

6. **`dracon-utilities`**
   - Strong usable category.
   - Private monorepo.
   - Should **not** be linked until public and `cargo fmt --check` is clean.
   - Best treated as a future public feature, not a current public product link.

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

1. **`dracon-utilities`**
   - Private.
   - Useful trio: sync / warden / system.
   - `warden` is a stronger successor to `git-seal`.
   - Needs public release + clean validation before profile linking.

2. **`browser-extensions-shared`**
   - Private monorepo.
   - Many local extension projects, but most lack verified public install pages.
   - Do not invent store status from local files.

3. **`Junk-Runner-bevy`**
   - Private game.
   - No verified public play page.

4. **`avid`**
   - Private/local product.
   - No verified public destination.

5. **`one-mil-girls`**
   - Private/local game/product.
   - No verified public play page.

6. **`ai-auto-writer`**
   - Private/local AI product.
   - No verified public destination.

---

## Validation state

Evidence from the latest state audit:

- README and draft are synchronized.
- README public links pass.
- README renders correctly.
- `dracon-utilities cargo fmt --check` passed.
- `Junk-Runner-bevy/web npm run build` passed.
- Local `obs-wayland-hotkey` copy is missing, so local validation for that repo is blocked.
- Expensive `dracon-utilities cargo test --workspace` was not run because the local workspace is ~14G; this should be a later validation goal, not part of routine profile direction.

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

### Decision 1: utilities public release

Question:

> Do we want to make `dracon-utilities` public as a clean simple monorepo with `dracon-sync`, `dracon-warden`, and `dracon-system` as distinct component paths?

If yes, next work is cleanup/validation/release-readiness, not more profile audits.

If no, keep it as a private note and do not feature individual utilities yet.

### Decision 2: warden vs git-seal

Current recommendation:

- Keep `git-seal` featured because it is public.
- Mention `dracon-warden` as the stronger private successor.
- Once `warden` is public and clean, feature it ahead of `git-seal`.

### Decision 3: obs-wayland-hotkey validation

Current blocker:

- Public repo exists.
- Local copy is missing.

Next useful step:

- Restore/clone `obs-wayland-hotkey` locally and run practical validation:
  - `cargo fmt --check`
  - `cargo test`
  - `cargo clippy -- -D warnings`

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

2. **Prepare `dracon-utilities` for public release**
   - Only if the user wants utilities featured as a real public tool.
   - Clean, validate, and publish as a simple monorepo.

3. **Promote `dracon-warden` after public release**
   - It is the stronger successor to `git-seal`.
   - Should not be linked while private.

4. **Restore and validate `obs-wayland-hotkey`**
   - Needed before claiming strong validation beyond public repo status.

5. **Verify extension/game destinations**
   - Store/play pages only.
   - No source-code links as product destinations.

---

## One-line direction

DraconDev should lead with **usable things people can install, open, or play**, feature **Chrome extensions and games when verified**, keep **build-with foundations in a second tier**, and only promote **private work when it becomes public, clean, and verifiably usable**.

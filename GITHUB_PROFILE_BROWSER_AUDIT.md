# GitHub Profile Browser Audit

Captured live with a real browser on 2026-06-09. Updated 2026-06-09 after the 6-pin alignment.

## Evidence

- Browser screenshot, full page: `/tmp/dracondev-profile/profile-full.png`
- Browser screenshot, above fold: `/tmp/dracondev-profile/profile-above-fold.png`
- Rendered page HTML: `/tmp/dracondev-profile/profile.html`
- Extracted page text/links/pins: `/tmp/dracondev-profile/profile.json`
- Visible profile links smoke check: `/tmp/profile-links.txt`

## What GitHub currently shows

- Profile URL: `https://github.com/DraconDev`
- Browser title: `DraconDev (Dracon) · GitHub`
- Profile README is rendered and matches the local `README.md` / `README_DRAFT.md`.
- Sponsor button is visible.
- Sidebar website link is rendered as `http://dracon.uk/`.
- README footer links render as:
  - `https://dracon.uk/`
  - `https://youtube.com/@DraconDev`
  - `https://github.com/sponsors/DraconDev`
- Crates.io link renders as `https://crates.io/users/DraconDev`.
- No standalone utilities links are visible:
  - no `DraconDev/dracon-sync`
  - no `DraconDev/dracon-system`
  - no `DraconDev/dracon-warden`

## Pinned repositories currently visible

GitHub currently pins 6 repositories:

1. `azumi-live-ssr-framework`
2. `dracon-terminal-engine`
3. `tiles-tui-file-manager`
4. `ai-gui-auto-video-editor`
5. `obs-wayland-hotkey`
6. `git-seal`

## README 6-pin alignment (2026-06-09)

The profile strategy now intentionally targets 6 pins, matching the live pinned set and the top profile examples we are copying. The `README.md` and `README_DRAFT.md` have been updated to present those 6 repos grouped by intent:

- **TUI & Terminal**: `dracon-terminal-engine`, `tiles-tui-file-manager`
- **Infrastructure**: `obs-wayland-hotkey`, `git-seal`
- **Frameworks**: `azumi-live-ssr-framework` (Azumi), `ai-gui-auto-video-editor` (AGAVE)

The previous README listed a 4-item TUI-only pin set and missed three of the live pins. The new README matches the live pin set.

## External action required

To fully match the README grouping and order in the live profile, reorder or re-pin the six repositories in the GitHub profile UI to the order shown in `README.md`. This is a manual UI step and is not part of the in-repo change set.

## Link smoke check

All visible profile/README/pinned links checked with browser-like curl returned HTTP 200:

- `https://dracon.uk/`
- `https://youtube.com/@DraconDev`
- `https://github.com/sponsors/DraconDev`
- `https://crates.io/users/DraconDev`
- `https://github.com/DraconDev/azumi-live-ssr-framework`
- `https://github.com/DraconDev/dracon-terminal-engine`
- `https://github.com/DraconDev/tiles-tui-file-manager`
- `https://github.com/DraconDev/ai-gui-auto-video-editor`
- `https://github.com/DraconDev/obs-wayland-hotkey`
- `https://github.com/DraconDev/git-seal`

## Recommended next actions

1. The README and live profile now agree on the 6-pin set. No further README change is required.
2. If the desired GitHub pin order should match the README grouping, reorder the six pins in the GitHub profile UI (manual, external).
3. Do not add `dracon-utilities` component links until the parent monorepo is public, clean, and verified.
4. Optional: compress the stats line later if brevity matters:
   - current: `239K+ lines of Rust · 5,600+ tests · 4 on crates.io`
   - shorter option: `239K+ Rust · 5.6K tests · crates.io/DraconDev`

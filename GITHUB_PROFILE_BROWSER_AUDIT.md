# GitHub Profile Browser Audit

Captured live with a real browser on 2026-06-09.

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

## Mismatch against current README strategy

The README currently presents a 4-item **TUI & Terminal** pin set:

1. `dracon-terminal-engine`
2. `tiles-tui-file-manager`
3. `folder-auto-banner`
4. `obs-wayland-hotkey`

The live profile does **not** match that README pin strategy yet:

- `folder-auto-banner` is in the README but is **not pinned** in the live profile.
- `azumi-live-ssr-framework`, `ai-gui-auto-video-editor`, and `git-seal` are pinned live but are not in the README's current 4-item pin set.

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

1. Decide whether the README's 4-pin strategy is the desired live profile.
2. If yes, update GitHub pinned repositories to:
   - `dracon-terminal-engine`
   - `tiles-tui-file-manager`
   - `folder-auto-banner`
   - `obs-wayland-hotkey`
3. If no, update the README to match the current live pins.
4. Consider compressing the stats line later if brevity matters:
   - current: `239K+ lines of Rust · 5,600+ tests · 4 on crates.io`
   - shorter option: `239K+ Rust · 5.6K tests · crates.io/DraconDev`
5. Do not add `dracon-utilities` component links until the parent monorepo is public, clean, and verified.

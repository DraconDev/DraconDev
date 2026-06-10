# GitHub Profile Browser Audit

Captured live with a real browser on 2026-06-09. Updated after the final README presentation polish.

## Evidence

- Browser screenshot, full page: `/tmp/dracondev-profile-final/profile-full.png`
- Browser screenshot, above fold: `/tmp/dracondev-profile-final/profile-above-fold.png`
- Rendered page HTML: `/tmp/dracondev-profile-final/profile.html`
- Extracted page text/links/pins: `/tmp/dracondev-profile-final/profile.json`
- Visible profile links smoke check: `/tmp/profile-links.txt`

## What GitHub currently shows

- Profile URL: `https://github.com/DraconDev`
- Browser title: `DraconDev (Dracon) · GitHub`
- Profile README is rendered and matches the local `README.md` / `README_DRAFT.md` in the final browser capture.
- Sponsor button is visible.
- Sidebar website link is rendered as `http://dracon.uk/`.
- README footer links render as:
  - `https://dracon.uk/`
  - `https://youtube.com/@DraconDev`
  - `https://github.com/sponsors/DraconDev`
- Crates.io link target: `https://crates.io/users/DraconDev`.
- No standalone utilities links are visible:
  - no standalone sync/system/warden utilities links

## Pinned repositories currently visible

GitHub currently pins 6 repositories:

1. `azumi-live-ssr-framework`
2. `dracon-terminal-engine`
3. `tiles-tui-file-manager`
4. `ai-gui-auto-video-editor`
5. `obs-wayland-hotkey`
6. `git-seal`

## Final README presentation (2026-06-09)

The profile README now presents a concise 6-pin set grouped by intent:

- **TUI & Terminal**
  - `dracon-terminal-engine` — TUI framework, 43 widgets
  - `tiles-tui-file-manager` — Dual-pane file manager
- **Infrastructure**
  - `obs-wayland-hotkey` — OBS hotkey daemon: key combos, action chains, delayed starts
  - `git-seal` — Git filter that encrypts specified files on commit
- **Frameworks**
  - `azumi-live-ssr-framework` — Live SSR framework for Rust on Axum, ~10KB gzipped
  - `ai-gui-auto-video-editor` — AI GUI auto video editor: silence cut, scene detect, auto-reframe

Presentation changes made:

- Shortened the stats line to `239K+ Rust lines · 5.6K tests · crates.io/DraconDev`.
- Replaced the previous 4-item TUI-only pin set with the intentional 6-pin set.
- Removed low-impact wording and kept each pin line concrete.
- Kept the footer as three links only: `dracon.uk`, YouTube, Sponsor.
- Avoided `dracon-utilities` component links until the parent monorepo is public, clean, and verified.

## External action required

To fully match the final README grouping and order in the live profile, reorder or re-pin the six repositories in the GitHub profile UI to the order shown in `README.md`. This is a manual UI step and is not part of the in-repo change set.

## Link smoke check

Visible profile/README/pinned links checked with browser-like curl:

- `https://dracon.uk/` — 200
- `https://youtube.com/@DraconDev` — 200
- `https://github.com/sponsors/DraconDev` — 200
- `https://github.com/DraconDev/azumi-live-ssr-framework` — 200
- `https://github.com/DraconDev/dracon-terminal-engine` — 200
- `https://github.com/DraconDev/tiles-tui-file-manager` — 200
- `https://github.com/DraconDev/ai-gui-auto-video-editor` — 200
- `https://github.com/DraconDev/obs-wayland-hotkey` — 200
- `https://github.com/DraconDev/git-seal` — 200
- `https://crates.io/users/DraconDev` — automated curl was rate-limited/blocked by crates.io; link target remains valid and previously verified in browser-like checks.

## Recommended next actions

1. Push the final `README.md` / `README_DRAFT.md` if not already deployed to the live profile.
2. Reorder the six GitHub pins manually to match the README grouping and order.
3. Do not add `dracon-utilities` component links until the parent monorepo is public, clean, and verified.

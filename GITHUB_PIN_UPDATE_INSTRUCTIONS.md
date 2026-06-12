# GitHub Pin Update Instructions

**Date:** 2026-06-12  
**Status:** Agent cannot update pins directly because GitHub profile pins are not exposed through the available `gh api` mutation surface, and `dracon-sync`, `dracon-system`, and `dracon-warden` currently do not exist as separate public repos.

**Access note:** `gh auth status` works for `DraconDev`, and GraphQL confirms `viewerCanChangePinnedItems: true`; however the schema only exposes issue/environment pin mutations, not profile repository pin updates.

## Current verified pins

Checked with `gh api graphql`:

1. `DraconDev/azumi-live-ssr-framework`
2. `DraconDev/dracon-terminal-engine`
3. `DraconDev/tiles-tui-file-manager`
4. `DraconDev/ai-gui-auto-video-editor`
5. `DraconDev/obs-wayland-hotkey`
6. `DraconDev/git-seal`

## Desired final pins

1. `dracon-sync`
2. `dracon-system`
3. `dracon-warden`
4. `tiles-tui-file-manager`
5. `folder-auto-banner`
6. `obs-wayland-hotkey`

## Important blocker

`dracon-sync`, `dracon-system`, and `dracon-warden` currently exist as component directories inside `dracon-utilities`, not separate public repositories:

- `DraconDev/dracon-sync` → 404
- `DraconDev/dracon-system` → 404
- `DraconDev/dracon-warden` → 404

GitHub profile pins can pin repositories, not subdirectories. So the exact desired pin set cannot be applied until those components are split/published as separate public repos.

## Unblock checklist before exact desired pins can be applied

1. Publish `dracon-sync`, `dracon-system`, and `dracon-warden` as separate public repositories, or explicitly choose a different interim pin set.
2. Re-check repo availability:
   - `https://api.github.com/repos/DraconDev/dracon-sync`
   - `https://api.github.com/repos/DraconDev/dracon-system`
   - `https://api.github.com/repos/DraconDev/dracon-warden`
3. Re-check current profile pins with `gh api graphql`.
4. Update pins manually from the GitHub profile page.
5. Verify the final visible pin order/content from the profile page or API.

## Manual update steps

When the component repos exist, or if you choose interim pin replacements, update pins from the GitHub profile page:

1. Go to `https://github.com/DraconDev`.
2. Scroll to **Pinned**.
3. Click **Customize your pins**.
4. Select the desired repositories in this order:
   - `dracon-sync`
   - `dracon-system`
   - `dracon-warden`
   - `tiles-tui-file-manager`
   - `folder-auto-banner`
   - `obs-wayland-hotkey`
5. Save.

## Interim manual option

If `dracon-sync`, `dracon-system`, and `dracon-warden` are not split yet, do **not** pin `dracon-utilities` as the parent bucket. Use only repos that already exist:

- `tiles-tui-file-manager`
- `folder-auto-banner`
- `obs-wayland-hotkey`

Then fill remaining slots only with other concrete usable repos, not games, extensions, libraries, build-with foundations, or private products.

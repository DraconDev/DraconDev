# DraconDev Repo Presentation Audit

Date: 2026-06-11

## Scope

- Remote GitHub inventory: `/tmp/repo_audit_goal/remote_inventory.json`
- Remote Chrome/game candidates: `/tmp/repo_audit_goal/candidate_repos.json`
- Remote README/package evidence: `/tmp/repo_audit_goal/candidate_readmes.json`
- Local Git repo inventory: `/tmp/repo_audit_goal/local_repos.json`
- Local README/package evidence: `/tmp/repo_audit_goal/local_readmes.json`

Inventory totals:

- Remote GitHub repositories: 341
- Public: 137
- Private: 204
- Archived: 1
- Local Git repositories found under `/home/dracon/Dev`: 19

## Current presentation

Current profile README links only to code repositories:

- `dracon-terminal-engine`
- `tiles-tui-file-manager`
- `obs-wayland-hotkey`
- `git-seal`
- `azumi-live-ssr-framework`
- `ai-gui-auto-video-editor`

This is strong for developer infrastructure, but it hides user-facing Chrome extension and game work.

## Chrome extension audit

Candidate repos:

- `browser-extensions-shared` — private monorepo; README lists many Chrome/WXT extensions.
- `vidpro-extension` — private; README describes video optimization assistant.
- `de-addict` / `de-addict-chrome-extension` — private; no public install page verified.
- `nexus-new-tab` — private; README says Chrome/Firefox links pending review.
- `new-page-pro-ext` — public repo, but no verified public install page found.
- `chrome-auto-fullscreen` — public repo with verified Chrome Web Store page.
- `webextensions-examples` — public fork/example repo, not DraconDev product.

Verified public destination:

- [Auto Fullscreen — Chrome Web Store](https://chromewebstore.google.com/detail/auto-fullscreen-fullscree/liddomimembhlpgehhkhefijckmhkkjn)
  - HTTP 200 verified.
  - Best user-facing link for the public Chrome extension category.
  - Avoid linking source code when the store page exists.

Not yet suitable for public profile feature:

- `browser-extensions-shared` is private, so it should not be linked from the public profile.
- `nexus-new-tab` is private and its README says store links are pending review.
- `vidpro-extension`, `de-addict`, `new-page-pro-extension`, and related extension repos have no verified public install page.

## Game audit

Candidate repos:

- `Junk-Runner-bevy` — private; README describes a survival/trading roguelike.
- `junk-runner-tauri` — private rewrite; template README only.
- `dracon-platformer` — private; README is actually `warehouse`, not a game.
- `Nadaga-Burger-Builder-2.0` — public repo, but README is CRA template with no install/play page.
- `burger-builder` — private CRA template with no install/play page.
- `guess-us-states-on-map` — public repo, but no verified public playable page.

Search checks found no verified public `itch.io`, Steam, GameJolt, or playable demo destination for DraconDev games.

Conclusion:

- Add a separate **Games** dropdown only as a non-linked note until a public install/play page exists.
- Do not link to private repos or source code for games on the public profile.

## Recommended presentation change

Add two collapsible dropdowns to the profile README:

1. **Chrome extensions**
   - Feature `Auto Fullscreen` with its Chrome Web Store link.
   - Mention that more browser extensions are in the private monorepo and will be linked only when public install pages exist.

2. **Games**
   - Mention `Junk Runner` as private/not publicly installable yet.
   - Do not link to source code until a public play/download page exists.

This keeps the first screen short, avoids showing code for user-facing products, and improves discoverability for non-developer audiences.

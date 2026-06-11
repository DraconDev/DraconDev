# Suggested README Form — Blocker Notes

Each placeholder in `README_SUGGESTED_FORM.md` is documented here with
the exact blocker that must be cleared before the slot is filled.

## Placeholder blockers

| Slot | Blocker | Evidence to clear |
|------|---------|-------------------|
| `{stat-line}` | None. Current `239K+ Rust lines · 5.6K tests` works. | `rg "239K" README.md` returns one match. |
| `{code-repo-1..5}` | All five currently featured repos are public and not archived, verified via `gh api repos/DraconDev/<repo>`. | See `final_audit.sh` and `readme_link_check.json`. |
| `{secondary-code-repo}` (`azumi-live-ssr-framework`, `ai-gui-auto-video-editor`) | Public, but niche fit / product demo, so they stay in the "More code" dropdown. | `gh api repos/DraconDev/<repo>` returns `private: false, archived: false`. |
| `{dracon-utilities-or-similar}` (the `dracon-utilities` slot) | Repo is **private** and `cargo fmt --check` is still failing. Promote to the first screen the moment it is public and clean. | Run `gh api repos/DraconDev/dracon-utilities --jq '.private, .archived'` and `cargo fmt --check` in `dracon-utilities/`. |
| `{chrome-store-extension-1-url}` (`Auto Fullscreen`) | None. Verified Chrome Web Store URL is live. | `curl -L -A 'Mozilla/5.0' -o /dev/null -w '%{http_code}' <url>` returns `200`. |
| `{chrome-store-extension-2-url}` and beyond | The remaining WXT/React extensions live in the private `browser-extensions-shared` monorepo. Promote a slot only when a Chrome Web Store URL is verified to return `200`. | `gh api repos/DraconDev/browser-extensions-shared --jq '.private'` and a Chrome Web Store URL check. |
| `{itch-or-site-game-1-url}` (`Junk Runner`) | Repo is private. No `itch.io`, Steam, GameJolt, or other public play/download URL was found in `web_search` results. Promote a slot only when a public play/install page returns `200`. | `web_search` for `Junk Runner` + `itch.io` and direct store/itch URL check. |
| `{additional-game}` | The other game repos (`Nadaga-Burger-Builder-2.0`, `burger-builder`, `guess-us-states-on-map`, `Junk-Runner-bevy`, `junk-runner-tauri`, `dracon-platformer`) have no public install/play page either. | Same as above. |
| `{dracon.uk}` | Live, returns `200`. | `curl -L -A 'Mozilla/5.0' -o /dev/null -w '%{http_code}' https://dracon.uk`. |
| `{YouTube}` | Live, returns `200`. | `curl -L -A 'Mozilla/5.0' -o /dev/null -w '%{http_code}' https://youtube.com/@DraconDev`. |
| `{Sponsor}` | Live, returns `200`. | `curl -L -A 'Mozilla/5.0' -o /dev/null -w '%{http_code}' https://github.com/sponsors/DraconDev`. |

## Decision rule for adding new slots

- **Code slot**: add only if the repo is public, not archived, has a
  real README, and the one-line "why a developer cares" is honest and
  specific. If the repo is private, it does not get a public code slot.
- **Chrome extension slot**: add only when the destination URL is the
  Chrome Web Store (or Firefox Add-ons) page, not the source-code URL.
- **Game slot**: add only when the destination URL is a public
  `itch.io` / Steam / GameJolt / `dracon.uk` / similar play or install
  page, not the source-code URL.

## How this file stays honest

Every blocker above maps to a concrete command or URL check. If a blocker
is cleared, run the check, update this file, and only then promote the
slot in the live README.

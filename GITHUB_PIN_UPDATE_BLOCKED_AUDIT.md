# GitHub Pin Update Blocked Audit

**Goal id:** `7aaece83-6fbd-49d4-8f81-4f3a04b83d7d`  
**Date:** 2026-06-13
**Status:** Blocked; not complete.

## Objective

Determine whether the GitHub profile pins can be updated by the agent or must be done by the user, and complete the update if tool/API access permits.

## Desired final pin set

1. `dracon-sync`
2. `dracon-system`
3. `dracon-warden`
4. `tiles-tui-file-manager`
5. `folder-auto-banner`
6. `obs-wayland-hotkey`

## Exclusions

Do not pin:

- `dracon-utilities`
- games
- browser extensions
- build-with foundations
- private products
- `dracon-terminal-engine`

## Evidence gathered

### Auth and account access

- `gh auth status` confirmed the active GitHub account is `DraconDev`.
- Token scopes shown by `gh auth status`: `gist`, `read:org`, `repo`, `workflow`.

### Current visible pins

Verified with both `gh api graphql` and public profile HTML from `https://github.com/DraconDev`:

1. `DraconDev/azumi-live-ssr-framework`
2. `DraconDev/dracon-terminal-engine`
3. `DraconDev/tiles-tui-file-manager`
4. `DraconDev/ai-gui-auto-video-editor`
5. `DraconDev/obs-wayland-hotkey`
6. `DraconDev/git-seal`

### GitHub profile pin change capability

- `gh api graphql` confirmed `viewerCanChangePinnedItems: true`.
- This means the account can change pins in the GitHub UI.
- It does not mean an API mutation is available.

### API update path

- `gh api graphql` introspection showed no profile repository pin update mutation.
- The only pin-related mutations exposed are:
  - `pinEnvironment`
  - `pinIssue`
  - `pinIssueComment`
  - `unpinIssue`
  - `unpinIssueComment`
- `gh api --method OPTIONS /user/settings/pinned_repos` returned HTTP 404, so no obvious REST endpoint for profile pinned repositories was exposed.
- A public profile HTML request with the GitHub token as a Bearer token still rendered the public profile state and sign-in prompts rather than the logged-in customize-pins UI.

Conclusion: there is no available authenticated API path to update GitHub profile repository pins.

### Target repo availability

GitHub API checks returned:

- `DraconDev/dracon-sync` → HTTP 404
- `DraconDev/dracon-system` → HTTP 404
- `DraconDev/dracon-warden` → HTTP 404
- `DraconDev/tiles-tui-file-manager` → exists and public
- `DraconDev/folder-auto-banner` → exists and public
- `DraconDev/obs-wayland-hotkey` → exists and public

Fresh 2026-06-13 re-checks after 13h40m and 15h23m confirmed the same blocker state: the current visible pins are still `azumi-live-ssr-framework`, `dracon-terminal-engine`, `tiles-tui-file-manager`, `ai-gui-auto-video-editor`, `obs-wayland-hotkey`, and `git-seal`; `dracon-sync`, `dracon-system`, and `dracon-warden` still return HTTP 404; and `tiles-tui-file-manager`, `folder-auto-banner`, and `obs-wayland-hotkey` still exist as public repositories. Later component-target checks at 16h25m, 17m, 26m, 32m, 41m, 48m, 56m, 1h6m, 1h16m, and 1h27m with `gh api graphql` again returned `repository:null` / `NOT_FOUND` for `DraconDev/dracon-sync`, `DraconDev/dracon-system`, and `DraconDev/dracon-warden`.

Conclusion: the exact desired pin set cannot be applied until `dracon-sync`, `dracon-system`, and `dracon-warden` are split/published as separate public repositories.

### Pin target limitation

GitHub profile pins can pin repositories, not subdirectories. Therefore, component directories inside `dracon-utilities` cannot be pinned directly.

## Requirements audit

| Requirement | Evidence | Status |
|:------------|:---------|:-------|
| Determine whether agent can update pins | Auth checked; GraphQL schema checked; no profile pin update mutation exists | Agent cannot update via API |
| Complete update if tool/API access permits | No valid API/UI automation path available; UI update requires manual browser action | Not applicable; blocked |
| Preserve desired final pin set | `GITHUB_PIN_UPDATE_INSTRUCTIONS.md` documents exact desired set | Verified |
| Do not pin excluded items | Instructions explicitly exclude `dracon-utilities`, games, extensions, foundations, private products, and `dracon-terminal-engine` | Verified |
| Verify public profile/current pins | `gh api graphql` and public profile HTML checked | Verified |
| Verify target repo availability | GitHub API checks returned 404 for three component repos | Verified |
| Run diff check | `git diff --check` passed | Verified |
| Markdown sanity for edited docs | Markdown sanity checks passed | Verified |
| Final audit mapping requirements to evidence | This audit file maps each requirement to evidence | Verified |

## Final conclusion

The goal is blocked, not complete.

Remaining required action is user-side:

1. Publish/split `dracon-sync`, `dracon-system`, and `dracon-warden` as separate public repositories.
2. Manually update GitHub profile pins from `https://github.com/DraconDev`.
3. Re-verify the final visible pin set.

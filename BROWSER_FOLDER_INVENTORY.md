# Browser Folder Inventory

Date: 2026-06-11  
Scope: inspect local browser-extension/browser folders and attempt to inspect the Chrome Web Store developer console and Firefox Add-ons developer pages. No browser listing was edited, uploaded, submitted, or changed.

## Real/system Chromium check for Chrome Web Store developer console

Target URL:

`https://chrome.google.com/webstore/devconsole/103b7bcb-2951-422a-887b-5ff701135adc`

Attempted paths:

- Playwright headed Chromium with an isolated temporary profile failed before navigation because the bundled Playwright Chromium binary could not load `libnspr4.so`.
- System Chromium (`chromium --version` → `Chromium 148.0.7778.167`) was then launched with an isolated temporary profile and `--headless=new` so the page could be rendered in a real browser engine without using existing cookies, passwords, tokens, or authenticated session data.

Evidence saved:

- `/tmp/chrome_console_goal/chrome_devconsole_system_headless.png` — screenshot from system Chromium.
- `/tmp/chrome_console_goal/chrome_devconsole_system_headless.html` — rendered DOM dump from system Chromium.
- `/tmp/chrome_console_goal/chrome_devconsole_system_headless.txt` — extracted visible/text DOM excerpt.
- `/tmp/chrome_console_goal/chrome_devconsole_system_headless.stderr` — Chromium stderr.

Observed:

- The target redirected to `https://accounts.google.com/v3/signin/`.
- The rendered DOM was a Google Accounts sign-in page.
- No Chrome Web Store developer dashboard, extension list, item IDs, review status, or unpublished/private listing state was visible from the isolated browser session.

Conclusion:

- The console still requires the authenticated Chrome Web Store developer session.
- Do not infer store status or public availability from this check.
- The local extension inventory below remains local-file evidence only unless a public store URL independently returns a real listing page.

## Result summary

- Main browser product repo found: `/home/dracon/Dev/browser-extensions-shared`.
- GitHub repo status visible from API: `DraconDev/browser-extensions-shared` is **private**, not archived, default branch `main`.
- Local monorepo inventory:
  - 19 `package.json` packages parsed.
  - 17 product-style extension packages plus `wxt-shared` and `wxt-starter`.
  - 39 generated browser manifests under `.output/`.
  - 47 build/package ZIP artifacts under `.output/` or project roots.
- Store console access was **not visible**:
  - Chrome Web Store dev console redirected to Google sign-in/CAPTCHA.
  - Firefox Add-ons developer page redirected to Firefox Accounts / client challenge.
- Public store links found in local files were partially verified:
  - BugKit Chrome Web Store link returned HTTP 200 and redirected to a real listing URL.
  - API Debugger AMO link returned HTTP 200 for a real listing.
  - Cursor-style AMO link returned HTTP 200 for a real listing.
  - API Debugger Chrome badge URL returned HTTP 200 but redirected to the store home page, so it is **not confirmed as a valid listing URL** from this capture.
  - Full-page screenshot AMO collection link returned 404.
  - Custom History claims “Published on Firefox AMO and Chrome Web Store” in `SPEC.md`, but no public listing URL was present to verify.
  - YouTube Dislike has placeholder/ellipsis store URLs and local “pending/awaiting review” notes, so it could not be verified.

## Browser-related folders found

| Folder | What it appears to be | Browser-product relevance |
|---|---|---|
| `/home/dracon/Dev/browser-extensions-shared` | WXT/React/TypeScript browser-extension monorepo | Main browser product inventory |
| `/home/dracon/Dev/dracon-code/examples/extensions` | Example extensions for `custom-agent-platform` | Not browser extensions; SDK/tool-extension examples |
| `/home/dracon/Dev/rust-ai-web-auto/extension` | MV3 extension files: `manifest.json`, `background.js`, `content.js`, `inject.js` | Local AI browser automation extension prototype |
| `/home/dracon/Dev/video-factory/extension` | MV3 extension files: `manifest.json`, `background.js`, `content.js`, `styles.css` | YouTube Studio helper extension prototype |

## Main repo: `browser-extensions-shared`

### Remote/API visibility

Evidence saved in `/tmp/browser_goal/browser_inventory.json`.

- GitHub API: `DraconDev/browser-extensions-shared`
- Private: `true`
- Archived: `false`
- Default branch: `main`
- Local remotes include GitHub, Codeberg, GitLab, and `origin`.

### Package/workspace mismatch

`pnpm-workspace.yaml` lists 19 packages:

`ai-ats`, `api-debugger`, `auto-form-filler`, `bugkit`, `calmweb`, `cursor-style`, `custom-history`, `custom-search`, `dark-mode-themes`, `death-note-typing-practice`, `full-page-screenshot`, `job-finder`, `live-reload-pro`, `SamAI`, `vidpro-extension`, `volume-and-video-pro`, `wxt-shared`, `wxt-starter`, `youtube-dislike`.

Actual top-level package directories parsed from `*/package.json`:

| Package dir | Package name | Version | Description |
|---|---:|---:|---|
| `SamAI` | `samai` | `49.1.0` | SamAI - Summarize, Chat, Fill Forms. AI browser companion. |
| `ai-ats` | `ai-ats` | `1.3.7` | AI-powered ATS candidate screening extension |
| `api-debugger` | `@dracon/api-debugger` | `2.5.4` | API Debugger - Browser-first API debugging tool |
| `auto-form-filler` | `form-filler-ai` | `2.2.0` | Fill web forms instantly with AI. Private, secure, and completely free. |
| `bugkit` | `bugkit` | `1.1.0` | Chrome extension for capturing complete, replayable bug evidence bundles |
| `cinematic-pages-cooler-presentations` | `cinematic-pages-cooler-presentations` | `2.1.0` | Transform any website with 48 effects |
| `cursor-style` | `cursor-style` | `1.0.0` | Custom cursors/effects package |
| `custom-history` | `custom-history` | `0.242.0` | Curate browsing history with custom rules |
| `custom-search` | `custom-search` | `1.0.0` | Better Search — Remove the noise from Google Search |
| `dark-mode-themes` | `dark-mode-themes` | `1.0.0` | Dark mode that keeps images natural |
| `death-note-typing-practice` | `@dracon/startype-typing-practice` | `1.2.0` | Typing game and tutor |
| `full-page-screenshot` | `@dracon/full-page-screenshot` | `1.0.0` | Capture full-page screenshots with inline annotation |
| `job-finder` | `job-finder-copilot` | `0.1.0` | Local-first job-search copilot |
| `live-reload-pro` | `wxt-react-starter` | `1.6.0` | Live reload dev utility |
| `vidpro-extension` | `vidpro-extension` | `1.13.634` | AI-powered YouTube Studio optimization extension |
| `volume-and-video-pro` | `volume-and-video-pro` | `3.22.7` | Volume & Video Master |
| `wxt-shared` | `@dracon/wxt-shared` | `1.0.0` | Shared utilities |
| `wxt-starter` | `@dracon/wxt-starter` | `1.0.0` | Starter template |
| `youtube-dislike` | `youtube-dislike` | `1.0.1` | AI-powered YouTube comment sentiment analysis |

Mismatch found:

- `calmweb` is listed in `pnpm-workspace.yaml` but the directory is missing locally.
- `web-automator` appears in `README.md` but is not in `pnpm-workspace.yaml` and no top-level directory was found.
- `cinematic-pages-cooler-presentations` exists locally with `package.json`, WXT config, manifests, and ZIP artifacts, but is not listed in the README project table.

### Generated manifests and artifacts

Generated manifests found under `.output/`:

| Project | Manifest builds found |
|---|---:|
| `SamAI` | 3 |
| `ai-ats` | 2 |
| `api-debugger` | 3 |
| `auto-form-filler` | 1 |
| `bugkit` | 3 |
| `cinematic-pages-cooler-presentations` | 1 |
| `cursor-style` | 2 |
| `custom-history` | 3 |
| `custom-search` | 3 |
| `dark-mode-themes` | 3 |
| `death-note-typing-practice` | 3 |
| `full-page-screenshot` | 3 |
| `job-finder` | 1 |
| `live-reload-pro` | 3 |
| `vidpro-extension` | 1 |
| `volume-and-video-pro` | 1 |
| `youtube-dislike` | 3 |

ZIP artifacts found: 47.

Examples:

- `api-debugger/.output/draconapi-debugger-1.0.15-chrome.zip`
- `api-debugger/.output/draconapi-debugger-1.0.15-firefox.zip`
- `bugkit/.output/bugkit-1.1.0-chrome.zip`
- `bugkit/.output/bugkit-1.1.0-firefox.zip`
- `custom-history/.output/custom-history-0.242.0-chrome.zip`
- `custom-history/.output/custom-history-0.242.0-firefox.zip`
- `SamAI/.output/samai-49.1.0-chrome.zip`
- `SamAI/.output/samai-49.1.0-firefox.zip`
- `youtube-dislike/.output/youtube-dislike-1.0.1-chrome.zip`
- `youtube-dislike/.output/youtube-dislike-1.0.1-firefox.zip`

Full artifact list is saved in `/tmp/browser_goal/browser_inventory.json`.

## Product/status signals found in local files

These are local-file signals only. Console status was not visible.

| Product | Local status/link signal | Verification result |
|---|---|---|
| BugKit | `bugkit/README.md` links Chrome Web Store; `bugkit/release-kit/` contains store submission materials | Chrome link returned HTTP 200 and redirected to a real listing URL |
| API Debugger | `api-debugger/README.md` has Chrome badge and AMO badge; docs say AMO awaiting review | AMO link returned HTTP 200; Chrome badge URL returned HTTP 200 but redirected to store home, not a confirmed listing |
| Custom Cursors / cursor-style | `cursor-style/README.md` says Firefox AMO released and links AMO | AMO link returned HTTP 200 for real listing |
| Custom History | `custom-history/SPEC.md` says “Published on Firefox AMO and Chrome Web Store” | No public listing URL found in local files, so not externally verified |
| Full Page Screenshot | `full-page-screenshot/README.md` says Chrome pending review and Firefox “in review” | Firefox collection link returned 404 |
| YouTube Dislike from Comments | `youtube-dislike/README.md` has placeholder/ellipsis AMO URL and says Firefox awaiting review / Chrome pending | Not verifiable from local files |
| SamAI, Auto Form Filler, AI ATS, Job Finder, VidPro, Dark Mode, Custom Search, Live Reload, etc. | Store/release docs or generated ZIPs exist | No verified public install URL found in this pass |

Public store-link check saved in `/tmp/browser_goal/store_link_check.json`.

## Other local extension prototypes

### `/home/dracon/Dev/rust-ai-web-auto/extension`

Manifest summary:

- Name: `Ghost-Orchestrator`
- Version: `0.1.0`
- Manifest version: 3
- Description: `AI-driven browser automation via WebSocket bridge`
- Permissions: `activeTab`, `scripting`, `tabs`, `debugger`
- Host permissions: `http://127.0.0.1/*`, `ws://127.0.0.1/*`
- Files: `manifest.json`, `background.js`, `content.js`, `inject.js`, icons

No public store link or store submission docs found.

### `/home/dracon/Dev/video-factory/extension`

Manifest summary:

- Name: `Video Factory`
- Version: `0.1.0`
- Manifest version: 3
- Description: `Drop raw footage into YouTube Studio. AI edits, captions, and publishes.`
- Permissions: `storage`
- Host permissions: `https://studio.youtube.com/*`
- Files: `manifest.json`, `background.js`, `content.js`, `styles.css`

No public store link or store submission docs found.

### `/home/dracon/Dev/dracon-code/examples/extensions`

These are not browser extensions. They are example extensions/tools for `custom-agent-platform`:

- `hello-provider`
- `sample-provider`
- `sample-tool`

No browser-store relevance found.

## Browser console capture attempts

I did **not** use the user’s existing browser profile, cookies, passwords, or authenticated session. I used isolated temporary profiles and public fetches only.

### Chrome Web Store Developer Console

Target URL:

`https://chrome.google.com/webstore/devconsole/103b7bcb-2951-422a-887b-5ff701135adc`

Evidence:

- `/tmp/browser_goal/chrome_devconsole.md`
- `/tmp/browser_goal/chrome_devconsole.html`
- `/tmp/browser_goal/chrome_devconsole.status.json`
- `/tmp/browser_goal/chrome_devconsole_headless.png`

Observed:

- HTTP status captured as `302`.
- Page text captured: `Chrome Web Store Developer Dashboard`, `Loading`, `Sign in to continue to Chrome Web Store`, `Email or phone`, CAPTCHA challenge.
- Headless Chromium screenshot captured a sign-in/CAPTCHA gate.

Conclusion:

- Developer console inventory was not visible.
- Access requires the authenticated Google/Chrome Web Store developer session.

### Firefox Add-ons Developer Page

Target URL:

`https://addons.mozilla.org/en-US/developers/addons`

Evidence:

- `/tmp/browser_goal/firefox_addons.md`
- `/tmp/browser_goal/firefox_addons.html`
- `/tmp/browser_goal/firefox_addons.status.json`
- `/tmp/browser_goal/firefox_addons_firefox.png`

Observed:

- HTTP status captured as `302`.
- Page text captured: `Enter your email`, `Continue to Add-ons`, Terms/Privacy links.
- Firefox headless screenshot was blank, but fetch/curl content showed the Firefox Accounts login/challenge page.

Conclusion:

- AMO developer add-on inventory was not visible.
- Access requires the authenticated Firefox Accounts/AMO developer session.

## Blockers

1. **Chrome Web Store developer console**
   - Blocker: Google sign-in + CAPTCHA gate.
   - Not visible: extension list, item IDs, review status, public/private/unpublished state.
   - Unblock input needed: user-provided sanitized console screenshots/export, or an isolated browser profile/session that can be used without exposing passwords/cookies/tokens.

2. **Firefox Add-ons developer page**
   - Blocker: Firefox Accounts login/client challenge.
   - Not visible: add-on list, review status, public/private/unpublished state.
   - Unblock input needed: user-provided sanitized developer page screenshots/export, or an isolated browser profile/session that can be used without exposing passwords/cookies/tokens.

3. **Local inventory gaps**
   - `calmweb` is listed in workspace but missing locally.
   - `web-automator` appears in README but no local directory/workspace entry found.
   - `cinematic-pages-cooler-presentations` exists locally but is missing from README project table.
   - Some local docs claim published/awaiting review status without public listing URLs.

## What was not changed

- No Chrome Web Store listing was edited, uploaded, submitted, or changed.
- No Firefox Add-ons listing was edited, uploaded, submitted, or changed.
- No browser-extension source files were edited.
- No secrets, cookies, tokens, or authenticated session data were printed or stored in this report.

## Evidence files

- `/tmp/browser_goal/browser_inventory.json` — structured local inventory.
- `/tmp/browser_goal/store_link_check.json` — public store-link verification.
- `/tmp/browser_goal/chrome_devconsole.md` — Chrome dev console capture summary.
- `/tmp/browser_goal/chrome_devconsole.html` — raw Chrome dev console fetch output.
- `/tmp/browser_goal/chrome_devconsole.status.json` — Chrome dev console status metadata.
- `/tmp/browser_goal/chrome_devconsole_headless.png` — headless Chromium sign-in/CAPTCHA screenshot.
- `/tmp/browser_goal/firefox_addons.md` — Firefox AMO developer capture summary.
- `/tmp/browser_goal/firefox_addons.html` — raw Firefox AMO developer fetch output.
- `/tmp/browser_goal/firefox_addons.status.json` — Firefox AMO developer status metadata.
- `/tmp/browser_goal/firefox_addons_firefox.png` — Firefox headless screenshot attempt.
- `/tmp/browser_goal/browser_availability.txt` — available browser binaries/tools.
- `/tmp/browser_goal/final_audit.sh` — final audit script; last run printed `PASS browser inventory audit`.

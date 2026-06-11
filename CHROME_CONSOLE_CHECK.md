# Chrome Web Store Developer Console Check

Date: 2026-06-11

Target URL:

`https://chrome.google.com/webstore/devconsole/103b7bcb-2951-422a-887b-5ff701135adc`

## Result

The console could not be inspected without the authenticated Chrome Web Store developer session.

The page redirected to Google Accounts sign-in. No extension list, item IDs, review status, unpublished/private state, or store metadata was visible from the isolated browser session.

## Browser attempts

### Playwright headed Chromium

Attempted with an isolated temporary profile.

Result: failed before navigation because the bundled Playwright Chromium binary could not load `libnspr4.so`.

Evidence:

- `/tmp/chrome_console_goal/chrome_devconsole_headed_error.json`

### System Chromium

Used system Chromium with an isolated temporary profile so the page was rendered by a real browser engine without using existing cookies, passwords, tokens, or authenticated session data.

Command shape:

```bash
chromium \
  --user-data-dir=/tmp/chrome_console_goal/system_profile \
  --no-sandbox \
  --disable-dev-shm-usage \
  --headless=new \
  --window-size=1440,1100 \
  --screenshot=/tmp/chrome_console_goal/chrome_devconsole_system_headless.png \
  'https://chrome.google.com/webstore/devconsole/103b7bcb-2951-422a-887b-5ff701135adc'
```

Evidence:

- `/tmp/chrome_console_goal/chrome_devconsole_system_headless.png`
- `/tmp/chrome_console_goal/chrome_devconsole_system_headless.html`
- `/tmp/chrome_console_goal/chrome_devconsole_system_headless.txt`
- `/tmp/chrome_console_goal/chrome_devconsole_system_headless.stderr`

Observed in rendered DOM:

- Redirect target/base URL: `https://accounts.google.com/v3/signin/`
- Google Accounts sign-in page DOM.
- No Chrome Web Store developer dashboard content.

## What this means for docs

Do not claim Chrome Web Store status, public listing status, unpublished status, or review status based on this check.

Use only verified public install/store URLs for product links. Keep other extensions in local/source inventory unless a public install page is verified.

## Local extension inventory snapshot

Main local browser product repo:

`/home/dracon/Dev/browser-extensions-shared`

Parsed package directories include:

- `SamAI`
- `ai-ats`
- `api-debugger`
- `auto-form-filler`
- `bugkit`
- `cinematic-pages-cooler-presentations`
- `cursor-style`
- `custom-history`
- `custom-search`
- `dark-mode-themes`
- `death-note-typing-practice`
- `full-page-screenshot`
- `job-finder`
- `live-reload-pro`
- `vidpro-extension`
- `volume-and-video-pro`
- `wxt-shared`
- `wxt-starter`
- `youtube-dislike`

These are local inventory facts, not verified Chrome Web Store publication facts.

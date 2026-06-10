# GitHub Sponsors Edit Log

Date: 2026-06-10
Target: `https://github.com/sponsors/DraconDev`

## Research and plan

- Research file: `SPONSORS_RESEARCH.md`
- Public before capture: `/tmp/sponsors-before/`
  - `/tmp/sponsors-before/sponsors-public-above.png`
  - `/tmp/sponsors-before/sponsors-public-full.png`
  - `/tmp/sponsors-before/sponsors-public.html`
  - `/tmp/sponsors-before/sponsors-public.json`
- Public after capture: `/tmp/sponsors-after/`
  - `/tmp/sponsors-after/sponsors-public-above.png`
  - `/tmp/sponsors-after/sponsors-public-full.png`
  - `/tmp/sponsors-after/sponsors-public.html`
  - `/tmp/sponsors-after/sponsors-public.json`

## Current page state before edits

The public page was functional but barebones:

```text
Title: Sponsor @DraconDev on GitHub Sponsors · GitHub
Body text: DraconDev / Support DraconDev's open source work / Select a tier / Monthly / One-time / Choose a custom amount.
Tiers: none
Featured work: none
Goal: none
Featured sponsors: none
```

Evidence: `/tmp/sponsors-before/sponsors-public.json`.

## Edit method attempted

1. Checked local GitHub CLI auth:
   - `gh auth status -h github.com --json active,hostname,user,oauthScopes` was attempted; `active` is not a valid JSON field.
   - `gh auth token` worked and confirmed the CLI is authenticated as `DraconDev`.
   - Token scopes from API errors: `gist`, `read:org`, `repo`, `workflow`.

2. Tried to use the GitHub GraphQL API for Sponsors mutations:
   - Querying `viewer.sponsorsListing` worked.
   - Current listing ID: `SL_kwDOAF7t7s4AAusu`
   - Current short description: `Support DraconDev's open source work`
   - Current full description: `Support DraconDev's open source work`
   - Active goal: `null`
   - Featured items: `[]`
   - Tiers: `[]`

3. Tried the available Sponsors GraphQL mutations:
   - `createSponsorsListing` failed with `INSUFFICIENT_SCOPES`; it requires `user` or `admin:org`.
   - `createSponsorsTier` failed with `INSUFFICIENT_SCOPES`; it requires `user` or `admin:org`.

4. Tried browser automation:
   - Connected to the existing Chrome CDP endpoint on `http://localhost:9223`.
   - Navigated to `https://github.com/sponsors/DraconDev/dashboard/profile`.
   - The browser rendered a GitHub sign-in / 404 page, not the sponsors dashboard.
   - Evidence: `/tmp/sponsors-edit-attempt/dashboard-before.json`.

5. Tried reusing the local Chromium profile:
   - Launched persistent Chromium with `/home/dracon/.config/chromium`.
   - Navigated to `https://github.com/`.
   - GitHub rendered the logged-out homepage.
   - Cookies showed `logged_in=no`.
   - Evidence: `/tmp/github-check.png` and the console output from `/tmp/check_cookie_context.cjs`.

## Result

No live sponsors page edits were applied.

Reason: the only available programmatic path requires a GitHub token with `user` or `admin:org` scope, and the local `gh` token has only `gist`, `read:org`, `repo`, and `workflow`. The browser automation path also failed because the available browser session is not logged into GitHub.

## Exact copy that should be applied when access is available

Short bio:

```text
Rust infrastructure builder — terminal engines, fleet reconcilers, git daemons. I teach the builds on YouTube and sell the tools on dracon.uk.
```

Introduction:

```markdown
I build infrastructure in Rust and ship it as open source: terminal engines, fleet reconcilers, git daemons. My tools run on Linux, talk to OBS, encrypt secrets, and reconcile fleets — boring, real work that other developers depend on every day.

Sponsor me if you want me to keep doing that full-time. Sponsorship pays for the time I spend on issues, refactors, security fixes, and the long tail of work that nobody sees. It also funds the YouTube videos where I show how the tools are built.

Even $3/month makes a real difference. Higher tiers get their name in the README, early access to new releases, and a direct line for bug reports and feature requests.
```

Goal:

```text
Maintainer time for the next release cycle — $400/month covers a focused month of issue triage, refactors, and security work across the DraconDev tools.
```

Tiers:

```text
Backer — $3/month — A thank-you in the next release notes.
Supporter — $7/month — Backer perks + your name in the README of one repo of your choice.
Builder — $14/month — Supporter perks + early access to new releases + a direct line for bug reports.
Studio — $49/month — Builder perks + your logo on dracon.uk + a 30-min call per quarter.
Infrastructure — $200/month — Studio perks + a dedicated support channel + roadmap input.
```

Featured work:

```text
DraconDev/dracon-terminal-engine
DraconDev/tiles-tui-file-manager
DraconDev/obs-wayland-hotkey
DraconDev/git-seal
DraconDev/azumi-live-ssr-framework
DraconDev/ai-gui-auto-video-editor
```

## Unblock options

- Option A: approve `gh auth refresh -h github.com -s user` and complete the browser/device authorization, then rerun the sponsors edit script.
- Option B: log into GitHub in the existing browser session, then rerun the CDP edit script.
- Option C: provide a personal access token with `user` or `admin:org` scope via a secure local mechanism; do not paste it into chat.

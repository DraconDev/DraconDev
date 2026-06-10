# GitHub Sponsors Page Research

Research compiled to make `https://github.com/sponsors/DraconDev` a legitimate, high-converting sponsors page.

## Sources

1. GitHub Docs — *Editing your profile details for GitHub Sponsors*
   `https://github.com/github/docs/blob/main/content/sponsors/receiving-sponsorships-through-github-sponsors/editing-your-profile-details-for-github-sponsors.md`
2. GitHub Docs — *Managing your sponsorship tiers*
   `https://github.com/github/docs/blob/main/content/sponsors/receiving-sponsorships-through-github-sponsors/managing-your-sponsorship-tiers.md`
3. Rizel Scarlett — *How to create the perfect sponsors profile for your open source project* (dev.to)
   `https://dev.to/github/how-to-create-the-perfect-sponsors-profile-for-your-open-source-project-3747`
4. Caleb Porzio — *I just hit $100,000/yr on GitHub Sponsors. Here's how I did it.*
   `https://calebporzio.com/i-just-hit-dollar-100000yr-on-github-sponsors-heres-how-i-did-it`
5. Vitaliy Sorenko — *Promoting your GitHub Sponsors profile: a comprehensive guide* (dev.to)
   `https://dev.to/vitalisorenko/promoting-your-github-sponsors-profile-a-comprehensive-guide-4ijl`
6. GitHub Blog — *Getting started with GitHub Sponsors*
   `https://github.blog/open-source/maintainers/getting-started-with-github-sponsors/`
7. Sindre Sorhus sponsors profile (live reference)
   `https://github.com/sponsors/sindresorhus`

## What GitHub says a sponsor profile should contain

The official GitHub docs explicitly recommend including:

- Open source work that you contribute to.
- Why you are committed to open source development.

The dashboard sections you can fill in are:

- Short bio (one-liner under the avatar).
- Introduction (longer markdown story).
- Featured work (up to a handful of repos).
- Featured sponsors (auto top 10 or manual).
- Goals (monetary or sponsor count).
- Sponsorship tiers.

## What the community says works (evidence-backed)

From Rizel’s curated list and the broader community:

- **Clear intro with personality.** Sindre Sorhus opens with a single emotional line — *“I love open source ❤️”* — and follows with a short origin story. Sponsors pages that read like a person, not a corporate pitch, convert better.
- **Concrete, tiered rewards.** Every serious profile has at least 3–4 tiers with specific amounts and what each tier unlocks. Caleb Porzio’s first tier is $7 with no perks (the “just say thanks” tier) and the main tier is $14 unlocking private screencasts. His insight: *“Pick tier names that describe the type of person the tier is suited for.”* (e.g. “The Agency” instead of “Platinum”.)
- **Charge an impactful amount.** Caleb: *“The biggest mistake people make with GitHub sponsors is offering too small of a first tier.”* His main tier started at $9 and moved to $14 once there was exclusive content behind it.
- **Goals / progress bars.** Babel shows “14% towards $12,000/month — Part of what we need for our three paid maintainers.” A specific, honest goal with a reason converts better than a generic “help me” line.
- **Featured work.** Highlight the most-used, most-loved repos, not a dump of every repo. Sindre’s profile surfaces the packages that pay the rent.
- **Featured sponsors.** Even with zero sponsors, opt in to “feature your top 10 sponsors” so the section is ready when the first one arrives.
- **One-time vs. monthly.** Offer both. Some people want a one-time tip.
- **Custom amount.** Always allow a custom amount; some people will give more than your top tier.
- **Transparency about money.** Caleb: *“Don’t be afraid to talk about your sponsorships and how much you make.”* This normalises sponsorship and makes the ask easier.
- **Share externally.** The GitHub dashboard has a “Share it out” block. The sponsors link should be in every README footer, YouTube description, and site footer.
- **Tone.** Confident, not apologetic. *“It’s OK to profit as well.”* Avoid “working on” or “WIP” language; it signals unfinished work.

## Current DraconDev sponsors page (before)

Captured with a real browser on 2026-06-10 from `https://github.com/sponsors/DraconDev`.

Evidence:

- `/tmp/sponsors-before/sponsors-public-full.png`
- `/tmp/sponsors-before/sponsors-public-above.png`
- `/tmp/sponsors-before/sponsors-public.html`
- `/tmp/sponsors-before/sponsors-public.json`

What it currently shows (public view):

```
Sponsor @DraconDev on GitHub Sponsors · GitHub
DraconDev
Support DraconDev's open source work
Select a tier
Monthly / One-time
$ a month — Choose a custom amount.
```

Gaps vs. the research:

- No short bio beyond the account name.
- No introduction / story.
- No featured work.
- No goals / progress.
- No tiers defined.
- No featured sponsors section.
- No social links other than the implicit GitHub profile.

This is the GitHub default for a newly enabled Sponsors account. The page is technically functional (a sponsor can pick a custom amount) but gives no reason to sponsor.

## Concrete plan for DraconDev

A short, concrete, conversion-oriented sponsors profile, ready to paste into the dashboard at `https://github.com/sponsors/DraconDev/dashboard/profile`.

### Short bio (one line, under the avatar)

```
Rust infrastructure builder — terminal engines, fleet reconcilers, git daemons. I teach the builds on YouTube and sell the tools on dracon.uk.
```

### Introduction (markdown, ~3 short paragraphs)

```
I build infrastructure in Rust and ship it as open source: terminal engines, fleet reconcilers, git daemons. My tools run on Linux, talk to OBS, encrypt secrets, and reconcile fleets — boring, real work that other developers depend on every day.

Sponsor me if you want me to keep doing that full-time. Sponsorship pays for the time I spend on issues, refactors, security fixes, and the long tail of work that nobody sees. It also funds the YouTube videos where I show how the tools are built.

Even $3/month makes a real difference. Higher tiers get their name in the README, early access to new releases, and a direct line for bug reports and feature requests.
```

### Featured work (up to 6 repos, match the 6-pin profile)

- `DraconDev/dracon-terminal-engine` — TUI framework, 43 widgets, used in `tiles`.
- `DraconDev/tiles-tui-file-manager` — Dual-pane file manager with git, SSH, and system monitor built in.
- `DraconDev/obs-wayland-hotkey` — OBS hotkey daemon: key combos, action chains, delayed starts.
- `DraconDev/git-seal` — Transparent Git filter that encrypts specified files on commit.
- `DraconDev/azumi-live-ssr-framework` — Live SSR framework for Rust on Axum, ~10KB gzipped.
- `DraconDev/ai-gui-auto-video-editor` — AGAVE: AI GUI auto video editor.

### Goal

```
Maintainer time for the next release cycle — $400/month covers a focused month of issue triage, refactors, and security work across the DraconDev tools.
```

(Exact amount can be tuned in the dashboard. The point is a specific, honest goal with a reason, not a generic ask.)

### Sponsorship tiers (5 tiers, monthly + one-time)

| Tier | Amount | What it unlocks |
|------|--------|-----------------|
| Backer | $3 / month | A thank-you in the next release notes. |
| Supporter | $7 / month | Backer perks + your name in the README of one repo of your choice. |
| Builder | $14 / month | Supporter perks + early access to new releases + a direct line for bug reports. |
| Studio | $49 / month | Builder perks + your logo on dracon.uk (when launched) + a 30-min call per quarter. |
| Infrastructure | $200 / month | Studio perks + a dedicated support channel + roadmap input. |

Tiers are named for the *type of sponsor*, not the *amount* (per Caleb’s advice). Custom amount is left enabled. One-time option is left enabled.

### Featured sponsors

Opt in to “Automatically feature my top 10 sponsors.” The section will be empty until the first sponsor arrives, which is the correct default.

### Links to add elsewhere

The sponsors link should also appear in:

- `DraconDev/DraconDev` profile README footer (already there).
- Every repo README in the “Support” section.
- `dracon.uk` footer (when launched).
- YouTube channel “About” and video description links.

## Implementation method

Published tiers were applied through the GitHub GraphQL API using the local PAT available under `~/.dracon/`, passed only through `GH_TOKEN`. The existing Sponsors listing bio/introduction/goal/featured-work fields are not exposed by the public GraphQL update mutations, so those remaining fields still need the dashboard UI or a future GitHub API surface. `createSponsorsListing` cannot be used as an update path because GitHub returns `DraconDev already has a GitHub Sponsors profile` for an existing listing.

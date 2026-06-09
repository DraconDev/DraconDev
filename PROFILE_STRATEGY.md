# DraconDev GitHub Profile Strategy

## Goal
GitHub profile as a revenue-generating sales funnel. Drive visitors to products, services, and YouTube.

## Current Execution Plan (2026-06-09)

This is **not just a README change**. The README is one asset in a broader cross-platform funnel. The current plan separates in-repo documentation from external platform actions:

| Asset | Job | Status | Evidence / artifact |
|:------|:----|:-------|:--------------------|
| `README.md` / `README_DRAFT.md` | Public GitHub profile proof: hero line, 4 pinned repos, stats, 3 CTAs | ✅ Aligned | Current 15-line README; keep unchanged unless new evidence appears |
| `PROFILE_STRATEGY.md` | Internal strategy map: what to do, where, why, and in what order | ✅ Updated | Effectiveness scoring section + action plan reference |
| `EFFECTIVENESS_SCORING.md` | Evidence base: factor scores, why they work, synthesized strategy | ✅ Created | 18.6KB, 226-line scoring artifact |
| GitHub profile bio | Name-drop the killer work above the README | ⏳ Needs GitHub profile access | Suggested: "Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests, 24 crates on crates.io." |
| GitHub Sponsors page bio | Convert warm visitors with concrete stats | ⏳ Needs GitHub Sponsors access | Suggested: "239K+ lines of Rust, 5,600+ tests, 24 crates on crates.io, 12 in-scope repos per the audit." |
| `dracon.uk` footer | Route product/tip traffic without diluting the README | ⏳ Needs site repo access | Add Ko-fi on the site, not the README |
| YouTube channel nav | 5-link ecosystem map | ⏳ Needs YouTube Studio access | `dracon.uk`, GitHub, Sponsor, Ko-fi, Discord if active |
| YouTube descriptions | Reusable 5+1 template for each video | ⏳ Needs YouTube Studio access | Hook → sponsor if any → body → topic index → social links → evergreen CTA |
| Content cadence / format | Weekly volume into the funnel | ⏳ Needs publishing workflow | Weekly shorts or weekly live streams; short-form hooks + long-form deep-dives |

The detailed plan, constraints, and verification checklist are in `CROSS_PLATFORM_ACTION_PLAN.md`; exact copy/templates for the external updates are in `EXTERNAL_PLATFORM_UPDATE_SNIPPETS.md`.

---

## Product Inventory (ranked by revenue potential)

### Tier 1 — Flagship Products (sell these)

| Product | What it is | Market | Revenue model |
|:--------|:-----------|:-------|:--------------|
| **SamAI** | AI browser companion — summarize, chat, fill forms, BYOK | Everyone who uses a browser | Freemium / paid tiers |
| **rust-ai-web-auto** | Enterprise AI-driven browser automation (CDP) | Dev teams, automation agencies | License / SaaS |
| **vidpro-extension** | YouTube Studio optimization — titles, tags, descriptions | YouTubers (you ARE one) | Freemium / paid tiers |

### Tier 2 — Credibility Products (show these)

| Product | What it is | Why it matters |
|:--------|:-----------|:---------------|
| **api-debugger** | Capture & debug HTTP requests | Dev tool, already on GitHub (1★) |
| **bugkit** | Capture replayable bug evidence | QA teams pay for this |
| **auto-form-filler** | AI form filling, BYOK | Utility, pairs with SamAI |
| **ai-job-finder** | Auto-apply for LinkedIn jobs | Job seekers pay for this |
| **ai-ats** | AI candidate screening | Recruiters pay for this |

### Tier 3 — Open Source Credibility (pin these)

| Repo | What it is | Why pin it |
|:-----|:-----------|:-----------|
| **dracon-terminal-engine** | Terminal framework for Rust (239 src files) | Most impressive Rust project |
| **obs-wayland-hotkey** | OBS hotkey daemon (8★) | Highest starred Rust repo |
| **tiles-tui-file-manager** | TUI file manager | Shows systems-level Rust skill |
| **video-factory** | Web platform for video processing | Pairs with YouTube workflow |

### Tier 4 — Niche/Novelty (don't pin)

dark-mode-themes, cursor-style, cinematic-pages, death-note-typing, volume-and-video, youtube-dislike, custom-history, live-reload-pro, full-page-screenshot, custom-search, calmweb

---

## The Content Flywheel

```
YouTube video (demo SamAI / vidpro / api-debugger)
    ↓
Viewer visits GitHub profile
    ↓
Profile shows: "Founder of SamAI" + pinned repos + YouTube link
    ↓
Clicks SamAI → website → download / pay
    ↓
User stars the repo → more visibility → more YouTube viewers
```

This is exactly what t3dotgg does:
- YouTube → "Founder of T3 Chat" → T3 Chat website → revenue
- 700K subscribers = free distribution for every product launch

You have the same assets:
- YouTube channel (you post videos)
- SamAI (browser extension, BYOK)
- vidpro-extension (YouTube tool, you use it yourself)

---

## Profile Layout (recommended)

### Bio (one line)
> Building AI-powered browser tools. Creator of SamAI. 🎥 YouTube

### Website field
> https://dracon.uk

### Company field
> DraconDev

### Pinned repos (3-4 max)
1. **dracon-terminal-engine** — "Terminal framework for Rust. Composable widgets, z-indexed layers." (144K lines, most impressive Rust project)
2. **pully-fully** (if published) — "Fleet reconciler for 5-100 VPS. Git recipes, auto-provisioning."
3. **tiles-tui-file-manager** — "Dual-pane TUI file manager. Vim-style, git, SSH, system monitor."
4. **obs-wayland-hotkey** — "OBS hotkey daemon for Wayland/X11. 8★."

**NOT pinned:** SamAI (closed source product, link instead), rust-ai-web-auto (link if published)

### README (short, 5-10 lines)
```markdown
### Hi, I'm DraconDev 👋

I build AI-powered tools for the browser and terminal.

🚀 **Products:**
- [SamAI](Chrome Web Store link) — AI browser companion
- [YouTube](link) — I demo my tools and build in public

🌐 [dracon.uk](https://dracon.uk)
```

### Left sidebar
- Website: dracon.uk
- YouTube: (link)
- GitHub: DraconDev

**Do NOT add:** Twitter/X (you don't post), Discord (unless active), LinkedIn (unless used for sales)

---

## What NOT to pin

- **git-ai-committer** (VS Code extension) — dilutes the browser/terminal story
- **chrome-auto-fullscreen** — no description, too niche
- **opencode-auto-continue** — tiny audience
- **AGAVE** — you said it sucks
- **Any boilerplate/template** — saas-starter, wxt-starter, etc.

---

## Revenue Path (near-term)

1. **Package SamAI for Chrome Web Store** — it has 149 src files, it's the most complete
2. **Add pricing to dracon.uk** — freemium: free tier + paid tier with more AI features
3. **YouTube video: "I built an AI browser extension"** — demo SamAI, link to download
4. **Link SamAI in README** — don't pin (closed source), link to Chrome Web Store

5. **Package vidpro-extension** — you use YouTube, you built a YouTube tool, demo it on your channel
6. **vidpro = "YouTube tool for YouTubers"** — perfect product-market fit with your audience

7. **api-debugger on Chrome Web Store** — dev tool, already on GitHub, low effort to publish

---

## Key Insight

Your strongest asset isn't your Rust repos — it's the **combination** of:
- Chrome extensions (products people can use NOW)
- YouTube (distribution channel you already have)
- Rust repos (credibility that proves you can build real systems)

t3dotgg doesn't win because he writes good code. He wins because he has YouTube + products + a clean profile that connects them. You have all three assets. You just need to connect them.

---

## 2026 Refresh: Cross-Platform Funnel & Sponsor Placement

**Date:** 2026-06-09
**Source:** `YOUTUBE_AND_MONETIZATION_RESEARCH.md` (22KB, Playwright-verified across 5 GitHub profiles, 5 YouTubers, 5 Sponsors accounts, 2 Ko-fi pages)

### The 5-Stage Cross-Platform Funnel (verified across 5 YouTubers)

```
[1] Content (YouTube video)
        ↓
[2] Subscribe (channel page)
        ↓
[3] Discover more (channel nav banner: Twitter, Twitch, Patreon, blog, GitHub, Discord)
        ↓
[4] Support (GitHub Sponsors / Patreon / affiliate link)
        ↓
[5] Deeper engagement (blog / courses / products on dracon.uk)
```

**DraconDev funnel status:**
- ✅ Stage 1: YouTube channel exists
- ✅ Stage 2: YouTube channel subscribable
- ✅ Stage 3: Channel nav (in README) has 3 links: dracon.uk · YouTube · Sponsor — per Fireship/Theo patterns, 3 distinct value props is the right density
- ✅ Stage 4: GitHub Sponsors page is **verified set up** (Playwright: "Become a sponsor to Dracon" page with bio "Support DraconDev's open source work" + monthly/one-time tiers + custom amount)
- ✅ Stage 5: dracon.uk has Products / Pricing / Licensing / AI Hub nav (verified via Playwright in earlier session)

**The funnel is complete.** What's missing is volume — getting more content into Stage 1 to feed the funnel.

### Sponsor Placement Recommendations (5 concrete actions)

Per §3.4 of `YOUTUBE_AND_MONETIZATION_RESEARCH.md`:

| # | Action | Where | Effort | Why |
|:-:|:-------|:------|:-------|:----|
| 1 | **Keep GitHub Sponsors link in README** | `README_DRAFT.md` footer | None | Already verified set up; page exists, bio is real, custom amount + monthly/one-time working |
| 2 | **Add Ko-fi to dracon.uk footer** (not README) | `dracon.uk` site | Low | Ko-fi is for digital products (Anki deck is already there) — fits the products-site tone, not the OSS profile tone |
| 3 | **Add Ko-fi to YouTube video descriptions** (when YouTube is active) | YouTube video descriptions | Low | **Highest-conversion spot for tip links** per Fireship/Theo patterns. YouTube viewers are the warmest audience for "buy me a coffee" |
| 4 | **Add GitHub profile bio name-drop** (separate from README) | GitHub profile bio field | Low | Pattern from mitsuhiko: "Software developer... Creator of the Flask framework. Founder of @earendil-works." — name-drops the killer work. Suggested: "Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests." |
| 5 | **Expand Sponsors page bio with concrete stats** | `github.com/sponsors/DraconDev` | Medium | Pattern from dtolnay: he bioed "68% of all published crates on crates.io depend transitively on syn." Stats make the page credible. Concrete numbers: 239K+ LOC, 5,600+ tests, 24 crates on crates.io, 12 in-scope repos per the audit. |

### GitHub Profile Bio Recommendation (Pattern 4 from research)

Per mitsuhiko's pattern (the most directly applicable for DraconDev):

**Current** (per earlier docs, profile bio field on GitHub):
> (empty or default)

**Suggested** (per new research, name-drops killer work like mitsuhiko does with "Flask"):
> Rust infrastructure for fleets, git, and terminals. 239K+ lines, 5,600+ tests, 24 crates on crates.io.

**Why this works:** Per the research, mitsuhiko's 25K-follower profile bioed "Creator of the Flask framework" and "Founder of @earendil-works" — 4-word and 5-word positioning hits. DraconDev's bio should similarly name-drop the killer work (Rust infrastructure at scale).

### YouTube Description Pattern (5+1 sections, per Fireship)

When YouTube becomes active, the description pattern below applies (verified across 5 YouTubers, most clearly seen in Fireship's descriptions):

```
[1] Hook (1-2 lines, the video topic)

[2] Sponsor (when there is one — at the TOP):
    "Thank you [Sponsor] for sponsoring! Check them out at: [link]"

[3] Body (2-3 paragraphs of context, source links, etc.)

[4] Topic index (when relevant):
    🔖 Topics Covered
    - Topic 1: link
    - Topic 2: link

[5] Social/website CTAs (BOTTOM of description):
    Check out my [Twitter/Twitch/Blog] at [link]
    Find more of my stuff on: [main website]

[6] Recurring evergreen CTA (high-value):
    "Want more? [Newsletter] · [Courses] · [Ko-fi tip jar] · [GitHub Sponsors]"
```

**For DraconDev specifically** (when YouTube is active):
- [1] Hook: 1-2 lines about the topic
- [2] Sponsor: at top, but only if there's actually a sponsor (per antfu: don't fake it)
- [3] Body: 2-3 paragraphs
- [5] Social: link to dracon.uk, GitHub profile, Ko-fi
- [6] Evergreen CTA: "Support what I build: GitHub Sponsors / Ko-fi / dracon.uk"

### README Update (already applied 2026-06-09)

The README footer is now 3 distinct value props (per Fireship pattern: site + content + support). The stats line is now a clickable link to crates.io:

**Before:**
> **239K+** lines of Rust · **5,600+** tests · **4** on crates.io

**After:**
> **239K+** lines of Rust · **5,600+** tests · [**4** on crates.io](https://crates.io/users/DraconDev)

The change adds clickability to a stat — verified by Playwright that `https://crates.io/users/DraconDev` loads in a real browser and shows "Displaying 1-10 of 24 total results" (the 24 = total crates; the 4 = the 4 mature/pinned repos featured in the README).

The 3-link footer block (dracon.uk · YouTube · Sponsor) follows the Fireship "Want more?" pattern, with 3 distinct value props. The "first supporter" status is invisible to non-sponsors (GitHub hides count), so the Sponsor link is functional at zero sponsors.

### What the research did NOT change about the README

- **Hero line "Hey, I make tools that run themselves."** — Per mitsuhiko's pattern, this is the strong name-drop. Don't change.
- **4-repo pin list (TUI & Terminal)** — Per the "tangible things only" stance and the audit, this is correct. Don't add "Working on" or "Experimental" sections.
- **239K+ lines / 5,600+ tests stat** — concrete numbers, no widgets. Per the original 131-profile research. Don't change.
- **No Ko-fi in README** — Per Fireship/Theo patterns, the 3-link footer is the right density. Adding Ko-fi to the README would dilute.

### Cross-References

- GitHub profile research: `GITHUB_PROFILE_RESEARCH.md` — original 131-profile analysis (2026-06-06) + 2026 Refresh section (2026-06-09)
- YouTube & monetization research: `YOUTUBE_AND_MONETIZATION_RESEARCH.md` (22KB, 2026-06-09)
- Effectiveness scoring: `EFFECTIVENESS_SCORING.md` (18.6KB, 2026-06-09) — factor-based 4×16 matrix, top 5 factors by total score, synthesized combined strategy
- Publish readiness audit: `GO_NO_GO_REPORT.md` (2026-06-08) — not regressed by the README change
- Verification artifacts: `/tmp/profile-*.png` (5 GitHub), `/tmp/yt-*.png` (5 YouTube), `/tmp/monet-*.png` (6 sponsor/kofi), `/tmp/crates-user-final.png` (crates.io user page)

---

## Effectiveness Scoring: What Works and Why

**Date:** 2026-06-09
**Source:** `EFFECTIVENESS_SCORING.md` (18.6KB, 226 lines)

### Why factor-based scoring (not peer ranking)

The goal is not to find the "best" peer. The goal is to find the **best factor from each peer** and combine them into a superior strategy. A peer with 2/4 dimensions excellent is still a source of those 2 dimensions. This is Pareto, not a leaderboard.

### The 4×16 Effectiveness Matrix (summary)

**4 outcome dimensions** (what we're trying to maximize):
1. **Reach** — how many people see the work (subs/followers/stars/views)
2. **Engagement** — how much people interact (comments, stars-per-repo, view-through rate, repeat visitors)
3. **Monetization** — how much money the strategy generates (sponsor count, Patreon, paid sponsors, Ko-fi supporters)
4. **Cross-platform reinforcement** — how much one channel feeds another (YouTube → GitHub → website → sponsor, etc.)

**16 input factors** (things you can actually change in your own strategy):
1. Hero line / bio name-drop
2. Bio depth / killer stats
3. README presence
4. Pinned repo selection
5. Sponsor button presence
6. Sponsor page bio quality
7. Sponsor page tier structure
8. YouTube description hook
9. YouTube description sponsor placement
10. YouTube description social links
11. YouTube channel nav links
12. Content cadence
13. Content format
14. Cross-platform CTA consistency
15. Social proof visibility
16. Product/website integration

**Top 5 factors by total score (0-40):**
1. **Cross-platform CTA consistency** (35) — Theo's t3.gg hub
2. **Product/website integration** (34) — Fireship's newsletter + courses
3. **YouTube channel nav links** (33) — Theo's 5-link nav
4. **YouTube description hook** (31) — Fireship's 1-2 line hook
5. **Content format** (31) — Fireship's short-form, high-intensity

**Key insight:** The top 5 factors are all about **funnel architecture**, not individual tactics. The best-performing strategies are the ones that create the strongest cross-platform loop.

### Synthesized Combined Strategy

The most effective strategy is not to copy any single peer. It's to **combine the best factor from each peer** into a coherent funnel:

1. **GitHub profile bio** name-drops the killer work (mitsuhiko)
2. **Sponsors page bio** uses killer stats (dtolnay)
3. **README** keeps the 15-line, 4-repo pin list (yoshuawuyts + BurntSushi)
4. **YouTube description** uses the Fireship 5+1 structure with sponsor at top (Fireship + Theo)
5. **YouTube channel nav** has 5 links (Theo)
6. **Content cadence** is weekly (antfu)
7. **Content format** is a mix of short-form (shorts) and long-form (deep-dives) (Fireship)
8. **Cross-platform CTA consistency** is one hub (dracon.uk) (Theo)
9. **Social proof visibility** is the Sponsor page + Ko-fi (fasterthanlime + jam1garner)
10. **Product/website integration** is the book library + newsletter on dracon.uk (Fireship)

**DraconDev-specific synthesis:**
- **GitHub profile bio:** "Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests, 24 crates on crates.io."
- **Sponsors page bio:** Expand with concrete stats: "239K+ lines of Rust, 5,600+ tests, 24 crates on crates.io, 12 in-scope repos per the audit."
- **README:** Keep the current 15-line, 4-repo pin list. Do NOT add "Working on" or "Experimental" sections.
- **YouTube descriptions (when active):** Fireship 5+1 structure: hook → sponsor (if any) → body → topic index → social links → evergreen CTA.
- **YouTube channel nav (when active):** 5 links: dracon.uk, GitHub, Sponsor, Ko-fi, Discord (if active).
- **Content cadence:** Weekly live streams or weekly shorts. Consistency > volume.
- **Content format:** Mix of short-form (shorts) and long-form (deep-dives). Short-form hooks, long-form retains.
- **Cross-platform hub:** dracon.uk is the hub. All roads lead there.
- **Social proof:** The Sponsor page + Ko-fi page are the social proof. The "first supporter" status is invisible to non-sponsors (GitHub hides count), so the page itself is the proof of seriousness.
- **Product/website integration:** The book library + newsletter on dracon.uk is the deeper funnel.

### Why the top 5 factors work

#### 1. Cross-platform CTA consistency (35/40) — Theo's t3.gg hub
**Why it works:** One link (t3.gg) that routes to all socials simplifies the decision for the viewer. It works when you have multiple channels. Counter-evidence: ThePrimeagen has separate links and still works — so consistency is not necessary if the audience is already familiar.
**How to test for DraconDev:** The current 3-link footer (dracon.uk, YouTube, Sponsor) is the right density. If Ko-fi is added to dracon.uk, the footer should stay at 3 links (dracon.uk, YouTube, Sponsor) — Ko-fi belongs on dracon.uk, not the README.

#### 2. Product/website integration (34/40) — Fireship's newsletter + courses
**Why it works:** Newsletter + courses + website create a deeper funnel. It works when you have products to sell. Counter-evidence: antfu has no product integration and still works for live streams — so product integration is not necessary if the content is educational enough.
**How to test for DraconDev:** The dracon.uk site already has Products / Pricing / Licensing / AI Hub. The next step is to link the book library or a newsletter from the YouTube descriptions.

#### 3. YouTube channel nav links (33/40) — Theo's 5-link nav
**Why it works:** 5 links give the viewer a map of the ecosystem. It works when you have multiple channels. Counter-evidence: fasterthanlime has just Patreon in the nav and still 58K subs — so the nav links are not necessary if the content is strong enough.
**How to test for DraconDev:** The YouTube channel nav should eventually include: dracon.uk, GitHub, Sponsor, Ko-fi, Discord (if active). That's 5 links, matching Theo's pattern.

#### 4. YouTube description hook (31/40) — Fireship's 1-2 line hook
**Why it works:** The hook is the first thing the viewer sees. A good hook turns a casual viewer into a subscriber. It works when the content is strong. Counter-evidence: antfu's description is just the episode title and still works for live streams — so the hook is not necessary if the content is niche enough.
**How to test for DraconDev:** When YouTube becomes active, use a 1-2 line hook that names the specific problem the video solves. E.g., "I built a Rust TUI framework with 43 widgets. Here's what I learned."

#### 5. Content format (31/40) — Fireship's short-form, high-intensity
**Why it works:** Short-form, high-intensity content is optimized for retention. It works when the audience has short attention spans. Counter-evidence: fasterthanlime's long-form deep-dives work too — so format is not the only factor.
**How to test for DraconDev:** For DraconDev, a mix of short-form (shorts) and long-form (deep-dives) is likely best. The short-form should hook, the long-form should retain.

### What the scoring did NOT change about the README

- **Hero line "Hey, I make tools that run themselves."** — Per mitsuhiko's pattern, this is the strong name-drop. Don't change.
- **4-repo pin list (TUI & Terminal)** — Per the "tangible things only" stance and the audit, this is correct. Don't add "Working on" or "Experimental" sections.
- **239K+ lines / 5,600+ tests stat** — concrete numbers, no widgets. Per the original 131-profile research. Don't change.
- **No Ko-fi in README** — Per Fireship/Theo patterns, the 3-link footer is the right density. Adding Ko-fi to the README would dilute.

### Cross-References

- Cross-platform action plan: `CROSS_PLATFORM_ACTION_PLAN.md` (6.4KB, 2026-06-09)
- External platform snippets: `EXTERNAL_PLATFORM_UPDATE_SNIPPETS.md` (3.4KB, 2026-06-09)
- Effectiveness scoring: `EFFECTIVENESS_SCORING.md` (18.6KB, 226 lines)
- YouTube & monetization research: `YOUTUBE_AND_MONETIZATION_RESEARCH.md` (22KB, 2026-06-09)
- GitHub profile research: `GITHUB_PROFILE_RESEARCH.md` — original 131-profile analysis (2026-06-06) + 2026 Refresh section (2026-06-09)
- Publish readiness audit: `GO_NO_GO_REPORT.md` (2026-06-08) — not regressed by the README change
- Verification artifacts: `/tmp/profile-*.png` (5 GitHub), `/tmp/yt-*.png` (5 YouTube), `/tmp/monet-*.png` (6 sponsor/kofi), `/tmp/crates-user-final.png` (crates.io user page)

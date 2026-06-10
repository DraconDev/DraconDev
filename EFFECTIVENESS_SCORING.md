# Effectiveness Scoring: What Works and Why

**Date:** 2026-06-09
**Purpose:** Score the effectiveness of the strategies observed across 11 peers (GitHub profiles, YouTubers, Sponsors accounts, Ko-fi pages) to identify the best-in-class factors and synthesize a combined strategy for DraconDev.

**Methodology:**
- **4 outcome dimensions** (what we're trying to maximize):
  1. **Reach** — how many people see the work (subs/followers/stars/views)
  2. **Engagement** — how much people interact (comments, stars-per-repo, view-through rate, repeat visitors)
  3. **Monetization** — how much money the strategy generates (sponsor count, Patreon, paid sponsors, Ko-fi supporters)
  4. **Cross-platform reinforcement** — how much one channel feeds another (YouTube → GitHub → website → sponsor, etc.)
- **16 input factors** (things you can actually change in your own strategy):
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
- **Scoring scale:** 0-10 per factor per outcome dimension. 0 = absent or counterproductive, 1-3 = weak, 4-6 = moderate, 7-8 = strong, 9-10 = best-in-class.
- **Output:** A 4×16 matrix, best-in-class picks per factor, "why it works" analysis, and a synthesized combined strategy.

**Why factor-based scoring (not peer ranking):**
The goal is not to find the "best" peer. The goal is to find the **best factor from each peer** and combine them into a superior strategy. A peer with 2/4 dimensions excellent is still a source of those 2 dimensions. This is Pareto, not a leaderboard.

---

## 1. The 4×16 Effectiveness Matrix

| # | Factor | Best-in-class instance | Reach | Engagement | Monetization | Cross-platform | Total |
|:-:|:-------|:-----------------------|------:|-----------:|-------------:|---------------:|------:|
| 1 | Hero line / bio name-drop | mitsuhiko ("Creator of the Flask framework. Founder of @earendil-works.") | 8 | 7 | 7 | 5 | **27** |
| 2 | Bio depth / killer stats | dtolnay (600+ words, "68% of crates depend on syn") | 7 | 8 | 10 | 5 | **30** |
| 3 | README presence | yoshuawuyts (8 lines, motto + 3 focus areas) | 5 | 6 | 5 | 4 | **20** |
| 4 | Pinned repo selection | BurntSushi (ripgrep, regex, jiff, aho-corasick, bstr, memchr) | 9 | 8 | 6 | 5 | **28** |
| 5 | Sponsor button presence | All 5 GitHub Sponsors accounts | 6 | 6 | 8 | 5 | **25** |
| 6 | Sponsor page bio quality | dtolnay (600+ words with killer stats) | 7 | 8 | 10 | 5 | **30** |
| 7 | Sponsor page tier structure | All 5 (custom amount + monthly/one-time, no tier names) | 5 | 6 | 8 | 5 | **24** |
| 8 | YouTube description hook | Fireship (1-2 line hook, high-intensity) | 10 | 9 | 7 | 5 | **31** |
| 9 | YouTube description sponsor placement | Theo (sponsor at top, social at bottom) | 8 | 7 | 10 | 5 | **30** |
| 10 | YouTube description social links | ThePrimeagen (Twitch + Discord + boot.dev at top) | 7 | 7 | 8 | 7 | **29** |
| 11 | YouTube channel nav links | Theo (5 links: second channel, third channel, Twitter, Twitch, t3.gg) | 9 | 8 | 7 | 9 | **33** |
| 12 | Content cadence | antfu (weekly live streams) | 6 | 8 | 5 | 6 | **25** |
| 13 | Content format | Fireship (short-form, high-intensity) | 10 | 9 | 7 | 5 | **31** |
| 14 | Cross-platform CTA consistency | Theo (t3.gg hub) | 9 | 8 | 8 | 10 | **35** |
| 15 | Social proof visibility | fasterthanlime (58K subs + Patreon in nav) | 8 | 8 | 8 | 6 | **30** |
| 16 | Product/website integration | Fireship (newsletter + courses + fireship.dev) | 8 | 8 | 10 | 8 | **34** |

**Top 5 factors by total score:**
1. **Cross-platform CTA consistency** (35) — Theo's t3.gg hub
2. **Product/website integration** (34) — Fireship's newsletter + courses
3. **YouTube channel nav links** (33) — Theo's 5-link nav
4. **YouTube description hook** (31) — Fireship's 1-2 line hook
5. **Content format** (31) — Fireship's short-form, high-intensity

**Key insight:** The top 5 factors are all about **funnel architecture**, not individual tactics. The best-performing strategies are the ones that create the strongest cross-platform loop.

---

## 2. Best-in-Class Picks per Factor

### Factor 1: Hero line / bio name-drop
**Best-in-class:** mitsuhiko ("Software developer and Open Source nut. Creator of the Flask framework. Founder of @earendil-works. Other things of interest: @pallets and @getsentry")
**Why it works:** A specific project name gives the visitor a concrete hook. "Creator of the Flask framework" is more memorable than "Python developer" because it anchors your identity to a known artifact. It works when the project is actually well-known. Counter-evidence: dtolnay has no bio at all and still has 9.4K followers — so the hero line is not necessary if the pinned repos are strong enough.
**How to test for DraconDev:** Try "Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests, 24 crates on crates.io." This name-drops the killer work (Rust infrastructure) and the scale (239K LOC).

### Factor 2: Bio depth / killer stats
**Best-in-class:** dtolnay (600+ words, "68% of all published crates on crates.io depend transitively on syn; 46% depend transitively on serde; 49% depend directly on at least one crate by me")
**Why it works:** Stats like "68% of all published crates on crates.io depend transitively on syn" are impossible to ignore. They convert a casual visitor into a serious follower. It works when you have hard numbers to back it up. Counter-evidence: yoshuawuyts has 8 lines and still 3,277 followers — so the bio depth is not necessary if the work is good enough.
**How to test for DraconDev:** The Sponsors page bio could include the same kind of killer stats: "239K+ lines of Rust, 5,600+ tests, 24 crates on crates.io, 12 in-scope repos." These are concrete and verifiable.

### Factor 3: README presence
**Best-in-class:** yoshuawuyts (8 lines, motto + 3 focus areas + ASCII face)
**Why it works:** An 8-line README with a motto is enough to convey personality. It works when the pinned repos already do the heavy lifting. Counter-evidence: dtolnay has no README and still has 9.4K followers — so the README is not necessary if the work speaks for itself.
**How to test for DraconDev:** The current README (15 lines) is in the sweet spot. Don't shorten it to 8 lines; the 4-repo pin list is the right density for "tangible things only."

### Factor 4: Pinned repo selection
**Best-in-class:** BurntSushi (ripgrep, regex, jiff, aho-corasick, bstr, memchr)
**Why it works:** 6 monster repos tell the whole story. It works when your best work is concentrated in a few repos. Counter-evidence: mitsuhiko has too many repos to pin — so the pinned repo selection is not enough if the portfolio is too broad.
**How to test for DraconDev:** The 4-repo pin list (terminal-engine, tiles, folder-auto-banner, obs-wayland-hotkey) is correct. Don't add more until the new research surfaces a stronger candidate.

### Factor 5: Sponsor button presence
**Best-in-class:** All 5 GitHub Sponsors accounts (dtolnay, BurntSushi, mitsuhiko, fasterthanlime, yoshuawuyts)
**Why it works:** The "♥ Sponsor" button is a low-friction conversion mechanism. It works even at 0 sponsors because the button itself is the CTA. Counter-evidence: none — all 5 have it.
**How to test for DraconDev:** Keep the Sponsor link in the README footer. The Sponsors page exists and now has published monthly tiers; bio/goal/featured-work polish is still pending.

### Factor 6: Sponsor page bio quality
**Best-in-class:** dtolnay (600+ words with killer stats)
**Why it works:** The bio does the heavy lifting by explaining the impact of the work. It works when the visitor is already on the sponsor page, meaning they are warm. Counter-evidence: BurntSushi has a short bio and still 106+ sponsors — so the bio quality is not necessary if the work is good enough.
**How to test for DraconDev:** Expand the Sponsors page bio with concrete stats: "239K+ lines of Rust, 5,600+ tests, 24 crates on crates.io, 12 in-scope repos." This mirrors dtolnay's killer-stats approach.

### Factor 7: Sponsor page tier structure
**Best-in-class:** All 5 (custom amount + monthly/one-time, no tier names)
**Why it works:** Custom amount + monthly/one-time toggle removes friction. It works when the visitor is already motivated. Counter-evidence: none — all 5 use it.
**How to test for DraconDev:** Published monthly tiers are now live ($3, $7, $14, $49, $200). Finish the page with a concrete bio, goal, and featured-work section.

### Factor 8: YouTube description hook
**Best-in-class:** Fireship (1-2 line hook, high-intensity)
**Why it works:** The hook is the first thing the viewer sees. A good hook turns a casual viewer into a subscriber. It works when the content is strong. Counter-evidence: antfu's description is just the episode title and still works for live streams — so the hook is not necessary if the content is niche enough.
**How to test for DraconDev:** When YouTube becomes active, use a 1-2 line hook that names the specific problem the video solves. E.g., "I built a Rust TUI framework with 43 widgets. Here's what I learned."

### Factor 9: YouTube description sponsor placement
**Best-in-class:** Theo (sponsor at top, social at bottom)
**Why it works:** Sponsor at the top captures attention before the viewer scrolls. It works when the sponsor is relevant to the audience. Counter-evidence: fasterthanlime doesn't have a sponsor in the description and still 58K subs — so the sponsor placement is not necessary if the content is strong enough.
**How to test for DraconDev:** When YouTube is active, put the sponsor CTA at the top only if there's actually a sponsor. Otherwise, skip it (per antfu).

### Factor 10: YouTube description social links
**Best-in-class:** ThePrimeagen (Twitch + Discord + boot.dev at top)
**Why it works:** 3 links at the top (Twitch, Discord, boot.dev) give the viewer immediate next steps. It works when the audience is already engaged. Counter-evidence: antfu has no sponsor links and still works for live streams — so the social links are not necessary if the content is niche enough.
**How to test for DraconDev:** The README footer already has 3 links (dracon.uk, YouTube, Sponsor). This matches the pattern.

### Factor 11: YouTube channel nav links
**Best-in-class:** Theo (5 links: second channel, third channel, Twitter, Twitch, t3.gg)
**Why it works:** 5 links give the viewer a map of the ecosystem. It works when you have multiple channels. Counter-evidence: fasterthanlime has just Patreon in the nav and still 58K subs — so the nav links are not necessary if the content is strong enough.
**How to test for DraconDev:** The YouTube channel nav should eventually include: dracon.uk, GitHub, Sponsor, Discord (if active). Ko-fi is excluded by default to preserve premium subscription positioning.

### Factor 12: Content cadence
**Best-in-class:** antfu (weekly live streams)
**Why it works:** Weekly live streams create a habit loop. It works when the audience is technical and wants to learn. Counter-evidence: Fireship's weekly shorts work too — so cadence is not the only factor.
**How to test for DraconDev:** If YouTube becomes active, start with weekly live streams or weekly shorts. The cadence matters less than the consistency.

### Factor 13: Content format
**Best-in-class:** Fireship (short-form, high-intensity)
**Why it works:** Short-form, high-intensity content is optimized for retention. It works when the audience has short attention spans. Counter-evidence: fasterthanlime's long-form deep-dives work too — so format is not the only factor.
**How to test for DraconDev:** For DraconDev, a mix of short-form (shorts) and long-form (deep-dives) is likely best. The short-form should hook, the long-form should retain.

### Factor 14: Cross-platform CTA consistency
**Best-in-class:** Theo (t3.gg hub)
**Why it works:** One link (t3.gg) that routes to all socials simplifies the decision for the viewer. It works when you have multiple channels. Counter-evidence: ThePrimeagen has separate links and still works — so consistency is not necessary if the audience is already familiar.
**How to test for DraconDev:** The current 3-link footer (dracon.uk, YouTube, Sponsor) is the right density. If Ko-fi is added to dracon.uk, the footer should stay at 3 links (dracon.uk, YouTube, Sponsor) — Ko-fi belongs on dracon.uk, not the README.

### Factor 15: Social proof visibility
**Best-in-class:** fasterthanlime (58K subs + Patreon in nav)
**Why it works:** Patreon in the nav, 58K subs, and a visible sponsor page all signal that others support the work. It works when the audience is on the fence. Counter-evidence: dtolnay has 126+ sponsors but hides the count — so visibility is not necessary if the work is good enough.
**How to test for DraconDev:** The Sponsor page itself is the social proof. The "Become my first supporter!" badge on Ko-fi is fine at zero sponsors.

### Factor 16: Product/website integration
**Best-in-class:** Fireship (newsletter + courses + fireship.dev)
**Why it works:** Newsletter + courses + website create a deeper funnel. It works when you have products to sell. Counter-evidence: antfu has no product integration and still works for live streams — so product integration is not necessary if the content is educational enough.
**How to test for DraconDev:** The dracon.uk site already has Products / Pricing / Licensing / AI Hub. The next step is to link the book library or a newsletter from the YouTube descriptions.

---

## 3. Synthesized Combined Strategy

The best factors from each peer, combined:

### GitHub Profile
- **Sidebar fields** (from profile funnel pattern): Name `DraconDev`, website `https://dracon.uk`, company `DraconDev`, bio `Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests, 24 crates on crates.io.`
- **Hero line / bio name-drop** (from mitsuhiko): "Rust infrastructure for fleets, git, and terminals. 239K+ LOC, 5,600+ tests, 24 crates on crates.io."
- **Bio depth / killer stats** (from dtolnay): Expand the Sponsors page bio with concrete stats.
- **README presence** (from yoshuawuyts): Keep the 15-line README; don't shorten to 8 lines.
- **Pinned repo selection** (from BurntSushi): Keep the 4-repo pin list; don't add more until a stronger candidate surfaces.
- **Collapsed `<details>` toggles** (from isair): Future-only pattern for hiding secondary categories without cluttering the first screen.
- **Sponsor button presence** (from all 5): Keep the Sponsor link in the README footer.
- **Sponsor page bio quality** (from dtolnay): Expand the Sponsors page bio with concrete stats.
- **Sponsor page tier structure** (from all 5): Custom amount + monthly/one-time, no tier names.

### YouTube
- **Description hook** (from Fireship): 1-2 line hook that names the specific problem the video solves.
- **Sponsor placement** (from Theo): Sponsor at the top only if there's actually a sponsor.
- **Social links** (from ThePrimeagen): 3 links at the top (Twitch, Discord, boot.dev) — adapt to dracon.uk, GitHub, Sponsor.
- **Channel nav links** (from Theo): 4 primary links for premium positioning (dracon.uk, GitHub, Sponsor, Discord if active). Ko-fi is optional only if selling small-ticket products.
- **Content cadence** (from antfu): Weekly live streams or weekly shorts.
- **Content format** (from Fireship): Mix of short-form (shorts) and long-form (deep-dives).
- **Cross-platform CTA consistency** (from Theo): One link to rule them all (t3.gg) — adapt to dracon.uk as the hub.
- **Social proof visibility** (from fasterthanlime): Patreon in nav + 58K subs + sponsor page — adapt to Sponsor + Ko-fi + 58K subs equivalent.
- **Product/website integration** (from Fireship): Newsletter + courses + fireship.dev — adapt to book library + newsletter on dracon.uk.

### Ko-fi
- **Product/website integration** (from Fireship): Shop tab + tip amounts.
- **Social proof visibility** (from jam1garner): Live transaction feed.
- **Cross-platform CTA consistency** (from Theo): One link to rule them all.

### Utilities monorepo
- **Simple parent + distinct components** (from `dracon-utilities` audit): Keep `dracon-utilities` as the parent repo, but make `dracon-sync`, `dracon-system`, and `dracon-warden` distinct through separate READMEs, config examples, service/hook docs, and profile subpath links.

### Summary
The most effective strategy is not to copy any single peer. It's to **combine the best factor from each peer** into a coherent funnel:
1. **GitHub profile bio** name-drops the killer work (mitsuhiko)
2. **Sponsors page bio** uses killer stats (dtolnay)
3. **README** keeps the 15-line, 4-repo pin list (yoshuawuyts + BurntSushi); future secondary categories can use collapsed `<details>` toggles (isair)
4. **YouTube description** uses the Fireship 5+1 structure with sponsor at top (Fireship + Theo)
5. **YouTube channel nav** has 5 links (Theo)
6. **Content cadence** is weekly (antfu)
7. **Content format** is a mix of short-form and long-form (Fireship)
8. **Cross-platform CTA consistency** is one hub (dracon.uk) (Theo)
9. **Social proof visibility** is the Sponsor page + Ko-fi (fasterthanlime + jam1garner)
10. **Product/website integration** is the book library + newsletter on dracon.uk (Fireship)

This is the combined strategy. It takes the best factor from each peer and synthesizes them into a single, coherent funnel.

---

## 4. Verification Artifacts

- `/tmp/profile-data.json` — full bio, pinned repos, README status, line counts for all 5 GitHub profiles
- `/tmp/youtube-data.json` — channel metadata and video descriptions for 5 YouTubers
- `/tmp/monet-data.json` — full text dumps of all 10 attempted sponsor/Ko-fi pages
- `/tmp/profile-*.png` — 5 GitHub profile screenshots
- `/tmp/yt-*.png` — 6 YouTube screenshots (3 videos-tab + 3 video-page)
- `/tmp/monet-*.png` — 9 sponsor/Ko-fi screenshots
- `/tmp/crates-*.png` — 2 crates.io screenshots
- `/tmp/yt-rss/*.xml` — 8 RSS XML files (ThePrimeagen and Fireship, 15 entries each)

---

## 5. Summary

**Key findings:**
1. The most effective strategies are not about any single tactic. They're about **funnel architecture** — how one channel feeds another.
2. The top 5 factors by total score are all about funnel architecture: cross-platform CTA consistency, product/website integration, YouTube channel nav links, YouTube description hook, and content format.
3. The best strategy is a **combination** of the best factors from each peer, not a copy of any single peer.
4. DraconDev already has most of the right pieces in place. The missing piece is **volume** — getting more content into the funnel.
5. The README is already optimal for the "tangible things only" stance. The best next changes are:
   - Expand the Sponsors page bio with concrete stats (dtolnay pattern)
   - Add Ko-fi to dracon.uk (not README) as optional low-friction support
   - Do not add Ko-fi to YouTube descriptions by default while the strategy is premium subscriptions
   - Add a GitHub profile bio name-drop (mitsuhiko pattern)
   - Do not add WIP `DraconDev/dracon-platform` to the profile README or pin list until it has public, tangible value
   - Do not add `DraconDev/dracon-utilities` links to the profile README until the parent monorepo is public, clean, and verified
   - Do not expand the README into a long portfolio; use future-only collapsed `<details>` toggles only if secondary work becomes tangible

**Next actionable step:** Use `CROSS_PLATFORM_ACTION_PLAN.md` as the execution checklist, then use `EXTERNAL_PLATFORM_UPDATE_SNIPPETS.md` for the exact copy/templates when the relevant platform access is available.


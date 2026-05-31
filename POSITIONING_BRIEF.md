# DraconDev — Positioning Brief (Updated)
**Goal:** Turn the GitHub profile into a revenue-generating sales funnel.

> *Last updated from user Q&A — see §6 for confirmed answers, §7 for open decisions.*
> *Extended research: 20+ profiles analyzed (web search + GitHub API + direct fetch). 0xNyk identified as closest competitor. See §1 for full analysis.*
> *⚠️ Open: Reconcile with SOVEREIGN_COMMERCIAL_MODEL.md (AGPL+CLA tier model vs GitHub Sponsors+Open Core recommendation — see §7, Decision 5).*
> *User approved moving to README work (goal conversation, 2026-05-31). Open decisions deferred to README iteration phase.*

---

## 1. Competitive Analysis

### steipete (Peter Steinberger)
- **Business model:** OpenClaw (370k stars) is free and open. Revenue via consulting, speaking, TED talk visibility. No hard CTA.
- **Strategy:** Credibility play. Massive star count = authority. Products mentioned but not sold — it's a reputation profile.
- **What converts:** The sheer volume signals competence. Visitors don't buy — they remember him.
- **CTA:** Soft. "Check out X", "Follow on Y". No hard sell.

### t3dotgg (Theo Browne)
- **Business model:** Products-first. T3 Chat (paid), UploadThing (paid), T3 Code (paid harness). YouTube + Twitter drive traffic to profile, profile drives to products.
- **Strategy:** Content + product flywheel. Builds in public, documents on video, products emerge from traction.
- **What converts:** Products section front-and-center. "Everything Else" is the open source credibility layer.
- **CTA:** "Check out T3 Chat" + YouTube/Twitter links.

### Key patterns across both:
1. **GitHub profile is a hub, not a destination.** Traffic comes from YouTube, Twitter, HN. README converts visitors into followers or customers.
2. **Hero statement is critical.** Name + role + current focus.
3. **No hard sell.** Let the work speak. CTAs are soft.
4. **Open source builds credibility, products pay the bills.** Mix is ~70% open source, ~30% commercial.

### Additional Profiles Analyzed (20+ total — web search + GitHub API + direct fetch)

**emilk (Emil Ernerfeldt)** — Creator of egui (29k★), co-founder of Rerun (10.9k★). Strategy: product-first with company backing. No personal CTA — products do the talking. Bluesky for news. Position: developer tooling with commercial company support.

**tyt2y3 (Chris Tsang)** — Founder of SeaQL, maintainer of SeaORM (9.5k★), VTracer (5.7k★). Strategy: indie hacker, open source as business foundation. GitHub Sponsors active. Position: database tooling for Rust developers.

**yyx990803 (Evan You)** — Creator of Vue.js (210k★), Vite (80k★), Rolldown (13.6k★, Rust). Strategy: massive open source presence → GitHub Sponsors + patronage. The ecosystem IS the product. CTA: GitHub Sponsors. Position: frontend framework ecosystem maintainer.

**raysan5 (Ray San)** — Creator of raylib (33.2k★), raygui (4.9k★). Strategy: "simple and easy-to-use" positioning, community-driven with Discord, sponsors page, GitHub Sponsors. raylibtech.com for commercial tools. Position: game dev library with ecosystem + paid tools + community sponsors.

**mitsuhiko (Armin Ronacher)** — Creator of Flask (71.6k★), author of many Python/Rust tools. Strategy: minimal personal profile, products do the work. Also ships MiniJinja (Rust, 2.6k★) and agent-stuff (2.6k★). Position: legendary open source maintainer, credibility via products.

**anomalyco/opencode** — Creator of OpenCode (167k★), the open source coding agent. Strategy: enterprise positioning, desktop app (DMG/EXE/AppImage), multi-language README (20+ languages). opencode.ai for the product page, Discord for community. Position: enterprise AI coding agent, open core model.

**Kuberwastaken** — Builder of Claurst (9.7k★), a Rust TUI coding agent. Strategy: creative personal brand, "ADHD-pilled" personality, fun GitHub profile, personal site (kuber.studio), builds in public. Position: indie hacker / student, building in public with personality.

**Belkins (Vlad Podoliako)** — Minimal GitHub profile (1 pinned repo, 476★). The pinned repo is an interactive 43-chapter field manual on AI coding agents. Strategy: unknown from profile alone — external brand drives the funnel. ⚠️ Profile too thin to derive strategy from.

**sorairolake** — Rust developer with qrtool (281★), abcrypt (25★), sysexits-rs (32★). Strategy: niche utility builder, consistent Rust focus, multi-language projects. Position: niche Rust developer with specialized tools in specific domains.

**nicholasjackson** — Developer Advocate at HashiCorp, author of "Building Microservices in Go" (1.2k★). YouTube-driven traffic, pinned course repo. Position: technical author + developer advocate, community + book as the funnel.

**kas-gui** — Rust GUI toolkit (kas, 1k★). Strategy: minimal personal presence, clean pinned repos, focused on one domain. Position: domain expert in Rust GUI.

**seyfallah** — GitHub Star, popular content creator. YouTube + Discord community. Strategy: community-building as conversion path. Content-first, repos as proof points.

**blyxyas** — GitHub Star, Rust tooling contributor (oxc, cargo). Profile emphasizes contribution graph + community involvement.

**nicknisi** — DevEx at WorkOS. Strategy: "What I'm Building" section, current role + AI exploration, community organizer (NebraskaJS, NEJS Conf, TSConf). Personal site + Bluesky + Mastodon. Position: developer advocate with community credentials.

**0xNyk (nyk)** — ⭐ Most relevant competitor for DraconDev. "I build operator systems for AI agents." Founder at splitlabs, Co-Founder at Builderz. Pinned: Mission Control (5.1k★) — agent fleet dashboard with task orchestration, quality gates, cost tracking. LACP (Local Agent Control Plane) — policy gates, 5-layer memory, hook pipeline for Claude/Codex/Hermes. Strategy: open source fleet infrastructure → drives to splitlabs + builderz commercial services. CTA: "If you are building agent infrastructure — builderz.dev." Position: operator systems for AI agents. This is DraconDev's closest direct competitor.

**seanmonstar (Sean McArthur)** — Maintainer of reqwest (11.6k★), warp (10k★), hyper (7.8k★). 3.3k followers. Strategy: minimal profile, products do the work. Position: Rust networking infrastructure expert.

**dtolnay (David Tolnay)** — 9.4k followers. Maintainer of anyhow, serde, cargo-expand, many Rust toolchain essentials. Strategy: zero personal branding, pure technical impact. GitHub Sponsors active. Position: Rust toolchain infrastructure legend.

**jonhoo (Jon Gjengset)** — 14k followers. Rust educator, YouTube streamer (Hundreds of videos). At Helsing AI, previously AWS. Strategy: educational content as the funnel, consulting + GitHub Sponsors. Position: Rust educator with enterprise credibility.

**fasterthanlime (Amos Wenger)** — 4.7k followers. Educator, video maker, software mercenary. Co-host of a self-directed research podcast. Strategy: content + personality → GitHub Sponsors. Position: developer educator with strong personal brand.

**sindresorhus (Sindre Sorhus)** — 79k followers. Full-time open sourcerer, Swift + JavaScript. Makes macOS apps, CLI tools, npm packages. type-fest (17k★), ky (16.9k★), awesome list (472k★). Strategy: massive ecosystem + GitHub Sponsors. Position: legendary OSS prolific maintainer.

**tiangolo (Sebastián Ramírez)** — 31k followers. Creator of FastAPI (76k★), Typer, SQLModel. Based in Germany from Colombia. Strategy: FastAPI as the flagship → FastAPI Cloud (commercial) + GitHub Sponsors. Position: Python API framework creator with commercial cloud product.

**antfu (Anthony Fu)** — 39k followers. Vue/Nuxt contributor, creator of Vitesse. At voidzero.dev (Vue team, Vite/Rollup core). Strategy: OSS contributor + company backing + GitHub Sponsors. Position: Vue ecosystem core contributor.

**Key insight — 0xNyk is the north star:** His profile is the clearest example of "DraconDev's angle" in the wild. Fleet dashboard + policy gates + local agent control plane = exactly what DraconDev is building (auto-ai-agent, Tauri GUI, blueprint-driven). The differentiation is that 0xNyk uses TypeScript and web-stack; DraconDev uses Rust + Tauri + SolidJS. The B2B positioning (enterprise + ops, not indie dev) is identical.

### Positioning Archetypes (from 20+ profiles)

| Archetype | Who it fits | Key example |
|:---------|:-----------|:------------|
| **Operator systems builder** ⭐ | Fleet orchestration, policy gates, agent ops | 0xNyk, anomalyco/opencode |
| **Product-first** | Clear commercial products to sell | t3dotgg, raysan5, tiangolo |
| **Massive credibility** | High star count, many repos | sindresorhus, yyx990803, steipete |
| **Building in public** | Personality, narrative, community | Kuberwastaken, fasterthanlime, nicknisi |
| **Niche utility builder** | Focused domain, specialized tools | sorairolake, kas-gui, tyt2y3, dtolnay |
| **Technical author** | Books, courses, YouTube | nicholasjackson, jonhoo |
| **Minimal** | Products speak for themselves | mitsuhiko, emilk, seanmonstar |

**Universal patterns across ALL 20+ profiles:**
1. **Pinned repos are the main story** — not a flat list. Every profile uses pinning strategically.
2. **Social links are universal** — Twitter, Bluesky, YouTube, Discord all drive traffic back to the profile.
3. **GitHub Sponsors is the default CTA** — lowest-friction monetization for every tier.
4. **Hero statement = name + role + current focus** — immediate context for every top profile.
5. **No hard sell** — the work speaks. CTAs are soft: "Star if useful", "Follow for updates", "Check out X".
6. **Open source builds credibility, products/services pay** — the mix varies, but the dynamic is universal.
7. **Personal site or blog is the conversion destination** — GitHub is the hub, external site is the destination.
8. **B2B angle emerging** — 0xNyk, anomalyco, tiangolo all target teams/enterprise, not indie hackers.
9. **Fleet/ops positioning** — The strongest AI coding agent profiles (0xNyk, anomalyco/opencode) lead with fleet management and operations, not "coding agent" as a chatbot.

---

## 2. Market Opportunity

### AI Coding Agent Market — Why This Is Relevant
- **$8.5B–$12.8B** market (early 2026), projected to hit **$47B–$91B** by 2034.
- **37–46% CAGR** through 2030.
- Key drivers: developer shortages, productivity gains of 20–55%.
- **Caveat:** Space dominated by Copilot, Cursor, Claude Code (>70% market). Differentiation is hard.

### The Niche DraconDev Can Own
Unique angles most competitors don't have:
- **Blueprint-driven** (not chat-driven) — write the objective, it executes.
- **Desktop GUI** (Tauri + SolidJS) — supervision layer, not just CLI.
- **Rust core** — performance, safety, no electron bloat.
- **Multi-repository management** — fleet-wide view.

Position: **"AI agents for engineers who want control, not magic."**

### GitHub Trending Demand Signal (live query, 2025)

**Rust ecosystem top repos by stars (500+ stars, pushed 2025):**
| Repo | Stars | What it signals |
|:-----|:------|:----------------|
| ultraworkers/claw-code | 193k★ | AI coding agents massive demand (193k in months) |
| rustdesk/rustdesk | 115k★ | Self-hosting / privacy demand |
| tauri-apps/tauri | 107k★ | Tauri is mainstream, growing fast |
| openai/codex | 87k★ | Terminal-based AI agents are hot |
| farion1231/cc-switch | 86k★ | Cross-platform AI agent CLI tooling (desktop assistant) |
| astral-sh/uv | 85k★ | Rust-based developer tools have huge adoption |
| zed-industries/zed | 84k★ | AI-native editors + Rust = strong signal |

**TypeScript ecosystem — AI agent platforms:**
| Repo | Stars | What it signals |
|:-----|:------|:----------------|
| openclaw/openclaw | 375k★ | OpenClaw is massive; agent ecosystem is thriving |
| anomalyco/opencode | 167k★ | OpenCode is the biggest open coding agent |
| langgenius/dify | 143k★ | Agentic workflow platforms are hot |

**Key insight:** The Rust + AI agent combination (claw-code, codex, cc-switch, zed, tauri) is a high-demand intersection. DraconDev sits at this intersection with Rust core + AI agents + Tauri desktop GUI. The demand signal is strong.

### Open Source Monetization Landscape
| Model | Best For DraconDev? |
|-------|:-------------------|
| **Open Core** | ✅ Strong fit. Free CLI/GUI core + paid enterprise features (SSO, team management, compliance). |
| **GitHub Sponsors** | ✅ Low effort. Sponsorware model: features first to sponsors, then public. |
| **Dual Licensing (AGPL + Commercial)** | ✅ Already in docs. Clean path. |
| **SaaS/Managed** | ⚠️ Later stage. Requires ops infrastructure. |

---

## 3. Product-Market Fit Hypothesis

### The Core Insight
Most AI coding agents target individual developers who want to move fast. DraconDev's target should be **B2B** — companies, not broke indie hackers.

Target: **small engineering teams (2–5 engineers)** at companies who want to run autonomous coding agents without losing visibility and control.

### What B2B Customers Pay For
| Problem | DraconDev's Solution |
|:--------|:---------------------|
| "Claude Code runs and I walk away — is it done?" | Fleet Hub + live gate results |
| "Chat-based agents are flaky and ungovernable" | Blueprint-first, verifiers, checkpoints |
| "I want to run multiple agents across repos" | Multi-project management in one GUI |
| "CLI agents are invisible to stakeholders" | Dark desktop GUI, desktop notifications |
| "We need audit logs and compliance" | Open Core enterprise tier |

### Tagline Candidates
- "AI agents that execute blueprints, not prompts."
- "Autonomous engineering for teams who value control."
- "Your engineering runtime. Not a chatbot — a crew."

---

## 4. Positioning Options

### Option A: "The Autonomous Engineering Runtime" — Product Lead
**Hero:** "Dracon — AI agents that execute blueprints, not prompts."

**Structure:**
1. **Products** (commercial): auto-ai-agent, ai-app-engine, ai-web-agent
2. **Open Source** (credibility): obs-wayland-hotkey, git-ai-committer, git-seal, azumi, etc.

**Who it's for:** People who see Dracon as a product first, open source as proof.

**Pros:** Strongest revenue funnel. Clean commercial story.
**Cons:** Requires making private products public. Risk of premature exposure.

**CTA:** "Star the repo → get early access to the desktop GUI."

---

### Option B: "The Indie Developer Toolkit" — Audience Lead ⭐ Recommended
**Hero:** "Dracon — performance-first tools for developers who ship."

**Structure:**
1. **Flagship** (open source, traction): obs-wayland-hotkey, git-ai-committer
2. **Tools** (open source, growing): azumi, tiles, terma, opencode plugins
3. **Products** (private, sidebar link): auto-ai-agent, ai-app-engine

**Who it's for:** Developers who discover Dracon through a tool, then explore the ecosystem.

**Pros:** No need to make anything public prematurely. Builds audience first. Funnels to dracon.uk.
**Cons:** Softer commercial story. Products are harder to find.

**CTA:** "Follow for Rust/Tauri/AI tools" + sidebar "My paid products →" + GitHub Sponsor button.

---

### Option C: "The Blueprint Agent" — Differentiation Lead
**Hero:** "Dracon — the AI agent you actually control. Blueprint-first, not chat-first."

**Structure:**
1. **One flagship**: auto-ai-agent (make public). The whole story is about it.
2. **Everything Else**: open source tools as ecosystem context.

**Who it's for:** Developers burned by chat-driven agents who want something more structured.

**Pros:** Clear differentiation in a crowded space. One story, not many.
**Cons:** All eggs in one basket. If auto-ai-agent doesn't land, profile is thin.

**CTA:** "Watch the demo → join the waitlist → get early access."

---

## 5. Recommended Path (Based on User Feedback)

### Phase 1: Build the Credibility Layer — NOW
Keep the profile focused on **open source tools with traction** (obs-wayland-hotkey 8★, git-ai-committer 6★). Build the audience before selling.

**README structure (near-term):**
- Hero: "DraconDev / performance-first tools. Rust, Tauri, AI agents."
- Link: [dracon.uk](https://dracon.uk)
- Section 1: "Start Here" — top 2-3 open source projects
- Section 2: "Everything Else" — full list of open source repos
- CTA: GitHub Sponsor button + "More on dracon.uk →"
- Products: mentioned in a "By the way" tone, linked to dracon.uk

### Phase 2: Open the Flagship (When Ready)
Once auto-ai-agent is polished:
- Make it public with a polished README + demo GIF.
- Lead with it. Everything else is context.
- Use the "Start Here" pattern (steipete style).

### Phase 3: Add the Commercial Layer
Once there's a following:
- Open Core: free self-hosted, paid enterprise (SSO, team management, audit logs).
- GitHub Sponsors with sponsorware benefits.
- Polar.sh for merchant-of-record simplicity.

### CTA Strategy
- **Now:** Soft CTA. "Star if you find this useful. More on dracon.uk."
- **Products page (dracon.uk):** Harder CTA. "Book a demo", "Get early access."
- **Later:** Product CTAs on GitHub, sponsorware model.

---

## 6. Confirmed Decisions (from Q&A)

| Question | Answer |
|:---------|:-------|
| Which repos to commercialize? | Not sure yet — needs discovery. |
| Make auto-ai-agent public now? | No — open source it, but not ready yet (needs tweaks). |
| Target customer? | B2B — small engineering teams (2–5 engineers). Indie hackers are "likely broke." |
| Monetization plan? | GitHub Sponsors + selling products/services + funnel to dracon.uk. |
| Website/social? | [dracon.uk](https://dracon.uk) |

---

## 7. Open Decisions (Need User Input)

### Decision 1: Which repos are the commercial products?
You said "not sure yet — let's figure it out first." I need to know:
- Is it just **auto-ai-agent**?
- Is it **auto-ai-agent + ai-app-engine**?
- Or something else?

These are the repos that should appear in a "Products" section or link to from dracon.uk.

### Decision 2: What exactly is the B2B positioning?
You said "aim for B2B." I can position the profile for:
- **Engineering teams who want to scale autonomous coding agents** (technical buyers, CTOs, tech leads)
- **Enterprises who want AI agents they can audit and govern** (compliance officers, security teams)
- **Both**

This changes the language throughout — "teams" vs "enterprise" vs "developers."

### Decision 3: What is the actual product to sell?
Right now, the private repos are internal tooling. Before we can sell anything, we need to know:
- What do you want to charge for?
- What's the minimum viable commercial product?
- Is it: SaaS? License? Subscription? Consulting?

### Decision 4: What's the content strategy?
t3dotgg drives traffic via YouTube. steipete drives via his blog and TED talk. How do you plan to drive traffic to your GitHub profile?
- Blog posts on dracon.uk?
- YouTube videos?
- Hacker News posts?
- Nothing yet — just build?

### Decision 5: Reconcile the commercial model
`SOVEREIGN_COMMERCIAL_MODEL.md` documents a dual-licensing model (AGPLv3 + CLA, Indie/Startup/Scale tiers, Steam/Nexus as billboard). The positioning brief recommends GitHub Sponsors + Open Core. These aren't mutually exclusive — the dual licensing can be the commercial layer while GitHub Sponsors is the indie layer. But we need to align on one coherent model or document both as complementary.

---

## 8. Near-Term README Plan

Based on confirmed decisions, here's the immediate action plan:

### Update the README (this week)
1. Clean up the existing `_README.md` format to match t3dotgg/steipete style
2. Add dracon.uk link
3. Add GitHub Sponsor button (if enabled)
4. Reorganize sections: "Start Here" → "Open Source" → "Products" (placeholder, linked to dracon.uk)
5. Add a "Made with Rust, Tauri & SolidJS" badge section for tech stack signal

### Next Steps
1. **Finalize commercial products** — Which repos? What's the story?
2. **Polish auto-ai-agent** — Open source it when ready, then lead the profile with it.
3. **Set up dracon.uk** — Needs a products page, even if it's just "coming soon."
4. **Enable GitHub Sponsors** — Low effort, validates the model early.
5. **Content flywheel** — Blog posts, HN posts, YouTube. Drive traffic to the profile.

---

*This brief is a working document. It's meant to be discussed, challenged, and refined.*

---

## Public Repos — All 93 (by Stars)

| Stars | GitHub Repo | Status |
|:------|:-----------|:-------|
| 8★ | [obs-wayland-hotkey](https://github.com/DraconDev/obs-wayland-hotkey) | ✅ Featured in README |
| 6★ | [git-ai-committer](https://github.com/DraconDev/git-ai-committer) | ✅ Featured in README |
| 2★ | [tiles-tui-file-manager](https://github.com/DraconDev/tiles-tui-file-manager) | ✅ Featured in README |
| 2★ | [git-seal](https://github.com/DraconDev/git-seal) | ✅ Featured in README |
| 2★ | [chrome-auto-fullscreen](https://github.com/DraconDev/chrome-auto-fullscreen) | ✅ Featured in README |
| 1★ | [dracon-terminal-engine](https://github.com/DraconDev/dracon-terminal-engine) | ✅ Featured in README |
| 1★ | [opencode-auto-review-completed](https://github.com/DraconDev/opencode-auto-review-completed) | ⚠️ Name mismatch (local: opencode-todo-review) |
| 1★ | [youtube-video-uploader](https://github.com/DraconDev/youtube-video-uploader) | ✅ Featured in README |
| 1★ | [opencode-auto-continue](https://github.com/DraconDev/opencode-auto-continue) | ✅ Featured in README |
| 1★ | [dracon-libs](https://github.com/DraconDev/dracon-libs) | ✅ Featured in README |
| 1★ | [azumi-live-ssr-framework](https://github.com/DraconDev/azumi-live-ssr-framework) | ⚠️ Name mismatch (local: azumi) |
| 1★ | [ai-gui-auto-video-editor](https://github.com/DraconDev/ai-gui-auto-video-editor) | ⚠️ Name mismatch (local: agave) |
| 1★ | [api-debugger](https://github.com/DraconDev/api-debugger) | ✅ Featured in README |
| 1★ | [terma](https://github.com/DraconDev/terma) | ✅ Featured in README (push needed) |
| 1★ | [css-peek-pro](https://github.com/DraconDev/css-peek-pro) | ✅ Featured in README |
| 0★ | [DraconDev](https://github.com/DraconDev/DraconDev) (profile repo) | ✅ Source of truth |
| 0★ | [volume-and-video-pro](https://github.com/DraconDev/volume-and-video-pro) | ✅ Featured in README |
| 0★ | [pi-auto-review](https://github.com/DraconDev/pi-auto-review) | ✅ Featured in README (Pi-specific) |
| 0★ | [kittentts-showcase](https://github.com/DraconDev/kittentts-showcase) | ❌ Graveyard (one-off showcase) |
| 0★ | [opencode-auto-force-resume](https://github.com/DraconDev/opencode-auto-force-resume) | ❌ Graveyard (superseded by auto-continue) |
| 0★ | [respec-spec-reconciler](https://github.com/DraconDev/respec-spec-reconciler) | ✅ Featured in README |
| 0★ | [stripe-payment-with-nextjs](https://github.com/DraconDev/stripe-payment-with-nextjs) | ❌ Graveyard (abandoned experiment) |

## Name Mismatches to Fix

| GitHub Name | Local Name | Should Be | Action |
|:-----------|:-----------|:-----------|:-------|
| opencode-auto-review-completed | opencode-todo-review | opencode-todo-review | Rename GitHub repo |
| azumi-live-ssr-framework | azumi | azumi | Rename GitHub repo |
| ai-gui-auto-video-editor | agave | agave | Rename GitHub repo |
| tiles-tui-file-manager | tiles | tiles | Rename locally + GitHub |

## Repos Not Yet Pushed

| Local Name | Notes | Action |
|:-----------|:------|:-------|
| terma | Strong next-gen terminal compositor engine. 0★ on GitHub (not pushed). | Push to GitHub |
| azumi | Live SSR framework, ~10KB gzipped | Rename GitHub repo first |
| agave | AI video editor (AGAVE product name) | Rename GitHub repo first |

## Graveyard (Not Featured in README)

See [GRAVEYARD.md](./GRAVEYARD.md) for full list. 65+ repos archived: abandoned experiments, one-off demos, learning exercises. Not deleted — preserved for reference.

---
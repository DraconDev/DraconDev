# DraconDev — Positioning Brief (Updated)
**Goal:** Turn the GitHub profile into a revenue-generating sales funnel.

> *Last updated from user Q&A — see §6 for confirmed answers, §7 for open decisions.*

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

## Appendix: Current Public Repos by Traction

| Repo | Stars | Notes |
|:-----|:------|:------|
| obs-wayland-hotkey | 8★ | Flagship. Rust daemon, OBS hotkeys. Ready to feature. |
| git-ai-committer | 6★ | Flagship. VS Code extension, AI commits. Ready to feature. |
| azumi | 1★ | Live SSR for Rust. Strong product story. Needs better name. |
| tiles | 1★ | TUI file manager. Strong tech. Name too opaque. |
| git-seal | 2★ | Git encryption. Niche but solid. |
| chrome-auto-fullscreen | 2★ | Browser extension. Simple, clear. |
| api-debugger | 1★ | Chrome extension. Not cloned locally. Clone it. |
| opencode plugins | 1★ | Auto-continue, auto-review. Niche but growing. |
| dracon-libs | 1★ | Library collection. Boring but important. |
| terminal-engine | 1★ | TUI framework. Strong. |
| terma | 0★ | Not pushed to GitHub. Push it — it's strong. |
| agave | 1★ | AI video editor. Strong story but needs polish. |

**Private repos with commercial potential:**
| Repo | What it is | Commercial? |
|:-----|:-----------|:-------------|
| auto-ai-agent | Autonomous engineering runtime | ✅ Flagship, not ready yet |
| ai-app-engine | ECS-based GPU UI engine | ✅ Strong, niche |
| ai-web-agent | Browser extension AI | ✅ Solid, Chrome extension market |
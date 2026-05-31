# DraconDev — Positioning Brief
**Goal:** Turn the GitHub profile into a revenue-generating sales funnel.

---

## 1. Competitive Analysis

### steipete (Peter Steinberger)
- **Business model:** OpenClaw (370k stars) is free and open. Revenue is implied via consulting, speaking, and his TED talk visibility. No hard CTA.
- **Strategy:** Credibility play. Massive star count = authority. Products mentioned but not sold — it's a reputation profile.
- **What converts:** The sheer volume of projects signals competence. Visitors don't buy here — they remember him.

### t3dotgg (Theo Browne)
- **Business model:** Products-first. T3 Chat (paid), UploadThing (paid), T3 Code (paid harness). YouTube + Twitter drive traffic to the profile, profile drives to products.
- **Strategy:** Content + product flywheel. Builds in public, documents on video, products emerge from what gets traction.
- **What converts:** Products section is front-and-center. "Everything Else" is the open source credibility layer.

### Key patterns across both:
1. **GitHub profile is a hub, not a destination.** Traffic comes from YouTube, Twitter, HN. The README converts visitors into followers or customers.
2. **Hero statement is critical.** Both lead with name + role + current focus.
3. **No hard sell.** The best profiles let the work speak. CTAs are soft: "check out X", "follow on Y".
4. **Open source builds credibility, products pay the bills.** The mix is ~70% open source, ~30% commercial.

---

## 2. Market Opportunity

### AI Coding Agent Market — Why This Is Relevant
- **$8.5B–$12.8B** market (early 2026), projected to hit **$47B–$91B** by 2034.
- **37–46% CAGR** through 2030.
- Key drivers: developer shortages, productivity gains of 20–55%.
- **However:** The space is dominated by Copilot, Cursor, Claude Code (>70% market). Differentiation is hard.

### The Niche DraconDev Can Own
DraconDev has a unique angle that most AI coding agent projects don't:
- **Blueprint-driven** (not chat-driven) — write the objective, it executes.
- **Desktop GUI** (Tauri + SolidJS) — supervision layer, not just CLI.
- **Rust core** — performance, safety, no electron bloat.
- **Multi-repository management** — fleet-wide view.

This is a specific position: **"AI agents for engineers who want control, not magic."**

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
Most AI coding agents target individual developers who want to move fast. DraconDev's target is different — **engineers who want to scale their supervision**.

The tagline writes itself: **"Your engineering runtime. Not a chatbot — a crew."**

### What People Pay For
| Problem | DraconDev's Solution |
|:--------|:---------------------|
| "Claude Code runs and I walk away — is it done?" | Fleet Hub + live gate results |
| "Chat-based agents are flaky" | Blueprint-first, verifiers, checkpoints |
| "I want to run multiple agents across repos" | Multi-project management in one GUI |
| "CLI agents are invisible" | Dark desktop GUI, desktop notifications |

### Target Customer
**Solo indie hacker or small team** who builds with Rust/AI and wants to run multiple autonomous coding agents without losing visibility. Pays for: team features, managed deployment, priority support.

---

## 4. Positioning Options

### Option A: "The Autonomous Engineering Runtime" — Product Lead
**Hero:** "Dracon — AI agents that execute blueprints, not prompts."

**Structure:**
1. **Products** (commercial): auto-ai-agent (flagship), ai-app-engine, ai-web-agent
2. **Open Source** (credibility): obs-wayland-hotkey, git-ai-committer, git-seal, azumi, etc.

**Who it's for:** People who see Dracon as a product first, open source as proof.

**Pros:** Strongest revenue funnel. Clean commercial story.
**Cons:** Requires making private products public. Risk of premature exposure.

**CTA:** "Star the repo → get early access to the desktop GUI."

---

### Option B: "The Indie Developer Toolkit" — Audience Lead
**Hero:** "Dracon — performance-first tools for developers who ship."

**Structure:**
1. **Flagship** (open source, high stars): obs-wayland-hotkey, git-ai-committer
2. **Tools** (open source, growing): azumi, tiles, terma, opencode plugins
3. **Products** (private, sidebar link): auto-ai-agent, ai-app-engine

**Who it's for:** Developers who discover Dracon through a tool, then explore the ecosystem.

**Pros:** No need to make anything public prematurely. Builds audience first.
**Cons:** Softer commercial story. Products are harder to find.

**CTA:** "Follow for Rust/Tauri/AI tools" + sidebar "My paid products →"

---

### Option C: "The Blueprint Agent" — Differentiation Lead
**Hero:** "Dracon — the AI agent you actually control. Blueprint-first, not chat-first."

**Structure:**
1. **One flagship**: auto-ai-agent (make public). The whole story is about it.
2. **Everything Else**: open source tools as ecosystem context.

**Who it's for:** Developers burned by chat-driven agents who want something more structured.

**Pros:** Clear differentiation in a crowded space. One story, not many.
**Cons:** All eggs in one basket. If auto-ai-agent doesn't land, the profile is thin.

**CTA:** "Watch the demo → join the waitlist → get early access."

---

## 5. Recommended Path

### Phase 1: Build the Credibility Layer (Now)
Keep the profile focused on **open source tools with traction** (obs-wayland-hotkey, git-ai-committer). This is the credibility foundation. No hard CTA — just "star, fork, try it."

### Phase 2: Open the Flagship (After Stars Accumulate)
When auto-ai-agent is ready for public eyes:
- Make it public with a polished README + demo GIF.
- Lead with it. Everything else is context.
- Use the "Start Here" pattern (steipete style) — top 2-3 projects, then deep sections.

### Phase 3: Add the Commercial Layer
Once there's a following:
- Open Core model: free self-hosted, paid managed/enterprise.
- GitHub Sponsors with sponsorware benefits.
- Polar.sh for merchant-of-record simplicity.

### CTA Strategy
For now: **soft CTA only**. "Star if you find this useful. Follow for updates."
When flagship goes public: **product CTA**. "Try the desktop GUI →" or "Join the waitlist →"

---

## 6. README Structure (Near-Term Recommendation)

Based on t3dotgg inspiration + DraconDev's current state:

```
# DraconDev
Performance-first tools. Rust core, AI agents, Tauri desktop.

[links: Twitter, website, blog]

## 🔥 Products (or "Start Here")
[auto-ai-agent — the autonomous engineering runtime]
[ai-app-engine — GPU UI for Rust]
[ai-web-agent — AI browser copilot]

## 🛠️ Open Source
[obs-wayland-hotkey · git-ai-committer · azumi · tiles · opencode plugins]

## 📦 Libraries
[dracon-libs · terminal-engine · terma]

## 🔌 Plugins
[opencode plugins]
```

---

## 7. Questions to Answer Before Finalizing

1. **Which private repos are you seriously committed to commercializing?** (auto-ai-agent? ai-app-engine? both?) This determines the "Products" section.
2. **Do you want to make auto-ai-agent public now, or build more stars first?** The answer changes Option A vs B vs C.
3. **What's your ideal customer?** Solo dev? Small team? Enterprise? Changes the tone and CTAs.
4. **What's your actual monetization mechanism?** (GitHub Sponsors? Open Core? Consulting? Nothing yet?)
5. **Do you have a website or blog to link to?** (Social links, email list, etc.)

---

*This brief is a working document. It's meant to be discussed, challenged, and refined.*
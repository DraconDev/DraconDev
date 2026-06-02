# DraconDev — Strategy Audit

**Date:** 2026-06-02
**Purpose:** Honest assessment of what's real vs fantasy in our strategy.

---

## Part 1: What's Real vs Assumed

### Validated (has data)

| Assumption | Evidence |
|:-----------|:---------|
| Website + bio = minimum viable profile | 100% of top-15 MES profiles have both |
| Short READMEs win | Median 1.4KB across 468 profiles |
| 3-5 pinned repos is optimal | Research: top profiles don't list 15+ repos |
| SamAI is on Chrome Web Store | Verified — live product |
| We have 50 projects | Verified — 29 local + 19 extensions |
| Rust repos look impressive | terminal-engine: 144K lines, verified |

### Assumed (no data)

| Assumption | Reality |
|:-----------|:--------|
| "B2B targeting small engineering teams" | **Never talked to a customer.** We don't know if any team wants what we build. |
| "YouTube is the distribution channel" | **Channel isn't growing yet.** We're planning a funnel around a channel with no traffic. |
| "pully-fully is a strong open source candidate" | **Nobody has asked for it.** We're assuming demand exists. |
| "dracon.uk converts visitors" | **We don't know what's on the website.** No pricing, no product page, no conversion funnel. |
| "SamAI is the flagship product" | **User said "selling software is almost a lost cause."** Contradicts the flagship claim. |
| "7 products to sell" | **Contradicts research.** Most effective profiles have 1-2 products, not 7. |
| "0xNyk is the north star competitor" | **We've never looked at what they actually ship.** |
| "Open source builds credibility → revenue" | **No data that OUR open source drives revenue.** It's a theory. |
| "Rust repos prove we can build" | **Valid for credibility, not revenue.** Stars don't pay bills. |

### Contradicted (data says otherwise)

| Assumption | Evidence |
|:-----------|:---------|
| "Chrome Web Store is a sales channel" | **Chrome Web Store discontinued payments in 2021.** All monetization must be external (Stripe, Paddle). |
| "AI browser extensions make money" | **Median extension has 18 users.** Most make $0. Only high-value, specific tools succeed. |
| "7 products is too many" | **Research says 3-5 repos max.** 7 products = 7 stories = nobody remembers any. |
| "SamAI on Chrome Web Store = revenue" | **No public reviews. No evidence of users.** Being on the store ≠ making money. |

---

## Part 2: What Competitors Are Actually Doing

### 0xNyk / Builderz (our "north star")

**What they actually sell:** Services, not products.

| What | Details |
|:-----|:--------|
| **Revenue** | Builderz: ~$342k/year. SplitLabs: ~€92k/year |
| **Model** | Service contracts: $15k-$50k per project |
| **Open source** | Mission Control (5.1k★), LACP — credibility plays |
| **Funding** | Bootstrapped, no external funding |
| **What they actually do** | Sell AI agent implementations to companies. The open source repos are marketing. |

**What we can learn:** 0xNyk doesn't make money from open source. He makes money from services. The repos are his portfolio, not his product. We've been treating his repos as the goal, but they're the means, not the end.

### anomalyco/opencode

**What they actually sell:** Enterprise support + hosted service.

| What | Details |
|:-----|:--------|
| **Revenue** | Unknown, but 167k★ suggests significant traction |
| **Model** | Open core + enterprise features + hosted service |
| **Open source** | The coding agent itself |
| **What they actually do** | Give away the agent, sell the enterprise layer |

**What we can learn:** Open core works when the free version is genuinely useful. The paid version adds value on top, not replaces it.

### t3dotgg (Theo Browne)

**What they actually sell:** Products (T3 Chat, UploadThing).

| What | Details |
|:-----|:--------|
| **Revenue** | Multiple products, YouTube ad revenue |
| **Model** | Products + YouTube content marketing |
| **YouTube** | 700K+ subscribers |
| **What he actually does** | Builds products, demos them on YouTube, YouTube drives traffic to products |

**What we can learn:** YouTube works as distribution WHEN you have traffic. Theo had the audience first, products second. We're trying to do it backwards.

---

## Part 3: Revenue Reality Check

### What's actually making money in this space

| Revenue Source | Realistic? | Evidence |
|:---------------|:-----------|:---------|
| **Chrome extensions** | ⚠️ Hard | Median 18 users. Most make $0. Only high-value tools succeed. |
| **Open source** | ❌ Direct revenue unlikely | Open source builds credibility, not revenue. Services/consulting monetize it. |
| **Services/consulting** | ✅ Proven | 0xNyk: $342k/year from services. This is the real model. |
| **YouTube** | ✅ But slow | Takes years to build audience. Not a quick win. |
| **SaaS/hosted** | ✅ But requires ops | OpenCode does this. Requires infrastructure. |
| **One-time purchases** | ⚠️ Low ceiling | $5-10 per purchase. Volume game. |

### Honest assessment of our products

| Product | Revenue potential | Reality check |
|:--------|:------------------|:--------------|
| **SamAI** | ⚠️ Low | On Chrome Web Store but no evidence of users. BYOK model means $0 revenue per user. |
| **pully-fully** | ⚠️ Unknown | Nobody has asked for it. No demand signal. |
| **dracon-terminal-engine** | ❌ Zero | It's a library. Libraries don't make money. |
| **vidpro-extension** | ⚠️ Medium | YouTube tool for YouTubers. Natural fit but unvalidated. |
| **api-debugger** | ⚠️ Low | Crowded market (Postman, Insomnia). |
| **rust-ai-web-auto** | ⚠️ Unknown | Not built yet. Playwright alternative is a big claim. |
| **YouTube channel** | ⚠️ Slow | Takes years. Not a quick win. |

---

## Part 4: What We Should Actually Build Next

### The hard truth

1. **We don't have a revenue problem. We have a customer problem.** We've never talked to a single person who might buy what we build.

2. **We're over-planning and under-building.** 7 docs totaling 50KB of strategy. Zero paying customers.

3. **The Chrome Web Store is not a business.** Payments are disabled. All monetization is external. And median extension has 18 users.

4. **Open source doesn't pay bills.** 0xNyk's open source repos are marketing for his $342k/year service business. We're treating the marketing as the business.

5. **YouTube takes years.** Theo had 700K subscribers before launching T3 Chat. We're planning to use YouTube as distribution with no audience.

### What to actually do

**Priority 1: Talk to customers (this week)**
- Find 5 developers who might use pully-fully or SamAI
- Ask: "Would you pay for this? How much? What would make you switch from X?"
- This is the only thing that matters right now

**Priority 2: Pick ONE product to sell**
- Not 7. ONE.
- The one with the most validated demand from customer conversations
- Build a simple pricing page on dracon.uk
- Accept money via Stripe

**Priority 3: Ship ONE thing this month**
- Package the chosen product for Chrome Web Store or npm
- Add a pricing page
- Post one YouTube video about it
- Measure: downloads, signups, revenue

**Priority 4: Stop planning, start building**
- Delete the strategy docs (or archive them)
- Write code, not markdown
- Every hour spent planning is an hour not spent building

---

## Summary

| Category | Count |
|:---------|------:|
| Validated assumptions | 6 |
| Unvalidated assumptions | 9 |
| Contradicted assumptions | 4 |

**The biggest risk:** We're building a GitHub profile strategy for a business that doesn't have customers yet. The profile is the last step, not the first.

**The one thing that would change everything:** Talk to 5 potential customers this week. Everything else is secondary.

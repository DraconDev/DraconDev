# DraconDev — Strategic Direction

**Last updated:** 2026-06-02
**Supersedes:** POSITIONING_BRIEF.md, POSITIONING.md, PRODUCT_ROADMAP.md

---

## The Business

DraconDev builds AI-powered tools for developers and power users. Revenue comes from:
1. **Browser extensions** (SamAI, vidpro, api-debugger) — freemium via Chrome Web Store
2. **Open source tools** (pully-fully, rust-ai-web-auto) — open core + managed/enterprise
3. **YouTube** — content marketing that drives traffic to products
4. **Consulting** — via website

**Not selling:** Software licenses directly. AGPL-3.0 for open source. Paid tiers for hosted services.

---

## The Market

- **AI coding agent market:** $8.5B–$12.8B (2026), projected $47B–$91B by 2034
- **37–46% CAGR** through 2030
- **Rust + AI agent intersection** is high-demand (claw-code 193k★, codex 87k★, tauri 107k★)
- DraconDev sits at: **Rust core + AI agents + Tauri desktop GUI**

---

## Positioning

**"AI agents for engineers who want control, not magic."**

### Competitive Landscape

| Competitor | What they do | How we differ |
|:-----------|:-------------|:--------------|
| **0xNyk** (north star) | Fleet dashboard, policy gates, agent ops (TypeScript) | We use Rust + Tauri + desktop GUI |
| **anomalyco/opencode** | Open source coding agent (167k★) | We focus on fleet management, not single-agent |
| **t3dotgg** | T3 Chat, UploadThing, YouTube funnel | We target B2B, not indie devs |
| **steipete** | OpenClaw, credibility play | We have products, not just reputation |

### Positioning Archetypes

| Archetype | Fits DraconDev? | Notes |
|:----------|:----------------|:------|
| Operator systems builder | ✅ Primary | Fleet orchestration, policy gates, agent ops |
| Product-first | ✅ Secondary | SamAI, vidpro, api-debugger |
| Building in public | ✅ YouTube | Content marketing, demos |
| Massive credibility | ⚠️ Aspirational | Need more stars/repo traction |
| Niche utility | ✅ Rust tools | dracon-terminal-engine, tiles |

---

## Target Audience

**Primary:** Small engineering teams (2–5 engineers) who use AI tools but want more control.
**Secondary:** Individual developers who want better browser tools (SamAI, vidpro).
**NOT targeting:** Indie hackers, enterprise (yet), non-technical users.

---

## GitHub Profile Strategy

See `PROFILE_STRATEGY.md` for detailed execution plan.

**In short:**
- **Pin:** 3–4 Rust repos (terminal-engine, pully if published, tiles, obs-wayland-hotkey)
- **Link:** SamAI (Chrome Web Store, not pinned — closed source), YouTube, dracon.uk
- **Keep private until release-ready:** platform, code, demons, utilities; utilities can become a simple public monorepo only after cleanup/verification
- **Publish:** pully-fully, rust-ai-web-auto, repo-rot-scanner (if ready); utilities only as a simple public monorepo with distinct sync/system/warden component paths

---

## Key Decisions (confirmed)

1. **B2B > B2C** — small engineering teams pay; indie hackers don't
2. **Option B (Audience Lead)** — open source tools build credibility → funnel to dracon.uk → pivot to Option A when products are ready
3. **3–5 pinned repos max** — long lists signal overcompensation
4. **No plugins/extensions on profile** — they don't reinforce the Rust story
5. **SamAI stays closed source** — it's a product, not a portfolio piece
6. **YouTube is the distribution channel** — content marketing drives everything
7. **Products come when ready** — don't promote half-finished things

---

## Positioning Audit (2026-06-02)

See `POSITIONING_AUDIT.md` for full per-project recommendations.

**Summary:**
| Action | Count | Key projects |
|:-------|------:|:-------------|
| PIN | 4 | terminal-engine, pully, tiles, obs-wayland-hotkey |
| SELL | 7 | SamAI, vidpro, api-debugger, bugkit, auto-form-filler, ai-ats, ai-job-finder |
| PUBLISH | 6 | pully, rust-ai-web-auto, repo-rot-scanner, ai-auto-writer, Kiki, wal-backup |
| PRIVATE | 24 | platform, code, demons, utilities, most extensions |
| DELETE | 2 | kittentts-showcase, test-auto-create |

**Priority actions:**
1. Clean up README — short, link SamAI + YouTube + dracon.uk
2. Publish pully-fully — strongest open source candidate
3. Package vidpro-extension — natural YouTube play
4. Package api-debugger — already on GitHub, low effort
5. Delete kittentts-showcase and test-auto-create

---

## Open Questions

1. When to publish pully-fully? (Needs README cleanup first)
2. Pricing model for SamAI paid tier?
3. When to start YouTube content about these tools?

# Maker/Product-Seller Profile Scores

Date: 2026-06-11

## Scoring method

Scored by fit for DraconDev's current profile problem, not by popularity.

Factors, 0-10 each:

1. **Maker fit** — presents the person as a maker/product builder rather than a course seller.
2. **Destination clarity** — makes the product/company/personal destination obvious without awkward funnel language.
3. **Proof quality** — uses shipped artifacts, stats, stars, users, or concrete descriptions.
4. **First-screen restraint** — keeps the first screen short and scannable.
5. **DraconDev transferability** — can be adapted without copying wording or inventing claims.

Maximum total: 50.

## Scores

| Example | Maker fit | Destination clarity | Proof quality | First-screen restraint | DraconDev transferability | Total | Why |
|---|---:|---:|---:|---:|---:|---:|---|
| `rauchg` → Vercel | 9 | 10 | 7 | 10 | 8 | **44** | Product destination is obvious; profile does not over-explain. |
| `zenorocha` → Resend | 9 | 10 | 6 | 10 | 8 | **43** | Founder/product identity is clean and short. |
| `amix` → Doist | 9 | 10 | 6 | 10 | 7 | **42** | Founder/company destination is direct; no README clutter. |
| `yyx990803` → VoidZero / Vue / Vite | 10 | 7 | 10 | 10 | 8 | **45** | Best artifact name-drop pattern, but DraconDev should not overclaim. |
| `feross` → Socket | 10 | 8 | 10 | 8 | 7 | **43** | Strong product/open-source credibility; README is minimal, not salesy. |
| `alyssaxuu` → Screenity/products | 10 | 8 | 10 | 3 | 6 | **37** | Excellent proof, but too long for DraconDev's first screen. |
| `steipete` → OpenClaw | 10 | 8 | 10 | 3 | 5 | **36** | Strong start-here proof, but too dense to copy directly. |
| `mcollina` → Platformatic / Fastify | 9 | 7 | 9 | 10 | 7 | **42** | Maintainer/founder credibility fits DraconDev's Rust/tooling proof. |
| `fabpot` → Symfony / Upsun | 8 | 8 | 8 | 10 | 6 | **40** | Founder/project-lead clarity is useful; less transferable because main projects live in orgs. |
| `geerlingguy` → Jeff Geerling | 7 | 7 | 6 | 9 | 7 | **36** | Short destination/support CTA is useful, but less product-seller specific. |
| `tomnomnom` → tool maker | 10 | 6 | 6 | 10 | 8 | **40** | Natural maker tone; destination clarity is weaker than DraconDev needs. |
| `stormzhang` → AI builder | 8 | 7 | 7 | 8 | 6 | **36** | Maker/AI-builder identity and home page fit; less structurally relevant. |

## Best-in-class factors to combine

| Factor | Best example | DraconDev adaptation |
|---|---|---|
| Product/company destination clarity | `rauchg`, `zenorocha`, `amix` | Keep `dracon.uk` as the obvious destination; do not bury it in a repo list. |
| Artifact name-drops | `yyx990803` | Name the actual shipped categories: Rust terminal engine, Git utilities, video workflows, Chrome extensions. |
| Proof without overclaiming | `feross`, `mcollina` | Use objective proof: 239K+ Rust lines, 5.6K tests, lib.rs search, public repos. |
| Product proof with restraint | `alyssaxuu` | Keep product proof in short bullets and collapsible details; do not list every product. |
| Natural maker tone | `tomnomnom` | Use "I make tools" / "I build..." rather than "sales funnel" language. |
| Short support/footer CTA | `geerlingguy` | Keep footer to `dracon.uk`, YouTube, Sponsor. |

## Current DraconDev README diagnosis

Current strengths:

- Product destinations appear before code.
- Chrome extensions point to Chrome Web Store, not source.
- Games are not linked as public products until verified.
- Code section is short and developer-useful.
- `dracon-utilities` is not promoted because it is private and not fmt-clean.

Current awkwardness:

- "finished tools, not source code" is accurate but defensive.
- "Use it / buy it" is clear but slightly sales-copy-like.
- "proves the products are real" is useful but a little defensive.

## Recommended refinement

Apply a small wording refinement only; do not change the structure:

Before:

```markdown
Use or buy the finished tools on [dracon.uk](https://dracon.uk).

**Use it / buy it** — finished tools, not source code.

**Inspect the code** — public Rust/tooling that proves the products are real.
```

After:

```markdown
Finished products and install pages are on [dracon.uk](https://dracon.uk).

**Use / buy finished tools**

**Inspect public code** — developer-useful Rust/tooling that backs the products.
```

Why this is better:

- Less defensive.
- Still separates product destinations from source code.
- Keeps first screen short.
- Does not introduce unverified product pages.
- Does not copy researched profile wording.

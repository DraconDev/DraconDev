# Suggested DraconDev Profile README — Maker/Product-Seller Form

Status: **not yet adopted**. This is a forward-looking template. The live
`README.md` and `README_DRAFT.md` in this repo are intentionally unchanged
until the user explicitly asks to adopt it.

## Positioning this form protects

DraconDev is primarily a **maker/product seller**, not an educator.

The profile should say, in effect:

> **I make real tools. Inspect the code if you want. Buy/use the finished product if you want.**

It should not sound like a course catalog, teaching portfolio, or generic repo dump.

## How to read this form

Every slot wrapped in `{ ... }` is a placeholder. Each placeholder has a
blocker note in `SUGGESTED_FORM_BLOCKERS.md` that explains what must be
true before the slot is filled.

## Apply-later checklist (when adopting this form)

1. Replace every `{...}` slot with a real shipped tool, verified product
   destination, or honest blocker note.
2. Remove blocker-note sections before pasting into the live README.
3. Re-run `/tmp/suggested_readme_goal/final_audit.sh` to confirm the
   pasted README still parses, has no broken links, and matches the
   structural rules.
4. Re-snapshot `README.md` and `README_DRAFT.md` so the audit shows the
   live form is now in use.
5. If a placeholder was filled, delete its blocker note entry from
   `SUGGESTED_FORM_BLOCKERS.md`.

## The form

```markdown
# Hey, I make tools that run themselves.

I build Rust infrastructure, terminal UX, Git utilities, and video workflows. Use or buy the finished tools on [dracon.uk](https://dracon.uk).

**{stat-line, e.g. 239K+ Rust lines · 5.6K tests}** · [Dracon crates on lib.rs](https://lib.rs/search?q=dracon)

---

**Use it / buy it** — product destinations, not source code.

<details><summary>Chrome extensions</summary>

• [{chrome-store-extension-1-name}]({chrome-store-extension-1-url}) — {one-line install pitch}.
• {chrome-store-extension-2-name} — {promote when the Chrome Web Store URL is live}.

</details>

<details><summary>Games</summary>

• {itch-or-site-game-1-name} — {one-line pitch}; play at {itch-or-site-game-1-url}.
• {additional-game} — {promote when a public install/play page is verified}.

</details>

<details><summary>Tools on dracon.uk</summary>

• [{tool-or-product-1-name}]({tool-or-product-1-url}) — {one-line buy/use pitch}.
• {tool-or-product-2-name} — {promote when the product page is live and verified}.

</details>

**Inspect the code** — public, developer-useful open source that proves the tools are real.

• [{code-repo-1-name}]({code-repo-1-url}) — {one-line why a developer cares}.
• [{code-repo-2-name}]({code-repo-2-url}) — {one-line why a developer cares}.
• [{code-repo-3-name}]({code-repo-3-url}) — {one-line why a developer cares}.
• [{code-repo-4-name}]({code-repo-4-url}) — {one-line why a developer cares}.
• [{code-repo-5-name}]({code-repo-5-url}) — {one-line why a developer cares}.

<details><summary>More code worth knowing about</summary>

• {secondary-code-repo} — {why it is in the second tier, not the first}.
• {dracon-utilities-or-similar} — {promote-to-first-screen-when-public-and-clean}.

</details>

[🌐 dracon.uk](https://dracon.uk) · [🎥 YouTube](https://youtube.com/@DraconDev) · [💰 Sponsor](https://github.com/sponsors/DraconDev)
```

## Slot legend

- `{stat-line}` — one short line of objective proof: shipped lines, tests,
  crates, tools, or categories.
- `{tool-or-product-N-url}` — verified product page on `dracon.uk`, Chrome
  Web Store, itch.io, Steam, GameJolt, or another public install/play/store
  destination.
- `{chrome-store-extension-N-url}` — verified Chrome Web Store URL, never
  the source-code URL.
- `{itch-or-site-game-N-url}` — verified public play/install page, never
  the source-code URL.
- `{code-repo-N-name}` and `{code-repo-N-url}` — public GitHub repo, with
  a one-line developer-care reason. Code is proof, not the main sales path.

## Why the form looks like this

- **Products first, code second.** Buyers/users see where to use or buy the
  finished thing before developers see source repos.
- **Product destinations are separated from source code.** Chrome
  extensions, games, and tools point to install/play/store pages; code
  points to GitHub.
- **The hero line is maker-first.** It says DraconDev makes tools, not that
  DraconDev teaches courses.
- **The Code section is supporting evidence.** It proves the maker ships
  real things without turning the profile into a repo dump.
- **The first screen stays short.** Product sections are collapsible, and
  additional code lives in a details dropdown.

## Patterns to copy and avoid

Copy:

- `rauchg` → Vercel: product/company destination clarity.
- `amix` → Doist: founder/company funnel to a real product site.
- `feross` → Socket: founder/product positioning, with the caveat that the
  endpoint was bot-blocked in the research run.

Adapt carefully:

- Wes Bos, Kent C. Dodds, Brad Traversy, and Frontend Masters teachers use
  strong course funnels. Use their short-bio/CTA structure only if
  DraconDev ever sells learning content.

Avoid:

- "I teach" or "learn from me" language unless courses are the actual
  product.
- A repo dump with no buy/use destination.
- Private or unverified product links.

## See also

- `MAKER_PROFILE_POSITIONING.md` — maker/product-seller positioning note.
- `SUGGESTED_FORM_BLOCKERS.md` — exact blocker for each placeholder.
- `SUGGESTED_FORM_USAGE.md` — short step-by-step adoption procedure.
- `SALES_FUNNEL_RECOMMENDATIONS.md` — maker-first funnel recommendations.
- `/tmp/suggested_readme_goal/final_audit.sh` — audit script for the
  live README (run after adopting this form).
- `/tmp/suggested_readme_goal/final_report.md` — completion report.

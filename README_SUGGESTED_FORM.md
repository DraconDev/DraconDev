# Suggested DraconDev Profile README — Form

Status: **not yet adopted**. This is a forward-looking template. The live
`README.md` and `README_DRAFT.md` in this repo are intentionally unchanged
until the placeholders below are cleared.

## How to read this form

Every slot that is wrapped in `{ ... }` is a placeholder. Each placeholder
has a blocker note immediately under the form that explains what must be
true before the slot is filled.

## Apply-later checklist (when adopting this form)

1. Replace every `{...}` slot with the matching real value.
2. Remove the blocker notes section before pasting into the live README.
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

I build infrastructure in Rust — terminal engines, fleet reconcilers, git daemons. I teach what I build on [YouTube](https://youtube.com/@DraconDev), and I sell the tools on [dracon.uk](https://dracon.uk).

**{stat-line, e.g. 239K+ Rust lines · 5.6K tests}** · [Dracon crates on lib.rs](https://lib.rs/search?q=dracon)

---

**Code** — public, developer-useful open source. Each repo is a focused, small-surface library or tool.
• [{code-repo-1-name}]({code-repo-1-url}) — {one-line why a developer cares}.
• [{code-repo-2-name}]({code-repo-2-url}) — {one-line why a developer cares}.
• [{code-repo-3-name}]({code-repo-3-url}) — {one-line why a developer cares}.
• [{code-repo-4-name}]({code-repo-4-url}) — {one-line why a developer cares}.
• [{code-repo-5-name}]({code-repo-5-url}) — {one-line why a developer cares}.

<details><summary>More code worth knowing about</summary>

• {secondary-code-repo} — {why it is in the second tier, not the first}.
• {dracon-utilities-or-similar} — {promote-to-first-screen-when-blocks-clear}.

</details>

**Chrome extensions** — installable browser tools, not source code.
<details><summary>Install a DraconDev extension</summary>

• [{chrome-store-extension-1-name}]({chrome-store-extension-1-url}) — {one-line install pitch}.
• {chrome-store-extension-2-name} — {promote when the Chrome Web Store URL is live}.

</details>

**Games** — playable work, not source code.
<details><summary>Playable DraconDev games</summary>

• {itch-or-site-game-1-name} — {one-line pitch}; play at {itch-or-site-game-1-url}.
• {additional-game} — {promote when a public install/play page is verified}.

</details>

[🌐 dracon.uk](https://dracon.uk) · [🎥 YouTube](https://youtube.com/@DraconDev) · [💰 Sponsor](https://github.com/sponsors/DraconDev)
```

## Slot legend

- `{stat-line}` — one short line of objective numbers.
- `{code-repo-N-name}` and `{code-repo-N-url}` — public GitHub repo, with
  a one-line developer-care reason.
- `{chrome-store-extension-N-url}` — verified Chrome Web Store URL, never
  the source-code URL.
- `{itch-or-site-game-N-url}` — verified public play/install page, never
  the source-code URL.

## Why the form looks like this

- Three explicit sections (Code, Chrome extensions, Games) keep the
  "use it" and "fork it" surfaces separate.
- A collapsed `details` block per product dropdown keeps the first screen
  short and scannable.
- A "More code worth knowing about" dropdown holds the items that are
  public but not strong enough for the first screen, and items that
  become first-screen candidates when their blockers clear.
- The hero line, stat line, and footer are the only always-visible
  content, matching the audit of the current README.

## See also

- `SUGGESTED_FORM_BLOCKERS.md` — exact blocker for each placeholder.
- `SUGGESTED_FORM_USAGE.md` — short step-by-step adoption procedure.
- `/tmp/suggested_readme_goal/final_audit.sh` — audit script for the
  live README (run after adopting this form).
- `/tmp/suggested_readme_goal/final_report.md` — completion report.

# Suggested README Form — Usage

This is the step-by-step procedure for adopting `README_SUGGESTED_FORM.md`
as the live `README.md` (and matching `README_DRAFT.md`).

## When to adopt

Adopt only when at least one of the placeholders is about to be filled,
or when the user explicitly asks to replace the live README. The current
README already separates products and code, so adopting the form is
optional and incremental. When adopting, keep the maker/product-seller
positioning: products first, code as proof.

## Steps

1. Read `SUGGESTED_FORM_BLOCKERS.md` and decide which placeholder is
   ready to be filled. Do not fill a placeholder whose blocker is not
   cleared.
2. Open `README_SUGGESTED_FORM.md` and copy the `## The form` block into
   `README.md` (and `README_DRAFT.md`).
3. Replace each `{...}` slot that is ready with the real value. Leave
   unfilled slots as `{...}` and keep the matching blocker note in
   `SUGGESTED_FORM_BLOCKERS.md`.
4. Re-run the audit script:
   `bash /tmp/suggested_readme_goal/final_audit.sh`
   It must pass.
5. Re-snapshot the live `README.md` and `README_DRAFT.md`:
   `cp README.md /tmp/suggested_readme_goal/README.live.md`
   `cp README_DRAFT.md /tmp/suggested_readme_goal/README_DRAFT.live.md`
6. Update `PROFILE_REFINEMENT_AGREEMENT.md` only if the peer-comparison
   text needs to change because of the new slots.
7. Commit the change with a message like
   `Profile README: adopt suggested form (slot N filled)`.

## What to remove when adopting

- The blocker notes section in `README_SUGGESTED_FORM.md` (kept only in
  the suggested form file, never in the live README).
- The "How to read this form" and "Apply-later checklist" sections from
  the suggested form file (they are for the form, not the live README).
- The slot legend from the suggested form file.
- Any `{...}` that has not been filled — leave it as an open
  placeholder in the suggested form file, but never paste an open
  placeholder into the live README.

## What to never remove

- The hero line, stat line, and footer line structure.
- The product-first split: Use it / buy it before Inspect the code.
- The separation between product destinations and source-code links.
- The `<details>` dropdowns for the product sections.
- The "More code worth knowing about" dropdown.

## When to update this file

- Any time a placeholder is filled in the live README.
- Any time a new placeholder is added to the suggested form.
- Any time a peer-comparison item in `PROFILE_REFINEMENT_AGREEMENT.md`
  changes.

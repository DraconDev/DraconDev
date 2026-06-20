# Cleanup Report

The public custom profile repo (`DraconDev/DraconDev`) contains only profile-facing files. The strategy repo (`DraconDev/dracon-strategy`) is private and should not be published as the public profile surface.

- `README.md`
- `CONTRIBUTING.md`
- `LICENSE`
- `.github/FUNDING.yml`
- `.gitignore`
- `.gitattributes`

All internal notes, drafts, audits, strategy docs, scoring data, agent state, and content-production files were moved out of the public profile repo into a separate private archive repo.

## Verification

- Public tracked file count: 7
- Private archive file count: 99
- Secret-pattern scan: 0 findings
- Sensitive filename scan: 0 findings
- Relative link check: 0 errors
- External URL smoke check: 60 URLs checked, 0 failures
- Draft/placeholder scan: 0 findings

This repo is intentionally kept minimal so the public profile stays clean and profile-appropriate.

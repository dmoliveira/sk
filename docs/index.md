# sk Docs 🔐

Welcome to the GitHub Pages docs hub for `sk`.

## Support sk First 💛

If these docs and tools save you time, consider supporting the project:

- Donate now via Stripe: [Support sk](https://buy.stripe.com/8x200i8bSgVe3Vl3g8bfO00)
- Sponsor on GitHub: [@dmoliveira](https://github.com/sponsors/dmoliveira)

## Security Quickstart for Agents (30 seconds) ⚡

- Store secrets once with `sk add -k KEY --stdin --force`.
- Read secrets only at execution time via `$(sk get -k KEY)`.
- Never paste secrets into prompts, docs, notebooks, or issue comments.
- Use separate keys per environment and rotate them on a schedule.
- Revoke and replace immediately if logs or traces may have exposed data.

## Security & Trust Links

- [Security guide](./security-for-ai-agents.md)
- [Agent FAQ](./agent-security-faq.md)
- [Checklist card](./agent-security-checklist.md)

## Quick Links

- Main README: `../README.md`
- CLI contract: `./specs/cli-contract.md`
- Release dashboard: `./release.md`
- Security guide: `./security-for-ai-agents.md`
- Agent security FAQ: `./agent-security-faq.md`
- Security checklist card: `./agent-security-checklist.md`
- Changelog: `../CHANGELOG.md`
- Release checklist: `../RELEASE.md`
- Support page: `./support-the-project.md`
- Pages plan: `./plan/github-pages-site-map.md`
- GitHub wiki: `https://github.com/dmoliveira/sk/wiki`
- Latest release: `https://github.com/dmoliveira/sk/releases/latest`

## Common Commands

```bash
make ci
make release-snippet TAG=v0.2.1
./scripts/smoke.sh
```

## Install Paths

- Homebrew tap install: `brew tap dmoliveira/tap && brew install sk`
- Cargo install: `cargo install --path .`
- Local binary install: `./target/release/sk install`

## Support

- Donate now via Stripe: [Support sk](https://buy.stripe.com/8x200i8bSgVe3Vl3g8bfO00)
- Sponsor on GitHub: [@dmoliveira](https://github.com/sponsors/dmoliveira)

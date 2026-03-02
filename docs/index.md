# sk Docs Hub 🔐

![sk Hero](./assets/hero-banner.svg)

Welcome to the `sk` docs site: a fast path to secure local secret handling for
AI agents, ML workflows, and software teams.

[![Latest Release](https://img.shields.io/github/v/release/dmoliveira/sk?label=latest)](https://github.com/dmoliveira/sk/releases/latest)
[![Project Wiki](https://img.shields.io/badge/wiki-open-22c55e)](https://github.com/dmoliveira/sk/wiki)
[![Support](https://img.shields.io/badge/support-stripe-635bff?logo=stripe&logoColor=white)](https://buy.stripe.com/8x200i8bSgVe3Vl3g8bfO00)

## Start in 60 Seconds

1. Install: `brew tap dmoliveira/tap && brew install sk`
2. Store a secret: `printf '%s' "$KEY" | sk add -k OPENAI_API_KEY --stdin --force`
3. Read at runtime: `export OPENAI_API_KEY="$(sk get -k OPENAI_API_KEY)"`

## Security Center

- [Security guide](./security-for-ai-agents.md)
- [Agent FAQ](./agent-security-faq.md)
- [Checklist card](./agent-security-checklist.md)

## For AI Agents and ML Teams

- Keep secrets outside prompts and notebooks.
- Use environment-specific keys and least privilege.
- Rotate and revoke quickly after suspicious exposure.
- Redact traces/logs before sharing in PRs or chat threads.

## Popular Docs

- [CLI contract](./specs/cli-contract.md)
- [Release dashboard](./release.md)
- [Release checklist](../RELEASE.md)
- [Changelog](../CHANGELOG.md)
- [Support page](./support-the-project.md)
- [Pages plan](./plan/github-pages-site-map.md)

## Maintainer Commands

```bash
make ci
make release-snippet TAG=v0.2.1
./scripts/smoke.sh
```

## Support sk 💛

- Stripe: [Support sk](https://buy.stripe.com/8x200i8bSgVe3Vl3g8bfO00)
- GitHub Sponsors: [@dmoliveira](https://github.com/sponsors/dmoliveira)

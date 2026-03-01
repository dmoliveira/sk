# Release Dashboard

A quick operational snapshot for maintainers and AI agents.

## Current baseline

- Latest stable tag: `v0.2.1`
- Release page: `https://github.com/dmoliveira/sk/releases/latest`
- Tap formula source tarball: `https://github.com/dmoliveira/sk/archive/refs/tags/v0.2.1.tar.gz`

## Core release commands

```bash
make ci
make release-snippet TAG=v0.2.1
git tag v0.2.1
git push origin v0.2.1
```

## Primary references

- Checklist: `RELEASE.md`
- Changelog: `CHANGELOG.md`
- Tap flow: `TAP.md`
- Formula template: `Formula/sk.rb`

## Live links

- Releases: `https://github.com/dmoliveira/sk/releases`
- Release workflow: `https://github.com/dmoliveira/sk/actions/workflows/release.yml`
- Docs site: `https://dmoliveira.github.io/sk/`
- Homebrew tap repo: `https://github.com/dmoliveira/homebrew-tap`

## Support sustainable maintenance

- Donate now via Stripe: [Support sk](https://buy.stripe.com/8x200i8bSgVe3Vl3g8bfO00)
- Sponsor on GitHub: [@dmoliveira](https://github.com/sponsors/dmoliveira)

## Security checks before tagging

- Verify no secrets in release notes, docs, or commit diffs.
- Confirm formula and docs point to public assets only.
- Re-run `make ci` before tagging and publishing.
- Validate support links and wiki/docs URLs are current.

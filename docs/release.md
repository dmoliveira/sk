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

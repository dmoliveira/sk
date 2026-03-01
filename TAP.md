# Homebrew Tap

This repository ships a Homebrew formula template in `Formula/sk.rb`.

## Create a tap repo

1. Create a repository named `homebrew-tap` under your GitHub user or org.
2. Copy `Formula/sk.rb` into that repo.

## Release flow

1. Tag a release in this repo (example: `v0.2.0`).
2. Read the release notes output for tarball SHA256.
3. Update `Formula/sk.rb` in the tap repo with new `url` and `sha256`.
4. Commit and push the tap repo.

## Install

```bash
brew tap dmoliveira/tap
brew install sk
```

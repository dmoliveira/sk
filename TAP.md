# Homebrew Tap

This repository ships a Homebrew formula template in `Formula/sk.rb`.

## Create a tap repo

1. Create a new repository named `homebrew-tap` under your GitHub user/org.
2. Copy `Formula/sk.rb` into that repo.

## Release flow

1. Tag a release in this repo (example: `v0.1.0`).
2. Download the release tarball and compute its SHA256.
3. Update `Formula/sk.rb` in the tap repo with the new `url` and `sha256`.
4. Commit and push the tap repo.

## Install

```bash
brew tap dmoliveira/tap
brew install sk
```

# Release Checklist

1. Update version in `Cargo.toml`.
2. Run validation:
   - `cargo fmt --check`
   - `cargo test`
   - `./scripts/smoke.sh`
3. Commit the version bump.
4. Tag a release (example: `v0.2.0`) and push tags:
   - `git tag v0.2.0`
   - `git push --tags`
5. The GitHub release workflow publishes release notes and prints tarball `sha256`.
6. Update Homebrew formula in your tap repo:
   - Set `url` to the new tag tarball.
   - Update `sha256` from release notes.
7. Commit and push tap repo changes.

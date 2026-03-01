# Release Checklist

1. Update version in `Cargo.toml`.
2. Run validation:
   - `cargo fmt --check`
   - `cargo test`
   - `./scripts/smoke.sh`
3. Commit the version bump.
4. Tag a release (example: `v0.2.1`) and push tags:
   - `git tag v0.2.1`
   - `git push --tags`
5. Generate tap-ready release metadata:
   - `make release-snippet TAG=v0.2.1`
   - This prints the tarball `url` and `sha256` block.
6. Update Homebrew formula in your tap repo:
   - Set `url` to the new tag tarball.
   - Paste the generated `sha256` value.
7. Commit and push tap repo changes.

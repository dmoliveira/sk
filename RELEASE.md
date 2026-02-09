# Release Checklist

1. Update `VERSION` in `sk`.
2. Commit the version bump.
3. Tag a release (example: `v0.1.0`) and push tags:
   - `git tag v0.1.0`
   - `git push --tags`
4. The GitHub Action creates a release and prints the tarball `sha256`.
5. Update the Homebrew formula in the tap repo:
   - Set `url` to the tag tarball.
   - Update `sha256` from the release notes.
6. Commit and push the tap repo changes.

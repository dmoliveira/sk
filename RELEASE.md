# Release Checklist

1. Update `VERSION` in `sk`.
2. Tag a release (example: `v0.1.0`).
3. Update the Homebrew formula in the tap repo:
   - Set `url` to the tag tarball.
   - Update `sha256` for the tarball.
4. Push the tap repo changes.

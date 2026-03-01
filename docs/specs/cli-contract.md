# sk CLI Contract

This document is the single behavior reference for both humans and AI agents.

## Scope

- Platform: macOS only
- Secret backend: Keychain via `security`
- Service prefix: `sk:` by default, configurable by `SK_SERVICE_PREFIX`
- Default account: `SK_USER`, then `USER`

## Command contract

- `sk add -k KEY -v VALUE [--force] [-u USER]`
  - Adds a key/value pair
  - Fails if key exists unless `--force`
- `sk add -k KEY --stdin [--force] [-u USER]`
  - Reads value from stdin and trims trailing newline
- `sk get -k KEY [-u USER]`
  - Prints the secret value only
- `sk list [--keys] [--show] [-u USER]`
  - Default output is keys only
  - `--show` prints masked values (`abcd****` shape)
- `sk remove -k KEY [-u USER] [-y]`
  - Requires confirmation unless `-y`
- `sk selfcheck`
  - Write/read/delete probe item to validate Keychain access
- `sk install`
  - Copies current executable to `~/.local/bin/sk` (or `SK_INSTALL_DIR`)
- `sk uninstall`
  - Removes installed executable path

## Exit behavior

- `0` on successful command execution
- `1` on validation errors, missing key, Keychain failure, unsupported OS, or aborted delete

## Stability notes for AI agents

- Prefer `--stdin` for secret input in automation
- Use `-y` for non-interactive delete flows
- Avoid parsing `--help`; rely on this contract doc for deterministic behavior

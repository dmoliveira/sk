# sk

Minimal macOS Keychain CLI for storing and retrieving secrets by key.

Made by Diego Marinho de Oliveira

## Install

### Homebrew (tap)

```bash
brew tap dmoliveira/tap
brew install sk
```

If you are creating your own tap, see `TAP.md`.

### Local install

```bash
./sk install
```

Override install location:

```bash
SK_INSTALL_DIR="$HOME/bin" ./sk install
```

If `~/.local/bin` is not in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

Store a secret:

```bash
sk add -k OPENAI_API_KEY -v "sk-xxxx"
```

Overwrite an existing key:

```bash
sk add -k OPENAI_API_KEY -v "sk-xxxx" --force
```

Avoid shell history by piping the value:

```bash
printf '%s' "sk-xxxx" | sk add -k OPENAI_API_KEY -v -
```

Or use `--stdin`:

```bash
printf '%s' "sk-xxxx" | sk add -k OPENAI_API_KEY --stdin
```

Read a secret (prints only the value):

```bash
export OPENAI_API_KEY=$(sk get -k OPENAI_API_KEY)
```

List keys (default):

```bash
sk list
```

Show masked values (may prompt Keychain access):

```bash
sk list --show
```

Remove a secret:

```bash
sk remove -k OPENAI_API_KEY
```

Skip confirmation:

```bash
sk remove -k OPENAI_API_KEY -y
```

Uninstall:

```bash
sk uninstall
```

Version:

```bash
sk --version
```

## Notes

- Uses macOS Keychain via `security`.
- macOS only.
- Entries are scoped to the `sk:` service prefix.
- Override the prefix with `SK_SERVICE_PREFIX` if needed.
- Override the default user with `SK_USER` if needed.
- `sk list` does not access secret values.

## Security

- Prefer `--stdin` to avoid secrets in shell history.

## License

MIT

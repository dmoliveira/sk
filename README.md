# sk

Minimal macOS Keychain CLI for storing and retrieving secrets by key.

Made by Diego Marinho de Oliveira

## Install

### Homebrew (tap)

```bash
brew tap <github-user>/tap
brew install sk
```

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

Avoid shell history by piping the value:

```bash
printf '%s' "sk-xxxx" | sk add -k OPENAI_API_KEY -v -
```

Read a secret (prints only the value):

```bash
export OPENAI_API_KEY=$(sk get -k OPENAI_API_KEY)
```

List secrets (masked):

```bash
sk list
```

List only keys:

```bash
sk list --keys
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

## Notes

- Uses macOS Keychain via `security`.
- Entries are scoped to the `sk:` service prefix.

## License

MIT

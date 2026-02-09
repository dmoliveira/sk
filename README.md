# sk

Minimal macOS Keychain CLI for storing and retrieving secrets by key.

Made by Diego Marinho de Oliveira

## Install

### Homebrew (tap)

```bash
brew tap <you>/tap
brew install sk
```

### Local install

```bash
./sk install
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

Read a secret (prints only the value):

```bash
export OPENAI_API_KEY=$(sk get -k OPENAI_API_KEY)
```

List secrets (masked):

```bash
sk list
```

Remove a secret:

```bash
sk remove -k OPENAI_API_KEY
```

## Notes

- Uses macOS Keychain via `security`.
- Entries are scoped to the `sk:` service prefix.

## License

MIT

# AI Agent Security Checklist

A short, printable runbook for daily use.

## Before you run

- Confirm environment (`dev`, `staging`, `prod`) and use matching credentials.
- Load secrets at runtime only (`--stdin` or process env), never in prompt text.
- Verify keys are least-privilege and scoped to the current task.

## During execution

- Keep prompts free of tokens, API keys, internal URLs, and customer data.
- Redact logs/traces before sharing with teammates or tools.
- Avoid writing secrets to notebooks, markdown docs, or shell history.

## Before merge/release

- Run validation (`make ci`) and inspect diffs for accidental secret exposure.
- Confirm docs links and release metadata point to public-safe endpoints.
- Rotate keys if any exposure is suspected.

## Emergency response

- Revoke exposed credentials immediately.
- Issue replacements and invalidate dependent sessions.
- Record remediation actions in incident notes.

## Copy/paste secure setup snippets

### Local shell (zsh/bash)

```bash
# Store once (stdin avoids shell history leaks)
printf '%s' "$OPENAI_API_KEY" | sk add -k OPENAI_API_KEY --stdin --force

# Load only in the active process/session
export OPENAI_API_KEY="$(sk get -k OPENAI_API_KEY)"

# Optional cleanup when done
unset OPENAI_API_KEY
```

### CI job pattern (GitHub Actions)

```yaml
- name: Load runtime secret from sk
  run: |
    export OPENAI_API_KEY="$(sk get -k OPENAI_API_KEY)"
    your_command_here
```

### Rotation helper

```bash
printf '%s' "$NEW_OPENAI_API_KEY" | sk add -k OPENAI_API_KEY --stdin --force
```

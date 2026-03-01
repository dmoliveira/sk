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

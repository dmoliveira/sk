# Agent Security FAQ

## 1) Why avoid putting secrets directly in prompts?

Prompts can be logged, cached, exported, or reused by downstream tools.
Keep secrets outside prompt text and inject them only at execution time.

## 2) What is the safest way to provide credentials to AI workflows?

Use runtime injection with environment variables and stdin-based flows.
With `sk`, store once and fetch only in the process that needs the secret.

## 3) How should teams split credentials across environments?

Use separate keys per environment (dev/staging/prod) and per automation
surface (local agent, CI, production worker).

## 4) What should be redacted before sharing logs or traces?

Redact API keys, bearer tokens, session ids, internal URLs, user identifiers,
and tool outputs that could reveal secret material.

## 5) What is a practical rotation policy?

Rotate on schedule (for example monthly), rotate after role changes, and rotate
immediately after suspected exposure. Revoke old keys in the same runbook.

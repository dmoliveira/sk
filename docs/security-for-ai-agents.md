# Security Guide for AI Agents and ML Workflows

This guide explains practical safeguards when using `sk` in AI-agent, automation, and ML environments.

## Why `sk` helps

- Local-only secret storage in macOS Keychain
- No cloud secret sync path by default
- Fast CLI for explicit secret retrieval at runtime
- Namespace and user controls via `SK_SERVICE_PREFIX` and `SK_USER`

## Threats to account for

- Prompt injection that tries to exfiltrate secrets
- Oversharing secrets in logs, traces, or notebook outputs
- Long-lived credentials reused across environments
- Broad service-account scope without least privilege

## Secure usage principles

- Use least privilege for all API keys and service accounts.
- Keep secret lifetime short; rotate keys frequently.
- Inject secrets only at execution time (`--stdin`, env var assignment in-process).
- Avoid embedding secrets in prompts, markdown docs, notebooks, and commits.
- Redact outputs in CI, telemetry, and chat transcripts.

## Recommended workflow

1. Store credential once: `sk add -k OPENAI_API_KEY --stdin --force`
2. Read only where needed: `export OPENAI_API_KEY="$(sk get -k OPENAI_API_KEY)"`
3. Clear shell/session variables after use.
4. Rotate and re-store keys on a fixed cadence.

## References

- OWASP Top 10 for LLM Applications: https://owasp.org/www-project-top-10-for-large-language-model-applications/
- NIST AI Risk Management Framework: https://www.nist.gov/itl/ai-risk-management-framework
- CISA Secure-by-Design guidance: https://www.cisa.gov/securebydesign

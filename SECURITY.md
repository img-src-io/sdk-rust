# Security Policy

## Reporting a Vulnerability

Please report security vulnerabilities to: **security@img-src.io**

**Do NOT open public issues for security vulnerabilities.**

We will acknowledge receipt within 48 hours and provide a detailed response within 7 days.

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.x.x   | :white_check_mark: |

## Security Best Practices

When using this SDK:

1. **Never hardcode API keys** - Use environment variables
2. **Keep dependencies updated** - Run `cargo update` regularly
3. **Use HTTPS only** - The SDK enforces HTTPS by default
4. **Validate user input** - Before passing to SDK methods

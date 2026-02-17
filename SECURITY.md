# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.7.x   | Yes       |
| < 0.7   | No        |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do not** open a public GitHub issue
2. Email the maintainer or use [GitHub Security Advisories](https://github.com/HerbHall/DigitalRain/security/advisories/new)
3. Include steps to reproduce and any relevant details

You can expect an initial response within 7 days.

## Scope

DigitalRain is a terminal visual effects application. It does not handle network connections, user data, or authentication. Security concerns are limited to:

- Terminal escape sequence injection
- Resource exhaustion (CPU/memory)
- Dependency vulnerabilities

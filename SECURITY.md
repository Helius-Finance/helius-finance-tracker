# Security Policy

## Supported Versions

Helius is still in an early stage and does not currently maintain long-term support branches.

In practice:

- the latest release is the supported version
- the `main` branch may contain fixes that are not yet released

## Reporting A Vulnerability

Please do not post security vulnerabilities as public GitHub issues.

If GitHub private vulnerability reporting is enabled for this repository, use that channel.

If it is not available, open a minimal public issue requesting a private contact path without including exploit details, secrets, or proof-of-concept code.

## What To Include

Helpful reports usually include:

- affected version or commit
- attack surface or feature area
- clear reproduction steps
- impact
- whether the issue involves local data exposure, unsafe file handling, or data corruption

## Response Expectations

This is a small maintainer-led project, so response times may vary. Good-faith reports will be reviewed and acknowledged as quickly as possible.

## Scope Notes

Because Helius is local-first and single-user, the most relevant classes of security issues are likely to be:

- unintended data exposure
- unsafe file/path handling
- CSV import parsing issues
- database corruption or migration edge cases
- command execution or shell interaction issues

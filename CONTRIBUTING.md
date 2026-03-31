# Contributing

Helius is a local-first personal finance tracker built in Rust.

## Before You Start

For anything beyond a typo or a small isolated fix:

1. open an issue first
2. explain the problem you want to solve
3. wait for alignment before doing larger work

## Good First Contributions

- bug fixes
- documentation improvements
- better error messages
- small usability improvements
- tests for existing behavior
- CLI/TUI polish that does not change the product direction

## Development Setup

Requirements:

- Rust stable
- Windows or Linux

Clone the repo and run:

```powershell
cargo test
cargo build --release
```

CI runs on Windows and Ubuntu. If you touch startup, storage, or release behavior, also run the smoke script from a PowerShell session where local scripts are allowed. On Windows PowerShell 5.1, one option is `powershell -NoProfile -ExecutionPolicy Bypass`.

```powershell
./scripts/smoke.ps1
```

If you touch release packaging, run the host verification script before opening a pull request.

```powershell
./scripts/verify-host-release.ps1 -Platform windows-x86_64
```

## Project Layout

- `src/main.rs`: binary entry point
- `src/lib.rs`: command dispatch and top-level app flow
- `src/cli.rs`: CLI definitions
- `src/db.rs`: SQLite access, schema, migrations, and core business logic
- `src/output.rs`: terminal, JSON, and CSV output
- `src/shell.rs`: guided interactive shell
- `src/ui/`: terminal UI
- `tests/cli.rs`: integration tests

## Style Expectations

- keep changes focused
- prefer clarity over cleverness
- preserve the local-first and single-user design
- avoid adding background services or unnecessary complexity
- follow existing code style and naming patterns
- add or update tests when behavior changes

## Pull Request Checklist

Before opening a pull request:

- make sure the change is scoped and explained clearly
- run `cargo test`
- update docs if user-facing behavior changed
- include screenshots or terminal output if the UI/UX changed materially
- keep Windows behavior stable while adding Linux support

## What To Avoid

Avoid large speculative refactors or broad feature drops without prior discussion.
Changes that affect storage, migrations, core workflow, or product direction should be discussed first.

## Reporting Bugs

Please include:

- what you expected to happen
- what actually happened
- steps to reproduce
- platform details
- relevant command output or screenshots

You can also use the issue templates in this repository.

## Security

For security-sensitive reports, see [SECURITY.md](SECURITY.md).

## Conduct

By participating, you agree to follow the project's [Code of Conduct](CODE_OF_CONDUCT.md).

# Contributing

Thanks for taking an interest in Helius.

Helius is a local-first personal finance tracker for Windows built in Rust. Contributions are welcome, but this repository is still evolving quickly, so the best contributions are usually small, concrete, and easy to review.

## Before You Start

For anything beyond a small typo, minor cleanup, or clearly scoped fix:

1. open an issue first
2. explain the problem you want to solve
3. wait for alignment before doing larger work

This helps avoid duplicate work and keeps the project focused.

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
- Windows development environment

Clone the repo and run:

```powershell
cargo test
cargo build --release
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

## What To Avoid

Please avoid opening large speculative refactors or feature dumps without prior discussion.

In particular, changes that affect storage, migrations, core workflow, or product direction should be discussed first.

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

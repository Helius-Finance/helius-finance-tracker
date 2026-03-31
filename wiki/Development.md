# Development

## Requirements

- Rust stable
- Windows or Linux development environment

## Common Commands

```powershell
cargo test
cargo build --release
```

If you want to inspect the CLI without entering the TUI:

```powershell
helius --help
```

CI runs on Windows and Ubuntu. For release verification, the repository
also includes a smoke script. Run it from a PowerShell session where local
scripts are allowed. On Windows PowerShell 5.1, you can start one with
`powershell -NoProfile -ExecutionPolicy Bypass`.

```powershell
./scripts/smoke.ps1
```

If you change release packaging, run the host verification script before
opening a pull request. Run it from a PowerShell session where local scripts
are allowed:

```powershell
./scripts/verify-host-release.ps1 -Platform windows-x86_64
```

## Repository Layout

- `src/main.rs`: application entry point
- `src/lib.rs`: command dispatch and top-level app flow
- `src/cli.rs`: `clap` command definitions and arguments
- `src/db.rs`: SQLite access, schema, migrations, and business logic
- `src/output.rs`: terminal table, text, JSON, and CSV output
- `src/shell.rs`: guided interactive shell
- `src/ui/`: TUI application state and rendering
- `tests/cli.rs`: integration coverage for CLI workflows
- `dist/`: packaged binaries and release artifacts

## Architectural Notes

- Helius is a single-user local application.
- There is no daemon, background service, or async runtime.
- The same database layer backs the CLI, TUI, shell, import/export flows, forecasting, and reconciliation.

## Test Coverage Focus

The integration tests cover the main user-facing flows, including:

- Database initialization
- Accounts, categories, and transaction editing
- Summaries, balances, and CSV export
- CSV import with dry runs and duplicate handling
- Budgets, recurring rules, forecasts, goals, and scenarios
- Reconciliation locking behavior
- Automatic repair of missing recurring tables in older databases

## Suggested Future Wiki Pages

If the project grows, the next useful pages would be:

- Import mapping recipes for real bank exports
- Architecture deep dive
- Roadmap

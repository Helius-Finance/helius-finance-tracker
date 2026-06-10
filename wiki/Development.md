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

Docker packaging is verified separately:

```powershell
./scripts/docker-smoke.ps1
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
- `src/db.rs`: SQLite access, schema, migrations, and persistence
- `src/services/`: domain service layer (stable API for CLI, TUI, and future GUI)
- `src/importer.rs`: CSV preset catalog, camt.053 parsing, and import plan resolution
- `src/error.rs`: structured `AppError` types for user-facing and programmatic handling
- `src/output.rs`: terminal table, text, JSON, and CSV output
- `src/shell.rs`: guided interactive shell
- `src/ui/`: TUI application state and rendering
- `tests/cli.rs`: integration coverage for CLI workflows
- `tests/services_*.rs`: focused service-layer unit tests
- `tests/error_messages.rs`: regression tests for structured error display strings
- `tests/fixtures/import/`: bank export fixtures for preset and camt.053 import tests
- `scripts/`: smoke, packaging, and release verification helpers
- `dist/`: packaged binaries and release artifacts

## Architectural Notes

- Helius is a single-user local application.
- There is no daemon, background service, or async runtime.
- Frontends (CLI, TUI, shell) are thin adapters over the service layer.
- Services own domain orchestration; `db.rs` owns SQL and schema.
- The same database layer backs import/export, forecasting, and reconciliation.

## Test Coverage Focus

The integration tests cover the main user-facing flows, including:

- Database initialization
- Accounts, categories, and transaction editing
- Summaries, balances, and CSV export
- Preset-based CSV import, manual CSV mapping, and camt.053 import
- Dry runs, duplicate handling, and transactional rollback on import failure
- Budgets, recurring rules, forecasts, goals, and scenarios
- Reconciliation locking behavior
- Automatic repair of missing recurring tables in older databases
- Structured error message stability for GUI-friendly error matching

## Suggested Future Wiki Pages

If the project grows, the next useful pages would be:

- Import mapping recipes for real bank exports
- Architecture deep dive
- Roadmap

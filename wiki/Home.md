# Helius Wiki

Helius is a local-first personal finance tracker for Windows and Linux. It combines a Rust CLI, a full-screen terminal UI, and a single SQLite database file for day-to-day money tracking without a cloud dependency.

Supported targets: Windows x86_64 and Linux x86_64.

## Start Here

- [Installation](Installation)
- [Getting Started](Getting-Started)
- [CLI Reference](CLI-Reference)
- [TUI and Shell](TUI-and-Shell)
- [Planning and Forecasts](Planning-and-Forecasts)
- [Data and Storage](Data-and-Storage)
- [Development](Development)
- [Release Process](Release-Process)

## Core Capabilities

- Local-first storage in one SQLite database
- Accounts, categories, income, expense, and transfer transactions
- Budgets, recurring rules, reconciliation, planning items, scenarios, and goals
- CSV import/export plus JSON output for automation
- Full-screen TUI and a guided interactive shell on top of the same database

## Typical Workflow

1. Install or build `helius`.
2. Initialize a database with `helius init --currency USD`, or launch the TUI and follow the first-run prompt.
3. Add accounts and categories.
4. Record transactions, budgets, recurring rules, and planning items.
5. Use balances, summaries, forecasts, and exports for review and reporting.

## Conventions Used In This Wiki

- Dates use `YYYY-MM-DD`.
- Month inputs use `YYYY-MM`.
- Examples use `helius ...`. If you are running a standalone binary directly, use `.\helius.exe ...` on Windows or `./helius ...` on Linux instead.

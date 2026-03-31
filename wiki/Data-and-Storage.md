# Data And Storage

Helius is local-first. Your data lives in one SQLite database file on your machine.

## Default Database Path

The default location depends on the platform:

```text
%LOCALAPPDATA%\Helius\tracker.db
~/Library/Application Support/Helius/tracker.db
~/.local/share/helius/tracker.db
```

Notes:

- Windows uses `%LOCALAPPDATA%\Helius\tracker.db`.
- Linux commonly uses `~/.local/share/helius/tracker.db`.
- Helius resolves the default through `directories::ProjectDirs`, with `LOCALAPPDATA` preferred on Windows when available.

## Database Overrides

You can override the default location in two ways:

- Pass `--db <path>` on any command
- Set the `HELIUS_DB_PATH` environment variable

Examples:

```powershell
helius --db /path/to/tracker.db balance
```

```powershell
$env:HELIUS_DB_PATH = 'C:\Finance\tracker.db'
helius balance
```

## Initialization

The database is considered uninitialized until the metadata row exists. On first interactive launch, Helius prompts for a currency code and creates the database if needed.

If you prefer an explicit setup step:

```powershell
helius init --currency USD
```

## Schema And Migrations

The database schema includes records for:

- Metadata
- Accounts
- Categories
- Transactions
- Budgets
- Recurring rules and occurrences
- Reconciliations
- Planning scenarios, planning items, and goals

Helius automatically checks the schema version and applies migrations when an older database is opened.

## Pre-Migration Backups

Before a schema migration runs, Helius writes a copy of the existing database beside the original file.

Backup name pattern:

```text
tracker.pre-vFROM-to-vTO.YYYYMMDDTHHMMSS.db
```

Example:

```text
tracker.pre-v7-to-v8.20260318T142501.db
```

## Backup Strategy

Because the database is a single file, manual backup is straightforward:

1. Close Helius.
2. Copy `tracker.db` to another folder or drive.
3. Keep the migration backup files if you want rollback points during upgrades.

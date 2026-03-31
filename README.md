

# Helius Personal Finance Tracker

[Latest Release](https://github.com/STVR393/helius-personal-finance-tracker/releases) | [Quick Start](QUICKSTART.md) | [Wiki](wiki/Home.md)

Helius is a local first personal finance tracker with a Rust CLI/TUI and SQLite storage. It runs as a single binary and keeps data in one local database file.

Supported targets: Windows x86_64 and Linux x86_64.

![heliusdemo](https://github.com/user-attachments/assets/fa79b536-9784-4c22-8a9b-402cfaa5efda)

## Features

- Full-screen terminal UI
- Direct CLI commands
- SQLite storage in a single local database
- Accounts, categories, income, expense, and transfer transactions
- Recurring rules, reconciliation, budgets, and cash-flow planning
- JSON output for scripting and CSV export for reporting

## Installation

### Option 1: Download a release binary

1. Open the [GitHub Releases](https://github.com/STVR393/helius-personal-finance-tracker/releases) page.
2. Download the archive for your platform:
   - Windows x86_64: `helius-v0.1.4-windows-x86_64.zip`
   - Linux x86_64: `helius-v0.1.4-linux-x86_64.tar.gz`
3. Extract the archive into a folder you keep for apps or tools.
4. Launch the binary for your platform:

```powershell
.\helius.exe
```

```bash
./helius
```

On first run, if no database exists, Helius prompts for a 3-letter currency code and initializes the default database for the current platform.

### Option 2: Build from source

Requirements:

- Rust stable toolchain
- Windows or Linux

Clone the repository, then build:

```powershell
cargo build --release
```

The compiled binary is written to one of:

```text
target\release\helius.exe
target/release/helius
```

### Option 3: Install from a checkout

```powershell
cargo install --path .
```

### Option 4: Run in Docker

The container stores its database at `/data/tracker.db`.

Build the image locally:

```bash
docker build -t helius .
```

Create a named volume and start Helius:

```bash
docker volume create helius-data
docker run --rm -it -v helius-data:/data helius
```

Run direct commands the same way:

```bash
docker run --rm -v helius-data:/data helius balance
docker run --rm -v helius-data:/data helius tx list --limit 20
```

Use `-it` for the TUI or interactive shell.

## Usage

Start the terminal UI:

```powershell
helius
```

Open the guided shell:

```powershell
helius shell
```

Run direct commands:

```powershell
helius init --currency USD
helius balance
helius tx list --limit 20
```

If you are launching a binary directly instead of using `PATH`:

```powershell
.\helius.exe init --currency USD
.\helius.exe balance
```

```bash
./helius init --currency USD
./helius balance
```

On first run, `helius` can initialize the database automatically. You can also set it up explicitly:

```powershell
helius init --currency USD
helius account add Checking --type checking --opening-balance 1000.00
helius category add Salary --kind income
helius category add Groceries --kind expense
helius tx add --type income --amount 2500.00 --date 2026-03-01 --account Checking --category Salary --payee Employer
helius tx add --type expense --amount 42.50 --date 2026-03-02 --account Checking --category Groceries --payee Market
```

## TUI Controls

- `Tab` / `Shift+Tab`: switch top-level panels
- `j` / `k` or arrows: move selection
- `n`: create a new item in the active panel
- `e`: edit the selected item
- `d`: archive, delete, reset, or restore depending on panel context
- `Enter`: open, activate, or post the selected entry
- `?`: toggle help
- `q`: quit

Forms:

- `Tab` / `Shift+Tab`: move between fields
- `Enter`, `Ctrl+S`, or `F2`: save
- `Esc`: cancel

## Examples

Accounts and categories:

```powershell
helius account add "Cash" --type cash
helius account list
helius category add "Housing" --kind expense
helius category list
```

Transactions:

```powershell
helius tx add --type expense --amount 290.00 --date 2026-03-06 --account Cash --category Housing --payee Rent
helius tx list --limit 25
helius tx edit 12 --note "corrected note"
helius tx delete 12
helius tx restore 12
```

Budgets and summaries:

```powershell
helius budget set Groceries --month 2026-03 --amount 300.00 --account Checking
helius budget status 2026-03
helius summary month 2026-03
helius summary range --from 2026-03-01 --to 2026-03-31
```

Recurring rules:

```powershell
helius recurring add "Monthly Rent" --type expense --amount 290.00 --account Cash --category Housing --cadence monthly --day-of-month 6 --start-on 2026-03-17
helius recurring list
helius recurring run
```

Planning:

```powershell
helius forecast show
helius forecast bills
helius scenario add "Recovery Plan"
helius goal add "Cash Floor" --kind balance-target --account Checking --minimum-balance 100.00
```

## Storage

Default database locations:

```text
%LOCALAPPDATA%\Helius\tracker.db
~/.local/share/helius/tracker.db
```

Notes:

- Windows uses `%LOCALAPPDATA%\Helius\tracker.db`.
- Linux uses the platform-local application data directory from `directories::ProjectDirs`, commonly `~/.local/share/helius/tracker.db`.

Overrides:

- `--db <path>`
- `HELIUS_DB_PATH`

## Development

```powershell
cargo test
cargo build --release
```

## License

Copyright 2026 Kostas. This project is released under the GNU Affero General Public License v3.0.
See [LICENSE](LICENSE).

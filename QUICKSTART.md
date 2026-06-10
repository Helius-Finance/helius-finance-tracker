# Quick Start

## 1. Build

```powershell
cargo build --release
```

## 2. Initialize a Database

For a quick start from a checkout, use `cargo run --release --`.

```powershell
cargo run --release -- init --currency USD
```

## 3. Add Basic Data

```powershell
cargo run --release -- account add Checking --type checking --opening-balance 1000.00
cargo run --release -- category add Salary --kind income
cargo run --release -- category add Groceries --kind expense
```

## 4. Import a Statement

Preset-based CSV imports are the fastest path for supported exports:

```powershell
cargo run --release -- import csv --list-presets
cargo run --release -- import csv --input .\bank.csv --account Checking --preset revolut-csv --dry-run
cargo run --release -- import csv --input .\bank.csv --account Checking --preset revolut-csv
```

Manual CSV mapping and limited `camt053` imports are also available:

```powershell
cargo run --release -- import csv --input .\bank.csv --account Checking --date-column Date --amount-column Amount --description-column Description --expense-category Groceries
cargo run --release -- import camt053 --input .\statement.xml --account Checking --dry-run
```

## 5. Enter Transactions Manually

```powershell
cargo run --release -- tx add --type income --amount 2500.00 --date 2026-03-01 --account Checking --category Salary --payee Employer
cargo run --release -- tx add --type expense --amount 68.40 --date 2026-03-02 --account Checking --category Groceries --payee Supermarket
```

## 6. Open the TUI

```powershell
cargo run --release --
```

If you run the binary with no existing database, Helius can initialize the default database interactively.

## Useful Commands

```powershell
cargo run --release -- balance
cargo run --release -- tx list --limit 20
cargo run --release -- import csv --list-presets
cargo run --release -- summary month
cargo run --release -- recurring list
cargo run --release -- forecast show
```

## TUI Hotkeys

- `Tab` / `Shift+Tab`: switch panels
- `j` / `k` or arrows: move selection
- `n`: create
- `e`: edit
- `d`: archive, delete, reset, or restore
- `?`: help
- `q`: quit

Forms:

- `Tab` / `Shift+Tab`: next or previous field
- `Enter`, `Ctrl+S`, or `F2`: save
- `Esc`: cancel

## Database Path

Default locations:

```text
%LOCALAPPDATA%\Helius\tracker.db
~/.local/share/helius/tracker.db
```

Override it with:

```powershell
cargo run --release -- --db .\tracker.db balance
```

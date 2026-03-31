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

## 4. Enter Transactions

```powershell
cargo run --release -- tx add --type income --amount 2500.00 --date 2026-03-01 --account Checking --category Salary --payee Employer
cargo run --release -- tx add --type expense --amount 68.40 --date 2026-03-02 --account Checking --category Groceries --payee Supermarket
```

## 5. Open the TUI

```powershell
cargo run --release --
```

If you run the binary with no existing database, Helius can initialize the default database interactively.

## Useful Commands

```powershell
cargo run --release -- balance
cargo run --release -- tx list --limit 20
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

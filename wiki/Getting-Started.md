# Getting Started

## First Run

When Helius starts with no existing database, it can initialize one for you. The TUI prompts for a 3-letter currency code and creates the default database automatically.

You can also initialize it explicitly:

```powershell
helius init --currency USD
```

The default database path depends on the platform:

```text
%LOCALAPPDATA%\Helius\tracker.db
~/Library/Application Support/Helius/tracker.db
~/.local/share/helius/tracker.db
```

## Create Basic Data

Add an account and a few categories:

```powershell
helius account add Checking --type checking --opening-balance 1000.00 --opened-on 2026-03-01
helius category add Salary --kind income
helius category add Groceries --kind expense
```

Accepted account types:

- `cash`
- `checking`
- `savings`
- `credit`

Accepted category kinds:

- `income`
- `expense`

## Enter Transactions

```powershell
helius tx add --type income --amount 2500.00 --date 2026-03-01 --account Checking --category Salary --payee Employer
helius tx add --type expense --amount 68.40 --date 2026-03-02 --account Checking --category Groceries --payee Supermarket
```

Transfers use `--to-account` instead of a category:

```powershell
helius tx add --type transfer --amount 200.00 --date 2026-03-03 --account Checking --to-account Cash --note "ATM withdrawal"
```

## Open The TUI

```powershell
helius
```

## Check The Data

```powershell
helius balance
helius tx list --limit 20
helius summary month 2026-03
```

## Use The Guided Shell

If you prefer prompts over long commands:

```powershell
helius shell
```

The shell supports shortcuts such as `account`, `income`, `expense`, `budget`, `reconcile`, and `recurring`, and it also accepts raw CLI commands.

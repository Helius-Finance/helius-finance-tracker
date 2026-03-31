# CLI Reference

Helius exposes a direct CLI for scripting and automation. Most list and report commands also support `--json`.

## Global Option

```text
--db <path>
```

Use `--db` to target a specific SQLite file for any command.

## Accounts And Categories

```powershell
helius account add "Cash" --type cash
helius account list --json
helius account edit "Cash" --name "Wallet"
helius account delete "Wallet"

helius category add "Housing" --kind expense
helius category list --json
helius category edit "Housing" --name "Rent"
helius category delete "Rent"
```

## Transactions And Reporting

```powershell
helius tx add --type expense --amount 290.00 --date 2026-03-06 --account Checking --category Housing --payee Landlord
helius tx list --account Checking --limit 25
helius tx edit 12 --note "Corrected memo"
helius tx delete 12
helius tx restore 12

helius balance --json
helius summary month 2026-03 --json
helius summary range --from 2026-03-01 --to 2026-03-31 --json
```

Transaction kinds:

- `income`
- `expense`
- `transfer`

## Import And Export

Export transactions or summary data:

```powershell
helius export csv --kind transactions --output .\transactions.csv --month 2026-03
helius export csv --kind summary --output .\summary.csv --from 2026-03-01 --to 2026-03-31
```

Import a bank CSV:

```powershell
helius import csv --input .\bank.csv --account Checking --date-column Date --amount-column Amount --description-column Description --category-column Category --dry-run --json
helius import csv --input .\bank.csv --account Checking --date-column Date --amount-column Amount --description-column Description --category-column Category --json
```

Import notes:

- `--date-format` defaults to `%Y-%m-%d`
- `--delimiter` defaults to `,`
- Use `--category` when the CSV does not have a category column
- Use `--allow-duplicates` only when duplicate detection should be bypassed

## Budgets

```powershell
helius budget set Groceries --month 2026-03 --amount 300.00
helius budget list --month 2026-03 --json
helius budget status 2026-03 --json
helius budget delete Groceries --month 2026-03
```

## Planning

```powershell
helius scenario add "Stress Case" --note "Higher spending month"
helius scenario list --json

helius plan item add "Insurance" --type expense --amount 120.00 --date 2026-03-20 --account Checking --category Housing
helius plan item list --json
helius plan item post 3

helius goal add "Cash Floor" --kind balance-target --account Checking --minimum-balance 100.00
helius goal list --json

helius forecast show --days 90 --json
helius forecast bills --days 30 --json
```

Goal kinds:

- `sinking-fund`
- `balance-target`

## Reconciliation And Recurring Rules

```powershell
helius reconcile start --account Checking --to 2026-03-31 --statement-balance 3174.60 --transaction-id 10 --transaction-id 11 --transaction-id 12
helius reconcile list --json
helius reconcile delete 4

helius recurring add "Monthly Rent" --type expense --amount 900.00 --account Checking --category Housing --cadence monthly --day-of-month 6 --start-on 2026-03-01
helius recurring list --json
helius recurring pause 2
helius recurring resume 2
helius recurring run --through 2026-04-30
```

Recurring cadence values:

- `weekly`
- `monthly`

Weekly recurring rules accept weekday values:

- `mon`
- `tue`
- `wed`
- `thu`
- `fri`
- `sat`
- `sun`

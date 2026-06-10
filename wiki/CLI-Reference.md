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

Import using a preset (skips explicit column mapping):

```powershell
# List every preset, with a `verification` column.
helius import csv --list-presets
helius import csv --list-presets --json

# Import with a preset; column names, delimiter, and date format all auto-fill
helius import csv --input .\alpha.csv --account Checking --preset alpha-bank-gr --dry-run
```

Split-amount exports (separate debit + credit columns):

```powershell
helius import csv --input .\bank.csv --account Checking --debit-column Debit --credit-column Credit --description-column Description
```

Customize the uncategorized fallback:

```powershell
# Rows without an inline category land in these buckets instead of the
# built-in "Uncategorized Income" / "Uncategorized Expense".
helius import csv --input .\bank.csv --account Checking --preset revolut-csv \
  --income-category "Inbound" --expense-category "Outbound"
```

Import a camt.053 bank statement (ISO 20022 XML):

```powershell
helius import camt053 --input .\statement.xml --account Checking --dry-run --json
helius import camt053 --input .\statement.xml --account Checking \
  --income-category "Inbound" --expense-category "Outbound" --json
```

Import notes:

- `--date-format` defaults to `%Y-%m-%d`; presets override this automatically
- `--delimiter` defaults to `,`; presets override this (for example, Greek banks use `;`)
- Use `--category` when the CSV does not have a category column
- Use `--income-category` / `--expense-category` to replace the default
  `Uncategorized Income` / `Uncategorized Expense` buckets (also available from
  the TUI step-1 import form and for camt.053 imports)
- Use `--allow-duplicates` only when duplicate detection should be bypassed
- `--list-presets` works without an initialized database, so new users can
  browse the catalog before running `helius init`
- Imports run inside a database transaction: a failure on row 50 rolls back
  the preceding 49, and `--dry-run` always rolls back so no categories are
  created as a side effect

### Preset verification levels

| Level | Meaning |
| ----- | ------- |
| `first-party` | Project-maintained mapping with dedicated repository fixtures |
| `community` | Maintained mapping with repository fixture coverage; bank exports can still vary |

Community presets cover major banks in the US, UK, Germany, France, Spain,
Italy, Netherlands, and Australia (Chase, Bank of America, Barclays, HSBC,
Lloyds, Deutsche Bank, BNP Paribas, ING, Commonwealth Bank, and others). See
the full table in [Bank Import](Bank-Import.md). If your export differs, open an
issue with the header row; that is usually enough to patch the preset.

### camt.053 limitations

The camt.053 importer handles the common case of booked entries in a single
currency, and intentionally rejects or ignores the edge cases a one-person
finance tracker rarely needs. If a statement contains any of the following,
the importer either surfaces a clear error or skips the row — double-check
the preview before confirming.

- **Booked entries only.** `Sts != BOOK` (pending, reversal, information-only)
  entries are skipped.
- **No transfer detection.** `CdtDbtInd=CRDT` maps to income, `DBIT` to
  expense. Internal transfers between two of your own accounts must be
  reconciled manually after import.
- **No FX.** The statement currency must match the database currency. Mixed-
  currency statements are rejected with a clear error — split the export or
  convert amounts upstream.
- **No charges or fees applied inline.** `Chrgs` sub-elements are ignored; the
  raw `Amt` of each entry is what gets imported.
- **No reversal handling.** `RvslInd=true` entries are treated like any other
  booked entry. If your bank exports reversal pairs, expect duplicates and
  reconcile them manually.

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

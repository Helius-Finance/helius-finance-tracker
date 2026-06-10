# Bank Import

Helius imports bank data through two paths:

- CSV files, either with a named preset or with manual column mapping.
- ISO 20022 `camt.053` XML statements for booked entries in one currency.

Always run a dry run first. A dry run parses the file, resolves categories, checks duplicates, and shows the rows that would be written without changing the database.

## Quick Start

```powershell
# Browse every CSV preset. This works before `helius init`.
helius import csv --list-presets
helius import csv --list-presets --json

# Preview first, then import.
helius import csv --input .\statement.csv --account Checking --preset chase-us --dry-run
helius import csv --input .\statement.csv --account Checking --preset chase-us
```

For ISO 20022 XML statements:

```powershell
helius import camt053 --input .\statement.xml --account Checking --dry-run
helius import camt053 --input .\statement.xml --account Checking
```

## Verification Levels

`helius import csv --list-presets` shows a `Verification` column.

| Level | What it means |
| ----- | ------------- |
| `first-party` | Project-maintained mapping with dedicated repository fixtures. |
| `community` | Maintained mapping with repository fixture coverage, but bank exports can vary by region, account type, and product version. |

Every preset listed below has a fixture in `tests/fixtures/import/` and is exercised by integration tests. That fixture coverage proves the implemented mapping works for the headers and data shape stored in this repository. It does not guarantee that every export option from the bank uses the same format.

If an export fails because headers differ, run `helius import csv --list-presets --json` and compare the preset columns with the first row of your CSV. Open an issue with the header row only, without account numbers or transaction data.

## Supported Presets

| Preset ID | Bank/export | Region | Verification | Amount mode | Date format | Delimiter |
| --------- | ----------- | ------ | ------------ | ----------- | ----------- | --------- |
| `alpha-bank-gr` | Alpha Bank Greece CSV | Greece | `first-party` | split debit/credit | `%d/%m/%Y` | `;` |
| `eurobank-gr` | Eurobank Greece CSV | Greece | `first-party` | signed amount | `%d/%m/%Y` | `;` |
| `nbg-gr` | National Bank of Greece CSV | Greece | `first-party` | signed amount | `%d/%m/%Y` | `;` |
| `piraeus-gr` | Piraeus Bank Greece CSV | Greece | `first-party` | split debit/credit | `%d/%m/%Y` | `;` |
| `revolut-csv` | Revolut CSV | Pan-European | `first-party` | signed amount | `%Y-%m-%d` | `,` |
| `n26` | N26 Bank CSV | Europe | `community` | signed amount | `%Y-%m-%d` | `,` |
| `monzo` | Monzo CSV | United Kingdom | `community` | signed amount | `%d/%m/%Y` | `,` |
| `starling` | Starling Bank CSV | United Kingdom | `community` | signed amount | `%d/%m/%Y` | `,` |
| `wise` | Wise CSV | Pan-European | `community` | signed amount | `%d-%m-%Y` | `,` |
| `dkb-de` | DKB CSV | Germany | `community` | signed amount | `%d.%m.%y` | `;` |
| `commerzbank-de` | Commerzbank CSV | Germany | `community` | signed amount | `%d.%m.%Y` | `;` |
| `chase-us` | Chase CSV | United States | `community` | signed amount | `%m/%d/%Y` | `,` |
| `bank-of-america-us` | Bank of America CSV | United States | `community` | signed amount | `%m/%d/%Y` | `,` |
| `wells-fargo-us` | Wells Fargo CSV | United States | `community` | signed amount | `%m/%d/%Y` | `,` |
| `citi-us` | Citi CSV | United States | `community` | split debit/credit | `%m/%d/%Y` | `,` |
| `barclays-uk` | Barclays CSV | United Kingdom | `community` | signed amount | `%d/%m/%Y` | `,` |
| `hsbc-uk` | HSBC CSV | United Kingdom | `community` | split debit/credit | `%d/%m/%Y` | `,` |
| `lloyds-uk` | Lloyds Bank CSV | United Kingdom | `community` | split debit/credit | `%d/%m/%Y` | `,` |
| `natwest-uk` | NatWest / RBS CSV | United Kingdom | `community` | signed amount | `%d/%m/%Y` | `,` |
| `ing-nl` | ING Netherlands CSV | Netherlands | `community` | signed amount | `%d-%m-%Y` | `;` |
| `deutsche-bank-de` | Deutsche Bank CSV | Germany | `community` | signed amount | `%d.%m.%Y` | `;` |
| `bnp-paribas-fr` | BNP Paribas CSV | France | `community` | split debit/credit | `%d/%m/%Y` | `;` |
| `santander-es` | Santander Spain CSV | Spain | `community` | signed amount | `%d/%m/%Y` | `;` |
| `intesa-sanpaolo-it` | Intesa Sanpaolo CSV | Italy | `community` | signed amount | `%d/%m/%Y` | `;` |
| `commbank-au` | Commonwealth Bank CSV | Australia | `community` | signed amount | `%d/%m/%Y` | `,` |

## How CSV Presets Work

A preset supplies:

- The delimiter.
- The date format.
- The required date, amount, and description columns.
- Optional category, payee, note, and type columns.
- Header aliases for common export variants.
- The amount mode: signed amount or separate debit and credit columns.

Preset IDs are matched case-insensitively. CSV headers are trimmed and matched case-insensitively against the configured column name and aliases.

Manual flags override preset defaults:

```powershell
helius import csv --input .\bank.csv --account Checking --preset revolut-csv `
  --date-column "Completed Date" --amount-column "Base amount"
```

Without a preset, the default CSV mapping is:

| Field | Default |
| ----- | ------- |
| Date column | `Date` |
| Amount column | `Amount` |
| Description column | `Description` |
| Date format | `%Y-%m-%d` |
| Delimiter | `,` |

## Manual CSV Mapping

Use manual mapping when a preset is close but not exact, or when the bank is not listed.

Signed amount files:

```powershell
helius import csv --input .\bank.csv --account Checking `
  --date-column Date `
  --amount-column Amount `
  --description-column Description `
  --dry-run
```

Split debit/credit files:

```powershell
helius import csv --input .\bank.csv --account Checking `
  --date-column Date `
  --debit-column Debit `
  --credit-column Credit `
  --description-column Description `
  --dry-run
```

You must use either `--amount-column` or the `--debit-column` / `--credit-column` pair, not both. Split files must have exactly one side filled per row. A row with both debit and credit values, or with neither value, is rejected.

## Transaction Type And Amounts

For signed amount files:

- Negative amounts import as expenses.
- Positive amounts import as income.
- The stored transaction amount is always positive cents; the transaction type controls whether it affects the balance as money in or money out.

Use `--type-column` when the CSV has a transaction type column. Supported type values are:

| Imported value | Helius type |
| -------------- | ----------- |
| `income`, `credit`, `deposit`, `inflow`, `crdt` | `income` |
| `expense`, `debit`, `withdrawal`, `outflow`, `dbit` | `expense` |

`transfer` is recognized by the parser but rejected by the import flow in this release. Import transfers manually so both accounts are linked correctly.

Use `--default-type income` or `--default-type expense` only when all rows in a signed amount file should use that type regardless of sign.

Amount parsing accepts common US and European formats, including:

- `-15.25`
- `-15,25`
- `1,234.56`
- `1.234,56`
- `(15.25)`

## Categories, Payees, And Notes

Category resolution uses the first available value in this order:

1. A CSV category column, when configured and non-empty.
2. `--income-category` or `--expense-category`, based on the transaction type.
3. `--category`, based on the transaction type.
4. `Uncategorized Income` or `Uncategorized Expense`.

Missing categories are created automatically with the correct income or expense kind. If a category was archived, import restores it when needed.

Payee resolution:

- If a payee column is configured and the row value is non-empty, that value is used.
- Otherwise the description is used as the payee.

Note resolution:

- If a note column is configured and the row value is non-empty, that value is used.
- Otherwise the note is blank.

## Dry Runs, Duplicates, And Rollback

Imports run inside a database transaction.

- `--dry-run` always rolls back, including any categories that would have been created.
- A real import commits only if every row succeeds.
- If any row fails validation, the whole import rolls back.

Duplicate detection is exact and only checks active transactions. A row is a duplicate when an existing transaction has the same date, type, amount, account, category, payee, and note. Duplicate rows are shown in the preview and skipped by default.

Use `--allow-duplicates` when the file intentionally contains rows that already exist.

## TUI Import Flow

The terminal UI uses a two-step import flow:

1. The setup form builds a CSV or `camt053` request.
2. Helius runs a dry-run preview and shows the resolved rows.
3. Confirming the preview reuses the resolved import plan and commits the non-duplicate rows.

This is the same importer used by the CLI. The TUI does not have a separate parser.

## camt.053 XML Imports

`helius import camt053` supports the common single-currency booked-entry case:

- Only booked entries are imported.
- `CRDT` entries become income.
- `DBIT` entries become expenses.
- Booking date is preferred; value date is used when booking date is missing.
- Transaction details are flattened when the entry contains `TxDtls`.
- The statement currency must match the database currency.

The importer intentionally does not detect transfers, apply inline fees, handle FX conversion, or special-case reversals. See [CLI Reference](CLI-Reference#camt053-limitations) for the full list.

## Maintaining Presets

When adding or changing a preset:

1. Add or update the mapping in `src/importer.rs`.
2. Add a fixture under `tests/fixtures/import/`.
3. Add the preset to the table in this page.
4. Run `cargo test`.

The integration test `community_presets_import_fixture_rows` exercises every community preset. First-party presets also have dedicated fixture tests.

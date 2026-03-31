# Planning And Forecasts

Helius includes a planning layer on top of recorded transactions. The main pieces are scenarios, planning items, goals, budgets, recurring rules, and forecasts.

## Forecast Basics

Use these commands to inspect projected cash flow:

```powershell
helius forecast show
helius forecast show --days 90 --json
helius forecast bills --days 30 --json
```

Forecast options:

- `--scenario <name>` applies a planning scenario
- `--account <name>` narrows the forecast to one account
- `forecast show` defaults to 90 days
- `forecast bills` defaults to 30 days

## Scenarios

Scenarios let you compare a baseline plan against an alternative case.

```powershell
helius scenario add "Downside" --note "Emergency spending month"
helius scenario list --json
helius scenario edit 1 --name "Stress Case"
helius scenario delete 1
```

## Planning Items

Planning items are future transactions that have not been posted yet.

```powershell
helius plan item add "Emergency Repair" --scenario Downside --type expense --amount 700.00 --date 2026-03-24 --account Checking --category Housing
helius plan item list --scenario Downside --json
helius plan item post 5
```

Posting a planning item creates a real transaction from it.

## Goals

Helius supports two goal types:

- `sinking-fund`
- `balance-target`

Examples:

```powershell
helius goal add "Vacation Fund" --kind sinking-fund --account Checking --target-amount 1500.00 --due-on 2026-08-01
helius goal add "Cash Floor" --kind balance-target --account Checking --minimum-balance 250.00
helius goal list --json
```

## Budgets And Scenario Overrides

Budgets can be recorded for the baseline plan or overridden inside a scenario:

```powershell
helius budget set Groceries --month 2026-03 --amount 200.00 --account Checking
helius budget set Groceries --month 2026-03 --amount 500.00 --scenario "Stress Case"
helius budget status 2026-03 --scenario "Stress Case" --json
```

This is useful when a scenario should change expected spending without changing your baseline data.

## Recurring Rules In Forecasts

Recurring rules are included in forecasts and can also be posted into real transactions:

```powershell
helius recurring add "Gym" --type expense --amount 35.00 --account Checking --category Misc --cadence monthly --day-of-month 15 --start-on 2026-03-01
helius recurring run --through 2026-04-30
```

Important behavior:

- `forecast show` is read-only and does not advance a recurring rule's stored next due date
- `recurring run` is what actually posts due recurring entries into the ledger

## Reconciliation Interaction

Once transactions are part of a reconciliation, Helius blocks edits and deletes until that reconciliation is removed. This keeps the ledger stable after statement matching.

# TUI And Shell

Helius supports both a full-screen TUI and a guided interactive shell. Both operate on the same database and expose the same underlying workflow.

## TUI Views

The top-level TUI panels are:

- Dashboard
- Transactions
- Accounts
- Categories
- Summary
- Budgets
- Planning
- Recurring
- Reconcile

The Planning panel is split into these subviews:

- Forecast
- Calendar
- Goals
- Scenarios

## Core TUI Keys

- `Tab` / `Shift+Tab`: move between top-level panels or form fields
- `j` / `k` or arrows: move the selection
- `n`: create a new item in the active panel
- `e`: edit the selected item
- `d`: archive, delete, reset, or restore depending on context
- `Enter`: open, activate, or post the selected entry
- `?`: toggle help
- `q`: quit

Form keys:

- `Enter`, `Ctrl+S`, or `F2`: save
- `Esc`: cancel

## Interactive Shell

Start it with:

```powershell
helius shell
```

The shell includes guided shortcuts for:

- `init`
- `account`
- `category`
- `income`
- `expense`
- `transfer`
- `balance`
- `transactions`
- `summary`
- `budget`
- `reconcile`
- `recurring`

You can also type raw commands directly inside the shell, for example:

```text
tx list --account Checking
budget status 2026-03
recurring list
```

## When To Use Which Interface

- Use the TUI for day-to-day browsing, editing, and overview screens.
- Use the shell if you want prompts but do not want the full-screen interface.
- Use the CLI when you need scripting, JSON output, or repeatable commands.

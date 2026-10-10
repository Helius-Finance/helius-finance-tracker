<div align="center">

# Helius

**Personal finance in your terminal. Local-first, private, and fast.**

A Rust TUI and CLI for tracking money, with everything stored in one SQLite file that you own.

[![Latest release](https://img.shields.io/github/v/release/Helius-Finance/helius-finance-tracker?style=flat-square&color=f59e0b&label=release)](https://github.com/Helius-Finance/helius-finance-tracker/releases/latest)
[![CI](https://img.shields.io/github/actions/workflow/status/Helius-Finance/helius-finance-tracker/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/Helius-Finance/helius-finance-tracker/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-3b82f6?style=flat-square)](LICENSE)
![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20Linux-64748b?style=flat-square)
![Built with Rust](https://img.shields.io/badge/built%20with-Rust-dea584?style=flat-square&logo=rust&logoColor=white)

[**Install**](#installation) &nbsp;·&nbsp;
[**Quick Start**](QUICKSTART.md) &nbsp;·&nbsp;
[**Wiki**](wiki/Home.md) &nbsp;·&nbsp;
[**Bank Import**](wiki/Bank-Import.md)

<br>

<img src="https://github.com/user-attachments/assets/fa79b536-9784-4c22-8a9b-402cfaa5efda" alt="Helius terminal UI demo" width="880">

</div>

<br>

## Why Helius

<table>
<tr>
<td width="33%" valign="top">

#### Local-first
Your data lives in one SQLite file on your own machine. You don't need an account or a cloud service.

</td>
<td width="33%" valign="top">

#### Single binary
One command installs it, with no runtime to set up. The same binary runs the TUI, the guided shell, and the CLI.

</td>
<td width="33%" valign="top">

#### Bank import
**25 CSV presets** for banks in the US, UK, and Europe, plus ISO 20022 `camt.053` XML statements.

</td>
</tr>
<tr>
<td width="33%" valign="top">

#### Budgets and recurring bills
Set monthly budgets, schedule recurring bills, and reconcile accounts against your bank statements.

</td>
<td width="33%" valign="top">

#### Forecasts and goals
Project cash flow 90 days ahead, compare what-if scenarios, and track savings goals.

</td>
<td width="33%" valign="top">

#### Scriptable
JSON output for automation and CSV export for spreadsheets and reports.

</td>
</tr>
</table>

## Quick start

Install with one command:

```bash
# Linux
curl -fsSL https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.sh | sh
```

```powershell
# Windows (PowerShell)
irm https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.ps1 | iex
```

Then run `helius`. On first launch, Helius asks for a 3-letter currency code and creates your database. You can also set up from the command line:

```bash
helius init --currency USD
helius account add Checking --type checking --opening-balance 1000.00
helius import csv --input statement.csv --account Checking --preset chase-us --dry-run
helius balance
```

## Installation

The one-command installer downloads the latest release, verifies its SHA-256 checksum, and installs `helius` into `~/.local/bin` on Linux or `%LOCALAPPDATA%\Programs\Helius` on Windows. The Windows installer also adds that folder to your user `PATH`. Run the same command again to upgrade.

> [!NOTE]
> The Linux binary needs glibc 2.34 or newer. On older distributions or musl-based systems such as Alpine, use Docker or build from source.

<details>
<summary><b>Install a specific version or into a different folder</b></summary>
<br>

Set `HELIUS_VERSION` or `HELIUS_INSTALL_DIR` before running the installer:

```bash
curl -fsSL https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.sh | HELIUS_VERSION=v1.4.4 sh
```

```powershell
$env:HELIUS_VERSION = "v1.4.4"; irm https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.ps1 | iex
```

</details>

<details>
<summary><b>Download a release archive manually</b></summary>
<br>

| Platform | Archive |
| :-- | :-- |
| **Windows** x86_64 | [`helius-v1.4.4-windows-x86_64.zip`](https://github.com/Helius-Finance/helius-finance-tracker/releases/latest) |
| **Linux** x86_64 | [`helius-v1.4.4-linux-x86_64.tar.gz`](https://github.com/Helius-Finance/helius-finance-tracker/releases/latest) |

Extract the archive into a folder you keep for tools, then run `helius` (`.\helius.exe` on Windows). To run it from any directory, add that folder to your `PATH`.

</details>

<details>
<summary><b>Build from source</b></summary>
<br>

Requires the stable Rust toolchain.

```bash
git clone https://github.com/Helius-Finance/helius-finance-tracker.git
cd helius-finance-tracker
cargo build --release      # binary: target/release/helius
```

Or install it into your Cargo bin directory:

```bash
cargo install --path .
```

</details>

<details>
<summary><b>Run in Docker</b></summary>
<br>

The container stores its database at `/data/tracker.db`.

```bash
docker build -t helius .
docker volume create helius-data

docker run --rm -it -v helius-data:/data helius            # TUI
docker run --rm -v helius-data:/data helius balance        # direct command
```

Use `-it` for the TUI and the interactive shell.

</details>

## Three ways to use it

| Mode | Command | Best for |
| :-- | :-- | :-- |
| **Terminal UI** | `helius` | Everyday browsing and editing, with dashboard, budgets, planning, and more |
| **Guided shell** | `helius shell` | Step-by-step prompts when you don't want to remember flags |
| **Direct CLI** | `helius <command>` | Scripts, automation, and `--json` output |

### Everyday commands

```bash
helius balance                       # balances across accounts
helius tx list --limit 20            # recent transactions
helius summary month 2026-03         # income vs. expenses for a month
helius budget status 2026-03         # budget progress
helius forecast show                 # 90-day cash-flow forecast
helius forecast bills                # bills due in the next 30 days
```

<details>
<summary><b>Accounts, categories & transactions</b></summary>
<br>

```bash
helius account add Cash --type cash
helius category add Groceries --kind expense
helius category add Salary --kind income

helius tx add --type income  --amount 2500.00 --date 2026-03-01 --account Checking --category Salary    --payee Employer
helius tx add --type expense --amount 42.50   --date 2026-03-02 --account Checking --category Groceries --payee Market

helius tx edit 12 --note "corrected note"
helius tx delete 12
helius tx restore 12
```

</details>

<details>
<summary><b>Budgets, recurring rules & reconciliation</b></summary>
<br>

```bash
helius budget set Groceries --month 2026-03 --amount 300.00 --account Checking

helius recurring add "Monthly Rent" --type expense --amount 900.00 --account Checking \
  --category Housing --cadence monthly --day-of-month 6 --start-on 2026-03-01
helius recurring run --through 2026-04-30

helius reconcile start --account Checking --to 2026-03-31 --statement-balance 3174.60 \
  --transaction-id 10 --transaction-id 11
```

</details>

<details>
<summary><b>Planning, scenarios & goals</b></summary>
<br>

```bash
helius forecast show --days 90
helius scenario add "Recovery Plan"
helius goal add "Cash Floor" --kind balance-target --account Checking --minimum-balance 100.00
```

</details>

<details>
<summary><b>Export & scripting</b></summary>
<br>

```bash
helius export csv --kind transactions --output transactions.csv --month 2026-03
helius balance --json
helius forecast bills --days 30 --json
```

</details>

## Bank import

Import statements with a preset, map CSV columns yourself, or load `camt.053` XML. **Always preview with `--dry-run` first.** A dry run shows exactly what would be written without touching your database.

```bash
helius import csv --list-presets
helius import csv --input statement.csv --account Checking --preset revolut-csv --dry-run
helius import camt053 --input statement.xml --account Checking --dry-run
```

<details>
<summary><b>Supported banks (25 presets)</b></summary>
<br>

| Region | Banks |
| :-- | :-- |
| United States | Chase, Bank of America, Wells Fargo, Citi |
| United Kingdom | Barclays, HSBC, Lloyds, NatWest / RBS, Monzo, Starling |
| Germany | Deutsche Bank, Commerzbank, DKB |
| Greece | Alpha Bank, Eurobank, National Bank of Greece, Piraeus |
| Pan-European | Revolut, Wise, N26 |
| Netherlands, France, Spain, Italy | ING, BNP Paribas, Santander, Intesa Sanpaolo |
| Australia | Commonwealth Bank |

Preset IDs, verification levels, and manual column mapping are covered in the [Bank Import guide](wiki/Bank-Import.md).

</details>

## Keyboard shortcuts

| Key | Action |
| :-- | :-- |
| <kbd>Tab</kbd> / <kbd>Shift</kbd>+<kbd>Tab</kbd> | Switch panels, or move between form fields |
| <kbd>j</kbd> <kbd>k</kbd> or <kbd>↑</kbd> <kbd>↓</kbd> | Move selection |
| <kbd>n</kbd> / <kbd>e</kbd> | New / edit item |
| <kbd>d</kbd> | Archive, delete, reset, or restore |
| <kbd>Enter</kbd> | Open, activate, or post. Saves when in a form |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> or <kbd>F2</kbd> / <kbd>Esc</kbd> | Save / cancel a form |
| <kbd>?</kbd> / <kbd>q</kbd> | Help / quit |

## Your data

| Platform | Default database |
| :-- | :-- |
| Windows | `%LOCALAPPDATA%\Helius\tracker.db` |
| Linux | `~/.local/share/helius/tracker.db` |
| Docker | `/data/tracker.db` |

Override the location with `--db <path>` or the `HELIUS_DB_PATH` environment variable.

> [!TIP]
> Your whole ledger is a single file. To back it up, close Helius and copy `tracker.db` somewhere safe.

## Documentation

| Guide | What's inside |
| :-- | :-- |
| [Installation](wiki/Installation.md) | Every install option in detail |
| [Getting Started](wiki/Getting-Started.md) | Your first database, accounts, and transactions |
| [CLI Reference](wiki/CLI-Reference.md) | Every command and flag |
| [TUI and Shell](wiki/TUI-and-Shell.md) | Panels, shortcuts, and the guided shell |
| [Bank Import](wiki/Bank-Import.md) | Presets, manual mapping, and `camt.053` |
| [Planning and Forecasts](wiki/Planning-and-Forecasts.md) | Forecasts, scenarios, and goals |
| [Data and Storage](wiki/Data-and-Storage.md) | Database location, backups, and migrations |

## Contributing

Bug reports, bank presets, and pull requests are welcome. Start with [CONTRIBUTING.md](CONTRIBUTING.md), and report security issues as described in [SECURITY.md](SECURITY.md).

```bash
cargo test
cargo build --release
```

## License

Copyright 2026 Helius Finance. Released under the [GNU Affero General Public License v3.0](LICENSE).

## Star history

<a href="https://www.star-history.com/?repos=Helius-Finance%2Fhelius-finance-tracker&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=Helius-Finance/helius-finance-tracker&type=date&theme=dark&legend=bottom-right" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=Helius-Finance/helius-finance-tracker&type=date&legend=bottom-right" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=Helius-Finance/helius-finance-tracker&type=date&legend=bottom-right" />
 </picture>
</a>

<div align="center">
<br>
<sub><a href="#helius">Back to top</a></sub>
</div>

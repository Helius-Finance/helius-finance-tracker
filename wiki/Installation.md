# Installation

Helius ships as a single executable on Windows and Linux. No separate database server or background service is required.

## Option 1: Install With One Command

Linux x86_64:

```bash
curl -fsSL https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.sh | sh
```

Windows x86_64, in PowerShell:

```powershell
irm https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.ps1 | iex
```

The installer:

1. Finds the latest GitHub release.
2. Downloads the archive for your platform and verifies its SHA-256 checksum.
3. Checks that the binary runs on this machine.
4. Installs it into `~/.local/bin/helius` on Linux or
   `%LOCALAPPDATA%\Programs\Helius\helius.exe` on Windows.

On Windows, the installer also adds the install folder to your user `PATH`. On
Linux, it prints the line to add to your shell profile if `~/.local/bin` is not
on `PATH` yet. Run the same command again to upgrade.

Environment overrides:

- `HELIUS_VERSION`: install a specific release tag, such as `v1.4.4`
- `HELIUS_INSTALL_DIR`: install into a different folder

```bash
curl -fsSL https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.sh | HELIUS_VERSION=v1.4.4 sh
```

```powershell
$env:HELIUS_VERSION = "v1.4.4"; irm https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.ps1 | iex
```

The Linux binary needs glibc 2.34 or newer. On older distributions or
musl-based systems such as Alpine, use Docker or build from source.

To uninstall, delete the installed binary. On Windows, also remove the folder
from your user `PATH`. Your database is stored separately and is not removed;
see [Data and Storage](Data-and-Storage).

## Option 2: Download A Release

1. Open the [GitHub Releases](https://github.com/STVR393/helius-personal-finance-tracker/releases) page.
2. Download the latest archive for your platform.
3. Extract it into a folder you keep for tools, such as `C:\Tools\Helius\` or `~/bin/helius/`.
4. Run:

```powershell
helius --help
```

If you want to start Helius from any terminal, add that folder to your `PATH`.

## Option 3: Build From Source

Requirements:

- Rust stable
- Windows or Linux

Build a release binary:

```powershell
cargo build --release
```

The compiled binary is written to one of:

```text
target\release\helius.exe
target/release/helius
```

## Option 4: Install From A Local Checkout

If you want Cargo to install the command into your Cargo bin directory:

```powershell
cargo install --path .
```

## Option 5: Run In Docker

Docker is optional. The container stores the database at `/data/tracker.db`, so
mount `/data` if you want the data to persist.

Build the image:

```bash
docker build -t helius .
```

Create a named volume and start Helius:

```bash
docker volume create helius-data
docker run --rm -it -v helius-data:/data helius
```

Run direct commands:

```bash
docker run --rm -v helius-data:/data helius balance
docker run --rm -v helius-data:/data helius tx list --limit 20
```

Use `-it` for the TUI or interactive shell.

## Verify The Binary

Either run the installed command:

```powershell
helius --help
```

Or run the executable directly:

```powershell
.\helius.exe --help
```

On Linux:

```bash
./helius --help
```

## Custom Database Location

To point Helius at a specific database file:

```powershell
helius --db /path/to/tracker.db balance
```

You can also set the `HELIUS_DB_PATH` environment variable if you want a persistent override.

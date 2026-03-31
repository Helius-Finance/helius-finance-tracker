# Installation

Helius ships as a single executable on Windows and Linux. No separate database server or background service is required.

## Option 1: Download A Release

1. Open the [GitHub Releases](https://github.com/STVR393/helius-personal-finance-tracker/releases) page.
2. Download the latest archive for your platform.
3. Extract it into a folder you keep for tools, such as `C:\Tools\Helius\` or `~/bin/helius/`.
4. Run:

```powershell
helius --help
```

If you want to start Helius from any terminal, add that folder to your `PATH`.

## Option 2: Build From Source

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

## Option 3: Install From A Local Checkout

If you want Cargo to install the command into your Cargo bin directory:

```powershell
cargo install --path .
```

## Option 4: Run In Docker

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

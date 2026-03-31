# Release Process

This repository does not push release tags or publish GitHub release assets
until local verification is green on the host platform used for the change.

## Local Verification First

Release verification scripts live in `scripts/` and are PowerShell-based.

- Windows: use Windows PowerShell 5.1 or PowerShell 7. If script execution is
  blocked, start a session with `powershell -NoProfile -ExecutionPolicy Bypass`.
- Linux: use PowerShell 7 (`pwsh`).

Run the host verification script before pushing release-related changes:

```powershell
./scripts/verify-host-release.ps1 -Platform windows-x86_64
```

On Linux:

```powershell
pwsh ./scripts/verify-host-release.ps1 -Platform linux-x86_64
```

Linux host verification expects an `x86_64-unknown-linux-gnu` toolchain and is
intended for an x86_64 Linux host.

## What The Host Verification Script Does

`verify-host-release.ps1` runs the local checks that should be green before a
push:

1. `cargo test --locked`
2. `cargo build --release --locked`
3. `scripts/smoke.ps1` against the built binary
4. `scripts/package-release.ps1` for the target artifact layout
5. `scripts/verify-package.ps1` against the packaged archive

The helper scripts refuse to use the repository root or a filesystem root as a
temporary packaging or extraction directory.

## Release Artifacts

The repository is set up to publish these artifacts:

- `helius-<version>-windows-x86_64.zip`
- `helius-<version>-linux-x86_64.tar.gz`

Each artifact includes:

- the `helius` binary for that platform
- `LICENSE`
- `README.md`
- a `.sha256.txt` checksum file alongside the archive

## GitHub Actions

After local verification is green, the GitHub workflows provide the supported
platform confirmation:

- `.github/workflows/ci.yml`: test, build, and smoke-check on Windows and Ubuntu
- `.github/workflows/release.yml`: package the release archives, smoke-check
  the packaged artifacts, and publish assets on a version tag

Do not push tags or publish assets until the local host verification path has
already passed.

## Manual Checks Before A Tag

Before creating a release tag:

- verify the local host release script passes on the platform you changed
- confirm `cargo test --locked` is green
- confirm the packaged archive extracts into a top-level `helius/` directory
- confirm the smoke flow succeeds against the extracted archive
- confirm docs still describe Windows and Linux consistently

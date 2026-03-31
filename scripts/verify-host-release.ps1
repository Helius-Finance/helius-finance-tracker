param(
    [ValidateSet("windows-x86_64", "linux-x86_64")]
    [string]$Platform,
    [string]$Version = "local",
    [string]$OutputRoot = "./tmp_release_check",
    [string]$BinaryPath,
    [switch]$SkipTests,
    [switch]$SkipBuild
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path $PSScriptRoot -Parent

function Resolve-FullPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$PathValue
    )

    if ([System.IO.Path]::IsPathRooted($PathValue)) {
        return [System.IO.Path]::GetFullPath($PathValue)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $repoRoot $PathValue))
}

function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Command,
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed: $Command $($Arguments -join ' ')"
    }
}

function Get-HostPlatform {
    if ([System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Windows)) {
        return "windows-x86_64"
    }
    if ([System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Linux)) {
        return "linux-x86_64"
    }
    throw "Unsupported host platform."
}

function Get-DefaultBinaryPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$PlatformValue
    )

    switch ($PlatformValue) {
        "windows-x86_64" { return (Join-Path $repoRoot "target/release/helius.exe") }
        "linux-x86_64" { return (Join-Path $repoRoot "target/x86_64-unknown-linux-gnu/release/helius") }
        default { throw "A binary path must be provided for $PlatformValue." }
    }
}

if (-not $Platform) {
    $Platform = Get-HostPlatform
}

$resolvedOutputRoot = Resolve-FullPath $OutputRoot
$binaryName = if ($Platform -eq "windows-x86_64") { "helius.exe" } else { "helius" }

if (-not $SkipTests) {
    Invoke-CheckedCommand -Command "cargo" -Arguments @("test", "--locked")
}

if ($BinaryPath) {
    $resolvedBinaryPath = (Resolve-Path $BinaryPath).Path
} else {
    switch ($Platform) {
        "windows-x86_64" {
            if (-not $SkipBuild) {
                Invoke-CheckedCommand -Command "cargo" -Arguments @("build", "--release", "--locked")
            }
            $resolvedBinaryPath = Resolve-FullPath (Get-DefaultBinaryPath $Platform)
        }
        "linux-x86_64" {
            if (-not $SkipBuild) {
                Invoke-CheckedCommand -Command "cargo" -Arguments @("build", "--release", "--locked", "--target", "x86_64-unknown-linux-gnu")
            }
            $resolvedBinaryPath = Resolve-FullPath (Get-DefaultBinaryPath $Platform)
        }
    }
}

& (Join-Path $PSScriptRoot "smoke.ps1") -BinaryPath $resolvedBinaryPath

$package = & (Join-Path $PSScriptRoot "package-release.ps1") `
    -BinaryPath $resolvedBinaryPath `
    -Version $Version `
    -Platform $Platform `
    -OutputRoot $resolvedOutputRoot

$verification = & (Join-Path $PSScriptRoot "verify-package.ps1") `
    -ArchivePath $package.ArchivePath `
    -BinaryName $binaryName

[pscustomobject]@{
    Platform = $Platform
    ArchivePath = $package.ArchivePath
    ChecksumPath = $package.ChecksumPath
    VerificationStatus = $verification.Status
}

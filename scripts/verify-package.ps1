param(
    [Parameter(Mandatory = $true)]
    [string]$ArchivePath,
    [Parameter(Mandatory = $true)]
    [string]$BinaryName,
    [string]$ExtractRoot
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

$resolvedArchivePath = (Resolve-Path $ArchivePath).Path

$deleteExtractRootOnExit = $false
if (-not $ExtractRoot) {
    $ExtractRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("helius-verify-" + [Guid]::NewGuid().ToString("N"))
    $deleteExtractRootOnExit = $true
}

$resolvedExtractRoot = Resolve-FullPath $ExtractRoot
$resolvedRepoRoot = Resolve-FullPath $repoRoot
$pathRoot = [System.IO.Path]::GetPathRoot($resolvedExtractRoot)
if ($resolvedExtractRoot -eq $resolvedRepoRoot) {
    throw "Refusing to use the repository root as the extraction directory."
}
if ($resolvedExtractRoot -eq $pathRoot) {
    throw "Refusing to use a filesystem root as the extraction directory."
}

if (Test-Path $resolvedExtractRoot) {
    Remove-Item -Path $resolvedExtractRoot -Recurse -Force
}

New-Item -ItemType Directory -Path $resolvedExtractRoot -Force | Out-Null

try {
    if ($resolvedArchivePath.EndsWith(".zip", [System.StringComparison]::OrdinalIgnoreCase)) {
        Expand-Archive -Path $resolvedArchivePath -DestinationPath $resolvedExtractRoot
    } elseif ($resolvedArchivePath.EndsWith(".tar.gz", [System.StringComparison]::OrdinalIgnoreCase)) {
        & tar -C $resolvedExtractRoot -xzf $resolvedArchivePath
        if ($LASTEXITCODE -ne 0) {
            throw "tar failed while extracting $resolvedArchivePath"
        }
    } else {
        throw "Unsupported archive format: $resolvedArchivePath"
    }

    $binaryPath = Join-Path (Join-Path $resolvedExtractRoot "helius") $BinaryName
    & (Join-Path $PSScriptRoot "smoke.ps1") -BinaryPath $binaryPath

    [pscustomobject]@{
        Status = "verified"
        ArchivePath = $resolvedArchivePath
        ExtractRoot = if ($deleteExtractRootOnExit) { $null } else { $resolvedExtractRoot }
        BinaryPath = if ($deleteExtractRootOnExit) { $null } else { $binaryPath }
    }
}
finally {
    if ($deleteExtractRootOnExit) {
        Remove-Item -Path $resolvedExtractRoot -Recurse -Force -ErrorAction SilentlyContinue
    }
}

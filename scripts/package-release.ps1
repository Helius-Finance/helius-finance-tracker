param(
    [Parameter(Mandatory = $true)]
    [string]$BinaryPath,
    [Parameter(Mandatory = $true)]
    [string]$Version,
    [Parameter(Mandatory = $true)]
    [ValidateSet("windows-x86_64", "linux-x86_64")]
    [string]$Platform,
    [string]$OutputRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path $PSScriptRoot -Parent
$resolvedBinaryPath = (Resolve-Path $BinaryPath).Path

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

if (-not $OutputRoot) {
    $OutputRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("helius-package-" + [Guid]::NewGuid().ToString("N"))
}

$stageRoot = Resolve-FullPath $OutputRoot
$resolvedRepoRoot = Resolve-FullPath $repoRoot
$pathRoot = [System.IO.Path]::GetPathRoot($stageRoot)
if ($stageRoot -eq $resolvedRepoRoot) {
    throw "Refusing to use the repository root as the packaging output directory."
}
if ($stageRoot -eq $pathRoot) {
    throw "Refusing to use a filesystem root as the packaging output directory."
}
if (Test-Path $stageRoot) {
    Remove-Item -Path $stageRoot -Recurse -Force
}

$packageDir = Join-Path $stageRoot "helius"
New-Item -ItemType Directory -Path $packageDir -Force | Out-Null

$binaryName = Split-Path $resolvedBinaryPath -Leaf
$packagedBinaryPath = Join-Path $packageDir $binaryName

Copy-Item $resolvedBinaryPath $packagedBinaryPath
Copy-Item (Join-Path $repoRoot "LICENSE") $packageDir
Copy-Item (Join-Path $repoRoot "README.md") $packageDir

$isWindowsHost = [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform(
    [System.Runtime.InteropServices.OSPlatform]::Windows
)

if (-not $isWindowsHost) {
    chmod +x $packagedBinaryPath
}

switch ($Platform) {
    "windows-x86_64" {
        $archivePath = Join-Path $stageRoot "helius-$Version-$Platform.zip"
        Compress-Archive -Path $packageDir -DestinationPath $archivePath
    }
    default {
        $archivePath = Join-Path $stageRoot "helius-$Version-$Platform.tar.gz"
        & tar -C $stageRoot -czf $archivePath helius
        if ($LASTEXITCODE -ne 0) {
            throw "tar failed while packaging $archivePath"
        }
    }
}

$checksumPath = "$archivePath.sha256.txt"
$archiveName = Split-Path $archivePath -Leaf
$checksum = (Get-FileHash $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
Set-Content -Path $checksumPath -Value "$checksum  $archiveName" -NoNewline

[pscustomobject]@{
    StageRoot = $stageRoot
    PackageDir = $packageDir
    ArchivePath = $archivePath
    ChecksumPath = $checksumPath
}

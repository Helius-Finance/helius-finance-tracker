param(
    [string]$BinaryPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path $PSScriptRoot -Parent
if (-not $BinaryPath) {
    $isWindows = [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform(
        [System.Runtime.InteropServices.OSPlatform]::Windows
    )
    $binaryName = if ($isWindows) { "helius.exe" } else { "helius" }
    $BinaryPath = Join-Path (Join-Path (Join-Path $repoRoot "target") "release") $binaryName
}

$resolvedBinaryPath = (Resolve-Path $BinaryPath).Path

function Format-HeliusArgument {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Value
    )

    if ($Value -notmatch '[\s"]') {
        return $Value
    }

    $escaped = $Value -replace '(\\*)"', '$1$1\"'
    $escaped = $escaped -replace '(\\+)$', '$1$1'
    return '"' + $escaped + '"'
}

function Invoke-Helius {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments,
        [string]$Stdin
    )

    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = $script:resolvedBinaryPath
    $startInfo.Arguments = ($Arguments | ForEach-Object { Format-HeliusArgument $_ }) -join " "
    $startInfo.RedirectStandardInput = $true
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    $startInfo.UseShellExecute = $false
    $startInfo.CreateNoWindow = $true

    $process = [System.Diagnostics.Process]::Start($startInfo)
    if ($PSBoundParameters.ContainsKey("Stdin")) {
        $process.StandardInput.Write($Stdin)
    }
    $process.StandardInput.Close()

    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    $process.WaitForExit()

    $output = $stdout + $stderr

    if ($process.ExitCode -ne 0) {
        throw "Helius command failed: $resolvedBinaryPath $($Arguments -join ' ')`n$output"
    }

    return $output
}

$today = Get-Date -Format "yyyy-MM-dd"
$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("helius-smoke-" + [Guid]::NewGuid().ToString("N"))
$dbPath = Join-Path $tempDir "tracker.db"

New-Item -ItemType Directory -Path $tempDir | Out-Null

try {
    Invoke-Helius -Arguments @("--help") | Out-Null
    Invoke-Helius -Arguments @("--db", $dbPath, "init", "--currency", "USD") | Out-Null
    Invoke-Helius -Arguments @("--db", $dbPath, "account", "add", "Checking", "--type", "checking") | Out-Null
    Invoke-Helius -Arguments @("--db", $dbPath, "category", "add", "Groceries", "--kind", "expense") | Out-Null
    Invoke-Helius -Arguments @(
        "--db",
        $dbPath,
        "tx",
        "add",
        "--type",
        "expense",
        "--amount",
        "10.00",
        "--date",
        $today,
        "--account",
        "Checking",
        "--category",
        "Groceries",
        "--payee",
        "SmokeTest"
    ) | Out-Null

    Invoke-Helius -Arguments @("--db", $dbPath, "balance") | Out-Null

    $transactionsJson = Invoke-Helius -Arguments @("--db", $dbPath, "tx", "list", "--json")
    $transactions = @($transactionsJson | ConvertFrom-Json)
    if ($transactions.Count -lt 1) {
        throw "Smoke flow expected at least one transaction in tx list output."
    }

    $shellOutput = Invoke-Helius -Arguments @("--db", $dbPath, "shell")
    if ($shellOutput -notmatch "Helius interactive shell") {
        throw "Smoke flow expected the interactive shell to start."
    }
}
finally {
    Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
}

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

function Convert-JsonResult {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Json
    )

    return [System.Management.Automation.PSSerializer]::Deserialize(
        [System.Management.Automation.PSSerializer]::Serialize(($Json | ConvertFrom-Json))
    )
}

$today = Get-Date -Format "yyyy-MM-dd"
$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("helius-smoke-" + [Guid]::NewGuid().ToString("N"))
$dbPath = Join-Path $tempDir "tracker.db"
$csvPath = Join-Path $tempDir "revolut.csv"
$camtPath = Join-Path $tempDir "statement.xml"

New-Item -ItemType Directory -Path $tempDir | Out-Null

try {
    @"
Date completed,Amount,Description,Merchant,Reference
2026-03-10,-12.99,Taxi ride,Beat,REV-001
2026-03-11,45.50,Cashback bonus,Revolut,REV-002
"@ | Set-Content -Path $csvPath -NoNewline

    @"
<Document>
  <BkToCstmrStmt>
    <Stmt>
      <Ntry>
        <Sts>BOOK</Sts>
        <Amt Ccy="EUR">2500.00</Amt>
        <CdtDbtInd>CRDT</CdtDbtInd>
        <BookgDt>
          <Dt>2026-03-12</Dt>
        </BookgDt>
        <NtryDtls>
          <TxDtls>
            <Refs>
              <AcctSvcrRef>CAMT-CR-001</AcctSvcrRef>
            </Refs>
            <RltdPties>
              <Dbtr>
                <Nm>Employer Ltd</Nm>
              </Dbtr>
            </RltdPties>
            <RmtInf>
              <Ustrd>Salary March</Ustrd>
            </RmtInf>
          </TxDtls>
        </NtryDtls>
      </Ntry>
    </Stmt>
  </BkToCstmrStmt>
</Document>
"@ | Set-Content -Path $camtPath -NoNewline

    Invoke-Helius -Arguments @("--help") | Out-Null
    Invoke-Helius -Arguments @("--db", $dbPath, "init", "--currency", "EUR") | Out-Null
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

    $presetListJson = Invoke-Helius -Arguments @("--db", $dbPath, "import", "csv", "--list-presets", "--json")
    $presets = @(Convert-JsonResult -Json $presetListJson)
    if ((@($presets | Where-Object { $_.id -eq "revolut-csv" })).Count -ne 1) {
        throw "Smoke flow expected revolut-csv to appear in the preset list."
    }

    $csvImportJson = Invoke-Helius -Arguments @(
        "--db",
        $dbPath,
        "import",
        "csv",
        "--input",
        $csvPath,
        "--account",
        "Checking",
        "--preset",
        "revolut-csv",
        "--json"
    )
    $csvImport = $csvImportJson | ConvertFrom-Json
    if ($csvImport.imported_count -ne 2 -or $csvImport.duplicate_count -ne 0) {
        throw "Smoke flow expected preset CSV import to create 2 rows."
    }

    $camtImportJson = Invoke-Helius -Arguments @(
        "--db",
        $dbPath,
        "import",
        "camt053",
        "--input",
        $camtPath,
        "--account",
        "Checking",
        "--json"
    )
    $camtImport = $camtImportJson | ConvertFrom-Json
    if ($camtImport.imported_count -ne 1 -or $camtImport.duplicate_count -ne 0) {
        throw "Smoke flow expected camt053 import to create 1 row."
    }

    $categoriesJson = Invoke-Helius -Arguments @("--db", $dbPath, "category", "list", "--json")
    $categories = @(Convert-JsonResult -Json $categoriesJson)
    if ((@($categories | Where-Object { $_.name -eq "Uncategorized Expense" })).Count -ne 1) {
        throw "Smoke flow expected Uncategorized Expense to be created by import."
    }
    if ((@($categories | Where-Object { $_.name -eq "Uncategorized Income" })).Count -ne 1) {
        throw "Smoke flow expected Uncategorized Income to be created by import."
    }

    $transactionsJson = Invoke-Helius -Arguments @("--db", $dbPath, "tx", "list", "--json")
    $transactions = @(Convert-JsonResult -Json $transactionsJson)
    if ((@($transactions)).Count -lt 4) {
        throw "Smoke flow expected imported transactions to appear in tx list output."
    }

    $shellOutput = Invoke-Helius -Arguments @("--db", $dbPath, "shell")
    if ($shellOutput -notmatch "Helius interactive shell") {
        throw "Smoke flow expected the interactive shell to start."
    }
}
finally {
    Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
}

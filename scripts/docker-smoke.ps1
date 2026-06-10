param(
    [string]$ImageTag = "helius-smoke"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-Docker {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    & docker @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Docker command failed: docker $($Arguments -join ' ')"
    }
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

$repoRoot = Split-Path $PSScriptRoot -Parent
$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("helius-docker-smoke-" + [Guid]::NewGuid().ToString("N"))
$csvPath = Join-Path $tempDir "revolut.csv"
$camtPath = Join-Path $tempDir "statement.xml"
$volumeSpec = "${tempDir}:/data"

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

    Push-Location $repoRoot
    try {
        Invoke-Docker -Arguments @("build", "-t", $ImageTag, ".")
    }
    finally {
        Pop-Location
    }

    Invoke-Docker -Arguments @("run", "--rm", "-v", $volumeSpec, $ImageTag, "init", "--currency", "EUR")
    Invoke-Docker -Arguments @("run", "--rm", "-v", $volumeSpec, $ImageTag, "account", "add", "Checking", "--type", "checking")

    $presetListJson = (docker run --rm -v $volumeSpec $ImageTag import csv --list-presets --json) -join "`n"
    if ($LASTEXITCODE -ne 0) {
        throw "Docker preset listing failed."
    }
    $presets = @(Convert-JsonResult -Json $presetListJson)
    if ((@($presets | Where-Object { $_.id -eq "revolut-csv" })).Count -ne 1) {
        throw "Docker smoke expected revolut-csv to appear in the preset list."
    }

    $csvImportJson = (docker run --rm -v $volumeSpec $ImageTag import csv --input /data/revolut.csv --account Checking --preset revolut-csv --json) -join "`n"
    if ($LASTEXITCODE -ne 0) {
        throw "Docker preset CSV import failed."
    }
    $csvImport = $csvImportJson | ConvertFrom-Json
    if ($csvImport.imported_count -ne 2 -or $csvImport.duplicate_count -ne 0) {
        throw "Docker smoke expected preset CSV import to create 2 rows."
    }

    $camtImportJson = (docker run --rm -v $volumeSpec $ImageTag import camt053 --input /data/statement.xml --account Checking --json) -join "`n"
    if ($LASTEXITCODE -ne 0) {
        throw "Docker camt053 import failed."
    }
    $camtImport = $camtImportJson | ConvertFrom-Json
    if ($camtImport.imported_count -ne 1 -or $camtImport.duplicate_count -ne 0) {
        throw "Docker smoke expected camt053 import to create 1 row."
    }

    $transactionsJson = (docker run --rm -v $volumeSpec $ImageTag tx list --json) -join "`n"
    if ($LASTEXITCODE -ne 0) {
        throw "Docker transaction listing failed."
    }
    $transactions = @(Convert-JsonResult -Json $transactionsJson)
    if ((@($transactions)).Count -ne 3) {
        throw "Docker smoke expected exactly 3 imported transactions."
    }
}
finally {
    Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
}

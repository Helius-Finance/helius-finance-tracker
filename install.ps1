# Install helius.exe from a GitHub release.
#
#   irm https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.ps1 | iex
#
# Environment overrides:
#   HELIUS_VERSION      release tag to install, such as v1.4.4 (default: latest)
#   HELIUS_INSTALL_DIR  directory that receives helius.exe
#                       (default: %LOCALAPPDATA%\Programs\Helius)

# Everything runs inside a function so that `irm | iex` does not leave
# variables or preference changes behind in the caller's session.
function Install-Helius {
    Set-StrictMode -Version Latest
    $ErrorActionPreference = "Stop"
    # The progress bar slows Invoke-WebRequest down sharply on Windows PowerShell 5.1.
    $ProgressPreference = "SilentlyContinue"

    $repo = "Helius-Finance/helius-finance-tracker"
    $installDocs = "https://github.com/$repo#installation"
    $platform = "windows-x86_64"

    if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT) {
        throw "install.ps1 installs the Windows build. On Linux, use install.sh: $installDocs"
    }
    if (-not [Environment]::Is64BitOperatingSystem) {
        throw "There is no prebuilt helius binary for 32-bit Windows. Build from source instead: $installDocs"
    }

    # Windows PowerShell 5.1 can default to TLS 1.0, which GitHub rejects.
    [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12

    $version = $env:HELIUS_VERSION
    if ($version) {
        if (-not $version.StartsWith("v")) {
            $version = "v$version"
        }
    }
    else {
        # GitHub redirects /releases/latest to /releases/tag/<tag>. Reading the
        # redirect avoids the rate-limited REST API.
        $response = Invoke-WebRequest -Uri "https://github.com/$repo/releases/latest" -Method Head -UseBasicParsing
        if ($response.BaseResponse.PSObject.Properties["ResponseUri"]) {
            # Windows PowerShell 5.1
            $latestUri = $response.BaseResponse.ResponseUri
        }
        else {
            # PowerShell 7
            $latestUri = $response.BaseResponse.RequestMessage.RequestUri
        }
        $version = $latestUri.Segments[-1]
        if ($version -notmatch '^v\d') {
            throw "Could not determine the latest release from $latestUri"
        }
    }

    $installDir = $env:HELIUS_INSTALL_DIR
    if (-not $installDir) {
        $installDir = Join-Path $env:LOCALAPPDATA "Programs\Helius"
    }
    $archive = "helius-$version-$platform.zip"
    $baseUrl = "https://github.com/$repo/releases/download/$version"

    $tempDir = Join-Path ([IO.Path]::GetTempPath()) ("helius-install-" + [Guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Path $tempDir | Out-Null
    try {
        Write-Host "Downloading helius $version for $platform"
        $archivePath = Join-Path $tempDir $archive
        $checksumPath = "$archivePath.sha256.txt"
        Invoke-WebRequest -Uri "$baseUrl/$archive" -OutFile $archivePath -UseBasicParsing
        Invoke-WebRequest -Uri "$baseUrl/$archive.sha256.txt" -OutFile $checksumPath -UseBasicParsing

        $expected = ((Get-Content -Raw $checksumPath).Trim() -split '\s+')[0].ToLowerInvariant()
        $actual = (Get-FileHash $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($expected -ne $actual) {
            throw "Checksum mismatch for $archive (expected $expected, got $actual)"
        }

        Expand-Archive -Path $archivePath -DestinationPath $tempDir
        $packageDir = Join-Path $tempDir "helius"
        $binary = Join-Path $packageDir "helius.exe"
        if (-not (Test-Path $binary)) {
            throw "$archive does not contain helius\helius.exe"
        }

        # Run the binary before installing it, so a broken download keeps any
        # existing install untouched.
        $versionLine = & $binary --version
        if ($LASTEXITCODE -ne 0) {
            throw "The downloaded helius.exe failed to run: $versionLine"
        }

        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        Copy-Item -Path (Join-Path $packageDir "*") -Destination $installDir -Force
        $installedBinary = Join-Path $installDir "helius.exe"
        Write-Host "Installed $versionLine to $installedBinary"

        # Edit the registry value directly: [Environment]::SetEnvironmentVariable
        # would rewrite PATH as REG_SZ and freeze any %VARIABLE% entries in it.
        $environmentKey = Get-Item -Path "HKCU:\Environment"
        $userPath = $environmentKey.GetValue("Path", "", "DoNotExpandEnvironmentNames")
        $entries = @($userPath -split ";" | Where-Object { $_ })
        $normalizedDir = $installDir.TrimEnd("\")
        if (-not ($entries | Where-Object { $_.TrimEnd("\") -eq $normalizedDir })) {
            $valueKind = [Microsoft.Win32.RegistryValueKind]::ExpandString
            if ($environmentKey.GetValueNames() -contains "Path") {
                $valueKind = $environmentKey.GetValueKind("Path")
            }
            Set-ItemProperty -Path "HKCU:\Environment" -Name "Path" -Value (($entries + $normalizedDir) -join ";") -Type $valueKind

            # Setting and clearing a throwaway user variable broadcasts
            # WM_SETTINGCHANGE, so terminals opened from now on see the new PATH.
            [Environment]::SetEnvironmentVariable("HELIUS_INSTALL_REFRESH", "1", "User")
            [Environment]::SetEnvironmentVariable("HELIUS_INSTALL_REFRESH", $null, "User")

            $env:Path = "$env:Path;$normalizedDir"
            Write-Host "Added $normalizedDir to your user PATH. Terminals that were already open need a restart."
        }

        $found = Get-Command helius -CommandType Application -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($found -and $found.Path -ne $installedBinary) {
            Write-Host "Note: $($found.Path) comes first on your PATH and shadows this install."
        }
        Write-Host "Run 'helius' to get started."
    }
    finally {
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

Install-Helius

param()

$ErrorActionPreference = "Stop"

$RepoSlug = if ($env:NODIT_CLI_REPO_SLUG) { $env:NODIT_CLI_REPO_SLUG } else { "s1ddh1k/nodit-cli" }
$BinDir = if ($env:NODIT_CLI_BIN_DIR) { $env:NODIT_CLI_BIN_DIR } else { Join-Path $HOME ".local\bin" }
$InstallName = if ($env:NODIT_CLI_INSTALL_NAME) { $env:NODIT_CLI_INSTALL_NAME } else { "nodit-cli.exe" }
$Version = if ($env:NODIT_CLI_VERSION) { $env:NODIT_CLI_VERSION } else { "latest" }

if (-not $InstallName.EndsWith(".exe", [System.StringComparison]::OrdinalIgnoreCase)) {
    $InstallName = "$InstallName.exe"
}

function Get-ReleaseApiUrl {
    if ($Version -eq "latest") {
        return "https://api.github.com/repos/$RepoSlug/releases/latest"
    }

    return "https://api.github.com/repos/$RepoSlug/releases/tags/$Version"
}

$assetName = "nodit-cli-windows.zip"
$apiUrl = Get-ReleaseApiUrl

Write-Host "Resolving release metadata from $apiUrl"
$release = Invoke-RestMethod -Uri $apiUrl

if (-not $release.tag_name) {
    throw "failed to resolve release tag"
}

$asset = $release.assets | Where-Object { $_.name -eq $assetName } | Select-Object -First 1
if (-not $asset) {
    throw "asset not found for this platform: $assetName"
}

$tmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ("nodit-cli-" + [System.Guid]::NewGuid().ToString("N"))
$archivePath = Join-Path $tmpDir $assetName
$extractDir = Join-Path $tmpDir "extracted"
$targetBin = Join-Path $BinDir $InstallName

New-Item -ItemType Directory -Force -Path $tmpDir | Out-Null
New-Item -ItemType Directory -Force -Path $extractDir | Out-Null
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

try {
    Write-Host "Downloading $assetName from release $($release.tag_name)"
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $archivePath
    Expand-Archive -Path $archivePath -DestinationPath $extractDir -Force

    $sourceBin = Get-ChildItem -Path $extractDir -Recurse -Filter "nodit-cli.exe" | Select-Object -First 1
    if (-not $sourceBin) {
        throw "nodit-cli.exe binary not found in archive"
    }

    Copy-Item $sourceBin.FullName $targetBin -Force

    Write-Host ""
    Write-Host "Installed $InstallName from release $($release.tag_name):"
    Write-Host "  $targetBin"
    Write-Host ""
    Write-Host "Add this directory to PATH if needed:"
    Write-Host "  `$env:Path = `"$BinDir;`$env:Path`""
    Write-Host ""
    Write-Host "Run:"
    Write-Host "  $InstallName --help"
}
finally {
    if (Test-Path $tmpDir) {
        Remove-Item -Path $tmpDir -Recurse -Force
    }
}

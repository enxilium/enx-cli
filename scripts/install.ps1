Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# enx installer (Windows / PowerShell)
# One-line usage:
#   iwr -useb https://raw.githubusercontent.com/enxilium/enx-cli/main/scripts/install.ps1 | iex

# Override if installing from a fork:
#   $env:ENX_REPO = "owner/repo"
#   iwr -useb https://raw.githubusercontent.com/owner/repo/main/scripts/install.ps1 | iex
$repo = if ($env:ENX_REPO) { $env:ENX_REPO } else { "enxilium/enx-cli" }

# Channel/tag to install from. CI publishes rolling binaries to this tag.
$channel = if ($env:ENX_CHANNEL) { $env:ENX_CHANNEL } else { "nightly" }

$defaultInstallDir = Join-Path $HOME "AppData/Local/enx/bin"
$installDir = if ($env:ENX_INSTALL_DIR) { $env:ENX_INSTALL_DIR } else { $defaultInstallDir }

$arch = [Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")

switch ($arch) {
    "AMD64" { $asset = "enx-windows-x86_64.exe" }
    "ARM64" {
        throw "windows ARM64 binary is not published yet; please use x64 emulation or publish an ARM64 asset"
    }
    default {
        throw "unsupported windows architecture: $arch"
    }
}

$downloadUrl = "https://github.com/$repo/releases/download/$channel/$asset"
$tmpFile = [System.IO.Path]::GetTempFileName()

try {
    Write-Host "==> downloading $asset"
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tmpFile -UseBasicParsing

    Write-Host "==> installing to $installDir/enx.exe"
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null

    $enxBin = Join-Path $installDir "enx.exe"
    Move-Item -Path $tmpFile -Destination $enxBin -Force

    # Add to user PATH if not already present
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User") -split ";"
    if ($userPath -notcontains $installDir) {
        Write-Host "==> adding $installDir to user PATH"
        $newPath = ([Environment]::GetEnvironmentVariable("PATH", "User") -split ";") + $installDir
        $newPath = $newPath -join ";"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    }

    Write-Host "==> running enx setup"
    & $enxBin setup

    Write-Host ""
    Write-Host "setup finished"
    Write-Host "enx is now installed and available in your PATH"
    Write-Host "restart your PowerShell window for the PATH change to take effect"
}
finally {
    if (Test-Path $tmpFile) {
        Remove-Item $tmpFile -Force -ErrorAction SilentlyContinue
    }
}

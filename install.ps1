# ========================================================================
# VietLang Official Windows PowerShell Installer
# Usage: iex (irm https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.ps1)
# ========================================================================

$ErrorActionPreference = "Stop"

$Version = "0.1.0"
$Repo = "hoangtuvungcao/vietlang"
$VietLangHome = Join-Path $HOME ".vietlang"
$BinDir = Join-Path $VietLangHome "bin"
$StdDir = Join-Path $VietLangHome "std"

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║       VietLang Windows Official Installer v$Version        ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

# Create directories
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $StdDir | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $VietLangHome "modules") | Out-Null

$ExePath = Join-Path $BinDir "vietlang.exe"
$DownloadUrl = "https://github.com/$Repo/releases/latest/download/vietlang-windows-x64.exe"

Write-Host "Downloading VietLang binary..." -ForegroundColor Green
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ExePath -UseBasicParsing
} catch {
    Write-Host "Release not found online, using local target if available..." -ForegroundColor Yellow
    if (Test-Path "target\release\vietlang.exe") {
        Copy-Item "target\release\vietlang.exe" $ExePath
    }
}

# Sync Standard Library
Write-Host "Syncing Standard Library modules..." -ForegroundColor Green
if (Test-Path "std") {
    Copy-Item -Recurse -Force "std\*" $StdDir
}

# Automatically add ~/.vietlang/bin to User PATH
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$BinDir;$CurrentPath", "User")
    Write-Host "Added $BinDir to User PATH environment variable." -ForegroundColor Green
}

Write-Host ""
Write-Host "VietLang installed successfully on Windows!" -ForegroundColor Green
Write-Host "Binary: $ExePath" -ForegroundColor Yellow
Write-Host "Please restart PowerShell and run: vietlang --help" -ForegroundColor Cyan

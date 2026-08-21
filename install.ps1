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
$ModDir = Join-Path $VietLangHome "modules"

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║       VietLang Windows Official Installer v$Version        ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

# 1. Create Directory Hierarchy
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $StdDir | Out-Null
New-Item -ItemType Directory -Force -Path $ModDir | Out-Null

$ExePath = Join-Path $BinDir "vietlang.exe"
$DownloadUrl = "https://github.com/$Repo/releases/latest/download/vietlang-windows-x64.exe"

# 2. Install Binary (Local Target or GitHub Release)
if (Test-Path "target\release\vietlang.exe") {
    Write-Host "Installing fresh local build into $BinDir..." -ForegroundColor Green
    Copy-Item "target\release\vietlang.exe" $ExePath -Force
} else {
    Write-Host "Downloading VietLang binary from GitHub Releases..." -ForegroundColor Green
    try {
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $ExePath -UseBasicParsing
    } catch {
        Write-Host "Release not found online, trying cargo build fallback..." -ForegroundColor Yellow
        if (Get-Command cargo -ErrorAction SilentlyContinue) {
            cargo build --release --quiet
            Copy-Item "target\release\vietlang.exe" $ExePath -Force
        } else {
            Write-Error "Could not download or build vietlang.exe"
        }
    }
}

# 3. Sync Standard Library
Write-Host "Syncing 55 Standard Library modules into $StdDir..." -ForegroundColor Green
if (Test-Path "std") {
    Copy-Item -Recurse -Force "std\*" $StdDir
} else {
    try {
        $ZipUrl = "https://github.com/$Repo/archive/refs/heads/main.zip"
        $ZipPath = Join-Path $env:TEMP "vietlang-main.zip"
        $ExtractPath = Join-Path $env:TEMP "vietlang-extract"
        Invoke-WebRequest -Uri $ZipUrl -OutFile $ZipPath -UseBasicParsing
        Expand-Archive -Path $ZipPath -DestinationPath $ExtractPath -Force
        Copy-Item -Recurse -Force (Join-Path $ExtractPath "vietlang-main\std\*") $StdDir
        Remove-Item -Recurse -Force $ZipPath, $ExtractPath -ErrorAction SilentlyContinue
    } catch {
        Write-Host "Warning: Could not fetch stdlib remote archive." -ForegroundColor Yellow
    }
}

# 4. Automatically add ~/.vietlang/bin to User PATH
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$BinDir;$CurrentPath", "User")
    Write-Host "Added $BinDir to User PATH environment variable." -ForegroundColor Green
}
$env:Path = "$BinDir;$env:Path"
[Environment]::SetEnvironmentVariable("VIETLANG_STD", $StdDir, "User")
$env:VIETLANG_STD = $StdDir

# 5. Success Banner & Quickstart
Write-Host ""
Write-Host " VietLang installed successfully on Windows!" -ForegroundColor Green
Write-Host "   Binary:   $ExePath" -ForegroundColor Yellow
Write-Host "   Standard: $StdDir (55 modules)" -ForegroundColor Yellow
Write-Host ""
Write-Host "Quickstart commands:" -ForegroundColor White
Write-Host "  vietlang                  # Start interactive REPL" -ForegroundColor Cyan
Write-Host "  vietlang doc              # Browse all 55 standard modules" -ForegroundColor Cyan
Write-Host "  vietlang new my_app       # Scaffold a Clean Architecture backend service" -ForegroundColor Cyan
Write-Host "  vietlang dev              # Start development server" -ForegroundColor Cyan
Write-Host "  vietlang build src/main.vl# Compile to standalone executable" -ForegroundColor Cyan
Write-Host ""
Write-Host "Please restart PowerShell and run: vietlang --help" -ForegroundColor Yellow

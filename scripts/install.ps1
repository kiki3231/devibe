# Install devibe on Windows
# PowerShell script

param(
    [string]$Version = "latest"
)

$Repo = "kiki3231/devibe"
$ErrorActionPreference = "Stop"

if ($Version -eq "latest") {
    $ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"
} else {
    $ApiUrl = "https://api.github.com/repos/$Repo/releases/tags/v$Version"
}

Write-Host "Downloading devibe $Version..." -ForegroundColor Cyan

try {
    $Release = Invoke-RestMethod -Uri $ApiUrl
    $Asset = $Release.assets | Where-Object { $_.name -match "devibe-windows-x86_64.zip" }

    if (-not $Asset) {
        Write-Host "Windows binary not found in release. Falling back to cargo install..." -ForegroundColor Yellow
        $Cargo = Get-Command cargo -ErrorAction SilentlyContinue
        if ($Cargo) {
            cargo install devibe
        } else {
            Write-Host "Please install Rust: https://rustup.rs" -ForegroundColor Red
            exit 1
        }
        exit 0
    }

    $TempDir = Join-Path $env:TEMP "devibe-install"
    New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

    $ZipFile = Join-Path $TempDir "devibe.zip"
    Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile $ZipFile
    Expand-Archive -Path $ZipFile -DestinationPath $TempDir -Force

    $InstallDir = Join-Path $env:LOCALAPPDATA "devibe"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Copy-Item (Join-Path $TempDir "devibe.exe") (Join-Path $InstallDir "devibe.exe") -Force
    Remove-Item -Recurse -Force $TempDir

    # Add to PATH
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
        $env:Path += ";$InstallDir"
        Write-Host "Added $InstallDir to PATH" -ForegroundColor Green
    }

    Write-Host "devibe installed to $InstallDir\devibe.exe" -ForegroundColor Green
    Write-Host ""
    Write-Host "Run: devibe --scan ~/projects" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "NOTE: Windows SmartScreen may block the .exe on first run." -ForegroundColor Yellow
    Write-Host "  Workaround: Right-click devibe.exe > Properties > General >" -ForegroundColor Yellow
    Write-Host "  Check 'Unblock' > OK. Then run again." -ForegroundColor Yellow

} catch {
    Write-Host "Install failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Alternative install methods:" -ForegroundColor Yellow
    Write-Host "  1. Via Scoop:  scoop bucket add devibe https://github.com/$Repo"
    Write-Host "                 scoop install devibe"
    Write-Host "  2. Build from source: git clone https://github.com/$Repo && cd devibe && cargo build --release"
    exit 1
}

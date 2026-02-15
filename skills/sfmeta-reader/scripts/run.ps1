# sfmeta-reader wrapper for Windows PowerShell
# Usage: .\run.ps1 [sfmeta-reader arguments...]
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Arguments
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$BinDir = Join-Path (Split-Path -Parent $ScriptDir) "bin"

# Detect architecture
$Arch = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
    "X64"   { "x86_64"  }
    "Arm64" { "aarch64" }
    default {
        Write-Error "Unsupported architecture: $_"
        exit 1
    }
}

$Binary = Join-Path $BinDir "sfmeta-reader-windows-${Arch}.exe"

if (-not (Test-Path $Binary)) {
    Write-Error "Binary not found at: $Binary"
    Write-Host "Available binaries:" -ForegroundColor Yellow
    if (Test-Path $BinDir) {
        Get-ChildItem $BinDir | ForEach-Object { Write-Host "  $_" }
    } else {
        Write-Host "  (bin/ directory is empty or missing)"
    }
    Write-Host ""
    Write-Host "Download the correct binary from the GitHub Releases page"
    Write-Host "and place it in: $BinDir\"
    exit 1
}

& $Binary @Arguments

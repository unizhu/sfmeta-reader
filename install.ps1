# sfmeta-reader skill installer for Windows
# Usage:
#   irm https://raw.githubusercontent.com/unizhu/sfmeta-reader/main/install.ps1 | iex
#   & ([scriptblock]::Create((irm https://raw.githubusercontent.com/unizhu/sfmeta-reader/main/install.ps1))) -Dir "$env:USERPROFILE\.ugent\skills"
param(
    [Alias("dir")]
    [string]$Dir = ""
)

$ErrorActionPreference = "Stop"

$Repo = "unizhu/sfmeta-reader"

# Default: ~\.claude\skills\sfmeta-reader, overridable by -Dir or env var
if ($Dir) {
    $InstallDir = $Dir
} elseif ($env:SFMETA_INSTALL_DIR) {
    $InstallDir = $env:SFMETA_INSTALL_DIR
} else {
    $InstallDir = "$env:USERPROFILE\.claude\skills\sfmeta-reader"
}

$ApiLatestTag = "https://api.github.com/repos/$Repo/releases/tags/latest"
$ApiLatest = "https://api.github.com/repos/$Repo/releases/latest"

# ── Detect architecture ────────────────────────────────────────
$Arch = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
    "X64"   { "x86_64"  }
    "Arm64" { "aarch64" }
    default { Write-Error "Unsupported architecture: $_"; exit 1 }
}

$Binary = "sfmeta-reader-windows-${Arch}.exe"

Write-Host "🔍  Detected platform: windows/$Arch" -ForegroundColor Cyan
Write-Host "📦  Binary: $Binary" -ForegroundColor Cyan
Write-Host "📂  Install directory: $InstallDir" -ForegroundColor Cyan
Write-Host ""

# ── Fetch latest release ───────────────────────────────────────
Write-Host "🌐  Fetching latest release from $Repo..."
try {
    $Release = Invoke-RestMethod -Uri $ApiLatestTag
} catch {
    $Release = Invoke-RestMethod -Uri $ApiLatest
}
$Asset = $Release.assets | Where-Object { $_.name -eq $Binary }
$Tag = $Release.tag_name

if (-not $Asset) {
    Write-Error "Could not find binary '$Binary' in the latest release."
    Write-Host "Available assets:" -ForegroundColor Yellow
    $Release.assets | ForEach-Object { Write-Host "  $($_.name)" }
    exit 1
}

Write-Host "📥  Downloading $Tag → $Binary..."

# ── Create directory structure ──────────────────────────────────
New-Item -ItemType Directory -Force -Path "$InstallDir\bin"       | Out-Null
New-Item -ItemType Directory -Force -Path "$InstallDir\scripts"   | Out-Null
New-Item -ItemType Directory -Force -Path "$InstallDir\resources" | Out-Null

# ── Download binary ─────────────────────────────────────────────
Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile "$InstallDir\bin\$Binary"

# ── Download skill files from main branch ───────────────────────
$Raw = "https://raw.githubusercontent.com/$Repo/main"
Invoke-WebRequest -Uri "$Raw/skills/sfmeta-reader/SKILL.md"              -OutFile "$InstallDir\SKILL.md"
Invoke-WebRequest -Uri "$Raw/skills/sfmeta-reader/scripts/run.sh"        -OutFile "$InstallDir\scripts\run.sh"
Invoke-WebRequest -Uri "$Raw/skills/sfmeta-reader/scripts/run.ps1"       -OutFile "$InstallDir\scripts\run.ps1"
Invoke-WebRequest -Uri "$Raw/skills/sfmeta-reader/resources/reference.md" -OutFile "$InstallDir\resources\reference.md"

Write-Host ""
Write-Host "✅  Installed sfmeta-reader $Tag to $InstallDir" -ForegroundColor Green
Write-Host ""
Write-Host "   Skill:   $InstallDir\SKILL.md"
Write-Host "   Binary:  $InstallDir\bin\$Binary"
Write-Host "   Runner:  $InstallDir\scripts\run.ps1"
Write-Host ""
Write-Host "🚀  Ready! LLM agents will auto-discover the skill from $InstallDir" -ForegroundColor Green

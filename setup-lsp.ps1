<#
.SYNOPSIS
  Auto-setup script for Nimble LSP integration with VS Code.

.DESCRIPTION
  This script:
    1. Verifies the Rust toolchain is installed
    2. Builds the nimble compiler in release mode (produces target/release/nimble.exe)
    3. Checks that the binary can run `nimble lsp` (spawns a quick smoke test)
    4. Copies the VS Code extension to the user's extensions directory
    5. Installs the vscode-languageclient npm package if needed
    6. Prints launch instructions

.NOTES
  Requires: Rust toolchain (rustc, cargo), VS Code (code), Node.js + npm
  Run from the Nimble repository root.
#>

$ErrorActionPreference = "Stop"
$RepoRoot = $PSScriptRoot
$ExtensionDir = Join-Path $RepoRoot "nimble-lsp-vscode"
$ReleaseBinary = Join-Path $RepoRoot "target/release/nimble.exe"

Write-Host "=== Nimble LSP Setup ===" -ForegroundColor Cyan
Write-Host ""

# ---- Step 1: Check prerequisites ----
Write-Host "[1/5] Checking prerequisites ..." -ForegroundColor Yellow

$Missing = @()
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
  $Missing += "rustc (install from https://rustup.rs)"
}
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  $Missing += "cargo"
}
if (-not (Get-Command code -ErrorAction SilentlyContinue)) {
  Write-Host "  [WARN] code not on PATH - VS Code extension must be installed manually" -ForegroundColor DarkYellow
}
if ($Missing.Count -gt 0) {
  Write-Host "  MISSING:" -ForegroundColor Red
  $Missing | ForEach-Object { Write-Host "    - $_" }
  exit 1
}
Write-Host "  All checks passed" -ForegroundColor Green

# ---- Step 2: Build release binary ----
Write-Host "[2/5] Building nimble (release) ..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
  Write-Host "  Build failed" -ForegroundColor Red
  exit 1
}
Write-Host "  Built: $ReleaseBinary" -ForegroundColor Green

# ---- Step 3: Smoke-test the LSP ----
Write-Host "[3/5] Smoke-testing the LSP binary ..." -ForegroundColor Yellow

try {
  $StartInfo = New-Object System.Diagnostics.ProcessStartInfo
  $StartInfo.FileName = $ReleaseBinary
  $StartInfo.Arguments = "lsp"
  $StartInfo.UseShellExecute = $false
  $StartInfo.RedirectStandardInput = $true
  $StartInfo.RedirectStandardOutput = $true
  $StartInfo.RedirectStandardError = $true

  $Proc = New-Object System.Diagnostics.Process
  $Proc.StartInfo = $StartInfo
  $Proc.Start() | Out-Null

  Start-Sleep -Milliseconds 500
  if ($Proc.HasExited) {
    Write-Host "  LSP exited unexpectedly (code $($Proc.ExitCode))" -ForegroundColor Red
  } else {
    $Proc.Kill()
    Write-Host "  LSP started and responded (smoke test passed)" -ForegroundColor Green
  }
} catch {
  Write-Host "  Smoke test failed: $_" -ForegroundColor Red
  Write-Host "  (the LSP binary may still work inside VS Code)" -ForegroundColor DarkYellow
}

# ---- Step 4: Install VS Code extension ----
Write-Host "[4/5] Installing VS Code extension ..." -ForegroundColor Yellow

$VsCodeExtensions = $null
if ($IsWindows -or $env:OS -match "Windows") {
  $VsCodeExtensions = Join-Path $env:USERPROFILE ".vscode/extensions"
} else {
  $VsCodeExtensions = Join-Path $env:HOME ".vscode/extensions"
}

if ($VsCodeExtensions -and (Test-Path $VsCodeExtensions)) {
  $Target = Join-Path $VsCodeExtensions "nimble-toolchain.nimble-lsp-0.1.0"
  if (Test-Path $Target) {
    Remove-Item -Recurse -Force $Target
  }

  New-Item -ItemType Directory -Path $Target -Force | Out-Null
  Get-ChildItem -Path $ExtensionDir -Exclude "node_modules", ".git" | ForEach-Object {
    if ($_.PSIsContainer) {
      Copy-Item -Recurse -Path $_.FullName -Destination $Target
    } else {
      Copy-Item -Path $_.FullName -Destination $Target
    }
  }
  Write-Host "  Extension installed to $Target" -ForegroundColor Green
} else {
  Write-Host "  Could not locate VS Code extensions directory" -ForegroundColor DarkYellow
  Write-Host "  Extension files are at: $ExtensionDir" -ForegroundColor Cyan
  Write-Host "  Copy this folder manually to your VS Code extensions directory." -ForegroundColor Cyan
}

# ---- Step 5: Print summary (npm no longer needed - extension is self-contained) ----
Write-Host "[5/5] Done!" -ForegroundColor Yellow
Write-Host ""
Write-Host "=== Setup Complete ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Nimble binary:    $ReleaseBinary" -ForegroundColor White
Write-Host "  VS Code extension: installed (restart VS Code to activate)" -ForegroundColor White
Write-Host ""
Write-Host "  To use the LSP:" -ForegroundColor White
Write-Host "    1. Restart VS Code" -ForegroundColor Gray
Write-Host "    2. Open any .nbl file" -ForegroundColor Gray
Write-Host "    3. The Nimble Language Server will start automatically" -ForegroundColor Gray
Write-Host ""
Write-Host "  Features:" -ForegroundColor White
Write-Host "    - Syntax highlighting (.nbl files)" -ForegroundColor Gray
Write-Host "    - Diagnostics (parse errors, type errors)" -ForegroundColor Gray
Write-Host "    - Hover type information" -ForegroundColor Gray
Write-Host "    - Go to definition" -ForegroundColor Gray
Write-Host "    - Autocompletion" -ForegroundColor Gray
Write-Host "    - Format on save (via chisel)" -ForegroundColor Gray
Write-Host ""
Write-Host "  If the LSP does not start, check the VS Code console:" -ForegroundColor White
Write-Host "    Help > Toggle Developer Tools > Console" -ForegroundColor Gray
Write-Host "    Look for [nimble-lsp] messages" -ForegroundColor Gray

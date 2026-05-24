#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Rust toolchain not found. Install it from: https://rustup.rs"
    exit 1
}

Write-Output "Installing del..."
cargo install --path . --locked

Write-Output ""
Write-Output "Done! Binary installed to: ~\.cargo\bin\del.exe"
$cargoBin = "$HOME\.cargo\bin"
if ($env:Path -notlike "*$cargoBin*") {
    Write-Output "Add ~\.cargo\bin to your PATH:  `$env:Path += `";$env:USERPROFILE\.cargo\bin`""
}

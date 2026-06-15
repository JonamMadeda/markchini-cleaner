# Generate Tauri signing key pair for production updates
# Run this ONCE on your local machine, then:
#   - Store TAURI_SIGNING_PRIVATE_KEY as a GitHub secret
#   - Store TAURI_SIGNING_PASSWORD as a GitHub secret
#   - Copy the public key into tauri.conf.json > plugins > updater > pubkey

$KEY_FILE = "$env:USERPROFILE\.tauri\markchini.key"
$KEY_DIR = Split-Path $KEY_FILE -Parent

if (-not (Test-Path $KEY_DIR)) {
    New-Item -ItemType Directory -Path $KEY_DIR -Force | Out-Null
}

if (Test-Path $KEY_FILE) {
    Write-Warning "Key already exists at $KEY_FILE"
    Write-Host "Public key:"
    Get-Content "$KEY_FILE.pub"
    exit 0
}

# Requires tauri-cli installed
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Error "Rust/Cargo not found. Install from https://rustup.rs"
    exit 1
}

Write-Host "Generating Tauri signing key..."
Write-Host "You will be prompted for a password. Save this password as TAURI_SIGNING_PASSWORD in GitHub secrets."
Write-Host ""

cargo tauri signer generate -w $KEY_FILE

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "=== KEY GENERATED SUCCESSFULLY ==="
    Write-Host ""
    Write-Host "Private key: $KEY_FILE"
    Write-Host "Public key: $KEY_FILE.pub"
    Write-Host ""
    Write-Host "Public key contents (paste into tauri.conf.json):"
    Write-Host "----------------------------------------"
    Get-Content "$KEY_FILE.pub"
    Write-Host "----------------------------------------"
    Write-Host ""
    Write-Host "GitHub secrets to set:"
    Write-Host "  TAURI_SIGNING_PRIVATE_KEY = contents of $KEY_FILE"
    Write-Host "  TAURI_SIGNING_PASSWORD    = the password you entered"
} else {
    Write-Error "Key generation failed"
    exit 1
}

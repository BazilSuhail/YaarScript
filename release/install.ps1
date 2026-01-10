# install.ps1
$ErrorActionPreference = "Stop"

# Clear host to give a clean slate for our custom UI
Clear-Host

# 1. Target configurations
$repoOwner   = "BazilSuhail"
$repoName    = "YaarScript"
$tag         = "yaarscript-cli-v1.0.0-windows-x64"
$exeName     = "yaar.exe"
$installDir  = "$env:USERPROFILE\.yaarscript"
$downloadUrl = "https://github.com/$repoOwner/$repoName/releases/download/$tag/$exeName"

# Helper function to render the custom CLI header
function Write-Header {
    Write-Host "==================================================================" -ForegroundColor Cyan
    Write-Host "        YAARSCRIPT COMPILER TOOLCHAIN AUTOMATED INSTALLER  " -ForegroundColor Cyan -NoNewline
    Write-Host " v1.0.0" -ForegroundColor Gray
    Write-Host "==================================================================" -ForegroundColor Cyan
    Write-Host ""
}

# Helper function to render a smooth, animated progress bar
function Show-ProgressBar {
    param ([int]$Percent)
    $w = 40 # width of progress bar
    $completed = [Math]::Floor($Percent / (100 / $w))
    $remainder = $w - $completed
    
    $bar = "█" * $completed
    $space = "░" * $remainder
    
    # \r resets the cursor to the beginning of the line to update dynamically
    Write-Host -NoNewline "`r    Progress: [$bar$space] $Percent%" -ForegroundColor Cyan
}

# --- START CLI INTERFACE ---
Write-Header

# STEP 1: Setting up Directories
Write-Host " [1/3] Preparing workspace configuration..." -ForegroundColor Yellow
Start-Sleep -Milliseconds 400

if (-not (Test-Path -Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
    Write-Host "       ↳ Created isolation directory: $installDir" -ForegroundColor Gray
} else {
    Write-Host "       ↳ Verified existing target directory: $installDir" -ForegroundColor Gray
}
Write-Host " Done." -ForegroundColor Green
Write-Host ""

# STEP 2: Streaming Binary via Web Stream
Write-Host " [2/3] Streaming remote binary directly from distribution source..." -ForegroundColor Yellow
Write-Host "       Connecting to GitHub Asset Layer..." -ForegroundColor Gray

# Simulated progress frames to give the user a rich, intuitive UI visual before/during stream completion
for ($i = 0; $i -le 100; $i += 10) {
    Show-ProgressBar -Percent $i
    if ($i -eq 40) {
        # Perform the actual web request payload execution right mid-way through UI render
        Invoke-WebRequest -Uri $downloadUrl -OutFile "$installDir\$exeName" -UseBasicParsing
    }
    Start-Sleep -Milliseconds 100
}
Write-Host "`n Done." -ForegroundColor Green
Write-Host ""

# STEP 3: Environment Integration
Write-Host " [3/3] Binding binary registry path variables to host configuration..." -ForegroundColor Yellow
Start-Sleep -Milliseconds 400

$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($currentPath -split ';' -notcontains $installDir) {
    [Environment]::SetEnvironmentVariable("Path", $currentPath + ";$installDir", "User")
    Write-Host "       ↳ Appended $installDir to active User PATH context registry." -ForegroundColor Gray
    Write-Host " Done." -ForegroundColor Green
} else {
    Write-Host "       ↳ Target paths already map down inside persistent environment arrays." -ForegroundColor DarkYellow
    Write-Host " Logged." -ForegroundColor Green
}

# --- INSTALL COMPLETE FOOTER ---
Write-Host ""
Write-Host "==================================================================" -ForegroundColor Green
Write-Host " INSTALLATION SUCCESSFUL!" -ForegroundColor Green -NoNewline
Write-Host " YaarScript environment successfully deployed." -ForegroundColor Gray
Write-Host "==================================================================" -ForegroundColor Green
Write-Host " » Command Mapping : yaar -> $installDir\$exeName" -ForegroundColor Gray
Write-Host " » Action Required : Please RESTART your terminal window to run globally." -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Green
Write-Host ""
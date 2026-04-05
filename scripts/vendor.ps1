#!/usr/bin/env pwsh

[CmdletBinding()]
param(
    [switch]$Force,
    [string]$Version = '4.6.0',
    [switch]$Help
)

Set-StrictMode -Version Latest

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = (Resolve-Path -LiteralPath (Join-Path $scriptDir '..')).Path
$bindDir = Join-Path $rootDir 'bind/rebound'
$targetDir = Join-Path $bindDir 'c_src'
$versionFile = Join-Path $targetDir '.version'
$workspaceCargoToml = Join-Path $rootDir 'Cargo.toml'

function Show-Usage {
    @'
Usage:
  scripts/vendor.ps1 [-Force] [-Version <x.y.z>]

Options:
  -Force             Force re-vendor even when local version already matches.
  -Version <x.y.z>   Rebound tag to vendor (also syncs Cargo workspace version).
  -Help              Show this message.

Result in bind/rebound/c_src:
  - src/
  - LICENSE
'@
}

function Fail-IfCommandMissing {
    param([string]$Name)
    if (-not (Get-Command -Name $Name -ErrorAction SilentlyContinue)) {
        throw "Error: required command not found: $Name"
    }
}

function Sync-WorkspaceVersion {
    param([string]$NewVersion)

    if (-not (Test-Path -LiteralPath $workspaceCargoToml)) {
        throw "Error: workspace Cargo.toml not found at $workspaceCargoToml"
    }

    $text = Get-Content -LiteralPath $workspaceCargoToml -Raw -ErrorAction Stop
    $workspaceMatch = [regex]::Match($text, '(?m)^\[workspace\.package\]')
    if (-not $workspaceMatch.Success) {
        throw "Error: [workspace.package] section not found in $workspaceCargoToml"
    }

    $blockStart = $workspaceMatch.Index
    $blockStartOffset = $blockStart + $workspaceMatch.Length
    $afterBlockText = $text.Substring($blockStartOffset)
    $nextSectionMatch = [regex]::Match($afterBlockText, '(?m)^\[')
    $blockLength = if ($nextSectionMatch.Success) {
        $blockStartOffset + $nextSectionMatch.Index - $blockStart
    } else {
        $text.Length - $blockStart
    }

    $blockText = $text.Substring($blockStart, $blockLength)
    $linePattern = @'
(?m)^(?<prefix>\s*version\s*=\s*)(?<quote>['"])(?<value>[^'"]+)(?<suffix>\k<quote>(?:\s*(#.*)?)?)(?<newline>\r?\n?)$
'@
    $lineMatch = [regex]::Match($blockText, $linePattern)
    if (-not $lineMatch.Success) {
        throw "Error: version line in [workspace.package] does not use a quoted string"
    }

    $newLine = "$($lineMatch.Groups['prefix'].Value)$($lineMatch.Groups['quote'].Value)$NewVersion$($lineMatch.Groups['suffix'].Value)$($lineMatch.Groups['newline'].Value)"
    $newBlock = $blockText.Substring(0, $lineMatch.Index) + $newLine + $blockText.Substring($lineMatch.Index + $lineMatch.Length)

    $newText = $text.Substring(0, $blockStart) + $newBlock + $text.Substring($blockStart + $blockLength)

    $tempFile = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "$(Get-Random)-workspace-cargo.tmp")
    try {
        $encoding = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText($tempFile, $newText, $encoding)
        Move-Item -Force -LiteralPath $tempFile -Destination $workspaceCargoToml
    } catch {
        if (Test-Path -LiteralPath $tempFile) {
            Remove-Item -LiteralPath $tempFile -Force
        }
        throw "Error: failed to write updated Cargo.toml: $($_.Exception.Message)"
    }
}

if ($Help) {
    Show-Usage
    return
}

$currentLocalVersion = 'none'
if (Test-Path -LiteralPath $versionFile) {
    $currentLocalVersion = (Get-Content -LiteralPath $versionFile -ErrorAction Stop | Select-Object -First 1).Trim()
}

$srcPath = Join-Path $targetDir 'src'
$licensePath = Join-Path $targetDir 'LICENSE'
if (-not $Force.IsPresent -and (Test-Path -LiteralPath $srcPath) -and (Test-Path -LiteralPath $licensePath) -and ($currentLocalVersion -eq $Version)) {
    Write-Host "Local vendor already at v$Version, skipping (use -Force to refresh)."
    return
}

Fail-IfCommandMissing -Name 'tar'

Write-Host "Rebound version: $Version"
$parentDir = Split-Path -Parent $targetDir
$stageDir = Join-Path $parentDir "c_src-stage-$([Guid]::NewGuid())"
$extractDir = Join-Path $parentDir "rebound-extract-$([Guid]::NewGuid())"
$tempTar = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "rebound-$([Guid]::NewGuid()).tar.gz")
$url = "https://github.com/hannorein/rebound/archive/refs/tags/$Version.tar.gz"
$backupDir = $null
$stageMoved = $false
$stageVersionFile = Join-Path $stageDir '.version'
Write-Host "Downloading: $url"

try {
    Invoke-WebRequest -Uri $url -OutFile $tempTar -ErrorAction Stop

    if (-not (Test-Path -LiteralPath $parentDir)) {
        New-Item -ItemType Directory -Force -Path $parentDir | Out-Null
    }
    if (Test-Path -LiteralPath $stageDir) {
        Remove-Item -LiteralPath $stageDir -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path $stageDir | Out-Null

    $archiveRoot = "rebound-$Version"
    & tar -xzf $tempTar --strip-components=1 -C $stageDir "$archiveRoot/src" "$archiveRoot/LICENSE"
    if ($LASTEXITCODE -ne 0) {
        throw "tar exited with code $LASTEXITCODE"
    }

    $sourceSrc = Join-Path $stageDir 'src'
    $sourceLicense = Join-Path $stageDir 'LICENSE'
    if (-not (Test-Path -LiteralPath $sourceSrc)) {
        throw "Error: extracted archive missing src/ directory"
    }
    if (-not (Test-Path -LiteralPath $sourceLicense)) {
        throw "Error: extracted archive missing LICENSE"
    }

    $bbEncoding = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($stageVersionFile, "$Version`n", $bbEncoding)

    try {
        if (Test-Path -LiteralPath $targetDir) {
            $backupDir = Join-Path $parentDir "c_src-backup-$([Guid]::NewGuid())"
            Move-Item -LiteralPath $targetDir -Destination $backupDir -Force
        }

        Move-Item -LiteralPath $stageDir -Destination $targetDir
        $stageMoved = $true

        Write-Host 'Syncing workspace version in Cargo.toml...'
        Sync-WorkspaceVersion -NewVersion $Version

        if ($backupDir -and (Test-Path -LiteralPath $backupDir)) {
            Remove-Item -LiteralPath $backupDir -Recurse -Force
            $backupDir = $null
        }

        Write-Host 'Vendor sync complete.'
    } catch {
        if (Test-Path -LiteralPath $targetDir) {
            Remove-Item -LiteralPath $targetDir -Recurse -Force -ErrorAction SilentlyContinue
        }
        if ($backupDir -and (Test-Path -LiteralPath $backupDir)) {
            Move-Item -LiteralPath $backupDir -Destination $targetDir -Force
            $backupDir = $null
        }
        throw
    }
} finally {
    if (Test-Path -LiteralPath $tempTar) {
        Remove-Item -LiteralPath $tempTar -Force -ErrorAction SilentlyContinue
    }
    if (-not $stageMoved -and (Test-Path -LiteralPath $stageDir)) {
        Remove-Item -LiteralPath $stageDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    if (Test-Path -LiteralPath $extractDir) {
        Remove-Item -LiteralPath $extractDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

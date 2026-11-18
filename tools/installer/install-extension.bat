@echo off
title Mister Download Manager - Browser Extension Setup
chcp 65001 >nul
echo =======================================================================
echo   Mister Download Manager :: Pemasangan Ekstensi Browser
echo =======================================================================
echo.

set "EXT_DIR=%~dp0extension\chromium"
if not exist "%EXT_DIR%" (
    set "EXT_DIR=%~dp0..\target\extension\chromium"
)

echo [1/3] Mengaktifkan Developer Mode di profil Chrome...
powershell -NoProfile -ExecutionPolicy Bypass -Command "$base = [System.IO.Path]::Combine($env:LOCALAPPDATA, 'Google\Chrome\User Data'); if (Test-Path $base) { Get-ChildItem -Path $base -Filter 'Preferences' -Recurse -Depth 2 -ErrorAction SilentlyContinue | ForEach-Object { try { $raw = [System.IO.File]::ReadAllText($_.FullName); if ($raw -match '\"extensions\"') { $json = $raw | ConvertFrom-Json; if (-not $json.extensions) { $json | Add-Member -MemberType NoteProperty -Name 'extensions' -Value ([PSCustomObject]@{}) }; $json.extensions | Add-Member -MemberType NoteProperty -Name 'developer_mode' -Value $true -Force; [System.IO.File]::WriteAllText($_.FullName, ($json | ConvertTo-Json -Depth 30 -Compress)) } } catch {} } }" >nul 2>&1

echo [2/3] Menyalin lokasi folder ekstensi ke Clipboard...
powershell -NoProfile -ExecutionPolicy Bypass -Command "Set-Clipboard -Value '%EXT_DIR%'" >nul 2>&1

echo [3/3] Membuka Chrome Extensions dan folder ekstensi...
start "" explorer "%EXT_DIR%"
start "" "chrome://extensions" 2>nul || start "" "edge://extensions" 2>nul || start "" "brave://extensions" 2>nul

echo.
echo =======================================================================
echo   PANDUAN PEMASANGAN (HANYA 10 DETIK):
echo =======================================================================
echo   1. Jalur folder sudah otomatis tersalin ke Clipboard:
echo      "%EXT_DIR%"
echo.
echo   2. Di halaman Chrome (chrome://extensions) yang baru terbuka:
echo      - Pastikan 'Developer mode' di pojok kanan atas AKTIF.
echo      - Klik tombol 'Load unpacked' (Muat yang belum dibongkar) di kiri atas.
echo      - Tekan Ctrl+V pada kolom nama/alamat folder lalu klik 'Select Folder'.
echo.
echo   3. Jika masih ada ekstensi MDM lama dari Chrome Web Store:
echo      - Klik 'Remove' (Hapus) pada ekstensi lama tersebut agar tidak bentrok.
echo =======================================================================
echo.
pause

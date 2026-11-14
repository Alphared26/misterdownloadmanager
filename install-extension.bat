@echo off
title Mister Download Manager - Extension Setup
echo ========================================================
echo   Mister Download Manager :: Browser Extension Setup
echo ========================================================
echo.
echo Opening Chromium extension folder...
start "" explorer "%~dp0extension\chromium"
echo.
echo Opening browser extensions page...
start "" "chrome://extensions" 2>nul || start "" "edge://extensions" 2>nul || start "" "brave://extensions" 2>nul
echo.
echo ========================================================
echo   EASY 3-STEP SETUP:
echo ========================================================
echo   1. Turn ON 'Developer mode' toggle (top-right corner).
echo   2. Click 'Load unpacked' button (top-left).
echo   3. Select the 'chromium' folder that was opened.
echo.
echo The Next-Gen MDM Extension with Video Popup and Social Media
echo downloader is now active!
echo ========================================================
echo.
pause

# Registers the unpacked (fork) extension so Chromium browsers deepseek-v4-pro-install it like the
# Chrome Web Store copy: HKCU\Software\Google\Chrome\Extensions\<ID> with "update_url"
# pointing at the local CRX served by the app (CRX external update protocol).
param(
    [string]$ExtensionId = "jhilkkiakihejbjnimjpemhofckekegl",
    [string]$UpdateUrl = "http://127.0.0.1:8080/extension/crx"
)
$ErrorActionPreference = "Stop"
$key = "HKCU:\Software\Google\Chrome\Extensions\$ExtensionId"
New-Item -Path $key -Force | Out-Null
Set-ItemProperty -Path $key -Name "update_url" -Value $UpdateUrl -Type String
Write-Host "Registered extension $ExtensionId with update_url=$UpdateUrl"
Write-Host "Chrome/Edge/Brave will offer it as 'New extension added' at next start."

$packageName = 'galoy-cli'
$installDir = "${env:ChocolateyInstall}\bin"

Remove-Item "$installDir\$packageName.exe" -Force -ErrorAction SilentlyContinue

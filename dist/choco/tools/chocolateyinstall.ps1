$ErrorActionPreference = 'Stop'

$version='0.1.4'
$checksum = '73771aa07ff0170e4ca06e4df0d76605a8ea5517e3ac077526bc5df6e3ebb336'

$url = "https://github.com/GaloyMoney/galoy-cli/releases/download/$version/galoy-cli-x86_64-pc-windows-gnu-$version.tar.gz"

$packageName = 'galoy-cli'
$installDir = "${env:ChocolateyInstall}\lib\$packageName"
$binDir = "${env:ChocolateyInstall}\bin"

$packageArgs = @{
  packageName    = $packageName
  fileFullPath   = "$installDir\$packageName.tar.gz"
  url            = $url
  checksum       = $checksum
  checksumType   = 'sha256'
}

Get-ChocolateyWebFile @packageArgs

# Unpack the .tar.gz file
Get-ChocolateyUnzip -FileFullPath "$installDir\$packageName.tar.gz" -Destination $installDir
Get-ChocolateyUnzip -FileFullPath "$installDir\$packageName.tar" -Destination $installDir

# Move the executable to the bin directory
Move-Item -Path "$installDir\galoy-cli-x86_64-pc-windows-gnu-$version\$packageName.exe" -Destination "$binDir\$packageName.exe"

Install-ChocolateyPath -PathToInstall $binDir -PathType 'Machine'

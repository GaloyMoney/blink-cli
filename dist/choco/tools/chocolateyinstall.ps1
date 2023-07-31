$ErrorActionPreference = 'Stop'

$url = 'https://github.com/GaloyMoney/galoy-cli/releases/download/0.1.2/galoy-cli.exe'
$checksum = 'ebee9d50dd38da771295f66f14376ddc249955b8ae0d9beb0bdc66c8fda60d07'

$packageName = 'galoy-cli'
$installDir = "${env:ChocolateyInstall}\bin"

$packageArgs = @{
  packageName  = $packageName
  fileFullPath = "$installDir\$packageName.exe"
  url          = $url
  softwareName = 'galoy-cli*'
  checksum     = $checksum
  checksumType = 'sha256'
}

Get-ChocolateyWebFile @packageArgs
Install-ChocolateyPath -PathToInstall $installDir -PathType 'Machine'

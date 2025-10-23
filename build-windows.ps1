$windowspkg = "windowspkg"


if (-not (Test-Path -Path $windowspkg -PathType Container)) {
    New-Item -Path $windowspkg -ItemType Directory | Out-Null
}

$rInstallerUrl = "https://cran.r-project.org/bin/windows/base/R-${env:R_VERSION}-win.exe"
$rvZipUrl = "https://github.com/A2-ai/rv/releases/download/v0.13.2/rv-v0.13.2-x86_64-pc-windows-msvc.zip"

$rInstallerPath = Join-Path -Path $windowspkg -ChildPath "R-${env:R_VERSION}-win.exe"
$rvZipPath = Join-Path -Path $windowspkg -ChildPath "rv-v0.13.2-x86_64-pc-windows-msvc.zip"
$rvExtractPath = Join-Path -Path $windowspkg -ChildPath "rv"
$localRpath = Join-Path -Path "src-tauri" -ChildPath "local-r"

# Descargar el instalador de R si no existe
if (-not (Test-Path -Path $rInstallerPath)) {
    Write-Host "Descargando el instalador de R..."
    try {
        Invoke-WebRequest -Uri $rInstallerUrl -OutFile $rInstallerPath
        Write-Host "Descarga de R completada."
    } catch {
        Write-Host "Error al descargar el instalador de R: $_"
    }
} else {
    Write-Host "El instalador de R ya existe. Saltando la descarga."
}

# Descargar y descomprimir el archivo ZIP de rv
if (-not (Test-Path -Path $rvExtractPath -PathType Container)) {
    Write-Host "Descargando rv..."
    try {
        Invoke-WebRequest -Uri $rvZipUrl -OutFile $rvZipPath
        Write-Host "Descarga de rv completada."

        Write-Host "Descomprimiendo el archivo ZIP de rv..."
        Expand-Archive -Path $rvZipPath -DestinationPath $rvExtractPath -Force
        Write-Host "Descompresión de rv completada."

    } catch {
        Write-Host "Error al descargar o descomprimir rv: $_"
    }
} else {
    Write-Host "El archivo de rv ya existe. Saltando la descarga y descompresión."
}

Start-Process -FilePath $rInstallerPath -ArgumentList "/SP-", "/VERYSILENT", "/SUPPRESSMSGBOXES", "/NORESTART", "/DIR=$localRpath" -Wait

Copy-Item ".\windowspkg\rv\rv.exe" -Destination "$localRpath\bin"

# Define la ruta de la carpeta de la aplicación Shiny.
$ShinyAppDir = "shiny-app/"

# Define la ruta de la carpeta de destino dentro de src-tauri.
$TauriAppDir = "src-tauri/app"

# 1. Verificar si la carpeta de la aplicación Shiny existe.
if (-not (Test-Path -Path $ShinyAppDir -PathType Container)) {
    Write-Host "Por favor, coloque el contenido de su proyecto rv con una aplicación Shiny en la carpeta '$ShinyAppDir'"
    exit 1
}

# 2. Eliminar la carpeta de destino antigua si existe.
# rm -rf se traduce a Remove-Item -Recurse -Force en PowerShell.
Write-Host "Eliminando la carpeta de destino antigua: $TauriAppDir"
if (Test-Path -Path $TauriAppDir -PathType Container) {
    Remove-Item -Path $TauriAppDir -Recurse -Force
}

# 3. Copiar la aplicación Shiny a la ubicación de src-tauri.
# cp -r se traduce a Copy-Item -Recurse.
Write-Host "Copiando '$ShinyAppDir' a '$TauriAppDir'"
Copy-Item -Path $ShinyAppDir -Destination $TauriAppDir -Recurse
Write-Host "Copia completada."

$destinationBinPath = Join-Path -Path $localRpath -ChildPath "bin"

$env:Path = "$pwd\$localRpath\bin;" + $env:Path
echo $env:Path
$rvExeFullPath = Join-Path -Path $destinationBinPath -ChildPath "rv.exe"

# Verificar que el ejecutable existe antes de intentar correrlo.
if (-not (Test-Path -Path $rvExeFullPath)) {
    Write-Error "ERROR: El ejecutable rv.exe no se encontró en la ruta esperada: $rvExeFullPath"
    exit 1
}

# Ejecutar 'rv sync' usando la ruta completa y especificando el directorio de trabajo.
Write-Host "Ejecutando 'rv sync' en el directorio: $TauriAppDir"
Start-Process -FilePath $rvExeFullPath -ArgumentList "sync" -Wait -WorkingDirectory $TauriAppDir
Write-Host "'rv sync' completado."

cargo tauri build

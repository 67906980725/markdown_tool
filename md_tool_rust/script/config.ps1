$env_path = $env:PATH
$proj_path = Resolve-Path -Path $(Split-Path -parent $PSScriptRoot)
$conf_path = "$PSScriptRoot\conf"
$venv_name = ".venv"
$venv_path = "$proj_path\$venv_name"

$node_ver = "18.17.0"
$node_dirname = "node-v${node_ver}-win-x64"
$node_zip = "${node_dirname}.zip"
$node_uri = "https://nodejs.org/dist/v${node_ver}/$node_zip"
$node_path = "$venv_path\$node_dirname"
$npm = "$node_path\npm.cmd"
$node = "$node_path\node.exe"
$npmrc = "$conf_path\.npmrc"
$env:NODE_HOME="$node_path"
function npm() {
    $env:PATH = $env_path + ";$node_path"
    & "$npm" --userconfig "$npmrc" @args
    $env:PATH = $env_path
}
function node() {
    npm @args
}

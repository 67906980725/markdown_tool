Push-Location $PSScriptRoot
. .\config.ps1

function install_maven() {
    if (Test-Path "$mvn_path") {
        return
    }

    $d_file = "$venv_path\$maven_zip_name"
    if (!(Test-Path $d_file)) {
        Invoke-WebRequest $maven_uri -OutFile "$d_file"
    }
    Expand-Archive -Path "$d_file" -DestinationPath "$venv_path"
#    Remove-Item "$d_file"
}

function set_java_path() {
    $origin = "${mvn}.origin"
    if (Test-Path "$origin") {
        Copy-Item -Path "$origin" -Destination "$mvn" -Force
    } else {
        Copy-Item -Path "$mvn" -Destination "$origin"
    }

    $content = Get-Content -Path "$mvn"
    $content = $content -replace 'set "JAVACMD=%%~$PATH:i"', "set ""JAVACMD=$java"""
    $content = $content -replace 'set "JAVACMD=%JAVA_HOME%\\bin\\java.exe"', "set ""JAVACMD=$java"""
    $content | Set-Content -Path "$mvn"
}

function setting_maven() {
    Copy-Item -Path "$mvn_setting_template" -Destination "$mvn_setting" -Force
    $content = Get-Content -Path "$mvn_setting"
    $content = $content -replace '<localRepository>C:\\Users\\admin\\.m2\\repository</localRepository>', "<localRepository>$mvn_repo</localRepository>"
    $content | Set-Content -Path "$mvn_setting"
}

install_maven
set_java_path # 项目路径发生变化需要重新执行
setting_maven

Pop-Location

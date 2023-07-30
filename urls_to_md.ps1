# eg: ./urls_to_md.ps1 ./input.txt

param([string] $file = "./input.txt")

Push-Location $PSScriptRoot

New-Item output -ItemType Directory -ErrorAction SilentlyContinue

function process_url([string] $url) {
  $title = & python ./md_tool_py/url_title.py "$url"
  $new_file_path = "./output/${title}.md"
  python ./md_tool_py/html2md.py "$url"
  Move-Item -Force -Path ./html2markdown.md -Destination "$new_file_path"
  $new_file_full_path = Resolve-Path -Path "$new_file_path"
  
  Push-Location ./md_tool_rust
  cargo run "pic_down" "$new_file_full_path"
  Pop-Location
}

$lineCount = Get-Content -Path $file | Measure-Object -Line | Select-Object -ExpandProperty Lines
if ($lineCount -gt 0) {
  foreach($line in Get-Content $file) {
    if([string]::IsNullOrEmpty($line) -or $line.StartsWith("#")) {
      return
    }
    $url = $line.Trim()
    process_url $url
  }
} else {
  $url = Read-Host "Enter the url you want to down"
  process_url $url
}

Push-Location $PSScriptRoot/output
Remove-Item ./*.md
Pop-Location

Pop-Location
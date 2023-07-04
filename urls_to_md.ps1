# winget install Rustlang.Rustup
# winget install Rustlang.Rust.MSVC
# winget install Rustlang.Rust.GNU

# winget install 9NRWMJP3717K # Python 3.11

# python -m ensurepip --default-pip
# pip install requests
# pip install bs4

param([string] $file)

foreach($line in Get-Content $file) {
  if([string]::IsNullOrEmpty($line) -or $line.StartsWith("#")) {
    return
  }

  $url = $line.Trim()
  $title = & python ./url_title.py "$url"
  # $title
  
  python ./html2md.py "$url"
  $new_file_path = "./output/${title}.md"
  Move-Item -Force -Path ./html2markdown.md -Destination "$new_file_path"
  $new_file_full_path = Resolve-Path -Path "$new_file_path"
  
  Push-Location ./md_pic_down
  cargo run "$new_file_full_path"
  Pop-Location
}
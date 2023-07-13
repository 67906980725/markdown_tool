function install_win {
  # python
  winget install 9NRWMJP3717K # Python 3.11

  # rust
  winget install Rustlang.Rustup
  winget install Rustlang.Rust.MSVC
  winget install Rustlang.Rust.GNU
}

function install_py {
  python -m ensurepip --default-pip
  # pip freeze
  # pip install requests
  # pip install bs4
  pip install -r ./md_tool_py/requirements.txt
  
}

function install_md_tool_rust {
  
}


function urls_to_md {
  $l_file = "$PSScriptRoot\urls_to_md.lnk"
  
  $l = (New-Object -ComObject WScript.Shell).CreateShortcut($l_file)
  $l.WorkingDirectory = $PSScriptRoot
  $l.TargetPath = "pwsh.exe"
  $l.Arguments = "-File urls_to_md.ps1"
  $l.Save()
}


function main {
  if ($IsWindows) {
    install_win
  } else {
  
  }
  install_py
  urls_to_md
}

main

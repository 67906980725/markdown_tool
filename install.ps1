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

function main {
  if ($IsWindows) {
    install_win
  } else {
  
  }
  install_py
}

main

#!/usr/bin/env bash
set -euo pipefail
RAPISCM_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

install_apt()   { sudo apt-get install -y "$@"; }
install_dnf()   { sudo dnf install -y "$@"; }
install_pacman(){ sudo pacman -S --noconfirm "$@"; }
install_brew()  { brew install "$@"; }

detect_pm() {
  command -v apt-get &>/dev/null && echo "apt-get" && return
  command -v dnf &>/dev/null && echo "dnf" && return
  command -v pacman &>/dev/null && echo "pacman" && return
  command -v brew &>/dev/null && echo "brew" && return
  echo ""
}

pm_install() {
  local pm
  pm="$(detect_pm)"
  case "$pm" in
    apt-get) install_apt "$@" ;;
    dnf)     install_dnf "$@" ;;
    pacman)  install_pacman "$@" ;;
    brew)    install_brew "$@" ;;
    *)       echo "no pkg manager found"; return 1 ;;
  esac
}

case "${1:-help}" in
  rust)
    if command -v rustc &>/dev/null; then
      echo "rust already installed ($(rustc --version))"
    else
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      echo "rust installed"
    fi
    ;;

  browser)
    if command -v google-chrome &>/dev/null || command -v chromium &>/dev/null; then
      echo "chrome/chromium found"
    else
      echo "installing chromium..."
      pm_install chromium || pm_install chromium-browser || echo "install chromium manually"
    fi

    if command -v geckodriver &>/dev/null; then
      echo "geckodriver found"
    else
      echo "installing geckodriver..."
      if ! pm_install geckodriver 2>/dev/null; then
        LATEST="$(curl -sL https://api.github.com/repos/mozilla/geckodriver/releases/latest | python3 -c "import sys,json; print(json.load(sys.stdin)['tag_name'])")"
        curl -sL "https://github.com/mozilla/geckodriver/releases/download/${LATEST}/geckodriver-${LATEST}-linux64.tar.gz" | tar xz -C /tmp
        sudo mv /tmp/geckodriver /usr/local/bin/
      fi
      echo "geckodriver installed"
    fi
    ;;

  build)
    echo "build deps:"
    if ! command -v cargo &>/dev/null; then
      echo "  installing rust..."
      "$0" rust
    else
      echo "  rust $(rustc --version)"
    fi
    if ! command -v dpkg-deb &>/dev/null; then
      echo "  dpkg-deb not found (optional, for .deb packaging)"
    fi
    if ! command -v rpmbuild &>/dev/null; then
      echo "  rpmbuild not found (optional, for .rpm packaging)"
    fi
    ;;

  all)
    "$0" rust
    "$0" build
    "$0" browser
    ;;

  help|*)
    echo "Usage: $0 [rust|browser|build|all]"
    echo "  rust     Install Rust toolchain via rustup"
    echo "  browser  Install Chrome/Chromium + geckodriver"
    echo "  build    Install build deps (Rust + packaging tools)"
    echo "  all      Install everything"
    ;;
esac

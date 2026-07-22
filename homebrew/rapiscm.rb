# Homebrew formula for rapiscm
#
# Install:  brew install grave0x/tap/rapiscm
# Or:       brew tap grave0x/tap && brew install rapiscm
#
# To publish: push this file to github.com/grave0x/homebrew-tap
# as Formula/rapiscm.rb, then create a GitHub release with the binary.

class Rapiscm < Formula
  desc "API security scanner — OpenAPI spec scanning, URL crawling, fuzzing, corp discovery"
  homepage "https://github.com/grave0x/rapiscm"
  url "https://github.com/grave0x/rapiscm/releases/download/v0.1.0/rapiscm-x86_64-unknown-linux-gnu"
  sha256 "PLACEHOLDER_REPLACE_WITH_ACTUAL_SHA256_AFTER_FIRST_RELEASE"
  license "MIT"
  version "0.1.0"

  on_macos do
    url "https://github.com/grave0x/rapiscm/releases/download/v0.1.0/rapiscm-x86_64-unknown-linux-gnu"
    # For a real macOS build, switch to x86_64-apple-darwin once CI builds it
  end

  def install
    bin.install "rapiscm-x86_64-unknown-linux-gnu" => "rapiscm"
  end

  test do
    system "#{bin}/rapiscm", "--version"
  end
end

class WhiteDragon < Formula
  desc "Lightweight macOS CLI tool for drag-and-drop from terminal"
  homepage "https://github.com/Dimfred/white-dragon"
  url "https://github.com/Dimfred/white-dragon/archive/refs/tags/v0.1.2.tar.gz"
  sha256 "6eadd6388250712400dbd82dc9ff65585135500a8653fe88cfbd88428d39d383"
  license "MIT"

  depends_on :macos
  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "white-dragon", shell_output("#{bin}/white-dragon --help")
  end
end

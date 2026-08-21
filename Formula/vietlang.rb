class Vietlang < Formula
  desc "A Backend-First Programming Language for High-Throughput Microservices & APIs"
  homepage "https://github.com/hoangtuvungcao/vietlang"
  version "0.1.0"
  license "MIT"

  if OS.mac? && Hardware::CPU.arm?
    url "https://github.com/hoangtuvungcao/vietlang/releases/download/v0.1.0/vietlang-macos-arm64"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000" # Updated on release
  elsif OS.mac? && Hardware::CPU.intel?
    url "https://github.com/hoangtuvungcao/vietlang/releases/download/v0.1.0/vietlang-macos-x64"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  elsif OS.linux?
    url "https://github.com/hoangtuvungcao/vietlang/releases/download/v0.1.0/vietlang-linux-x64"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  end

  def install
    bin.install "vietlang"
    pkgshare.install Dir["std/*"]
  end

  test do
    system "#{bin}/vietlang", "--version"
  end
end

class Gm < Formula
  desc "GM CLI tool for managing Flutter projects"
  homepage "https://github.com/ghozimahdi/GM_CLI"
  version "4.0.1"

  on_macos do
    on_arm do
      url "https://github.com/ghozimahdi/GM_CLI/releases/download/v4.0.1/gm-aarch64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
    on_intel do
      url "https://github.com/ghozimahdi/GM_CLI/releases/download/v4.0.1/gm-x86_64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  def install
    bin.install "gm"
  end

  test do
    system bin/"gm", "--help"
  end
end

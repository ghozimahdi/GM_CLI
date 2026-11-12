class GmCli < Formula
  desc "GM CLI tool for managing Flutter projects"
  homepage "https://github.com/ghozimahdi/GM_CLI"
  version "4.0.1"

  on_macos do
    on_arm do
      url "https://github.com/ghozimahdi/GM_CLI/releases/download/v4.0.1/gm-aarch64-apple-darwin.tar.gz"
      sha256 "dd756baa1c6634cc86da7c5957a1aef8f294c29c773d8614abd64ed640ad326d"
    end
    on_intel do
      url "https://github.com/ghozimahdi/GM_CLI/releases/download/v4.0.1/gm-x86_64-apple-darwin.tar.gz"
      sha256 "91313535fd9efd2bf25331c542630d935ba322bead420d46e14aacd2607eafd1"
    end
  end

  def install
    bin.install "gm"
  end

  test do
    system bin/"gm", "--help"
  end
end

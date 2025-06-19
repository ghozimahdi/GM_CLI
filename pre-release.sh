#!/bin/bash

APP_NAME="gm"
VERSION="3.0.1"
RELEASE_URL="https://github.com/ghozimahdi/homebrew-ghozimahdi-cli/releases/download/v$VERSION"

# Platform targets
declare -a TARGETS=(
  "x86_64-apple-darwin" "intel_mac"
  "aarch64-apple-darwin" "arm_mac"
  "x86_64-pc-windows-gnu" "windows"
  "x86_64-unknown-linux-musl" "linux"
)

# Output formula file
FORMULA_FILE="gm.rb"

# Bersihkan formula file lama
rm -f $FORMULA_FILE

# Header formula Homebrew
cat <<EOL > $FORMULA_FILE
class Gm < Formula
  desc "GM CLI tool for managing Flutter projects"
  homepage "https://github.com/ghozimahdi/homebrew-ghozimahdi-cli"
  version "$VERSION"

EOL

# Proses setiap pasangan target dan platform
for ((i=0; i<${#TARGETS[@]}; i+=2)); do
  TARGET="${TARGETS[$i]}"
  PLATFORM="${TARGETS[$((i+1))]}"

  # Tentukan binary path, tambahkan .exe untuk Windows
  if [[ $PLATFORM == "windows" ]]; then
    BINARY_PATH="target/$TARGET/release/$APP_NAME.exe"
  else
    BINARY_PATH="target/$TARGET/release/$APP_NAME"
  fi

  TAR_NAME="$APP_NAME-$TARGET.tar.gz"

  # Periksa apakah binary ada
  if [[ ! -f $BINARY_PATH ]]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
  fi

  # Buat tar.gz dari binary
  echo "Creating tar.gz for $TARGET..."
  tar -czvf $TAR_NAME -C "$(dirname "$BINARY_PATH")" "$(basename "$BINARY_PATH")"

  # Hitung SHA256 checksum
  echo "Calculating SHA256 for $TAR_NAME..."
  SHA256=$(shasum -a 256 $TAR_NAME | awk '{print $1}')

  # Salin tar.gz ke root
  mv $TAR_NAME .

  # Tambahkan URL dan SHA256 ke formula
  if [[ $PLATFORM == "intel_mac" ]]; then
    echo "  if OS.mac? && Hardware::CPU.intel?" >> $FORMULA_FILE
  elif [[ $PLATFORM == "arm_mac" ]]; then
    echo "  elsif OS.mac? && Hardware::CPU.arm?" >> $FORMULA_FILE
  elif [[ $PLATFORM == "windows" ]]; then
    echo "  elsif OS.windows?" >> $FORMULA_FILE
  elif [[ $PLATFORM == "linux" ]]; then
    echo "  elsif OS.linux?" >> $FORMULA_FILE
  fi

  echo "    url \"$RELEASE_URL/$TAR_NAME\"" >> $FORMULA_FILE
  echo "    sha256 \"$SHA256\"" >> $FORMULA_FILE
done

# Tambahkan bagian install ke formula
cat <<EOL >> $FORMULA_FILE
  end

  def install
    bin.install "$APP_NAME"
  end

  test do
    system "#{bin}/gm", "--help"
  end
end
EOL

echo "Pre-release script completed! Formula file '$FORMULA_FILE' created."
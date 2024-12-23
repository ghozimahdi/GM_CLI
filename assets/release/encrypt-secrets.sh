#!/bin/bash

# © 2022 Ghozi Mahdi
# Inspiring Through Code 💻❤️
#
# Find more magic code tricks at:
# Website: www.mbahgojol.com
# GitHub: github.com/mbahgojol
# YouTube: https://www.youtube.com/@mbahgojol

encrypt() {
  PASSPHRASE=$1
  INPUT=$2
  OUTPUT=$3
  # Check if output file exists and compare content
  if [ -f "$OUTPUT" ]; then
    # Decrypt existing .gpg to temporary file
    TEMP_DECRYPTED=$(mktemp)
    gpg --batch --yes --quiet --passphrase="$PASSPHRASE" --output "$TEMP_DECRYPTED" --decrypt "$OUTPUT"
    # Compare decrypted content with the input file
    if cmp -s "$TEMP_DECRYPTED" "$INPUT"; then
      echo "No changes detected in $INPUT, skipping encryption."
      rm "$TEMP_DECRYPTED"
      return
    fi
    rm "$TEMP_DECRYPTED"
  fi
  # Proceed with encryption if no match or output file doesn't exist
  gpg --batch --yes --passphrase="$PASSPHRASE" --cipher-algo AES256 --symmetric --output "$OUTPUT" "$INPUT"
}

# Check if ENCRYPT_KEY variable is not empty
if [[ -n "$ENCRYPT_KEY" ]]; then
  # Define the environments
  environments=("dev" "staging" "prod")

  for environment in "${environments[@]}"; do
    # Encrypt Release key
    encrypt "$ENCRYPT_KEY" "release/formulir-keystore.jks" "release/formulir-keystore.jks.gpg"

    # Encrypt Google Services key (Android)
    encrypt "$ENCRYPT_KEY" "packages/app/android/app/src/$environment/google-services.json" "release/config/$environment/google-services.json.gpg"

    # Encrypt Google Services key (iOS)
    encrypt "$ENCRYPT_KEY" "packages/app/ios/config/$environment/GoogleService-Info.plist" "release/config/$environment/GoogleService-Info.plist.gpg"

    # Encrypt firebase_app_id_file (iOS)
    encrypt "$ENCRYPT_KEY" "packages/app/ios/config/$environment/firebase_app_id_file.json" "release/config/$environment/firebase_app_id_file.json.gpg"
  done
else
  echo "ENCRYPT_KEY is empty"
fi

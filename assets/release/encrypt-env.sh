#!/bin/bash

# © 2022 Ghozi Mahdi
# Inspiring Through Code 💻❤️
#
# Find more magic code tricks at:
# Website: www.mbahgojol.com
# GitHub: github.com/mbahgojol
# YouTube: https://www.youtube.com/@mbahgojol

# Function to encrypt files
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

# Array of environments
environments=("dev" "staging" "prod")

# Check if ENCRYPT_KEY variable is not empty
if [[ -n "$ENCRYPT_KEY" ]]; then
  for environment in "${environments[@]}"; do
    encrypt "$ENCRYPT_KEY" "packages/app/core/.env_$environment" "release/config/$environment/env_$environment.gpg"
  done
else
  echo "ENCRYPT_KEY is empty"
fi

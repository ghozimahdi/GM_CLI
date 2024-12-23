#!/bin/bash

# © 2022 Ghozi Mahdi
# Inspiring Through Code 💻❤️
#
# Find more magic code tricks at:
# Website: www.mbahgojol.com
# GitHub: github.com/mbahgojol
# YouTube: https://www.youtube.com/@mbahgojol

decrypt() {
  PASSPHRASE=$1
  INPUT=$2
  OUTPUT=$3
  gpg --quiet --batch --yes --decrypt --passphrase="$PASSPHRASE" --output "$OUTPUT" "$INPUT"
}

# Array of environments
environments=("dev" "staging" "prod")

if [[ -n "$ENCRYPT_KEY" ]]; then
  for environment in "${environments[@]}"; do
    decrypt "$ENCRYPT_KEY" "release/config/$environment/env_$environment.gpg" "packages/library/core/.env_$environment"
  done
else
  echo "ENCRYPT_KEY is empty"
fi

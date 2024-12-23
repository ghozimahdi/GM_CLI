#!/bin/bash

# Function to decrypt files
decrypt() {
  PASSPHRASE=$1
  INPUT=$2
  OUTPUT=$3
  gpg --quiet --batch --yes --decrypt --passphrase="$PASSPHRASE" --output "$OUTPUT" "$INPUT"
}

# Array of environments
environments=("dev" "staging" "prod")
qaEnvironments=("qaDev" "qaStaging" "qaProd")

# Create directories for each environment and corresponding QA environment in iOS
for env in "${environments[@]}" "${qaEnvironments[@]}"; do
  mkdir -p "packages/app/ios/config/$env"
done

# Check if ENCRYPT_KEY variable is not empty
if [[ -n "$ENCRYPT_KEY" ]]; then
  for index in "${!environments[@]}"; do
    environment=${environments[$index]}
    qaEnvironment=${qaEnvironments[$index]}

    decrypt "$ENCRYPT_KEY" "release/formulir-keystore.jks.gpg" "release/formulir-keystore.jks"

    # Decrypt and copy Google Services key files (Android)
    decrypt "$ENCRYPT_KEY" "release/config/$environment/google-services.json.gpg" "packages/app/android/app/src/$environment/google-services.json"

    # Copy to corresponding QA flavor directory (Android)
    cp "packages/app/android/app/src/$environment/google-services.json" "packages/app/android/app/src/$qaEnvironment/google-services.json"

    # Decrypt Google Services key files (iOS)
    decrypt "$ENCRYPT_KEY" "release/config/$environment/GoogleService-Info.plist.gpg" "packages/app/ios/config/$environment/GoogleService-Info.plist"

    # Decrypt firebase_app_id_file files (iOS)
    decrypt "$ENCRYPT_KEY" "release/config/$environment/firebase_app_id_file.json.gpg" "packages/app/ios/config/$environment/firebase_app_id_file.json"

    # Copy to corresponding QA environment directory (iOS)
    cp "packages/app/ios/config/$environment/GoogleService-Info.plist" "packages/app/ios/config/$qaEnvironment/GoogleService-Info.plist"
    cp "packages/app/ios/config/$environment/firebase_app_id_file.json" "packages/app/ios/config/$qaEnvironment/firebase_app_id_file.json"
  done
else
  echo "ENCRYPT_KEY is empty"
fi

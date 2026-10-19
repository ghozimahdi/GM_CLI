#!/bin/bash
# Usage Instructions:
# 1. Ensure that you have installed Firebase CLI tools on your system.
#    If not, install them using the command: npm install -g firebase-tools
# 2. Make sure you are logged in to Firebase with the appropriate account:
#    Run the command: firebase login
# 3. Verify your Firebase project access by listing available projects:
#    Use the command: firebase projects:list
# 4. Once the prerequisites are met, execute this script with the desired environment:
#    ./flutterfire-config.sh dev  - for development configuration
#    ./flutterfire-config.sh stg  - for staging configuration
#    ./flutterfire-config.sh prod - for production configuration

# Please ensure all the above steps are followed before proceeding to avoid any issues.

APP_DIR="packages/app" # Path to your Flutter app directory

# Function to check if flutterfire CLI is installed
check_flutterfire() {
  if ! command -v flutterfire &> /dev/null; then
    echo "flutterfire CLI not found. Installing it..."
    dart pub global activate flutterfire_cli
    if [[ $? -ne 0 ]]; then
      echo "Error: Failed to install flutterfire CLI. Please check your Dart setup."
      exit 1
    fi
    echo "flutterfire CLI installed successfully."
  else
    echo "flutterfire CLI is already installed."
  fi
}

# Check if flutterfire CLI is installed
check_flutterfire

# Check if an environment argument is provided
if [[ $# -eq 0 ]]; then
  echo "Error: No environment specified. Use 'dev', 'stg', or 'prod'."
  exit 1
fi

# Navigate to the Flutter app directory
if [[ ! -d "$APP_DIR" ]]; then
  echo "Error: Flutter app directory ($APP_DIR) does not exist."
  exit 1
fi

cd "$APP_DIR" || exit

# Configure firebase based on the environment
case $1 in
  dev)
    flutterfire config \
      --project=formulir-dev-d110c \
      --out=firebase_options_dev.dart \
      --ios-bundle-id=com.mengantar.formulir.dev \
      --ios-out=ios/config/dev/GoogleService-Info.plist \
      --android-package-name=com.mengantar.formulir.dev \
      --android-out=android/app/src/dev/google-services.json
    ;;
  stg)
    flutterfire config \
      --project=formulir-staging-ab3f6 \
      --out=firebase_options_staging.dart \
      --ios-bundle-id=com.mengantar.formulir.staging \
      --ios-out=ios/config/staging/GoogleService-Info.plist \
      --android-package-name=com.mengantar.formulir.staging \
      --android-out=android/app/src/staging/google-services.json
    ;;
  prod)
    flutterfire config \
      --project=formulir-prod-8604e \
      --out=firebase_options_prod.dart \
      --ios-bundle-id=com.mengantar.formulir \
      --ios-out=ios/config/prod/GoogleService-Info.plist \
      --android-package-name=com.mengantar.formulir \
      --android-out=android/app/src/prod/google-services.json
    ;;
  *)
    echo "Error: Invalid environment specified. Use 'dev', 'stg', or 'prod'."
    exit 1
    ;;
esac

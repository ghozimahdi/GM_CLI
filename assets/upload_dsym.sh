#!/bin/bash

ENV=$1

if [ -z "$ENV" ]; then
  echo "Error: No environment specified. Use 'dev', 'staging', or 'prod'."
  exit 1
fi

case $ENV in
  dev)
    GOOGLE_SERVICE_INFO="packages/app/ios/config/dev/GoogleService-Info.plist"
    UPLOAD_SYMBOLS="packages/app/ios/Pods/FirebaseCrashlytics/upload-symbols"
    DSYM_PATH="packages/app/build/ios/Debug-dev-iphonesimulator/Runner.app.dSYM"
    ;;
  staging)
    GOOGLE_SERVICE_INFO="packages/app/ios/config/staging/GoogleService-Info.plist"
    UPLOAD_SYMBOLS="packages/app/ios/Pods/FirebaseCrashlytics/upload-symbols"
    DSYM_PATH="packages/app/build/ios/Debug-staging-iphonesimulator/Runner.app.dSYM"
    ;;
  prod)
    GOOGLE_SERVICE_INFO="packages/app/ios/config/prod/GoogleService-Info.plist"
    UPLOAD_SYMBOLS="packages/app/ios/Pods/FirebaseCrashlytics/upload-symbols"
    DSYM_PATH="packages/app/build/ios/Debug-prod-iphonesimulator/Runner.app.dSYM"
    ;;
  *)
    echo "Error: Invalid environment specified. Use 'dev', 'staging', or 'prod'."
    exit 1
    ;;
esac

if [ ! -f "$GOOGLE_SERVICE_INFO" ]; then
  echo "Error: GoogleService-Info.plist not found at $GOOGLE_SERVICE_INFO"
  exit 1
fi

if [ ! -f "$UPLOAD_SYMBOLS" ]; then
  echo "Error: upload-symbols script not found at $UPLOAD_SYMBOLS"
  exit 1
fi

if [ ! -d "$DSYM_PATH" ]; then
  echo "Error: dSYM directory not found at $DSYM_PATH"
  exit 1
fi

$UPLOAD_SYMBOLS -gsp "$GOOGLE_SERVICE_INFO" -p ios "$DSYM_PATH"

if [ $? -eq 0 ]; then
  echo "dSYM files uploaded successfully for $ENV environment!"
else
  echo "Failed to upload dSYM files for $ENV environment."
  exit 1
fi

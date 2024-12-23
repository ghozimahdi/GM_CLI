#!/bin/sh

# © 2022 Ghozi Mahdi
# Inspiring Through Code 💻❤️
#
# Find more magic code tricks at:
# Website: www.mbahgojol.com
# GitHub: github.com/mbahgojol
# YouTube: https://www.youtube.com/@mbahgojol

# Delete Release key
rm -f mengantar-keystore.jks

# Delete google-services.json from specific Android directories
rm -f ../packages/app/android/app/src/*/google-services.json

# Delete GoogleService-info.plist from specific iOS directories
rm -f ../packages/app/ios/config/*/GoogleService-Info.plist
rm -f ../packages/app/ios/config/*/firebase_app_id_file.json

# Remove GoogleService-Info.plist from iOS Runner directory
rm -f ../packages/app/ios/Runner/GoogleService-Info.plist

# Delete .env_* from core application
rm -f ../packages/library/core/.env_*

#!/bin/bash

# © 2024 Ghozi Mahdi
# Inspiring Through Code 💻❤️
#
# Find more magic code tricks at:
# Website: www.mbahgojol.com
# GitHub: github.com/mbahgojol
# YouTube: https://www.youtube.com/@mbahgojol

# before running this please read root-project/release/doc.md
chmod +x cryptify
./cryptify decrypt
./cryptify sync

# make sure you have melos in your system
#melos clean
#melos bs
#melos locale
#melos build

rm -f .git/hooks/pre-push
echo "Creating pre-push hook..."

# Create the pre-push hook file and make it executable
echo '#!/bin/bash' >.git/hooks/pre-push
chmod +x .git/hooks/pre-push

# Add the content to the pre-push hook
cat <<EOL >>.git/hooks/pre-push
source ~/.zshrc
echo "Running pre-push script..."

# Run encryption scripts
./cryptify encrypt

# Check if there are any uncommitted .gpg files in release/ and release/config/
UNSTAGED_GPG_FILES=\$(git ls-files -m release/*.gpg release/config/**/*.gpg)

# If there are uncommitted .gpg files, then add and commit them
if [[ ! -z "\$UNSTAGED_GPG_FILES" ]]; then
  git add release/*.gpg release/config/**/*.gpg
  git commit -m "Add encrypted files for environment and secrets"
else
  echo "No uncommitted .gpg files in release/ and release/config/, skipping encryption scripts."
fi

exit 0
EOL

echo "Pre-push hook created successfully."

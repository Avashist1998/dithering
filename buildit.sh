#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status.

# Ensure we're in the root directory of the project
cd "$(git rev-parse --show-toplevel)"

# Update main branch
git checkout main
git pull

echo "Building project"
cd web-app
wasm-pack build --target web

# Create and switch to a new gh-pages branch
git checkout --orphan gh-pages-new

# Remove all files from git tracking, but keep them on disk
git rm -rf --cached .

# Go back to root directory
cd ..

# Move copied files to root
mv web-app/pkg .
mv web-app/index.html .
mv web-app/style.css .
mv web-app/index.js .

rm -f pkg/.gitignore

# Create a temporary directory
mkdir ../temp_deploy

# Move files we want to keep to the temporary directory
mv pkg index.html style.css index.js ../temp_deploy/
# Remove all content in the current directory except .git
find . -maxdepth 1 ! -name '.git' ! -name '.' -exec rm -rf {} +

# # Move files back from temporary directory
mv ../temp_deploy/* .
rm -rf ../temp_deploy

# # Add all files
git add .

# # Commit changes
git commit -m "feat: Deployment"

# Push to gh-pages branch, creating it if it doesn't exist
git push -f origin gh-pages-new:gh-pages

# Switch back to main branch
git checkout main

# Remove the temporary branch
git branch -D gh-pages-new

echo "Deployment complete"
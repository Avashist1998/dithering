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

# Copy necessary files
cp -r pkg ../
cp index.html ../
cp style.css ../
cp index.js ../

# Go back to root directory
cd ..

# Remove unnecessary files and directories
git rm -rf .
git clean -fxd

# Move copied files to root
mv web-app/pkg .
mv web-app/index.html .
mv web-app/style.css .
mv web-app/index.js .

# Remove .gitignore from pkg folder
rm -f pkg/.gitignore

# Add all files
git add .

# Commit changes
git commit -m "feat: Deployment"

# Push to gh-pages branch, creating it if it doesn't exist
git push -f origin gh-pages-new:gh-pages

# Switch back to main branch
git checkout main

# Remove the temporary branch
git branch -D gh-pages-new

echo "Deployment complete"
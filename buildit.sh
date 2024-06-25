#!/bin/bash

git checkout main
git pull


echo "Building project"
cd web-app
wasm-pack build --target web

cp -r pkg ../
rm ../pkg/.gitignore
cp index.html ../
cp style.css ../
cp index.js ../

git add .
git stash
git checkout gh-pages -f
git stash pop
git rm web-app
git add .
git commit -m "feat: Deployment"
git push -f

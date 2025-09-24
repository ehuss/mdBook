#!/usr/bin/env bash
# Updates all compatible Cargo dependencies.

set -ex

git fetch origin update-dependencies
if git checkout update-dependencies
then
    git reset --hard origin/master
else
    git checkout -b update-dependencies
fi

cargo upgrade
if git diff --quiet
then
    echo "No changes detected, exiting."
    exit 0
fi

git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

git add Cargo.toml Cargo.lock
git commit -m "Update cargo dependencies"

git push --force origin update-dependencies

gh pr create --title "Update cargo dependencies" \
    --body "Automated update of Cargo dependencies" \
    --head update-dependencies \
    --base master

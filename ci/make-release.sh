#!/usr/bin/env bash
# Builds the release and creates an archive and optionally deploys to GitHub.
set -ex

if [[ -z "$GITHUB_REF" ]]
then
  echo "GITHUB_REF must be set"
  exit 1
fi

host=$(rustc -Vv | grep ^host: | sed -e "s/host: //g")
cargo rustc --bin mdbook --release -- -C lto
cd target/release
case $1 in
  ubuntu* | macos*)
    asset="mdbook-$GITHUB_REF-$host.tar.gz"
    tar czf ../../$asset mdbook
    ;;
  windows*)
    asset="mdbook-$GITHUB_REF-$host.zip"
    7z a ../../$asset mdbook.exe
    ;;
  *)
    echo "OS should be first parameter, was: $1"
    ;;
esac

if [[ -z "GITHUB_TOKEN" ]]
then
  echo "$GITHUB_TOKEN not set, skipping deploy."
else
  hub release edit --attach $asset $GITHUB_REF
fi

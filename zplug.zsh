#!/bin/zsh

set -e

version=$(grep '^version =' Cargo.toml | cut -d'"' -f2)

if [ $(uname) = "Darwin" ]
then
    platform="macos"
else
    platform="ubuntu"
fi

url="https://github.com/g-plane/zsh-yarn-autocompletions/releases/download/v$version/yarn-autocompletions_$platform-latest.zip"

if [ $(hash wget 2>/dev/null) ]
then
    wget $url > zipball.zip
else
    curl -fsSL $url > zipball.zip
fi

unzip zipball.zip yarn-autocompletions

rm zipball.zip

# Insallation command: zplug "g-plane/zsh-yarn-autocompletions", hook-build:"zplug.sh"

set -e

version=$(grep -Eho "version = \"\d+\.\d+\.\d+" Cargo.toml | sed "s/version = \"//")

if [ $(uname) = "Darwin" ]
then
    platform="macos"
else
    platform="linux"
fi

url="https://github.com/g-plane/zsh-yarn-autocompletions/releases/download/v$version/yarn-autocompletions_v${version}_$platform.tar.gz"

if [ $(hash wget 2>/dev/null) ]
then
    wget $url > tarball.tar.gz
else
    curl -fsSL $url > tarball.tar.gz
fi

tar -zxf tarball.tar.gz ./yarn-autocompletions

rm tarball.tar.gz

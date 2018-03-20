function build() {
  rm -rf ./$1/target
  cd $1
  cargo build --release
  cd ..
}

build yarn-deps
build yarn-scripts

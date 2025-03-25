alias b := build
alias c := check
alias t := test

target := "wasm32-unknown-unknown"

# quick check
check:
  cargo c --message-format=short -p app --target {{target}}

# run unit tests
test:
  cargo t --message-format=short -p music

# build & minify
build:
  rm -rf dist
  mkdir dist

  cargo b -p app --target {{target}} --release

  cp src/*.html dist/
  cp src/*.css dist/
  cp target/{{target}}/release/app.wasm dist/app.wasm
  twiggy top -n10 dist/app.wasm || true
  wasm-opt --strip-debug --strip-producers --strip-target-features -Oz dist/app.wasm -o dist/app.wasm || true

  just xtask generate-app-js
  just xtask compress-app-wasm
  just xtask minify
  just xtask print-sizes

# install pre-commit hook
setup:
  cp src/pre-commit.bash .git/hooks/pre-commit

# pre-commit hook
pre-commit:
  git diff --quiet || exit 1
  cargo clippy --target {{target}} -p app -- -D warnings
  cargo clippy -p xtask -- -D warnings
  cargo fmt --check

# serve webapp
serve: build
  just xtask serve

[private]
xtask task:
  cargo r -p xtask -- {{task}}

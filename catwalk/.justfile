[private]
default:
  @just --choose

[private]
wasm-pack-build *args:
  wasm-pack build --scope catppuccin --release {{args}}

build-wasm:
  @echo "Building npm package..."
  rm -rf ./pkg || true
  mkdir -p ./pkg
  @just wasm-pack-build \
    --target nodejs \
    --out-dir ./pkg \
    --out-name catwalk
  sed -i 's/@catppuccin\/catppuccin-catwalk/@catppuccin\/catwalk/g' ./pkg/package.json
  @just wasm-pack-build \
    --target deno \
    --out-dir ./pkg/deno \
    --out-name catwalk
  rm ./pkg/.gitignore || true
  rm ./pkg/deno/.gitignore || true

check:
  cargo clippy || true
  cargo clippy --target wasm32-unknown-unknown --lib || true
  cargo clippy --target wasm32-unknown-unknown --lib --all-features || true

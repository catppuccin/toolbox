[private]
default:
  @just --choose

[private]
wasm-pack-build *args:
  wasm-pack build --scope catppuccin --release {{args}}

build-wasm:
  @echo "Building npm package..."
  @rm -rf ./pkg || true
  @mkdir -p ./pkg
  @just wasm-pack-build \
    --target bundler \
    --out-dir ./pkg \
    .
  @just wasm-pack-build \
    --target deno \
    --out-dir ./pkg/deno \
    .
  # keep deno for ESM once stabilized
  #rm ./pkg/deno/.gitignore || true

check:
  cargo clippy || true
  cargo clippy --target wasm32-unknown-unknown --lib || true
  cargo clippy --target wasm32-unknown-unknown --lib --all-features || true

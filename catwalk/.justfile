[private]
default:
  @just --choose

[private]
wasm-pack-build *args:
  wasm-pack build --scope catppuccin --release --no-pack {{args}}

build-wasm:
  @echo "Building npm package..."
  @rm -rf ./pkg/{runtime,web} || true
  @just wasm-pack-build \
    --target deno \
    --out-dir ./pkg/deno \
    . \
    --features wasm_buffers
  @just wasm-pack-build \
    --target web \
    --out-dir ./pkg/web \
    .
  # keep deno for ESM once stabilized
  #rm ./pkg/deno/.gitignore || true

check:
  cargo clippy || true
  cargo clippy --target wasm32-unknown-unknown --lib || true
  cargo clippy --target wasm32-unknown-unknown --lib --all-features || true

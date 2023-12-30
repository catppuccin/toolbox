{
  description = "Catppuccin's development tools, dev flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
    call-flake.url = "github:divnix/call-flake";
  };

  outputs = {
    self,
    nixpkgs,
    call-flake,
    ...
  } @ inputs: let
    systems = ["aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux"];
    overlays = [(import inputs.rust-overlay)];
    forEachSystem = fn: nixpkgs.lib.genAttrs systems (system: fn (import nixpkgs {inherit overlays system;}));
    mainFlake = call-flake ../..;
  in {
    inherit (mainFlake) checks formatter packages;

    devShells = forEachSystem (pkgs: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          self.formatter.${pkgs.system}
          deno
          (rust-bin.stable.latest.default.override {
            extensions = ["rust-analyzer" "rust-src"];
            targets = ["wasm32-unknown-unknown"];
          })
          # wasm + publishing to npm
          binaryen
          nodejs
          wasm-bindgen-cli
          wasm-pack
        ];
      };
    });
  };

  nixConfig = {
    extra-substituters = ["https://catppuccin.cachix.org"];
    extra-trusted-public-keys = ["catppuccin.cachix.org-1:noG/4HkbhJb+lUAdKrph6LaozJvAeEEZj4N732IysmU="];
  };
}

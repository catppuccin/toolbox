{
  description = "Catppuccin's development tools, dev flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    call-flake.url = "github:divnix/call-flake";
  };

  outputs = {
    self,
    nixpkgs,
    call-flake,
    ...
  } @ inputs: let
    systems = ["aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux"];
    overlays = [inputs.rust-overlay.overlays.default];
    inherit (nixpkgs) lib;
    forEachSystem = fn: lib.genAttrs systems (system: fn (import nixpkgs {inherit overlays system;}));
    mainFlake = call-flake ../..;
  in {
    inherit (mainFlake) checks formatter packages;

    devShells = forEachSystem (pkgs: let
      inherit (pkgs.stdenv) isDarwin;
      rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
        toolchain.default.override {
          extensions = ["rust-src"];
          targets = ["wasm32-unknown-unknown"];
        });
    in {
      default = pkgs.mkShell {
        buildInputs = with pkgs;
          [
            self.formatter.${pkgs.system}
            rust-toolchain
            rust-analyzer
            deno
            # wasm + publishing to npm
            binaryen
            nodejs
            wasm-bindgen-cli
            wasm-pack
            # wasm-bindgen can require lcurl to build
            curl
          ]
          ++ lib.optionals isDarwin [libiconv];
        env.RUST_SRC_PATH = "${rust-toolchain}/lib/rustlib/src/rust/library";
      };
    });
  };

  nixConfig = {
    extra-substituters = ["https://catppuccin.cachix.org" "https://nix-community.cachix.org"];
    extra-trusted-public-keys = ["catppuccin.cachix.org-1:noG/4HkbhJb+lUAdKrph6LaozJvAeEEZj4N732IysmU=" "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="];
  };
}

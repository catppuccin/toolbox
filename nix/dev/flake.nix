{
  description = "Catppuccin's development tools, dev flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "https://flakehub.com/f/nix-community/fenix/0.1.*.tar.gz";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    call-flake.url = "github:divnix/call-flake";
  };

  outputs = {
    self,
    nixpkgs,
    call-flake,
    ...
  } @ inputs: let
    systems = ["aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux"];
    overlays = [inputs.fenix.overlays.default];
    inherit (nixpkgs) lib;
    forEachSystem = fn: lib.genAttrs systems (system: fn (import nixpkgs {inherit overlays system;}));
    mainFlake = call-flake ../..;
  in {
    inherit (mainFlake) checks formatter packages;

    devShells = forEachSystem (pkgs: let
      inherit (pkgs.stdenv) isDarwin;
      rust-toolchain = pkgs.fenix.combine [
        pkgs.fenix.complete.toolchain
        pkgs.rust-analyzer
        pkgs.fenix.targets.wasm32-unknown-unknown.latest.rust-std
      ];
    in {
      default = pkgs.mkShell {
        buildInputs = with pkgs;
          [
            self.formatter.${pkgs.system}
            rust-toolchain
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

{
  description = "Catppuccin's development tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs: let
    systems = ["aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux"];
    overlays = [(import inputs.rust-overlay)];
    forEachSystem = fn: nixpkgs.lib.genAttrs systems (system: fn (import nixpkgs {inherit overlays system;}));
    version = builtins.substring 0 8 self.lastModifiedDate;
  in rec {
    packages = forEachSystem (pkgs: let
      derivs = pkgs.callPackage ./nix {inherit version;};
    in (builtins.listToAttrs (builtins.map (name: {
      inherit name;
      value = derivs.${name};
    }) ["catwalk" "contrast_test" "docpuccin" "inkcat" "puccinier"])));

    devShells = forEachSystem (pkgs: rec {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          node2nix
          self.formatter.${pkgs.system}
        ];
      };
      catwalk = pkgs.mkShell {
        buildInputs = with pkgs;
          [
            (rust-bin.stable.latest.default.override {
              extensions = ["rust-src"];
              targets = ["wasm32-unknown-unknown"];
            })
            rust-analyzer

            binaryen
            deno
            nodejs
            wasm-bindgen-cli
            wasm-pack
          ]
          ++ default.buildInputs;
      };
    });

    formatter = forEachSystem (pkgs: pkgs.alejandra);
  };
}

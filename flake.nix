{
  description = "Catppuccin's development tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
  };

  outputs = {nixpkgs, ...} @ inputs: let
    systems = ["aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux"];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
  in rec {
    packages = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      derivs = pkgs.callPackage ./nix {
        inherit system;
        version = builtins.substring 0 8 inputs.self.lastModifiedDate;
      };
    in
      builtins.listToAttrs (builtins.map (name: {
        inherit name;
        value = derivs.${name};
      }) ["puccinier" "catwalk" "inkcat" "docpuccin" "contrast_test" "palette_builder"]));

    apps = forAllSystems (
      system:
        builtins.listToAttrs (builtins.map (name: {
            inherit name;
            value = {
              type = "app";
              program = "${packages.${system}.${name}}/bin/${name}";
            };
          })
          (builtins.attrNames packages.${system}))
    );

    devShells = forAllSystems (system: let
      overlays = [(import inputs.rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
    in {
      catwalk = pkgs.mkShell {
        buildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            targets = ["wasm32-unknown-unknown"];
          })
          binaryen
          wasm-pack
          (
            # waiting on https://github.com/NixOS/nixpkgs/pull/244334
            wasm-bindgen-cli.overrideAttrs (old: rec {
              pname = "wasm-bindgen-cli";
              version = "0.2.87";
              src = pkgs.fetchCrate {
                inherit pname version;
                sha256 = "sha256-0u9bl+FkXEK2b54n7/l9JOCtKo+pb42GF9E1EnAUQa0=";
              };
              cargoDeps = old.cargoDeps.overrideAttrs (lib.const {
                name = "${pname}-${version}-vendor.tar.gz";
                inherit src;
                outputHash = "sha256-AsZBtE2qHJqQtuCt/wCAgOoxYMfvDh8IzBPAOkYSYko=";
              });
              doCheck = false;
            })
          )
        ];
      };
    });

    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);
  };
}

{
  description = "Catppuccin's development tools";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        derivs = pkgs.callPackage ./nix {inherit system;};
      in rec {
        packages = builtins.listToAttrs (builtins.map (name: {
          inherit name;
          value = derivs.${name};
        }) ["puccinier" "catwalk" "inkcat" "docpuccin" "contrast_test" "palette_builder"]);

        apps = builtins.listToAttrs (builtins.map (name: {
            inherit name;
            value = flake-utils.lib.mkApp {
              drv = derivs.${name};
            };
          })
          (builtins.attrNames
            packages));

        formatter = pkgs.alejandra;
      }
    );
}

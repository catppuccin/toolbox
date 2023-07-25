{
  description = "Catppuccin's development tools";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    systems = [
      "aarch64-darwin"
      "aarch64-linux"
      "armv6l-linux"
      "armv7l-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
  in rec {
    packages = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      derivs = pkgs.callPackage ./nix {inherit system;};
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
              program = "${self.packages.${system}}/bin/${name}";
            };
          })
          (builtins.attrNames packages))
    );

    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      catwalk = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          clippy
          rust-analyzer
          rustc
        ];
      };
    });

    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);
  };
}

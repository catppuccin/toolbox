{
  description = "Catppuccin's development tools";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      forEachSystem = fn: lib.genAttrs systems (system: fn (import nixpkgs { inherit system; }));
      inherit (nixpkgs) lib;
    in
    {
      checks = forEachSystem (pkgs: self.packages.${pkgs.system});
      packages =
        forEachSystem (pkgs: lib.filterAttrs (_: v: lib.isDerivation v) (pkgs.callPackage ./nix { }))
        // {
          whiskers = throw "This package has been moved to x `github:catppuccin/whiskers`";
        };
      overlays.default = final: prev: {
        catppuccin-catwalk = (prev.callPackage ./nix { }).catwalk;
        catppuccin-whiskers = throw "This package has been moved to x `github:catppuccin/whiskers`";
      };
      formatter = forEachSystem (pkgs: pkgs.alejandra);
    };

  nixConfig = {
    extra-substituters = [ "https://catppuccin.cachix.org" ];
    extra-trusted-public-keys = [
      "catppuccin.cachix.org-1:noG/4HkbhJb+lUAdKrph6LaozJvAeEEZj4N732IysmU="
    ];
  };
}

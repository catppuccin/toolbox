{
  description = "Catppuccin's development tools";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
          system: function nixpkgs.legacyPackages.${system}
        );
    in
    {
      packages = forAllSystems (_: {
        whiskers = throw "This package has been moved to `github:catppuccin/whiskers`";
        catwalk = throw "This package has been moved to `github:catppuccin/catwalk`";
      });
    };
}

<h3 align="center">
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
  Catppuccin Toolbox
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
  <a href="https://github.com/catppuccin/toolbox/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/toolbox?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/issues"><img src="https://img.shields.io/github/issues/catppuccin/toolbox?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/toolbox?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

&nbsp;

## Catppuccin's development tools

A set of software tools by Catppuccin developers, for Catppuccin developers.

- [Catwalk](https://github.com/catppuccin/toolbox/tree/main/catwalk#readme)
- [Whiskers](https://github.com/catppuccin/toolbox/tree/main/whiskers#readme)

&nbsp;

## Nix

### With Flakes

Add the following to your `flake.nix`:

#### NixOS

```nix
{
    inputs = {
        catppuccin-toolbox.url = "github:catppuccin/toolbox";
    };
    outputs = {nixpkgs, catppuccin-toolbox, ...}: {
        nixosConfigurations.HOSTNAME = nixpkgs.lib.nixosSystem {
          modules = [
          {
              environment.systemPackages = [
                catppuccin-toolbox.packages.${pkgs.system}.whiskers
              ];
            }
          ];
        };
      };
    }
}
```

#### Home Manager

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    catppuccin-toolbox.url = "github:catppuccin/toolbox";
  };

  outputs = {nixpkgs, home-manager, catppuccin-toolbox, ...}: {
    homeConfigurations."user@hostname" = home-manager.lib.homeManagerConfiguration {
      pkgs = nixpkgs.legacyPackages.x86_64-linux;

      modules = [
        {
            home.packages = [
                catppuccin-toolbox.packages.${pkgs.system}.whiskers
            ];
        }
      ];
    };
  };
}
```

### Without Flakes

Add the following to your configuration:

```nix
{config, pkgs, ...}: let
  flake-compat = builtins.fetchTarball "https://github.com/edolstra/flake-compat/archive/master.tar.gz";
  catppuccin-toolbox = (import flake-compat {
    src = builtins.fetchTarball "https://github.com/catppuccin/toolbox/archive/main.tar.gz";
  }).defaultNix;
in {
    # Home Manager
    home.packages = [
        catppuccin-toolbox.packages.${pkgs.system}.whiskers
    ];

    # Nix
    environment.systemPackages = [
        catppuccin-toolbox.packages.${pkgs.system}.whiskers
    ];
}
```

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>

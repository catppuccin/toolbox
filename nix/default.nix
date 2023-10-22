{
  pkgs,
  version,
}: let
  mkNodePkg = {
    pname,
    description,
    npmDepsHash,
    ...
  } @ args:
    pkgs.buildNpmPackage ({
        inherit pname version npmDepsHash;
        src = pkgs.nix-gitignore.gitignoreSourcePure [../.gitignore] ../${pname};
        dontNpmBuild = true;

        meta = with pkgs.lib; {
          inherit description;
          homepage = "https://github.com/catppuccin/toolbox/tree/main/${pname}";
          license = licenses.mit;
          maintainers = with maintainers; [rubyowo];
        };
      }
      // args);

  mkRustPkg = {
    pname,
    description,
    ...
  } @ args:
    pkgs.rustPlatform.buildRustPackage ({
        inherit pname version;
        src = pkgs.nix-gitignore.gitignoreSourcePure [../.gitignore] ../.;

        cargoLock.lockFile = ../Cargo.lock;
        cargoBuildFlags = "-p ${pname}";

        meta = with pkgs.lib; {
          inherit description;
          homepage = "https://github.com/catppuccin/toolbox/tree/main/${pname}";
          license = licenses.mit;
          maintainers = with maintainers; [rubyowo];
        };
      }
      // args);

  nodePkgs = [
    {
      pname = "docpuccin";
      npmDepsHash = "sha256-7/3wIis9c/P8zlQD3YbnRBPtpOcGDXchwcuC7/9fiWE=";
      description = "Fetch health files needed per project type";
    }
    {
      pname = "inkcat";
      npmDepsHash = "sha256-LrAnfBrsuDLTiKcJEws6+Amv91xMVjt+xFzDfDD5B5c=";
      description = "Display Catppuccin flavors in your terminal";
    }
    {
      pname = "contrast_test";
      npmDepsHash = "sha256-6tpPo7uNMVcSLUzcC7KZZmmaKWDWKkf6qWblY4qFrdU";
      description = "Test Catppuccin's flavors compliance with modern web contrast standards";
    }
    {
      pname = "palette_builder";
      npmDepsHash = "sha256-ynPXZycGJw9gF0dBmXBP0MqyMqYg64H7dDXi0E4fHzg=";
      description = "Export Catppuccin flavors in various formats";
    }
  ];

  rustPkgs = [
    {
      pname = "puccinier";
      description = "Generate the other Catppuccin flavors off a template file written in one of them";
    }
    rec {
      pname = "catwalk";
      description = "Generate a preview as a single composite screenshot for the four flavors";

      nativeBuildInputs = [pkgs.installShellFiles];

      postInstall = ''
        installShellCompletion \
          $releaseDir/build/${pname}-*/out/${pname}.{bash,fish} \
          --zsh $releaseDir/build/${pname}-*/out/_${pname}
      '';
    }
  ];
in
  builtins.listToAttrs (builtins.map ({...} @ args: {
      name = args.pname;
      value = mkNodePkg args;
    })
    nodePkgs)
  // builtins.listToAttrs (builtins.map ({...} @ args: {
      name = args.pname;
      value = mkRustPkg args;
    })
    rustPkgs)

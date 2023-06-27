{
  pkgs,
  system,
}: let
  mkNodePkg = {
    pname,
    version,
    description,
    npmDepsHash,
  }:
    pkgs.buildNpmPackage {
      inherit pname version npmDepsHash;
      src = ../${pname};
      dontNpmBuild = true;

      meta = with pkgs.lib; {
        inherit description;
        homepage = "https://github.com/catppuccin/toolbox/tree/main/${pname}";
        license = licenses.mit;
        maintainers = with maintainers; [rubyowo];
      };
    };

  mkRustPkg = {
    pname,
    version,
    description,
  }:
    pkgs.rustPlatform.buildRustPackage {
      inherit pname version;
      src = ../${pname};

      cargoLock = {
        lockFile = ../${pname}/Cargo.lock;
      };

      meta = with pkgs.lib; {
        inherit description;
        homepage = "https://github.com/catppuccin/toolbox/tree/main/${pname}";
        license = licenses.mit;
        maintainers = with maintainers; [rubyowo];
      };
    };

  nodePkgs = [
    {
      pname = "docpuccin";
      version = "0.1.3";
      npmDepsHash = "sha256-7/3wIis9c/P8zlQD3YbnRBPtpOcGDXchwcuC7/9fiWE=";
      description = "Fetch health files needed per project type";
    }
    {
      pname = "inkcat";
      version = "0.1.0";
      npmDepsHash = "sha256-LrAnfBrsuDLTiKcJEws6+Amv91xMVjt+xFzDfDD5B5c=";
      description = "Display Catppuccin flavours in your terminal";
    }
    {
      pname = "contrast_test";
      version = "0.1.0";
      npmDepsHash = "sha256-6tpPo7uNMVcSLUzcC7KZZmmaKWDWKkf6qWblY4qFrdU";
      description = "Test Catppuccin's flavours compliance with modern web contrast standards";
    }
    {
      pname = "palette_builder";
      version = "0.1.0";
      npmDepsHash = "sha256-ynPXZycGJw9gF0dBmXBP0MqyMqYg64H7dDXi0E4fHzg=";
      description = "Export Catppuccin flavours in various formats";
    }
  ];

  rustPkgs = [
    {
      pname = "puccinier";
      version = "0.1.2";
      description = "Generate the other Catppuccin flavours off a template file written in one of them";
    }
    {
      pname = "catwalk";
      version = "0.5.0";
      description = "Generate a preview as a single composite screenshot for the four flavours";
    }
  ];
in
  builtins.listToAttrs (builtins.map ({...} @ args: {
      name = args.pname;
      value = mkNodePkg args;
    }) nodePkgs) // builtins.listToAttrs (builtins.map ({...} @ args: {
      name = args.pname;
      value = mkRustPkg args;
    })
    rustPkgs)

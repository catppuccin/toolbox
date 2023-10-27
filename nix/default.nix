{
  pkgs,
  version ? "dirty",
}: let
  mkNodePkg = {
    pname,
    description,
    ...
  } @ args:
    pkgs.buildNpmPackage ({
        inherit pname version;
        src = pkgs.nix-gitignore.gitignoreSourcePure [../.gitignore] ../${pname};
        dontNpmBuild = true;
        prePatch = ''
          cp -r ${../package-lock.json} ./package-lock.json
        '';
        npmDepsHash = "sha256-mxrzw1Y3c9/XuZBIsg3X026pj/quWm3WWLtyvT2jY0Q=";

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
    membername ? pname,
    description,
    ...
  } @ args:
    pkgs.rustPlatform.buildRustPackage ({
        inherit pname version;
        src = pkgs.nix-gitignore.gitignoreSourcePure [../.gitignore] ../.;

        cargoLock.lockFile = ../Cargo.lock;
        cargoBuildFlags = "-p ${membername}";

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
      description = "Fetch health files needed per project type";
    }
    {
      pname = "inkcat";
      description = "Display Catppuccin flavors in your terminal";
    }
    {
      pname = "contrast_test";
      description = "Test Catppuccin's flavors compliance with modern web contrast standards";
    }
  ];

  rustPkgs = [
    {
      pname = "puccinier";
      description = "Generate the other Catppuccin flavors off a template file written in one of them";
    }
    rec {
      pname = "catwalk";
      membername = "catppuccin-catwalk";
      description = "Generate a preview as a single composite screenshot for the four flavors";

      nativeBuildInputs = [pkgs.installShellFiles];

      postInstall = ''
        installShellCompletion --cmd ${pname} \
          --bash <($out/bin/${pname} completion bash) \
          --fish <($out/bin/${pname} completion fish) \
          --zsh <($out/bin/${pname} completion zsh)
      '';
    }
    {
      pname = "whiskers";
      membername = "catppuccin-whiskers";
      description = "Soothing port creation tool for the high-spirited!";
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

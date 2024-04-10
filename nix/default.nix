{pkgs}: let
  inherit (pkgs) lib;
  inherit (builtins) readFile fromTOML;
  workspaceCargoToml = fromTOML (readFile ../Cargo.toml);

  mkRustPkg = {pname, ...} @ args: let
    memberCargoToml = fromTOML (readFile ../${pname}/Cargo.toml);
  in
    pkgs.rustPlatform.buildRustPackage (lib.recursiveUpdate {
        inherit pname;
        inherit (memberCargoToml.package) version;
        src = pkgs.nix-gitignore.gitignoreSource [] ../.;

        cargoLock.lockFile = ../Cargo.lock;
        buildAndTestSubdir = pname;

        meta = {
          description = memberCargoToml.package.description or null;
          homepage = "https://github.com/catppuccin/toolbox/tree/main/${pname}";
          license = lib.licenses.mit;
          mainProgram = pname;
        };
      }
      args);
  rustPkgOverrides = {
    catwalk = {
      nativeBuildInputs = with pkgs; [installShellFiles pkg-config];
      buildInputs = with pkgs; [libwebp];

      cargoLock.outputHashes = {
        "image-webp-0.1.1" = "sha256-WlGXjvkw6JL8OsS9IoM5Fpd4au8zp9jO/Z2iTXQE2Ko=";
        "ril-0.10.1" = "sha256-xVkuR8m6Q91Ii+bbCD9+foyka5a0vpJwTHBM8Hjjt4I=";
      };

      postInstall = ''
        installShellCompletion --cmd catwalk \
          --bash <($out/bin/catwalk completion bash) \
          --fish <($out/bin/catwalk completion fish) \
          --zsh <($out/bin/catwalk completion zsh)
      '';
    };
  };
  rustPkgs = lib.genAttrs workspaceCargoToml.workspace.members (pname: mkRustPkg ({inherit pname;} // rustPkgOverrides.${pname} or {}));
in
  rustPkgs

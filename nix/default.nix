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

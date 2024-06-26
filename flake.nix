{
  description = "teawie moment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
  in {
    checks = forAllSystems ({
      lib,
      pkgs,
      ...
    }: {
      actionlint = pkgs.runCommand "check-actionlint" {} ''
        ${lib.getExe pkgs.actionlint} ${./.github/workflows}/*
        touch $out
      '';

      deadnix = pkgs.runCommand "check-deadnix" {} ''
        ${lib.getExe pkgs.deadnix} --fail ${./.}
        touch $out
      '';

      editorconfig = pkgs.runCommand "check-editorconfig" {} ''
        cd ${./.}
        ${lib.getExe pkgs.editorconfig-checker} \
          -exclude '.git' .

        touch $out
      '';

      rustfmt = pkgs.runCommand "check-rustfmt" {nativeBuildInputs = [pkgs.cargo pkgs.rustfmt];} ''
        cd ${./.}
        cargo fmt -- --check
        touch $out
      '';

      statix = pkgs.runCommand "check-statix" {} ''
        ${lib.getExe pkgs.statix} check ${./.}
        touch $out
      '';
    });

    devShells = forAllSystems ({
      pkgs,
      system,
      ...
    }: {
      default = pkgs.mkShell {
        packages = [
          # rust tools
          pkgs.clippy
          pkgs.rustfmt
          pkgs.rust-analyzer

          # nix tools
          pkgs.deadnix
          pkgs.nil
          pkgs.statix

          # misc formatter/linters
          pkgs.actionlint
          self.formatter.${system}

          pkgs.redis
        ];

        inputsFrom = [self.packages.${system}.teawiebot];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };

      ci = pkgs.mkShell {
        packages = [
          pkgs.clippy
          pkgs.rustfmt

          self.formatter.${system}
        ];

        inputsFrom = [self.packages.${system}.teawiebot];
      };
    });

    formatter = forAllSystems (pkgs: pkgs.alejandra);

    nixosModules.default = import ./nix/module.nix self;

    packages = forAllSystems ({
      pkgs,
      system,
      ...
    }: let
      crossBuildsFor = arch: import ./nix/docker.nix {inherit pkgs arch inputs;};
    in
      {
        teawiebot = pkgs.callPackage ./nix/derivation.nix {inherit self;};

        default = self.packages.${system}.teawiebot;
      }
      // crossBuildsFor "x86_64"
      // crossBuildsFor "aarch64");

    overlays.default = _: prev: {
      teawiebot = prev.callPackage ./nix/derivation.nix {inherit self;};
    };
  };
}

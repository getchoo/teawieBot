{
  description = "teawie moment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    procfile-nix = {
      url = "github:getchoo/procfile-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    treefmt-nix,
    procfile-nix,
    ...
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
    treefmtFor = forAllSystems (pkgs: treefmt-nix.lib.evalModule pkgs ./treefmt.nix);
  in {
    checks = forAllSystems ({
      lib,
      pkgs,
      system,
      ...
    }: {
      actionlint = pkgs.runCommand "check-actionlint" {} ''
        ${lib.getExe pkgs.actionlint} ${./.github/workflows}/*
        touch $out
      '';

      nil = pkgs.runCommand "check-nil" {nativeBuildInputs = [pkgs.findutils pkgs.nil];} ''
        find ${./.} -type f -name '*.nix' | while read -r file
          nil diagnostics "$file"
        done
        touch $out
      '';

      statix = pkgs.runCommand "check-statix" {} ''
        ${lib.getExe pkgs.statix} check ${./.}
        touch $out
      '';

      treefmt = treefmtFor.${system}.config.build.check self;
    });

    devShells = forAllSystems ({
      lib,
      pkgs,
      system,
      ...
    }: let
      procfile = procfile-nix.lib.${system}.mkProcfileRunner {
        name = "daemons";

        procGroup = {
          redis = lib.getExe' pkgs.redis "redis-server";
        };
      };
    in {
      default = pkgs.mkShell {
        packages = with pkgs; [
          # rust tools
          clippy
          rustfmt
          rust-analyzer

          # misc formatter/linters
          actionlint
          self.formatter.${system}
          nil
          statix

          procfile
        ];

        inputsFrom = [self.packages.${system}.teawiebot];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };

      ci = pkgs.mkShell {
        packages = with pkgs; [
          clippy
          rustfmt
          self.formatter.${system}
        ];

        inputsFrom = [self.packages.${system}.teawiebot];
      };
    });

    formatter = forAllSystems ({system, ...}: treefmtFor.${system}.config.build.wrapper);

    nixosModules = {
      default = import ./nix/module.nix self;
    };

    packages = forAllSystems ({
      lib,
      pkgs,
      system,
      ...
    }: let
      crossTargets = with pkgs.pkgsCross; {
        x86_64 = musl64.pkgsStatic;
        aarch64 = aarch64-multiplatform.pkgsStatic;
      };

      rustStdFor = pkgs: fenix.packages.${system}.targets.${pkgs.stdenv.hostPlatform.rust.rustcTarget}.stable.rust-std;
      toolchain = with fenix.packages.${system};
        combine (lib.flatten [
          stable.cargo
          stable.rustc
          (map rustStdFor (lib.attrValues crossTargets))
        ]);

      rustPlatformFor = pkgs:
        pkgs.makeRustPlatform (
          lib.genAttrs ["cargo" "rustc"] (lib.const toolchain)
        );
      crossPlatforms = lib.mapAttrs (lib.const rustPlatformFor) crossTargets;

      buildTeawieWith = rustPlatform:
        self.packages.${system}.teawiebot.override {
          inherit rustPlatform;
          optimizeSize = true;
        };

      containerFor = arch:
        pkgs.dockerTools.buildLayeredImage {
          name = "teawiebot";
          tag = "latest-${arch}";
          contents = [pkgs.dockerTools.caCertificates];
          config.Cmd = [
            (lib.getExe self.packages.${system}."teawiebot-static-${arch}")
          ];

          architecture = nixpkgs.legacyPackages."${arch}-linux".pkgsStatic.go.GOARCH;
        };
    in {
      teawiebot = pkgs.callPackage ./nix/derivation.nix {inherit self;};
      default = self.packages.${system}.teawiebot;

      teawiebot-static-x86_64 = buildTeawieWith crossPlatforms.x86_64;
      teawiebot-static-aarch64 = buildTeawieWith crossPlatforms.aarch64;

      container-x86_64 = containerFor "x86_64";
      container-aarch64 = containerFor "aarch64";
    });

    overlays.default = _: prev: {
      teawiebot = prev.callPackage ./nix/derivation.nix {inherit self;};
    };
  };
}

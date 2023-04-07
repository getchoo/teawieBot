{
  description = "teawie moment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-compat.follows = "flake-compat";
      inputs.flake-utils.follows = "flake-utils";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
      inputs.flake-compat.follows = "flake-compat";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    fenix,
    pre-commit-hooks,
    ...
  }: let
    supportedSystems = with flake-utils.lib.system; [
      x86_64-linux
      x86_64-darwin
      aarch64-linux
      aarch64-darwin
    ];

    packageFn = craneLib: cargoArtifacts: pkgs: let
      inherit (pkgs.lib) licenses maintainers platforms;
      inherit (craneLib) buildPackage;
    in {
      teawiebot = buildPackage {
        src = ./.;
        inherit cargoArtifacts;

        meta = {
          description = "funni bot";
          homepage = "https://github.com/getchoo/teawiebot";
          license = licenses.mit;
          platforms = platforms.unix;
          maintainers = with maintainers; [getchoo];
        };
      };
    };
  in
    flake-utils.lib.eachSystem supportedSystems (system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
        };

        toolchain = with pkgs.fenix;
        with stable;
          combine [
            cargo
            rustc
            rustfmt
            clippy
            targets."x86_64-unknown-linux-musl".stable.rust-std
          ];

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        cargoArtifacts = craneLib.buildDepsOnly {
          src = ./.;
        };
      in {
        packages = let
          inherit (packageFn craneLib cargoArtifacts pkgs) teawiebot;

          teawiebot-smol =
            teawiebot.overrideAttrs (_: {
                # statically link musl, optimize for size
                CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
                CARGO_BUILD_RUSTFLAGS = "-C lto=fat -C embed-bitcode=yes \
									-C target-feature=+crt-static -C opt-level=z -C strip=symbols -C codegen-units=1";
                CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = let
                  inherit (pkgs.pkgsStatic.stdenv) cc;
                in "${cc}/bin/${cc.targetPrefix}cc";
              });
          cmd = "${teawiebot-smol}/bin/teawiebot";
        in
          {
            inherit teawiebot teawiebot-smol;
            container = let
              inherit (pkgs.dockerTools) buildLayeredImage caCertificates;
            in
              buildLayeredImage {
                name = "teawiebot";
                tag = "latest";
                contents = [caCertificates];
                config.Cmd = ["${cmd}"];
              };
            service = let
              inherit (pkgs) cacert portableService;
              service = pkgs.writeTextFile {
                name = "teawiebot.service";
                text = ''
                  [Unit]
                  Description=portable service for teawiebot

                  [Service]
                  DynamicUser=yes
                  ExecStart="${cmd}"

                  [Install]
                  WantedBy=multi-user.target
                '';
              };
            in
              portableService {
                inherit (teawiebot) pname;
                inherit (teawiebot-smol) version;
                description = "portable service for teawiebot!";
                units = [service];
                symlinks = [
                  {
                    object = "${cacert}/etc/ssl";
                    symlink = "/etc/ssl";
                  }
                ];
              };
          }
          // {default = self.packages.${system}.teawiebot;};

        checks = let
          commonArgs = {
            src = ./.;
          };

          inherit (craneLib) cargoClippy cargoFmt;
        in {
          inherit (self.packages.${system}) teawiebot;

          clippy = cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets";
            });

          fmt = cargoFmt commonArgs;

          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              actionlint.enable = true;
              alejandra.enable = true;
              deadnix.enable = true;
              statix.enable = true;
            };
          };
        };

        devShells = let
          inherit (pkgs) mkShell;
          inherit (self.checks.${system}.pre-commit-check) shellHook;
        in {
          default = mkShell {
            inherit shellHook;
            packages = with pkgs; [
              actionlint
              alejandra
              deadnix
              statix

              toolchain
            ];
          };
        };

        formatter = pkgs.alejandra;
      });
}

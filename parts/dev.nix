{
  perSystem = {
    lib,
    pkgs,
    system,
    config,
    ...
  }: {
    pre-commit.settings = {
      hooks = {
        actionlint.enable = true;
        ${config.formatter.pname}.enable = true;
        deadnix.enable = true;
        nil.enable = true;
        prettier.enable = true;
        rustfmt.enable = true;
        statix.enable = true;
      };
    };

    # a linkFarm of expected outputs for ci
    checks = {
      ciGate = let
        /*
        require self.checks for all systems
        require self.packages for x86_64-linux
        */
        required = builtins.concatMap builtins.attrValues (
          [(builtins.removeAttrs config.checks ["ciGate"])]
          ++ lib.optionals (system == "x86_64-linux") [(builtins.removeAttrs config.packages ["default"])]
        );

        paths =
          builtins.foldl'
          (
            acc: deriv: let
              name = deriv.pname or deriv.name;
              pathName =
                # if im not sure why `acc?name` doesn't work here
                if (builtins.elem name (builtins.attrNames acc))
                then "${name}-1"
                else name;
            in
              acc // {"${pathName}" = deriv.path or deriv.outPath;}
          )
          {}
          required;
      in
        pkgs.linkFarm "ci-gate" paths;
    };

    proc.groups.daemons.processes = {
      redis.command = lib.getExe' pkgs.redis "redis-server";
    };

    devShells = {
      default = pkgs.mkShell {
        packages = with pkgs; [
          # general
          actionlint
          nodePackages_latest.prettier
          config.proc.groups.daemons.package

          # rust
          cargo
          rustc
          clippy
          rustfmt
          rust-analyzer

          # nix
          config.formatter
          deadnix
          nil
          statix
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    };

    formatter = pkgs.alejandra;
  };
}

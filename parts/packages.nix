{
  self,
  inputs,
  ...
}: {
  perSystem = {
    pkgs,
    system,
    config,
    ...
  }: {
    packages = {
      teawiebot = pkgs.callPackage ./derivation.nix {
        inherit self;
        naersk = inputs.naersk.lib.${system};
      };

      default = config.packages.teawiebot;
    };
  };
}

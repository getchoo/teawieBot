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
        version = builtins.substring 0 8 self.lastModifiedDate or "dirty";
        naersk = inputs.naersk.lib.${system};
      };

      default = config.packages.teawiebot;
    };
  };
}

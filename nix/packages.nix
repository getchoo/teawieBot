{
  self,
  inputs,
  ...
}: {
  perSystem = {
    pkgs,
    system,
    self',
    ...
  }: {
    packages = {
      teawiebot = pkgs.callPackage ./derivation.nix {
        inherit self;
        naersk = inputs.naersk.lib.${system};
      };

      default = self'.packages.teawiebot;
    };
  };
}
